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
    axum::Server::bind(&"0.0.0.0:7582".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler(Query(params): Query<Params>) -> (HeaderMap, Vec<u8>) {
    let svg = format!("<?xml version='1.0'?>\n\
    <svg width='300' height='100' viewBox='0 0 300 100' version='1.1' xmlns='http://www.w3.org/2000/svg' >\n\
    <style>\n\
    /* <![CDATA[ */\n\
    text {{font-size: 10px;}}\n\
    /* ]]> */\n\
    </style>\n\
    <g><text x=\"30\" y=\"30\" value=\"aaaaaaaaaa\">{:?}</text></g>\n\
    </svg>\n", params);

    let mut h = HeaderMap::new();
    h.insert(
        axum::http::header::CONTENT_TYPE, // HeaderName::from_static("Content-type:"),
        HeaderValue::from_static("image/svg+xml"),
    );
    (h, svg.clone().into_bytes())
}

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    sfen: Option<String>,
    sname: Option<String>,
    gname: Option<String>,
    title: Option<String>,
    lm: Option<String>,
    r#type: Option<String>,
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
