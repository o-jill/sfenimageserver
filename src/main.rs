// #[macro_use]
// extern crate log;
// extern crate simplelog;

use axum::{
    extract::Query,
    http::header::{HeaderMap, HeaderValue},
    routing::get,
    Router,
};
use log::*;
use serde::{de, Deserialize, Deserializer};
use simplelog::*;
use std::{fmt, fs::File, str::FromStr};

mod myoptions;
mod sfen;
mod svg2png;
mod svgbuilder;

fn initlog(logpath: &str) {
    CombinedLogger::init(if logpath.is_empty() {
        vec![TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )]
    } else {
        vec![
            TermLogger::new(
                LevelFilter::Info,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create(logpath).unwrap(),
            ),
        ]
    })
    .unwrap();

    // debug!("start logging debug.");
    // info!("start logging info:");
    // warn!("warning! warning! warning!");
    // error!("some error happend!!");
}

static mo: once_cell::sync::OnceCell<myoptions::MyOptions> = once_cell::sync::OnceCell::new();

#[tokio::main]
async fn main() {
    mo.set(myoptions::MyOptions::new(std::env::args().collect()))
        .unwrap();
    info!("{:?}", mo.get().unwrap());

    initlog(&mo.get().unwrap().logpath);

    info!("CTRL + c to quit.");

    let portstr = format!("0.0.0.0:{}", mo.get().unwrap().port);
    info!("Listening to \"{}\" ...", portstr);
    axum::Server::bind(&portstr.parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/help", get(help))
}

async fn help() -> axum::response::Html<&'static str> {
    info!("call help()");
    axum::response::Html(
        "<html><head><title>help - sfenimageserver -</title></head>\
        <body><h1>sfenimageserver<h1>\
        <h2>options</h2>\
        <ul><li>sfen<br>sfen text. this must be given.<br>\
        ex. \"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1\"\
        <li>sname<br>sente's name.\
        <li>gname<br>gote's name.\
        <li>title<br>title.\
        <li>turn<br>turn. b, w, fb, fw or d.\
        <li>image<br>svg or png.\
        </ul>\
        <h2>example:</h2>\
        http://localhost:7582/?sfen=lnsg3nl%2F1k3s1r1%2Fppppppgpp%2F6p2%2F7P1%2F2P2PP2%2FPPBPP1N1P%2F3K2SR1%2FLNSG1G2L+w+b+20&lm=37&sname=o-jill&gname=%E3%81%A2%E3%82%8B&title=2022%2F03%2F04+12%3A46%3A30&turn=d&image=svg
        </body></html>",
    )
}

async fn handler(Query(params): Query<Params>) -> (HeaderMap, Vec<u8>) {
    let result: String;
    info!("call handler() : {:?}", params);
    if params.sfen.is_none() {
        let msg = "sfen is not specified...";
        let mut h = HeaderMap::new();
        h.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain"),
        );
        warn!("{}", msg);
        return (h, msg.into());
    } else {
        let sfen = sfen::Sfen::new(&params.sfen.unwrap());
        let lm = if params.lm.is_some() {
            match sfen::LastMove::read(&params.lm.unwrap()) {
                Ok(ret) => ret,
                Err(msg) => {
                    warn!("{}", msg);
                    sfen::LastMove::new()
                }
            }
        } else {
            sfen::LastMove::new()
        };
        match sfen.to_svg(
            lm.topos(),
            params.turn,
            params.sname,
            params.gname,
            params.title,
        ) {
            Ok(svg) => result = svg.to_string(),
            Err(msg) => {
                warn!("{}", msg);
                let mut h = HeaderMap::new();
                h.insert(
                    axum::http::header::CONTENT_TYPE,
                    HeaderValue::from_static("text/plain"),
                );
                return (h, msg.into_bytes());
            }
        }
    }

    let image = params.image.unwrap_or(String::from("svg"));
    if image == "png" {
        match svg2png::start(result.to_string(), mo.get().unwrap().svg2png) {
            Ok(png) => {
                let mut h = HeaderMap::new();
                h.insert(
                    axum::http::header::CONTENT_TYPE,
                    HeaderValue::from_static("image/png"),
                );
                (h, png)
            }
            Err(msg) => {
                warn!("{}", msg);
                let mut h = HeaderMap::new();
                h.insert(
                    axum::http::header::CONTENT_TYPE,
                    HeaderValue::from_static("text/plain"),
                );
                (h, msg.into_bytes())
            }
        }
    } else if image == "svg" {
        let mut h = HeaderMap::new();
        h.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("image/svg+xml"),
        );
        (h, result.clone().into_bytes())
    } else {
        let msg = format!("invalid image type. \"{}\"", image);
        let mut h = HeaderMap::new();
        h.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain"),
        );
        warn!("{}", msg);
        return (h, msg.into());
    }
}

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    sfen: Option<String>,
    sname: Option<String>,
    gname: Option<String>,
    title: Option<String>,
    lm: Option<String>,
    turn: Option<String>,
    image: Option<String>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
