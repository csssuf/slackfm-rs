use rocket::State;
use rocket::request::LenientForm;

use slack::*;

#[derive(FromForm)]
struct CommandRequest {
    token: String,
    team_id: String,
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    text: Option<String>,
    response_url: String,
    trigger_id: String,
}

#[post("/np", data = "<payload>")]
fn command_np(slack_client: State<SlackClient>, payload: LenientForm<CommandRequest>) -> Result<String, String> {
    let payload = payload.get();

    let lastfm_username = slack_client.get_custom_field("LastFM", &payload.user_id)?;

    Ok(format!("Your Last.FM username is: {:?}", lastfm_username))
}
