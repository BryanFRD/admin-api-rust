use quiche::Config;
use tokio::net::UdpSocket;
use std::{error::Error, net::SocketAddr};

pub async fn start_webtransport() -> Result<(), Box<dyn Error>> {
    let server_addr: SocketAddr = "0.0.0.0:4433".parse().unwrap();
    
    
    let mut config = Config::new(quiche::PROTOCOL_VERSION).unwrap();
    
    config.set_application_protos(&[b"webtransport"]).unwrap();
    
    config
        .load_cert_chain_from_pem_file("localhost.pem")
        .unwrap();
    config
        .load_priv_key_from_pem_file("localhost-key.pem")
        .unwrap();
    config.verify_peer(false);

    let socket = UdpSocket::bind(&server_addr).await.unwrap();
    
    let mut buf = [0; 65535];
    
    loop {
        let (read, from) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes", read);
        
        let scid = quiche::ConnectionId::from_vec(vec![0; quiche::MAX_CONN_ID_LEN]);
        
        let mut conn = match quiche::accept(&scid, None, server_addr, from, &mut config) {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Failed to accept connection: {:?}", e);
                continue;
            }
        };
        
        println!("New connection: {:?}", read);
        
        let recv_info = quiche::RecvInfo { from, to: server_addr };
        
        if let Err(e) = conn.recv(&mut buf[..read], recv_info) {
            eprintln!("Failed to parse packet: {:?}", e);
            continue;
        }
        
        let mut stream_buf = [0; 1024];
        while let Ok((stream_id, _)) = conn.stream_recv(0, &mut stream_buf) {
            let message = String::from_utf8_lossy(&stream_buf[..read]);
            println!("Received message on stream {}: {}", stream_id, message);
            
            let response = b"Hello, World!";
            conn.stream_send(stream_id.try_into().unwrap(), response, true).unwrap();
        }
    }
}