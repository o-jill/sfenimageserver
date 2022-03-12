pub struct MyOptions {
    pub port: String,
}

impl MyOptions {
    pub fn new(args: Vec<String>) -> MyOptions {
        let mut opt = MyOptions {
            port: String::from("7582"),
        };
        let mut old = String::new();
        for e in args {
            if e == "--port" {
                old = e;
            } else {
                if old == "--port" {
                    if e.parse::<u16>().is_ok() {
                        opt.port = e;
                    }
                    old = String::new();
                }
            }
        }
        opt
    }
}
