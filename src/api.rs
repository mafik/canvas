use std::fmt;
use std::ascii::AsciiExt;
use std::sync::mpsc::Receiver;

macro_rules! lowercase_display {
    ($T:ty) => {
        impl fmt::Display for $T {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", format!("{:?}", self).to_ascii_lowercase())
            }
        }
    }
}

pub struct TextMetrics {
    pub width: f64,
}

pub enum Event {
    Resized(f64, f64),
    MouseMove(f64, f64),
    MouseDown(f64, f64, u32),
    MouseUp(f64, f64, u32),
    MouseWheel(f64, f64),
    KeyDown { code: String, key: String },
    KeyUp { code: String, key: String },
}

#[derive(Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}
lowercase_display!(LineCap);

pub enum LineJoin {
    Bevel,
    Round,
    Miter,
}

#[derive(Debug)]
pub enum TextAlignment {
    Start,
    End,
    Left,
    Right,
    Center,
}
lowercase_display!(TextAlignment);

#[derive(Debug)]
pub enum TextBaseline {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideagraphic,
    Bottom,
}
lowercase_display!(TextBaseline);

#[derive(Debug)]
pub enum TextDirection {
    Ltr,
    Rtl,
    Inherit,
}
lowercase_display!(TextDirection);

#[allow(non_snake_case)]
pub trait Canvas {
    fn clearRect(&mut self, x: f64, y: f64, width: f64, height: f64);
    fn fillRect(&mut self, x: f64, y: f64, width: f64, height: f64);
    fn strokeRect(&mut self, x: f64, y: f64, width: f64, height: f64);

    fn fillText(&mut self, text: &str, x: f64, y: f64);
    fn strokeText(&mut self, text: &str, x: f64, y: f64);
    fn measureText(&mut self, text: &str) -> TextMetrics;

    fn lineWidth(&mut self, width: f64);
    fn lineCap(&mut self, lineCap: LineCap);
    fn miterLimit(&mut self, limit: f64);

    fn setLineDash(&mut self, dash: &Vec<f64>);
    fn lineDashOffset(&mut self, offset: f64);

    fn font(&mut self, font: &str);
    fn textAlign(&mut self, align: TextAlignment);
    fn textBaseline(&mut self, baseline: TextBaseline);
    fn direction(&mut self, direction: TextDirection);

    fn fillStyle(&mut self, style: &str);
    fn strokeStyle(&mut self, style: &str);

    // TODO: Gradients
    // TODO: Shadows

    fn beginPath(&mut self);
    fn closePath(&mut self);
    fn moveTo(&mut self, x: f64, y: f64);
    fn lineTo(&mut self, x: f64, y: f64);
    fn bezierCurveTo(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);
    fn quadraticCurveTo(&mut self, cpx: f64, cpy: f64, x: f64, y: f64);
    fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        startAngle: f64,
        endAngle: f64,
        anticlockwise: bool,
    );
    fn arcTo(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64);
    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64);

    fn fill(&mut self);
    fn stroke(&mut self);
    fn clip(&mut self);

    // TODO: isPointIn{Path,Stroke}

    fn rotate(&mut self, alpha: f64);
    fn scale(&mut self, scale: f64);
    fn translate(&mut self, x: f64, y: f64);

    // TODO: Transforms
    // TODO: Compositing
    // TODO: Drawing Images
    // TODO: Pixel Manipulation

    fn save(&mut self);
    fn restore(&mut self);

    fn events(&self) -> &Receiver<Event>;
}
