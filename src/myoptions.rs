pub struct MyOptions {
    pub port: String,
    pub logpath: String,
}

impl MyOptions {
    pub fn new(args: Vec<String>) -> MyOptions {
        let mut opt = MyOptions {
            port: String::from("7582"),
            logpath: String::new()
        };
        let mut old = String::new();
        for e in args {
            if e == "--port" {
                old = e;
            } else if e == "--log" {
                old = e;
            } else {
                if old == "--port" {
                    if e.parse::<u16>().is_ok() {
                        opt.port = e;
                    }
                } else if e == "--log" {
                    opt.logpath = e;
                }
                old = String::new();
            }
        }
        opt
    }
}
