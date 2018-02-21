use rocket::State;
use rocket::request::LenientForm;

use lastfm::*;
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
fn command_np(
    slack_client: State<SlackClient>,
    lastfm_client: State<LastfmClient>,
    payload: LenientForm<CommandRequest>,
) -> Result<(), String> {
    let payload = payload.get();

    if let Some(lastfm_username) = slack_client.get_custom_field("LastFM", &payload.user_id)? {
        let now_playing = lastfm_client.now_playing(&lastfm_username)?;
        let message = format!("{} is now playing: {} - {}", lastfm_username, now_playing.artist, now_playing.name);
        slack_client.post_message(&payload.channel_id, &message)?;
        Ok(())
    } else {
        Err(format!("Sorry, you don't have a Last.FM username set in your slack profile."))
    }
}
