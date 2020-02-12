use reqwest;
use seahorse::{color, SingleApp};
use serde_json::{self, Value};
use std::{
    env,
    io::{stdout, Write},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("$ {} [{}]", color::green("gtrans"), color::red("text"));
    let app = SingleApp::new()
        .name(color::green("gtrans"))
        .usage(usage)
        .version(color::yellow(env!("CARGO_PKG_VERSION")))
        .action(translate);

    app.run(args);
}

fn translate(v: Vec<String>) {
    let url = generate_url(v);
    let v = reqwest::blocking::get(&url)
        .and_then(|resp| resp.text())
        .and_then(|body| Ok(serde_json::from_str::<Vec<Value>>(&body)))
        .unwrap_or_else(|_| {
            eprintln!("{}", color::red("Network error..."));
            exit(1);
        })
        .unwrap_or_else(|_| {
            eprintln!("{}", color::red("JSON parse error..."));
            exit(1);
        });

    match v.first() {
        Some(item) => {
            let result = item
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s[0].as_str().unwrap())
                .collect::<Vec<&str>>()
                .join(" ");

            stdout()
                .lock()
                .write_all(format!("{}\n", result).as_bytes())
                .unwrap();
        }
        None => eprintln!("{}", color::red("Error...")),
    }
}

fn generate_url(v: Vec<String>) -> String {
    let base_url = "https://translate.googleapis.com/translate_a/single";
    let q = v.join(" ");
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
        base_url,
        "?client=gtx&ie=UTF-8&oe=UTF-8&dt=t",
        format!("{}{}", "&sl=", source),
        format!("{}{}", "&tl=", target),
        format!("&q={}", q)
    )
}
