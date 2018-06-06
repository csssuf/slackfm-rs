use diesel::prelude::*;
use rocket::State;
use rocket::request::LenientForm;

use db::*;
use db::models::*;
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
    conn: DbConn,
    slack_client: State<SlackClient>,
    lastfm_client: State<LastfmClient>,
    payload: LenientForm<CommandRequest>,
) -> Result<(), String> {
    use db::schema::oauth_tokens::dsl::*;

    let payload = payload.get();

    let token = oauth_tokens.filter(team_id.eq(&payload.team_id))
        .load::<OauthToken>(&*conn)
        .map_err(|e| format!("Couldn't get team OAuth token: {}", e))?
        .pop()
        .ok_or(format!("No OAuth token for team {}", &payload.team_id))?
        .oauth_token;

    if let Some(lastfm_username) = slack_client.get_lastfm_field(&payload.team_id, &payload.user_id, &token)? {
        let now_playing = lastfm_client.now_playing(&lastfm_username)?;
        let artist = escape_text(now_playing.artist.to_string());
        let track = escape_text(now_playing.name);
        let message = format!("<@{}> is now playing: {} - {}", payload.user_id, artist, track);
        slack_client.post_message(&token, &payload.channel_id, &message)?;
        Ok(())
    } else {
        Err(format!("Sorry, you don't have a Last.FM username set in your slack profile."))
    }
}
