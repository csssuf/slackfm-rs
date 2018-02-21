#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rustfm;
extern crate slack_api;

mod command;
mod lastfm;
mod slack;

use std::env;

use command::*;
use lastfm::*;
use slack::*;

fn main() {
    let slack_token = match env::var("SLACKFM_SLACK_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            println!("Couldn't get Slack token: {}", e);
            return;
        }
    };

    let slack = SlackClient::new(&slack_token);

    let lastfm_token = match env::var("SLACKFM_LASTFM_API_KEY") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("Couldn't get Last.fm API key: {}", e);
            return;
        }
    };

    let lastfm = LastfmClient::new(&lastfm_token);

    rocket::ignite()
        .manage(slack)
        .manage(lastfm)
        .mount("/", routes![command_np])
        .launch();
}
