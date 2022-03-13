use super::*;

#[derive(Debug)]
pub struct MyOptions {
    pub port: String,
    pub logpath: String,
    pub svg2png: svg2png::Type,
}

impl MyOptions {
    pub fn new(args: Vec<String>) -> MyOptions {
        let mut opt = MyOptions {
            port: String::from("7582"),
            logpath: String::new(),
            svg2png: svg2png::Type::RSVG,
        };
        let mut old = String::new();
        for e in args {
            if e == "--port" {
                old = e;
            } else if e == "--log" {
                old = e;
            } else if e == "--rsvg" {
                opt.svg2png = svg2png::Type::RSVG;
                old = String::new();
            } else if e == "--inkscape11" {
                opt.svg2png = svg2png::Type::INKSCAPE11;
                old = String::new();
            } else {
                if old == "--port" {
                    if e.parse::<u16>().is_ok() {
                        opt.port = e;
                    }
                } else if old == "--log" {
                    opt.logpath = e;
                }
                old = String::new();
            }
        }
        opt
    }
}
