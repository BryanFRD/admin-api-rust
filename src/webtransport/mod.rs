use std::{error::Error, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use wtransport::{endpoint::IncomingSession, Endpoint, Identity, ServerConfig};
use crate::services::docker;

pub async fn start_webtransport() -> Result<(), Box<dyn Error>> {
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

async fn handle_connection(incoming_session: IncomingSession, tx: broadcast::Sender<String>) -> Result<(), Box<dyn Error>> {
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
    
    log::info!("Accepted connection from {:?}", connection.remote_address());
    
    
    while let Ok((send_stream, mut recv_stream)) = connection.accept_bi().await {
        log::trace!("Accepted bidirectional stream");
        
        let mut rx = tx.subscribe();
        let send_stream = Arc::new(Mutex::new(send_stream));
        
        {
            let send_stream = Arc::clone(&send_stream);
            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                
                while let Ok(Some(bytes_read)) = recv_stream.read(&mut buffer).await {
                    let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    log::info!("Received message: {:?}", received_message);
                    
                    let mut send_stream = send_stream.lock().await;
                    let _ = send_stream.write_all(b"Hello from server!").await;
                    //TODO: Send response
                }
            });
        }
        
        {
            let send_stream = Arc::clone(&send_stream);
            tokio::spawn(async move {
                while let Ok(event) = rx.recv().await {
                    let mut send_stream = send_stream.lock().await;
                    let _ = send_stream.write_all(event.as_bytes()).await;
                }
            });
        }
    }
    
    Ok(())
}