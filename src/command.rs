use std::sync::mpsc::SyncSender;

use diesel::prelude::*;
use failure::Error;
use rocket::State;
use rocket::request::LenientForm;

use db::*;
use db::models::*;
use lastfm::*;
use slack::*;
use spotify::*;

#[derive(FromForm)]
pub(crate) struct CommandRequest {
    pub(crate) token: String,
    pub(crate) team_id: String,
    pub(crate) channel_id: String,
    pub(crate) channel_name: String,
    pub(crate) user_id: String,
    pub(crate) user_name: String,
    pub(crate) command: String,
    pub(crate) text: Option<String>,
    pub(crate) response_url: String,
    pub(crate) trigger_id: String,
}

#[post("/np", data = "<payload>")]
pub(crate) fn route_np(
    tx: State<SyncSender<CommandRequest>>,
    payload: LenientForm<CommandRequest>,
) -> Result<(), Error> {
    tx.send(payload.into_inner())?;

    Ok(())
}

pub(crate) fn command_np(
    conn: DbConn,
    slack_client: &SlackClient,
    lastfm_client: &LastfmClient,
    spotify_client: &SpotifyClient,
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

        let artist = now_playing.artist.to_string();
        let track = now_playing.name;

        let mut attachments = Vec::new();
        if let Some(spotify_track) = spotify_client.get_track_url(&artist, &track).unwrap_or(None) {
            attachments.push(Attachment {
                fallback: format!("Open in Spotify: {}", spotify_track),
                actions: vec![Action {
                    ty: ActionType::Button,
                    text: String::from("Open in Spotify"),
                    url: spotify_track,
                    style: Some(ActionStyle::Primary),
                }],
            });
        }

        let message = format!(
            "<@{}> is now playing: {} - {}",
            payload.user_id,
            escape_text(artist.clone()),
            escape_text(track.clone())
        );

        slack_client.post_message(&token, &payload.channel_id, &message, attachments)?;

        Ok(())
    } else {
        bail!("Sorry, you don't have a Last.FM username set in your slack profile.");
    }
}
