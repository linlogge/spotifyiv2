/// Version string of the form "spotifyiv2-<sha>"
pub const VERSION_STRING: &str = concat!("spotifyiv2-", env!("VERGEN_GIT_SHA"));

/// Generate a timestamp string representing the build date (UTC).
pub const BUILD_DATE: &str = env!("VERGEN_BUILD_DATE");

/// Short sha of the latest git commit.
pub const SHA_SHORT: &str = env!("VERGEN_GIT_SHA");

/// Date of the latest git commit.
pub const COMMIT_DATE: &str = env!("VERGEN_GIT_COMMIT_DATE");

/// spotifyiv2 crate version.
pub const SEMVER: &str = env!("CARGO_PKG_VERSION");

/// A random build id.
pub const BUILD_ID: &str = env!("spotifyiv2_BUILD_ID");

/// The protocol version of the Spotify desktop client.
pub const SPOTIFY_VERSION: u64 = 117300517;

/// The protocol version of the Spotify mobile app.
pub const SPOTIFY_MOBILE_VERSION: &str = "8.6.84";

/// The user agent to fall back to, if one could not be determined dynamically.
pub const FALLBACK_USER_AGENT: &str = "Spotify/117300517 Linux/0 (spotifyiv2)";

pub fn spotify_version() -> String {
    match std::env::consts::OS {
        "android" | "ios" => SPOTIFY_MOBILE_VERSION.to_owned(),
        _ => SPOTIFY_VERSION.to_string(),
    }
}
