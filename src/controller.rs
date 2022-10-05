use std::sync::{mpsc, Arc};

use tracing::{debug, info};
use zmq::Socket;

use crate::ui::UI;

pub struct Controller {
    _data: Arc<()>,
    pub c_rx: mpsc::Receiver<ControllerMessage>,
    pub ui: UI,
    pub client: vrsc_rpc::Client,
}

impl Controller {
    pub fn new(_data: Arc<()>) -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();
        // Self::zmq_tx_notify(c_tx.clone());
        Self::zmq_block_notify(c_tx.clone());

        let client = vrsc_rpc::Client::chain("vrsctest", vrsc_rpc::Auth::ConfigFile, None)
            .expect("a verus daemon");

        Controller {
            _data,
            c_rx,
            ui: UI::new(),
            client,
        }
    }

    pub fn start(&mut self) {
        self.ui.siv.set_autorefresh(false);

        while self.ui.step() {
            if let Some(message) = self.c_rx.try_iter().next() {
                match message {
                    ControllerMessage::NewBlock(blockhash) => {
                        info!("new block arrived: {}", blockhash);

                        // need to get some data from the UI.
                        // create me own view with data i can query using `self.ui.siv.call_on()`
                        // or trigger a controllermessage when selecting a new currency and update it here.
                        // self.client.get_currency_converters();

                        // how do i know that a specific basket was selected?
                        // - can we have multiple baskets at the same time? why not? (maybe v2)

                        // at this point, i need to start querying the daemon for
                        // the latest currency state (getcurrencystate 'currency'), based on
                        // - some selected currency or currencies that exist in the reserve.
                        //   where should i store selected currencies? it's the resulting list of currencies for `getcurrencyconverters`
                        // - a specific basket itself
                        // and then i need to send a message to the UI thread to update the view (table).
                        // the message needs to contain:
                        // - amount of x in reserve
                        // - price in reserve, relative to the basket currency

                        // - update a log view (v2)
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
