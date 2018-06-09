use diesel::prelude::*;
use failure::Error;
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
fn route_np(
    conn: DbConn,
    slack_client: State<SlackClient>,
    lastfm_client: State<LastfmClient>,
    payload: LenientForm<CommandRequest>,
) -> Result<(), Error> {
    let payload = payload.get();

    match command_np(conn, &slack_client, &lastfm_client, payload) {
        Ok(_) => Ok(()),
        Err(e) => slack_client.respond_error(&payload.response_url, format!("{}", e)),
    }
}

fn command_np(
    conn: DbConn,
    slack_client: &SlackClient,
    lastfm_client: &LastfmClient,
    payload: &CommandRequest,
) -> Result<(), Error> {
    use db::schema::oauth_tokens::dsl::*;

    let token = oauth_tokens.filter(team_id.eq(&payload.team_id))
        .load::<OauthToken>(&*conn)?
        .pop()
        .ok_or(format_err!("No OAuth token for team {}", payload.team_id))?
        .oauth_token;

    if let Some(lastfm_username) = slack_client.get_lastfm_field(
        &payload.team_id,
        &payload.user_id,
        &token
    )? {
        let now_playing = match lastfm_client.now_playing(&lastfm_username) {
            Ok(np) => np,
            Err(e) => bail!("Last.FM API call failed: {}", e),
        };
        let artist = escape_text(now_playing.artist.to_string());
        let track = escape_text(now_playing.name);
        let message = format!("<@{}> is now playing: {} - {}", payload.user_id, artist, track);
        slack_client.post_message(&token, &payload.channel_id, &message)?;
        Ok(())
    } else {
        bail!("Sorry, you don't have a Last.FM username set in your slack profile.");
    }
}
