use reqwest;
use seahorse::{color, SingleApp};
use serde_json::{self, Value};
use std::env;

const BASE_URL: &str = "https://translate.googleapis.com/translate_a/single";

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("{} [text]", color::red("gtrans"));
    let app = SingleApp::new()
        .name(color::red("gtrans"))
        .usage(usage)
        .version(color::yellow(env!("CARGO_PKG_VERSION")))
        .action(translate);

    app.run(args);
}

fn translate(v: Vec<String>) {
    let url = generate_url(v);
    let resp = reqwest::blocking::get(&url).unwrap();
    let body = resp.text().unwrap();
    let v: Vec<Value> = serde_json::from_str(&body).unwrap();
    for i in v[0].as_array().unwrap() {
        println!("{}", i[0].as_str().unwrap());
    }
}

fn generate_url(v: Vec<String>) -> String {
    let q = v.join("");
    let source = match env::var("GTRANS_SOURCE") {
        Ok(sl) => sl,
        Err(_) => "ja".to_string(),
    };

    let target = match env::var("GTRANS_TARGET") {
        Ok(tl) => tl,
        Err(_) => "en".to_string(),
    };

    format!(
        "{}{}{}{}{}",
        BASE_URL,
        "?client=gtx&ie=UTF-8&oe=UTF-8&dt=t",
        format!("{}{}", "&sl=", source),
        format!("{}{}", "&tl=", target),
        format!("&q={}", q)
    )
}
