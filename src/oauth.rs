use diesel::{self, RunQueryDsl};
use rocket::State;
use rocket::response::Redirect;

use db::*;
use db::models::*;
use db::schema::oauth_tokens;
use slack::*;

#[derive(FromForm, Debug)]
struct OauthParams {
    code: Option<String>,
    oauth_team_id: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

#[get("/oauth?<oauth_params>")]
fn oauth_route(conn: DbConn, slack: State<SlackClient>, oauth_params: OauthParams) -> Result<Redirect, String> {
    if oauth_params.error.is_some() {
        return Err("OAuth error. Did you decline the installation?".to_string());
    }

    let code = oauth_params.code.unwrap();

    let token = slack.oauth_access(&code)?;
    let team = slack.get_team_id(&token)?;

    let db_oauth_token = NewOauthToken {
        oauth_token: &token,
        team_id: &team,
    };
    diesel::insert_into(oauth_tokens::table)
        .values(&db_oauth_token)
        .execute(&*conn)
        .map_err(|e| format!("{}", e))?;

    Ok(Redirect::to("https://google.com/"))
}
