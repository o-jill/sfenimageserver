use super::*;

/// Options specified in command line args.
/// See 'options:' section in Readme.md.
#[derive(Debug)]
pub struct MyOptions {
    /// port number to listen.
    pub port: String,
    /// log path to put.
    pub logpath: String,
    /// svg2png type.
    pub svg2png: svg2png::Type,
    /// background color.
    pub bgcolor: String,
    /// foreground color.
    pub fgcolor: String,
}

impl MyOptions {
    /// instantiate MyOptions.
    ///
    /// # Arguments
    /// * `args` - args from command line.
    ///
    /// # Return value
    /// instance of MyOptions.<br>
    /// default:<br>
    /// - port: "7582"
    /// - logpath: ""
    /// - svg2png: svg2png::Type::RSVG
    /// - bgcolor: "white"
    /// - fgcolor: "black"
    pub fn new(args: Vec<String>) -> MyOptions {
        let mut opt = MyOptions {
            port: String::from("7582"),
            logpath: String::new(),
            svg2png: svg2png::Type::RSVG,
            bgcolor: String::from("white"),
            fgcolor: String::from("black"),
        };
        let mut old = String::new();
        for e in args {
            if e == "--port" {
                old = e;
            } else if e == "--log" {
                old = e;
            } else if e == "--dark" {
                opt.bgcolor = String::from("black");
                opt.fgcolor = String::from("white");
                old = String::new();
            } else if e == "--light" {
                opt.bgcolor = String::from("white");
                opt.fgcolor = String::from("black");
                old = String::new();
            } else if e == "--bgcolor" {
                old = e;
            } else if e == "--fgcolor" {
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
                } else if old == "--bgcolor" {
                    opt.bgcolor = e;
                } else if old == "--fgcolor" {
                    opt.fgcolor = e;
                }
                old = String::new();
            }
        }
        opt
    }
}
