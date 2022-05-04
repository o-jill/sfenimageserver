use std::io::Write;

/// Converter type.
#[derive(Clone, Copy, Debug)]
pub enum Type {
    /// use rsvg-converter.
    RSVG,
    /// use inkscape v1.1.
    INKSCAPE11,
}

/// config for converter.
pub struct Svg2PngConfig {
    /// Converter type.
    pub typ: Type,
    /// background color.
    pub bgcolor: String,
}

impl Svg2PngConfig {
    /// Returns Svg2PngConfig{Type::RSVG, "white"}.
    pub fn new() -> Svg2PngConfig {
        Svg2PngConfig {
            typ: Type::RSVG,
            bgcolor: String::from("white"),
        }
    }
}

/// start converting.
/// 
/// # Arguments
/// * `svg` - svg image text.
/// * `opt` - converter config.
/// # Return value
/// PNG data or error message.
pub fn start(svg: String, opt: Svg2PngConfig) -> Result<Vec<u8>, String> {
    match opt.typ {
        Type::RSVG => start_rsvg(svg, opt),
        Type::INKSCAPE11 => start_inkscape(svg, opt),
    }
}

/// convert w/ rsvg-convert version 2.50
/// 
/// # Arguments
/// * `svg` - svg image text.
/// * `opt` - converter config.
/// # Return value
/// PNG data or error message.
#[allow(dead_code)]
pub fn start_rsvg(svg: String, opt: Svg2PngConfig) -> Result<Vec<u8>, String> {
    let mut cmd = match std::process::Command::new("rsvg-convert")
        .arg("--format=png")
        .arg("-b")
        .arg(&opt.bgcolor)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Err(msg) => return Err(format!("error running png converter... [{}]", msg)),
        Ok(prcs) => prcs,
    };

    match cmd.stdin.as_mut() {
        None => {
            return Err(String::from("child_stdin is None."));
        }
        Some(child_stdin) => {
            match child_stdin.write_all(&svg.as_bytes()) {
                Ok(_) => {
                    let w = cmd.wait_with_output().unwrap();
                    // println!("{} bytes.", w.stdout.len());
                    Ok(w.stdout)
                }
                Err(msg) => Err(format!("error running png converter... [{}]", msg)),
            }
        }
    }
}

/// convert w/ inkscape version 1.1
///
/// # Arguments
/// * `svg` - svg image text.
/// * `opt` - converter config.
/// # Return value
/// PNG data or error message.
#[allow(dead_code)]
pub fn start_inkscape(svg: String, opt: Svg2PngConfig) -> Result<Vec<u8>, String> {
    let mut cmd = match std::process::Command::new("inkscape")
        .arg("--pipe")
        .arg("--export-filename=-")
        .arg("--export-type=png")
        .arg("-b")
        .arg(&opt.bgcolor)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Err(msg) => return Err(format!("error running png converter... [{}]", msg)),
        Ok(prcs) => prcs,
    };

    match cmd.stdin.take().unwrap().write_all(&svg.as_bytes()) {
        Ok(_) => {
            let w = cmd.wait_with_output().unwrap();
            // println!("{} bytes.", w.stdout.len());
            Ok(w.stdout)
        }
        Err(msg) => Err(format!("error running png converter... [{}]", msg)),
    }
}
