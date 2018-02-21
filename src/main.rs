#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate slack_api;

mod command;
mod slack;

use std::env;

use command::*;
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

    rocket::ignite()
        .manage(slack)
        .mount("/", routes![command_np])
        .launch();
}

