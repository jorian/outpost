use std::sync::mpsc;

use tracing::{debug, error, info};
use zmq::Socket;

use crate::controller::ControllerMessage;

#[allow(unused)]
pub fn zmq_tx_notify(c_tx: mpsc::Sender<ControllerMessage>) {
    let zmq_port = 27779; // TODO something to set in a config

    let socket = zmq_socket_setup(zmq_port);
    let c_tx_clone = c_tx.clone();

    std::thread::spawn(move || loop {
        let data = socket.recv_multipart(0).unwrap();
        let tx_hex = data[1]
            .iter()
            .map(|b| format!("{:02x}", *b))
            .collect::<Vec<_>>()
            .join("");

        debug!("new tx: {}", &tx_hex);

        c_tx_clone
            .send(ControllerMessage::NewTransaction(tx_hex))
            .unwrap();
    });
}

pub fn zmq_block_notify(c_tx: mpsc::Sender<ControllerMessage>) {
    let zmq_port = 27780; // TODO something to set in a config
    let socket = zmq_socket_setup(zmq_port);

    let c_tx_clone = c_tx.clone();

    std::thread::spawn(move || loop {
        let data = socket.recv_multipart(0).unwrap();
        let block_hash = data[1]
            .iter()
            .map(|b| format!("{:02x}", *b))
            .collect::<Vec<_>>()
            .join("");

        debug!("new block: {}", &block_hash);

        if let Err(e) = c_tx_clone.send(ControllerMessage::NewBlock(block_hash)) {
            error!("NewBlock send error: {:?}", e)
        }
    });
}

pub fn zmq_socket_setup(port: u16) -> Socket {
    let zmq_context = zmq::Context::new();

    let socket = zmq_context.socket(zmq::SUB).expect("a new zmq socket");
    socket
        .connect(&format!("tcp://127.0.0.1:{}", port))
        .expect("a connection to the zmq socket");
    socket
        .set_subscribe(b"hash")
        .expect("failed subscribing to zmq");

    info!("ZMQ listening on port {}", port);

    socket
}
