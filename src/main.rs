use axum::{
    extract::Query,
    http::header::{HeaderMap, HeaderValue},
    routing::get,
    Router,
};
use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

mod sfen;
mod svg2png;
mod svgbuilder;

#[tokio::main]
async fn main() {
    println!("CTRL + c to quit.");
    axum::Server::bind(&"0.0.0.0:7582".parse().unwrap())
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
    axum::response::Html(
        "<html><head><title>help - sfenimageserver -</title></head>\
        <body><h1>sfenimageserver<h1>\
        <h2>options</h2>\
        <ul><li>sfen<br>sfen text. this must be given.<br>\
        ex. \"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1\"\
        <li>sname<br>sente's name.
        <li>gname<br>gote's name.
        <li>title<br>title.
        <li>turn<br>turn. b, w, fb, fw or d.
        <li>image<br>svg or png.
        </ul>
        </body></html>",
    )
}

async fn handler(Query(params): Query<Params>) -> (HeaderMap, Vec<u8>) {
    let result: String;
    if params.sfen.is_none() {
        result = format!("<?xml version='1.0'?>\n\
            <svg width='300' height='100' viewBox='0 0 300 100' version='1.1' xmlns='http://www.w3.org/2000/svg' >\n\
            <style>\n\
            /* <![CDATA[ */\n\
            text {{font-size: 10px;}}\n\
            /* ]]> */\n\
            </style>\n\
            <g><text x=\"30\" y=\"30\" value=\"aaaaaaaaaa\">{:?}</text></g>\n\
            </svg>\n", params);
    } else {
        let sfen = sfen::Sfen::new(&params.sfen.unwrap());
        let lm = if params.lm.is_some() {
            sfen::LastMove::read(&params.lm.unwrap()).unwrap_or(sfen::LastMove::new())
        } else {
            sfen::LastMove::new()
        };
        result = sfen
            .to_svg(lm.topos(), params.turn, params.sname, params.gname, params.title)
            .unwrap()
            .to_string();
    }
    let mut h = HeaderMap::new();
    h.insert(
        axum::http::header::CONTENT_TYPE, // HeaderName::from_static("Content-type:"),
        HeaderValue::from_static("image/svg+xml"),
    );
    (h, result.clone().into_bytes())
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
