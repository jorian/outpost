use std::sync::{mpsc, Arc};

use tracing::info;

pub struct Controller {
    _data: Arc<()>,
    pub c_rx: mpsc::Receiver<ControllerMessage>,
}

impl Controller {
    pub fn new(_data: Arc<()>) -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();
        Self::zmq_notify(c_tx);

        Controller { _data, c_rx }
    }

    pub fn start(&self) {
        info!("let's start this thing up!");
    }

    fn zmq_notify(c_tx: mpsc::Sender<ControllerMessage>) {
        let zmq_port = 27779; // TODO something to set in a config

        let zmq_context = zmq::Context::new();

        let socket = zmq_context.socket(zmq::SUB).expect("a new zmq socket");
        socket
            .connect(&format!("tcp://localhost:{}", zmq_port))
            .expect("a connection to the zmq socket");

        info!("ZMQ thread started...");
        let c_tx_clone = c_tx.clone();
        std::thread::spawn(move || loop {
            let data = socket.recv_multipart(0).unwrap();
            let tx_hex = data[1]
                .iter()
                .map(|b| format!("{:02x}", *b))
                .collect::<Vec<_>>()
                .join("");

            c_tx_clone
                .send(ControllerMessage::NewBlock(tx_hex))
                .unwrap();
        });
    }
}

pub enum ControllerMessage {
    // NewBlock(txid)
    NewBlock(String),
}
