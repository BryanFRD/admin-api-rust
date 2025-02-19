use std::error::Error;
use wtransport::{endpoint::IncomingSession, Endpoint, Identity, ServerConfig};

pub async fn start_webtransport() -> Result<(), Box<dyn Error>> {
    let identity = match Identity::load_pemfiles("certs/localhost.crt", "certs/localhost.key").await {
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
    
    loop {
        let incoming_session = server.accept().await;
        log::info!("Incoming session from {:?}", incoming_session.remote_address());
        
        tokio::spawn(async move {
            if let Err(e) = handle_connection(incoming_session).await {
                log::error!("Failed to handle connection: {:?}", e);
            }
        });
    }
}

async fn handle_connection(incoming_session: IncomingSession) -> Result<(), Box<dyn Error>> {
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
    
    while let Ok((mut recv_stream, mut send_stream)) = connection.accept_bi().await {
        log::trace!("Accepted bidirectional stream");
        
        let mut buffer = vec![0; 1024];
    };
    
    Ok(())
}