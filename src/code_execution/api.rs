use crate::converter::ReqwestClient;
use color_eyre::Result;
use reqwest::header;
use serde::{Deserialize, Deserializer, Serialize};
use serenity::client::Context;
use std::collections::HashMap;
use std::str::FromStr;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct CommandFlags {
    pub channel: Channel,
    pub mode: Mode,
    pub edition: Edition,
    pub warn: bool,
}

#[derive(Debug, Serialize)]
pub struct PlaygroundRequest<'a> {
    pub channel: Channel,
    pub edition: Edition,
    pub code: &'a str,
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,
    pub mode: Mode,
    pub tests: bool,
}

#[derive(Debug, Serialize)]
pub struct MiriRequest<'a> {
    pub edition: Edition,
    pub code: &'a str,
}

pub type MacroExpansionRequest<'a> = MiriRequest<'a>;

#[derive(Debug, Serialize)]
pub struct ClippyRequest<'a> {
    pub edition: Edition,
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,
    pub code: &'a str,
}

#[derive(Debug, Serialize)]
pub struct FormatRequest<'a> {
    pub code: &'a str,
    pub edition: Edition,
}

#[derive(Debug, Deserialize)]
pub struct FormatResponse {
    pub success: bool,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
}

impl FromStr for Channel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stable" => Ok(Channel::Stable),
            "beta" => Ok(Channel::Beta),
            "nightly" => Ok(Channel::Nightly),
            _ => Err(format!("invalid release channel `{}`", s).into()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Edition {
    #[serde(rename = "2015")]
    E2015,
    #[serde(rename = "2018")]
    E2018,
    #[serde(rename = "2021")]
    E2021,
}

impl FromStr for Edition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2015" => Ok(Edition::E2015),
            "2018" => Ok(Edition::E2018),
            "2021" => Ok(Edition::E2021),
            _ => Err(format!("invalid edition `{}`", s).into()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum CrateType {
    #[serde(rename = "bin")]
    Binary,
    #[serde(rename = "lib")]
    Library,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Debug,
    Release,
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(Mode::Debug),
            "release" => Ok(Mode::Release),
            _ => Err(format!("invalid compilation mode `{}`", s).into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PlayResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl<'de> Deserialize<'de> for PlayResult {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // The playground occasionally sends just a single "error" field, for example with
        // `loop{println!("a")}`. We put the error into the stderr field

        #[derive(Deserialize)]
        #[serde(untagged)]
        pub enum RawPlayResponse {
            Err {
                error: String,
            },
            Ok {
                success: bool,
                stdout: String,
                stderr: String,
            },
        }

        Ok(match RawPlayResponse::deserialize(deserializer)? {
            RawPlayResponse::Ok {
                success,
                stdout,
                stderr,
            } => PlayResult {
                success,
                stdout,
                stderr,
            },
            RawPlayResponse::Err { error } => PlayResult {
                success: false,
                stdout: String::new(),
                stderr: error,
            },
        })
    }
}

/// Returns a gist ID
pub async fn post_gist(ctx: &Context, code: &str) -> Result<String> {
    let mut payload = HashMap::new();
    payload.insert("code", code);

    let client = ctx
        .data
        .read()
        .await
        .get::<ReqwestClient>()
        .cloned()
        .unwrap();
    let result = client
        .post("https://play.rust-lang.org/meta/gist/")
        .header(header::REFERER, "Ferris")
        .json(&payload)
        .send()
        .await?;

    let mut result: HashMap<String, String> = result.json().await?;
    info!("gist response: {:?}", result);

    let gist_id = result.remove("id").ok_or("no gist found").unwrap();
    Ok(gist_id)
}

pub fn url_from_gist(flags: &CommandFlags, gist_id: &str) -> String {
    format!(
        "https://play.rust-lang.org/?version={}&mode={}&edition={}&gist={}",
        match flags.channel {
            Channel::Nightly => "nightly",
            Channel::Beta => "beta",
            Channel::Stable => "stable",
        },
        match flags.mode {
            Mode::Debug => "debug",
            Mode::Release => "release",
        },
        match flags.edition {
            Edition::E2015 => "2015",
            Edition::E2018 => "2018",
            Edition::E2021 => "2021",
        },
        gist_id
    )
}

pub async fn apply_online_rustfmt(
    ctx: &Context,
    code: &str,
    edition: Edition,
) -> Result<PlayResult> {
    let client = ctx
        .data
        .read()
        .await
        .get::<ReqwestClient>()
        .cloned()
        .unwrap();
    let result = client
        .post("https://play.rust-lang.org/format")
        .json(&FormatRequest { code, edition })
        .send()
        .await?
        .json::<FormatResponse>()
        .await?;

    Ok(PlayResult {
        success: result.success,
        stdout: result.code,
        stderr: result.stderr,
    })
}
