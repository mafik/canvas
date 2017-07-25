extern crate websocket;

use std;
use std::net::SocketAddr;
use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;


pub fn start() -> (SocketAddr, Sender<Action>) {

    return (addr, tx);
}
