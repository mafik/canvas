// TODO: use the right websocket port in the script
// TODO: implement text measurment
// TODO: write a bench test (ping-pong)

#[macro_use]
extern crate lazy_static;
extern crate websocket;
extern crate tokio_core;
extern crate futures;

mod server;
mod api;
mod json_sender;

use std::io;
use std::net::SocketAddr;

use futures::stream;
use tokio_core::reactor::Handle;
use websocket::async::futures::{Stream, BoxFuture, Future, Sink};

pub use api::*;

trait JsonSender {
    fn cleared(&mut self);
    fn send_json(&mut self, json: String);
    fn measure_text(&mut self, text: &str) -> BoxFuture<TextMetrics, ()>;
}

#[allow(non_snake_case)]
impl Canvas for WebCanvas {
    fn clearRect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.send_json(format!(r#"["clearRect",{},{},{},{}]"#, x, y, width, height));
        self.cleared();
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
    fn measureText(&mut self, text: &str) -> BoxFuture<TextMetrics, ()> {
        self.measure_text(text)
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
}

pub struct Config {
    pub http_addr: SocketAddr,
    pub websocket_addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http_addr: ([0, 0, 0, 0], 8080).into(),
            websocket_addr: ([0, 0, 0, 0], 8081).into(),
        }
    }
}

type Client = websocket::async::Client<websocket::async::TcpStream>;

pub struct WebCanvas {
    initial_commands: *mut Vec<String>,
    clients: *mut Vec<Client>,
    config: Config,
}

impl WebCanvas {
    pub fn new(config: Config) -> Self {
        server::start(config.http_addr);
        WebCanvas {
            initial_commands: Box::into_raw(Box::new(Vec::new())),
            clients: Box::into_raw(Box::new(Vec::new())),
            config,
        }
    }
    pub fn start(&mut self, handle: &Handle) -> Box<Future<Item = (), Error = ()> + 'static> {
        let server = websocket::async::Server::bind(self.config.websocket_addr, handle).unwrap();
        let initial_commands = self.initial_commands;
        let clients = self.clients;

        Box::new(
            server
                .incoming()
                .map_err(|websocket::server::InvalidConnection { error, .. }| {
                    println!("Error: {}", error)
                })
                .for_each(move |(upgrade, addr)| {
                    let desc = format!("Client {}", addr);
                    let desc2 = desc.clone();
                    upgrade
                        .accept()
                        .and_then(move |(mut client, _)| {
                            let message_stream = unsafe {
                                stream::iter((*initial_commands).iter().map(|command| {
                                    Ok(websocket::OwnedMessage::Text(command.clone())) as
                                        Result<_, io::Error>
                                }))
                            };
                            client.send_all(message_stream).map(
                                move |(client, _)| unsafe {
                                    (*clients).push(client);
                                },
                            )
                        })
                        .map_err(move |e| println!("{}: '{:?}'", desc, e))
                        .map(move |_| println!("{}: Finished.", desc2))
                }),
        )
    }
}

impl JsonSender for WebCanvas {
    fn cleared(&mut self) {
        unsafe {
            (*self.initial_commands).clear();
        }
    }
    fn send_json(&mut self, json: String) {
        unsafe {
            let clients = self.clients;
            for i in 0..(*clients).len() {
                (*clients)[i]
                    .start_send(websocket::OwnedMessage::Text(json.clone()))
                    .unwrap();
                (*clients)[i].poll_complete();
            }
            (*self.initial_commands).push(json);
        }
    }
    fn measure_text(&mut self, text: &str) -> BoxFuture<TextMetrics, ()> {
        let cmd = format!("[\"measureText\", \"{}\"]", text);
        unsafe {
            println!("Requesting measure text...");
            (*self.clients)[0]
                .start_send(websocket::OwnedMessage::Text(cmd))
                .unwrap();
            (*self.clients)[0].poll_complete();
            (*self.clients)[0]
                .by_ref()
                //.take(1)
                .map(|v| {
                    println!("Received: {:?}", v);
                })
                .poll();
        }
        futures::future::ok(TextMetrics { width: 10.0 }).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_core::reactor::Core;

    #[test]
    fn it_works() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let mut canvas = WebCanvas::new(Config::default());
        let future = canvas.start(&handle);

        let timeout = tokio_core::reactor::Timeout::new(std::time::Duration::new(5, 0), &handle)
            .unwrap()
            .then(|x| {
                canvas.clearRect(0., 0., 30., 30.);
                canvas.fillStyle("white");
                canvas.beginPath();
                canvas.rect(50., 100., 200., 100.);
                canvas.fill();
                canvas.textAlign(TextAlignment::Center);
                canvas.measureText("text");
                return x;
            })
            .map_err(|_| ());
        core.run(future.join(timeout)).unwrap();
    }
}
