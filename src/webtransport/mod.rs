use std::error::Error;
use axum::Json;
use tokio::sync::broadcast;
use wtransport::{endpoint::IncomingSession, Connection, Endpoint, Identity, ServerConfig};
use crate::datas::EventDTO;
use crate::events::docker::DockerEvent;
use crate::events::system::SystemEvent;
use crate::services::docker;
use crate::events::Event;

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
    
    log::info!("Listening on port 4433");
    
    let (tx, _rx) = broadcast::channel::<String>(100);
    
    tokio::spawn(docker::listen_docker_events(tx.clone()));
    
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
    
    let datagram_handle = tokio::spawn(handle_datagram(connection.clone(), tx.clone()));
    
    let bidirectional_handle = tokio::spawn(handle_bidirectionnal(connection, tx));
    
    let _ = tokio::join!(datagram_handle, bidirectional_handle);
    Ok(())
}

async fn handle_datagram(connection: Connection, tx: broadcast::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::info!("Accepted datagram connection from {:?}", connection.remote_address());
    let mut rx = tx.subscribe();
    
    loop {
        let datagram = match connection.receive_datagram().await {
            Ok(datagram) => datagram,
            Err(e) => {
                log::error!("Failed to receive datagram: {:?}", e);
                return Err(Box::new(e));
            }
        };
        let received_message = String::from_utf8_lossy(&datagram);
        log::info!("Received datagram message: {:?}", received_message);
        
        let event: Event = match serde_json::from_str(&received_message) {
            Ok(event) => event,
            Err(e) => {
                log::error!("Failed to parse event: {:?}", e);
                continue;
            }
        };
        
        match &event {
            Event::Docker(docker_event) => {
                match docker_event {
                    DockerEvent::DockerStatus => {
                        let event = "DockerStatus".to_string();
                        connection.send_datagram(event.as_bytes())?;
                    },
                    DockerEvent::DockerContainersRestart { data } => {
                        let containers = docker::get_containers().await?;
                        log::info!("Restarting container: {:?}", containers.to_json().as_bytes().len());
                        if let Err(error) = connection.send_datagram(containers.to_json()) {
                            log::error!("Failed to send event: {:?}", error);
                        }
                    }
                }
            },
            Event::System(system_event) => {
                match system_event {
                    SystemEvent::SystemStatus => {
                        let event = "SystemStatus".to_string();
                        connection.send_datagram(event.as_bytes())?;
                    }
                }
            },
        }
    }
}

async fn handle_bidirectionnal(connection: Connection, tx: broadcast::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::info!("Accepted bidirectional connection from {:?}", connection.remote_address());
    
    while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
        log::trace!("Accepted bidirectional stream");
        
        let mut rx = tx.subscribe();
        
        tokio::spawn(async move {
            while let Ok(event) = rx.recv().await {
                let _ = send_stream.write_all(event.as_bytes()).await;
            }
        });
    }
    
    Ok(())
}

fn handle_event(event: String, data: Json<String>) {
    log::info!("Received event: {:?}", event);
    log::info!("Received data: {:?}", data);
    
}