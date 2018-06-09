#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate percent_encoding;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate rspotify;
extern crate rustfm;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slack_api;

mod command;
mod db;
mod health;
mod lastfm;
mod oauth;
mod slack;
mod spotify;

use std::env;

use failure::Error;

use command::*;
use db::*;
use health::*;
use lastfm::*;
use oauth::*;
use slack::*;
use spotify::*;

fn main() -> Result<(), Error> {
    let slack_client_id = env::var("SLACKFM_SLACK_CLIENT_ID")?;
    let slack_client_secret = env::var("SLACKFM_SLACK_CLIENT_SECRET")?;
    let slack = SlackClient::new(&slack_client_id, &slack_client_secret)?;

    let lastfm_token = env::var("SLACKFM_LASTFM_API_KEY")?;
    let lastfm = LastfmClient::new(&lastfm_token);

    let spotify_client_id = env::var("SLACKFM_SPOTIFY_CLIENT_ID")?;
    let spotify_client_secret = env::var("SLACKFM_SPOTIFY_CLIENT_SECRET")?;
    let spotify = SpotifyClient::new(&spotify_client_id, &spotify_client_secret);

    let pool = init_pool()?;

    rocket::ignite()
        .manage(slack)
        .manage(lastfm)
        .manage(spotify)
        .manage(pool)
        .mount("/", routes![route_np, oauth_route, health_check])
        .launch();

    Ok(())
}
