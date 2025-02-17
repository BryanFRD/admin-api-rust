use quiche::Config;
use quiche::ConnectionId;
use quiche::RecvInfo;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub async fn start_webtransport() -> Result<(), Box<dyn Error>> {
    let server_addr: SocketAddr = "0.0.0.0:4433".parse().unwrap();

    // Configuration de Quiche
    let mut config = Config::new(quiche::PROTOCOL_VERSION).unwrap();
    config.set_application_protos(&[b"h3"]).unwrap();
    config.set_initial_max_data(10_000_000);
    config.set_initial_max_stream_data_bidi_local(1_000_000);
    config.set_initial_max_stream_data_bidi_remote(1_000_000);
    config.set_initial_max_streams_bidi(100);

    config
        .load_cert_chain_from_pem_file("localhost.pem")
        .unwrap();
    config
        .load_priv_key_from_pem_file("localhost-key.pem")
        .unwrap();
    config.verify_peer(false);

    // Création du socket UDP
    let socket = UdpSocket::bind(&server_addr).await.unwrap();
    println!("Serveur en écoute sur {}", server_addr);

    let mut connections: HashMap<ConnectionId, quiche::Connection> = HashMap::new();
    let mut buf = [0; 65535];

    loop {
        // Recevoir des données du socket
        let (read, from) = socket.recv_from(&mut buf).await?;
        println!("Reçu {} bytes de {}", read, from);

        // Identifier la connexion
        let scid = ConnectionId::from_ref(&buf[..quiche::MAX_CONN_ID_LEN].to_vec());

        // Accepter ou récupérer la connexion
        let conn = connections.entry(scid.clone()).or_insert_with(|| {
            quiche::accept(&scid, None, server_addr, from, &mut config).unwrap()
        });

        let recv_info = RecvInfo { from, to: server_addr };

        // Traiter les données reçues
        match conn.recv(&mut buf[..read], recv_info) {
            Ok(_) => {
                // Traiter les événements de la connexion
                process_connection(conn, &socket).await?;
            }
            Err(e) => {
                eprintln!("Erreur lors de la réception des données: {:?}", e);
                continue;
            }
        }
    }
}

async fn process_connection(conn: &mut quiche::Connection, socket: &UdpSocket) -> Result<(), Box<dyn Error>> {
    // Lire les données des flux
    for stream_id in conn.readable() {
        let mut buf = [0; 1024];
        match conn.stream_recv(stream_id, &mut buf) {
            Ok((len, _)) => {
                if len > 0 {
                    let message = String::from_utf8_lossy(&buf[..len]);
                    println!("Message reçu sur le flux {} : {}", stream_id, message);

                    // Répondre au client
                    let response = format!("Réponse du serveur : {}", message);
                    conn.stream_send(stream_id, response.as_bytes(), true)?;
                }
            }
            Err(quiche::Error::Done) => break,
            Err(e) => {
                eprintln!("Erreur lors de la lecture du flux : {:?}", e);
                break;
            }
        }
    }

    // Envoyer les données en attente
    let mut out_buf = [0; 65535];
    loop {
        let (len, send_info) = match conn.send(&mut out_buf) {
            Ok(v) => v,
            Err(quiche::Error::Done) => break,
            Err(e) => {
                eprintln!("Erreur lors de l'envoi des données: {:?}", e);
                break;
            }
        };

        socket.send_to(&out_buf[..len], send_info.to).await?;
    }

    Ok(())
}