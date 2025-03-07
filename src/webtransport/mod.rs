use std::error::Error;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use wtransport::{endpoint::IncomingSession, Connection, Endpoint, Identity, ServerConfig};
use crate::services;
use crate::events::Event;

pub mod system;
pub mod docker;

pub async fn start_webtransport() -> Result<(), Box<dyn Error + Send + Sync>> {
    let identity = match Identity::load_pemfiles("localhost.pem", "localhost-key.pem").await {
        Ok(identity) => identity,
        Err(e) => {
            log::error!("Failed to load identity: {:?}", e);
            return Err(Box::new(e));
        }
    };
    
    let config = ServerConfig::builder()
        .with_bind_default(4433)
        .with_identity(identity)
        .build();
    
    let server = match Endpoint::server(config) {
        Ok(server) => server,
        Err(e) => {
            log::error!("Failed to create server: {:?}", e);
            return Err(Box::new(e));
        }
    };
    
    let (tx, _rx) = broadcast::channel::<String>(100);
    
    //TODO handle errors
    tokio::spawn(services::docker::listen_docker_events(tx.clone()));
    
    loop {
        let incoming_session = server.accept().await;
        log::info!("Incoming session from {:?}", incoming_session.remote_address());
        
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(incoming_session, tx_clone).await {
                log::error!("Failed to handle connection: {:?}", e);
            }
        });
    }
}

async fn handle_connection(incoming_session: IncomingSession, tx: broadcast::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let request = match incoming_session.await {
        Ok(request) => request,
        Err(e) => {
            log::error!("Failed to accept incoming session: {:?}", e);
            return Err(Box::new(e));
        }
    };
    
    let connection = match request.accept().await {
        Ok(connection) => connection,
        Err(e) => {
            log::error!("Failed to accept incoming request: {:?}", e);
            return Err(Box::new(e));
        }
    };
    
    // let datagram_handle = tokio::spawn(handle_datagram(connection.clone(), tx.clone()));
    
    let _ = tokio::spawn(handle_bidirectionnal(connection, tx));
    
    // let _ = tokio::join!(datagram_handle, bidirectional_handle);
    Ok(())
}

// ! Problems with datagrams, they have a maximum size of 65507 bytes

// async fn handle_datagram(connection: Connection, tx: broadcast::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
//     log::info!("Accepted datagram connection from {:?}", connection.remote_address());
//     let mut rx = tx.subscribe();
    
//     loop {
//         let datagram = match connection.receive_datagram().await {
//             Ok(datagram) => datagram,
//             Err(e) => {
//                 log::error!("Failed to receive datagram: {:?}", e);
//                 return Err(Box::new(e));
//             }
//         };
//         let received_message = String::from_utf8_lossy(&datagram);
//         log::info!("Received datagram message: {:?}", received_message);
        
//         let event: Event = match serde_json::from_str(&received_message) {
//             Ok(event) => event,
//             Err(e) => {
//                 log::error!("Failed to parse event: {:?}", e);
//                 continue;
//             }
//         };
        
//         match &event {
//             Event::Docker(docker_event) => {
//                 match docker_event {
//                     DockerEvent::DockerStatus => {
//                         let event = "DockerStatus".to_string();
//                         connection.send_datagram(event.as_bytes())?;
//                     },
//                     DockerEvent::DockerContainersRestart { data } => {
//                         let containers = docker::get_containers().await?;
//                         let mut buffer = Vec::new();
//                         {
//                             let mut compressor = CompressorWriter::new(&mut buffer, 4096, 11, 22);
//                             compressor.write_all(containers.to_json().to_string().as_bytes())?;
//                         } 
//                         log::info!("Restarting container: {:?}:{:?} max size: {:?}", containers.to_json().to_string().as_bytes().len(), buffer.len(), connection.max_datagram_size());
//                         if let Err(error) = connection.send_datagram(buffer) {
//                             log::error!("Failed to send event: {:?}", error);
//                         }
//                     }
//                 }
//             },
//             Event::System(system_event) => {
//                 match system_event {
//                     SystemEvent::SystemStatus => {
//                         let event = "SystemStatus".to_string();
//                         connection.send_datagram(event.as_bytes())?;
//                     }
//                 }
//             },
//         }
//     }
// }

async fn handle_bidirectionnal(connection: Connection, tx: broadcast::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::info!("Accepted bidirectional connection from {:?}", connection.remote_address());
    
    while let Ok((send_stream, mut recv_stream)) = connection.accept_bi().await {
        log::trace!("Accepted bidirectional stream");
        
        let mut rx = tx.subscribe();
        
        let send_stream = Arc::new(Mutex::new(send_stream));
        let send_stream_clone = send_stream.clone();
        
        tokio::spawn(async move {
            while let Ok(event) = rx.recv().await {
                let mut stream = send_stream_clone.lock().await;
                let _ = stream.write_all(event.as_bytes()).await;
            }
        });
        
        tokio::spawn(async move {
           let mut buffer = [0; 1024];
           loop {
            match recv_stream.read(&mut buffer).await {
                Ok(Some(0)) => {
                    log::info!("Bidirectional connection closed");
                    break;
                },
                Ok(Some(n)) => {
                    let received_message = String::from_utf8_lossy(&buffer[..n]);
                    log::info!("Received bidirectional message: {:?}", received_message);
                    let mut stream = send_stream.lock().await;
                    handle_message(&mut *stream, received_message.to_string()).await;
                },
                Ok(None) => {
                    log::info!("No bidirectional data received");
                    break;
                },
                Err(e) => {
                    log::error!("Failed to read from stream: {:?}", e);
                    break;
                }
            }
           } 
        });
    }
    
    Ok(())
}

async fn handle_message(send_stream: &mut wtransport::SendStream, message: String) {
    log::info!("Received message: {:?}", message);
    let event: Event = match serde_json::from_str(&message) {
        Ok(event) => event,
        Err(e) => {
            log::error!("Failed to parse event: {:?}", e);
            return;
        }
    };
    
    match &event {
        Event::Docker(docker_event) => {
            docker::handle_message(send_stream, docker_event).await;
        },
        Event::System(system_event) => {
            system::handle_message(send_stream, system_event).await;
        },
    }
}