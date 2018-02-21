use std::collections::HashMap;
use std::sync::Mutex;

use slack_api::requests as slack_request;
use slack_api::users_profile;

pub(crate) struct SlackClient {
    client: slack_request::Client,
    token: String,
    field_ids: Mutex<HashMap<String, String>>,
}

impl SlackClient {
    pub(crate) fn new(token: &str) -> SlackClient {
        SlackClient {
            client: slack_request::default_client().unwrap(),
            token: String::from(token),
            field_ids: Mutex::new(HashMap::new()),
        }
    }

    fn get_custom_field_id(&self, field_name: &str, user_id: &str) -> Result<String, String> {
        let mut lock = self.field_ids.lock().unwrap();

        let user_request = users_profile::GetRequest {
            user: Some(user_id),
            include_labels: Some(true),
        };
        let user = users_profile::get(&self.client, &self.token, &user_request).unwrap();

        let fields = user.profile.unwrap().fields.unwrap();

        for (field_id, field_values) in &fields {
            if field_values.label == Some(field_name.to_string()) {
                lock.insert(field_name.to_string(), field_id.to_string());
                return Ok(field_id.to_string());
            }
        }

        Err(format!(
            "Your Slack doesn't have the \"{}\" field enabled - talk to an owner.",
            field_name
        ))
    }

    fn lookup_field_id(&self, field_name: &str) -> Option<String> {
        let lock = self.field_ids.lock().unwrap();

        lock.get(field_name).cloned()
    }

    pub(crate) fn get_custom_field(
        &self,
        field_name: &str,
        user_id: &str,
    ) -> Result<Option<String>, String> {
        let target_field_id = match self.lookup_field_id(field_name) {
            Some(id) => id,
            None => self.get_custom_field_id(field_name, user_id)?,
        };

        let user_request = users_profile::GetRequest {
            user: Some(user_id),
            include_labels: Some(false),
        };
        let user = users_profile::get(&self.client, &self.token, &user_request).unwrap();
        let fields = user.profile.unwrap().fields.unwrap();

        Ok(fields[&target_field_id].clone().value)
    }
}
