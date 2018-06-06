use schema::*;

#[derive(Queryable)]
pub(crate) struct OauthToken {
    pub(crate) id: i32,
    pub(crate) oauth_token: String,
    pub(crate) team_id: String,
}

#[table_name = "oauth_tokens"]
#[derive(Insertable)]
pub(crate) struct NewOauthToken<'a> {
    pub(crate) oauth_token: &'a str,
    pub(crate) team_id: &'a str,
}
