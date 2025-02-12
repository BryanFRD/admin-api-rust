use socketioxide::{extract::SocketRef, layer::SocketIoLayer, SocketIo};

use crate::services::docker;

pub fn create_websocket_layer() -> SocketIoLayer {
    let (socket_layer, io) = SocketIo::new_layer();
    
    io.ns("/", |socket: SocketRef| async move {
        println!("Client connected : {}", socket.id);
        
        socket.on("docker:containers", |socket: SocketRef| async move {
            let containers = docker::get_containers().await.unwrap_or_else(|_| vec![]);
            
            let _ = socket.emit("docker:containers", &containers);
        });
        
        socket.on_disconnect(|socket: SocketRef| async move {
            println!("Client disconnected : {}", socket.id);
        });
    });
    
    socket_layer
}