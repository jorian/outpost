use std::sync::{mpsc, Arc};

use tracing::{debug, info};
use zmq::Socket;

use crate::ui::UI;

pub struct Controller {
    _data: Arc<()>,
    pub c_rx: mpsc::Receiver<ControllerMessage>,
    pub ui: UI,
}

impl Controller {
    pub fn new(_data: Arc<()>) -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();
        // Self::zmq_tx_notify(c_tx.clone());
        Self::zmq_block_notify(c_tx.clone());

        Controller {
            _data,
            c_rx,
            ui: UI::new(),
        }
    }

    pub fn start(&mut self) {
        self.ui.siv.set_autorefresh(false);

        self.ui
            .ui_tx
            .send(crate::ui::UIMessage::UpdateReserveOverview)
            .unwrap();

        while self.ui.step() {
            if let Some(message) = self.c_rx.try_iter().next() {
                match message {
                    ControllerMessage::NewBlock(blockhash) => {
                        info!("new block arrived: {}", blockhash);
                    }
                    ControllerMessage::NewTransaction(txid) => {
                        info!("new tx arrived: {}", txid);
                    }
                }
            }
        }
    }

    fn zmq_tx_notify(c_tx: mpsc::Sender<ControllerMessage>) {
        let zmq_port = 27779; // TODO something to set in a config

        let socket = Self::zmq_socket_setup(zmq_port);
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

    fn zmq_block_notify(c_tx: mpsc::Sender<ControllerMessage>) {
        let zmq_port = 27780; // TODO something to set in a config
        let socket = Self::zmq_socket_setup(zmq_port);

        let c_tx_clone = c_tx.clone();

        std::thread::spawn(move || loop {
            let data = socket.recv_multipart(0).unwrap();
            let block_hash = data[1]
                .iter()
                .map(|b| format!("{:02x}", *b))
                .collect::<Vec<_>>()
                .join("");

            debug!("new block: {}", &block_hash);

            c_tx_clone
                .send(ControllerMessage::NewBlock(block_hash))
                .unwrap();
        });
    }

    fn zmq_socket_setup(port: u16) -> Socket {
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
}

pub enum ControllerMessage {
    // NewBlock(blockhash)
    NewBlock(String),
    // NewTransaction(txid),
    NewTransaction(String),
}
