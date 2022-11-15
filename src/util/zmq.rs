use std::{
    sync::{mpsc, Arc},
    thread,
};

use tracing::{debug, error, info};
use zmq::{Context, Socket};

use crate::controller::ControllerMessage;

pub struct ZMQController {
    context: Context,
    c_tx: mpsc::Sender<ControllerMessage>,
}

impl ZMQController {
    pub fn new(c_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let context = zmq::Context::new();

        ZMQController { context, c_tx }
    }

    pub fn tx_listener(&mut self, port: u16) {
        let socket = self.context.socket(zmq::SUB).expect("a new zmq socket");
        let socket = socket;
        socket
            .connect(&format!("tcp://127.0.0.1:{}", port))
            .expect("a connection to the zmq socket");
        socket
            .set_subscribe(b"hash")
            .expect("failed subscribing to zmq");

        info!("ZMQ listening for transactions on port {}", port);

        let c_tx_clone = self.c_tx.clone();

        let handle = std::thread::spawn(move || loop {
            let data = socket.recv_multipart(0).unwrap();
            let hex = data[1]
                .iter()
                .map(|b| format!("{:02x}", *b))
                .collect::<Vec<_>>()
                .join("");

            debug!("new tx: {}", &hex);

            c_tx_clone
                .send(ControllerMessage::NewTransaction(hex))
                .unwrap();
        });
    }

    pub fn block_listener(&mut self, port: u16) {
        let socket = self.context.socket(zmq::SUB).expect("a new zmq socket");
        let socket = socket;
        socket
            .connect(&format!("tcp://127.0.0.1:{}", port))
            .expect("a connection to the zmq socket");
        socket
            .set_subscribe(b"hash")
            .expect("failed subscribing to zmq");

        info!("ZMQ listening for blocks on port {}", port);

        let c_tx_clone = self.c_tx.clone();

        let handle = std::thread::spawn(move || loop {
            let data = socket.recv_multipart(0).unwrap();
            let hex = data[1]
                .iter()
                .map(|b| format!("{:02x}", *b))
                .collect::<Vec<_>>()
                .join("");

            debug!("new block: {}", &hex);

            c_tx_clone.send(ControllerMessage::NewBlock(hex)).unwrap();
        });
    }
}
