use failure::Error;
use percent_encoding::{utf8_percent_encode, QUERY_ENCODE_SET};
use rspotify::spotify::{client::Spotify, oauth2::SpotifyClientCredentials};

define_encode_set! {
    pub SPOTIFY_Q_ENCODE_SET = [QUERY_ENCODE_SET] | {'&'}
}

pub(crate) struct SpotifyClient {
    client: Spotify,
}

impl SpotifyClient {
    pub(crate) fn new(client_id: &str, client_secret: &str) -> SpotifyClient {
        let credentials = SpotifyClientCredentials::default()
            .client_id(client_id)
            .client_secret(client_secret)
            .build();

        SpotifyClient {
            client: Spotify::default()
                .client_credentials_manager(credentials)
                .build(),
        }
    }

    pub(crate) fn get_track_url(&self, artist: &str, track: &str) -> Result<Option<String>, Error> {
        let search_string = utf8_percent_encode(
            &format!("artist:{} track:{}", artist, track),
            SPOTIFY_Q_ENCODE_SET
        ).to_string();

        let track_results = self.client
            .search_track(&search_string, 1, 0, None)
            .map_err(|e| format_err!("{}", e))?;
        
        if track_results.tracks.items.len() > 0 {
            let track = &track_results.tracks.items[0];
            Ok(track.external_urls.get("spotify").cloned())
        } else {
            Ok(None)
        }
    }
}
