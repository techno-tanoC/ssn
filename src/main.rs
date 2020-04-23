use clap::{App, Arg, ArgMatches};
use serde_json::json;
use serde::Serialize;

fn matches() -> ArgMatches<'static> {
    App::new("ssn")
        .version("0.1.0")
        .about("ssn: Simple Slack Notifier")
        .arg(
            Arg::with_name("slack_url")
                .long("slack-url")
                .short("u")
                .env("SLACK_URL")
                .hide_env_values(true)
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("text")
                .long("text")
                .short("t")
                .env("TEXT")
                .hide_env_values(true)
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .required(false)
        )
        .get_matches()
}

fn build_json(text: impl AsRef<str>) -> impl Serialize {
    json!({
        "blocks": [{
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": text.as_ref()
            }
        }]
    })
}

fn main() {
    let matches = matches();
    let url = matches.value_of("slack_url").expect("slack url is not found");
    let text = matches.value_of("text").expect("text is not found");
    let verbose = matches.is_present("verbose");

    let json = build_json(&text);

    if verbose {
        let string = serde_json::to_string(&json).unwrap();
        println!("{}", string);
    }

    let res = reqwest::blocking::Client::new()
        .post(url)
        .json(&json)
        .send()
        .expect("Failed to send");

    if verbose {
        println!("{}", res.text().unwrap());
    }
}
