use std::collections::HashMap;
use std::sync::Mutex;

use slack_api::chat;
use slack_api::oauth;
use slack_api::requests as slack_request;
use slack_api::team;
use slack_api::users_profile;

pub(crate) struct SlackClient {
    client: slack_request::Client,
    field_ids: Mutex<HashMap<String, String>>,
    client_id: String,
    client_secret: String,
}

impl SlackClient {
    pub(crate) fn new(client_id: &str, client_secret: &str) -> SlackClient {
        SlackClient {
            client: slack_request::default_client().unwrap(),
            field_ids: Mutex::new(HashMap::new()),
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
        }
    }

    fn get_custom_field_id(
        &self,
        team_id: &str,
        user_id: &str,
        token: &str,
    ) -> Result<String, String> {
        let mut field_ids = self.field_ids.lock().unwrap();

        let user_request = users_profile::GetRequest {
            user: Some(user_id),
            include_labels: Some(true),
        };
        let user = users_profile::get(&self.client, token, &user_request).unwrap();

        if let Some(fields) = user.profile.unwrap().fields {
            for (field_id, field_values) in &fields {
                if field_values.label == Some("LastFM".to_string()) {
                    field_ids.insert(team_id.to_string(), field_id.to_string());

                    return Ok(field_id.to_string());
                }
            }
        }

        Err("Your Slack doesn't have the LastFM field enabled - talk to an owner.".to_string())
    }

    fn lookup_field_id(&self, team_id: &str) -> Option<String> {
        let field_ids = self.field_ids.lock().unwrap();

        field_ids.get(team_id).cloned()
    }

    pub(crate) fn get_lastfm_field(
        &self,
        team_id: &str,
        user_id: &str,
        token: &str,
    ) -> Result<Option<String>, String> {
        let target_field_id = match self.lookup_field_id(team_id) {
            Some(id) => id,
            None => self.get_custom_field_id(team_id, user_id, token)?,
        };

        let user_request = users_profile::GetRequest {
            user: Some(user_id),
            include_labels: Some(false),
        };
        let user = users_profile::get(&self.client, &token, &user_request).unwrap();
        let fields = user.profile.unwrap().fields.unwrap();

        Ok(fields[&target_field_id].clone().value)
    }

    pub(crate) fn post_message(
        &self,
        token: &str,
        channel_id: &str,
        message: &str,
    ) -> Result<(), String> {
        let mut post_request = chat::PostMessageRequest::default();
        post_request.channel = channel_id;
        post_request.text = message;

        chat::post_message(&self.client, token, &post_request)
            .map(|_| ())
            .map_err(|e| format!("{}", e))
    }

    pub(crate) fn oauth_access(&self, code: &str) -> Result<String, String> {
        let request = oauth::AccessRequest {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            code: code,
            redirect_uri: None,
        };

        oauth::access(&self.client, &request)
            .map(|response| String::from(response.access_token.unwrap()))
            .map_err(|e| format!("{}", e))
    }

    pub(crate) fn get_team_id(&self, token: &str) -> Result<String, String> {
        team::info(&self.client, token)
            .map(|response| String::from(response.team.unwrap().id.unwrap()))
            .map_err(|e| format!("{}", e))
    }
}
