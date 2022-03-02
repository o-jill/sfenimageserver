use axum::{extract::Query, routing::get, Router};
use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

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

/*async fn handler(query: Query<collections::HashMap<String, String>>) -> String {
    format!("{:?}", query)
}*/

async fn handler(Query(params): Query<Params>) -> String {
    format!("{:?}", params)
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
