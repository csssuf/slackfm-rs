use std::sync::Mutex;

use failure::Error;
use rustfm::{self, user::recent_tracks::Track};

pub(crate) struct LastfmClient {
    client: Mutex<rustfm::Client>,
}

impl LastfmClient {
    pub(crate) fn new(api_key: &str) -> LastfmClient {
        LastfmClient {
            client: Mutex::new(rustfm::Client::new(api_key)),
        }
    }

    pub(crate) fn now_playing(&self, user: &str) -> Result<Track, Error> {
        let mut client = self.client.lock().unwrap();

        let now_playing = client.recent_tracks(user)
            .with_limit(1)
            .send()
            .map_err(|e| format_err!("{:?}", e))?;

        Ok(now_playing.tracks[0].clone())
    }
}
