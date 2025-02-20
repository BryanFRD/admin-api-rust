use std::error::Error;
use wtransport::{config::TlsClientConfig, endpoint::IncomingSession, ClientConfig, Endpoint, Identity, ServerConfig};

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
    
    while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
        log::trace!("Accepted bidirectional stream");
        
        let mut buffer = vec![0; 1024];
        let bytes_read = match recv_stream.read(&mut buffer).await {
            Ok(Some(bytes_read)) => bytes_read,
            Ok(None) => {
                log::info!("Stream closed");
                break;
            }
            Err(e) => {
                log::error!("Failed to read from stream: {:?}", e);
                break;
            }
        };
        
        log::info!("Received data! {:?}", &buffer[..bytes_read]);
        
        match send_stream.write_all(&buffer[..bytes_read]).await {
            Ok(_) => {},
            Err(e) => {
                log::error!("Failed to write to stream: {:?}", e);
                break;
            }
        };
    };
    
    Ok(())
}

pub async fn start_client() -> Result<(), Box<dyn Error>> {
    let config = ClientConfig::default();
    
    let client = match Endpoint::client(config) {
        Ok(client) => client,
        Err(e) => {
            log::error!("Failed to create client: {:?}", e);
            return Err(Box::new(e));
        }
    };
    
    let connection = match client.connect("https://localhost:4433").await {
        Ok(connection) => connection,
        Err(e) => {
            log::error!("Failed to connect to server: {:?}", e);
            return Err(Box::new(e));
        }
    };
    
    log::info!("Connected to server at {:?}", connection.remote_address());
    
    let (mut send_stream, mut recv_stream) = connection.open_bi().await.unwrap().await.unwrap();
    
    let message = b"Hello, world!";
    send_stream.write_all(message).await?;
    log::info!("Sent message: {:?}", message);
    
    let mut buffer = vec![0; 1024];
    let bytes_read = recv_stream.read(&mut buffer).await.unwrap().unwrap();
    log::info!("Received data: {:?}", &buffer[..bytes_read]);
    
    Ok(())
}