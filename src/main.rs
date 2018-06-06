#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate rustfm;
extern crate slack_api;

mod command;
mod db;
mod lastfm;
mod slack;

use std::env;

use command::*;
use db::*;
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

    let pool = match init_pool() {
        Ok(pool) => pool,
        Err(e) => {
            println!("Couldn't init_pool(): {}", e);
            return;
        }
    };

    rocket::ignite()
        .manage(slack)
        .manage(lastfm)
        .manage(pool)
        .mount("/", routes![command_np])
        .launch();
}
