// TODO:

extern crate websocket;
extern crate tokio_core;

mod http_server;
mod api;

use std::net::SocketAddr;
use std::iter::Iterator;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

pub use api::*;

#[allow(non_snake_case)]
impl Canvas for WebCanvas {
    fn clearRect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.send_json(format!(r#"["clearRect",{},{},{},{}]"#, x, y, width, height));
    }
    fn fillRect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.send_json(format!(r#"["fillRect",{},{},{},{}]"#, x, y, width, height));
    }
    fn strokeRect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.send_json(format!(
            r#"["strokeRect",{},{},{},{}]"#,
            x,
            y,
            width,
            height
        ));
    }

    fn fillText(&mut self, text: &str, x: f64, y: f64) {
        self.send_json(format!(r#"["fillText","{}",{},{}]"#, text, x, y));
    }
    fn strokeText(&mut self, text: &str, x: f64, y: f64) {
        self.send_json(format!(r#"["strokeText","{}",{},{}]"#, text, x, y));
    }
    fn measureText(&mut self, text: &str) -> TextMetrics {
        self.send_json(format!("[\"measureText\", \"{}\"]", text));
        let text = self.receive_json();
        return TextMetrics { width: text.parse().unwrap() };
    }

    fn lineWidth(&mut self, width: f64) {
        self.send_json(format!(r#"["lineWidth",{}]"#, width));
    }
    fn lineCap(&mut self, lineCap: LineCap) {
        self.send_json(format!(r#"["lineCap","{}"]"#, lineCap));
    }
    fn miterLimit(&mut self, limit: f64) {
        self.send_json(format!(r#"["miterLimit",{}]"#, limit));
    }

    fn setLineDash(&mut self, dash: &Vec<f64>) {
        let v: Vec<String> = dash.iter().map(|x| x.to_string()).collect();
        self.send_json(format!(r#"["setLineDash",[{}]]"#, v.join(",")));
    }
    fn lineDashOffset(&mut self, offset: f64) {
        self.send_json(format!(r#"["lineDashOffset",{}]"#, offset));
    }

    fn font(&mut self, font: &str) {
        self.send_json(format!(r#"["font","{}"]"#, font));
    }
    fn textAlign(&mut self, align: TextAlignment) {
        self.send_json(format!(r#"["textAlign","{}"]"#, align));
    }
    fn textBaseline(&mut self, baseline: TextBaseline) {
        self.send_json(format!(r#"["textBaseline","{}"]"#, baseline));
    }
    fn direction(&mut self, direction: TextDirection) {
        self.send_json(format!(r#"["direction","{}"]"#, direction));
    }

    fn fillStyle(&mut self, style: &str) {
        self.send_json(format!(r#"["fillStyle","{}"]"#, style));
    }
    fn strokeStyle(&mut self, style: &str) {
        self.send_json(format!(r#"["strokeStyle","{}"]"#, style));
    }

    // TODO: Gradients
    // TODO: Shadows

    fn beginPath(&mut self) {
        self.send_json(String::from("[\"beginPath\"]"));
    }
    fn closePath(&mut self) {
        self.send_json(String::from("[\"closePath\"]"));
    }
    fn moveTo(&mut self, x: f64, y: f64) {
        self.send_json(format!(r#"["moveTo",{},{}]"#, x, y));
    }
    fn lineTo(&mut self, x: f64, y: f64) {
        self.send_json(format!(r#"["lineTo",{},{}]"#, x, y));
    }
    fn bezierCurveTo(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.send_json(format!(
            r#"["bezierCurveTo",{},{},{},{},{},{}]"#,
            cp1x,
            cp1y,
            cp2x,
            cp2y,
            x,
            y
        ));
    }
    fn quadraticCurveTo(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.send_json(format!(
            r#"["quadraticCurveTo",{},{},{},{}]"#,
            cpx,
            cpy,
            x,
            y
        ));
    }
    fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        startAngle: f64,
        endAngle: f64,
        anticlockwise: bool,
    ) {
        self.send_json(format!(
            r#"["arc",{},{},{},{},{},{}]"#,
            x,
            y,
            radius,
            startAngle,
            endAngle,
            anticlockwise
        ));
    }
    fn arcTo(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        self.send_json(format!(
            r#"["arcTo",{},{},{},{},{}]"#,
            x1,
            y1,
            x2,
            y2,
            radius
        ));
    }
    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.send_json(format!(r#"["rect",{},{},{},{}]"#, x, y, width, height));
    }

    fn fill(&mut self) {
        self.send_json(String::from("[\"fill\"]"));
    }
    fn stroke(&mut self) {
        self.send_json(String::from("[\"stroke\"]"));
    }
    fn clip(&mut self) {
        self.send_json(String::from("[\"clip\"]"));
    }

    // TODO: isPointIn{Path,Stroke}

    fn rotate(&mut self, alpha: f64) {
        self.send_json(format!(r#"["rotate",{}]"#, alpha));
    }
    fn scale(&mut self, scale: f64) {
        self.send_json(format!(r#"["scale",{}]"#, scale));
    }
    fn translate(&mut self, x: f64, y: f64) {
        self.send_json(format!(r#"["translate",{},{}]"#, x, y));
    }

    // TODO: Transforms
    // TODO: Compositing
    // TODO: Drawing Images
    // TODO: Pixel Manipulation

    fn save(&mut self) {
        self.send_json(String::from("[\"save\"]"));
    }
    fn restore(&mut self) {
        self.send_json(String::from("[\"restore\"]"));
    }

    fn events(&self) -> &Receiver<Event> {
        &self.rx
    }
}

pub struct WebCanvas {
    client: WsClient,
    rx: Receiver<Event>,
}

const HEADER: &'static str = include_str!("static/index.html");
const SCRIPT: &'static str = include_str!("static/script.js");
type WsServer = websocket::server::WsServer<
    websocket::server::NoTlsAcceptor,
    std::net::TcpListener,
>;
type WsClient = websocket::sync::Client<std::net::TcpStream>;

fn start_ws_server() -> (WsServer, SocketAddr) {
    let server = Server::bind("0.0.0.0:0").unwrap();
    let addr = server.local_addr().unwrap();
    return (server, addr);
}

fn accept_one(mut server: WsServer) -> (WsClient, SocketAddr) {
    let request = server.accept().ok().unwrap();
    let client = request.accept().unwrap();
    let ip = client.peer_addr().unwrap();
    return (client, ip);
}

fn make_html(sync_addr: SocketAddr, async_addr: SocketAddr) -> String {
    format!(
        "{}<script>var action_port={},event_port={};{}</script>",
        HEADER,
        sync_addr.port(),
        async_addr.port(),
        SCRIPT
    )
}

impl WebCanvas {
    pub fn start(http_addr: SocketAddr) -> WebCanvas {
        let (sync_server, sync_addr) = start_ws_server();
        let (async_server, async_addr) = start_ws_server();

        let html = make_html(sync_addr, async_addr);
        http_server::start(http_addr, html);

        let (client, ip) = accept_one(sync_server);
        println!("Connection from {}", ip);

        let (mut aclient, _) = accept_one(async_server);
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || for message in aclient.incoming_messages() {
            match message {
                Ok(OwnedMessage::Text(text)) => {
                    println!("Received event: {}", text);
                    let mut toks = text.split(' ');
                    let command = toks.next().unwrap();
                    let event = match command {
                        "onmousedown" => Some(Event::MouseDown(
                            toks.next().unwrap().parse().unwrap(),
                            toks.next().unwrap().parse().unwrap(),
                            toks.next().unwrap().parse().unwrap(),
                        )),
                        "onmouseup" => Some(Event::MouseUp(
                            toks.next().unwrap().parse().unwrap(),
                            toks.next().unwrap().parse().unwrap(),
                            toks.next().unwrap().parse().unwrap(),
                        )),
                        "onmousemove" => Some(Event::MouseMove(
                            toks.next().unwrap().parse().unwrap(),
                            toks.next().unwrap().parse().unwrap(),
                        )),
                        "onwheel" => Some(Event::MouseWheel(
                            toks.next().unwrap().parse().unwrap(),
                            toks.next().unwrap().parse().unwrap(),
                        )),
                        "onkeydown" => Some(Event::KeyDown {
                            code: toks.next().unwrap().to_string(),
                            key: toks.next().unwrap().to_string(),
                        }),
                        "onkeyup" => Some(Event::KeyUp {
                            code: toks.next().unwrap().to_string(),
                            key: toks.next().unwrap().to_string(),
                        }),
                        _ => None,
                    };
                    match event {
                        Some(event) => tx.send(event).unwrap(),
                        _ => (),
                    };
                }
                _ => (),
            }
        });

        WebCanvas { client, rx }
    }
    fn send_json(&mut self, json: String) {
        let start = std::time::Instant::now();
        self.client.send_message(&OwnedMessage::Text(json)).unwrap();
        let end = std::time::Instant::now();
        let dur = end.duration_since(start);
        println!("Send took {:?}", dur);
    }
    fn receive_json(&mut self) -> String {
        self.client.set_nodelay(true).unwrap();
        let start = std::time::Instant::now();
        let msg = match self.client.recv_message() {
            Ok(OwnedMessage::Text(text)) => text,
            _ => "".to_string(),
        };
        let end = std::time::Instant::now();
        let dur = end.duration_since(start);
        println!("Receive took {:?}", dur);
        self.client.set_nodelay(false).unwrap();
        return msg;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
