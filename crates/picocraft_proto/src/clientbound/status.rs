use crate::prelude::*;

// use core_json_derive::{JsonDeserialize, JsonSerialize};

#[derive(Debug, Packet, bon::Builder)]
#[packet(id = 0x00, state = State::Status)]
pub struct StatusResponsePacket<const N: usize> {
    json_response: String<N>,
}

#[derive(Debug, Packet, bon::Builder)]
#[packet(id = 0x01, state = State::Status)]
pub struct PongResponsePacket {
    timestamp: Long,
}

/// Sample Json output for status response:
/// ```json
/// {
///     "version": {
///         "name": "1.19.4",
///         "protocol": 761
///     },
///     "players": {
///         "max": 100,
///         "online": 5,
///         "sample": [
///             {
///                 "name": "thinkofdeath",
///                 "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
///             }
///         ]
///     },
///     "description": {
///         "text": "Hello world"
///     },
///     "favicon": "data:image/png;base64,<data>",
///     "enforcesSecureChat": true
///     }
/// }
/// ```
#[derive(Debug, bon::Builder, Default)]
// #[derive(JsonDeserialize,JsonSerialize)]
pub struct JsonStatusResponse {
    #[builder(into, default)]
    pub version: Version,

    #[builder(with = |max: i32, online: i32| Players::new(online, max))]
    pub players: Players,

    ///doesn't need to be the same level as [Players] or [Version], as [Chat]
    /// already has the `text` field as the description field has.
    #[builder(into, default)]
    pub description: Chat,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // favicon: Option<String>,
    #[builder(default)]
    pub enforces_secure_chat: bool,
}

#[derive(Debug)]
// #[derive(JsonDeserialize,JsonSerialize)]
pub struct Version {
    pub name: String<8>,
    pub protocol: VarInt,
}

impl Default for Version {
    fn default() -> Self {
        Self::from(ProtocolVersion::default())
    }
}

impl From<ProtocolVersion> for Version {
    fn from(protocol_version: ProtocolVersion) -> Self {
        Self {
            name: String::from(protocol_version),
            protocol: *protocol_version,
        }
    }
}

#[derive(Debug, Default)]
// #[derive(JsonDeserialize,JsonSerialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    //TODO consider never using this field
    pub sample: Vec<PlayerSample, 2>,
}

impl Players {
    fn new(max: i32, online: i32) -> Self {
        Self {
            max,
            online,
            sample: Vec::new(),
        }
    }
}

#[derive(Debug)]
// #[derive(JsonDeserialize,JsonSerialize)]
pub struct Chat {
    pub text: String<128>,
}

impl Chat {
    pub fn new(text: &str) -> Self {
        Self {
            text: String::from_str(text)
                .expect("Chat description must be less than 128 characters long"),
        }
    }
}

impl From<String<128>> for Chat {
    fn from(text: String<128>) -> Self {
        Self { text }
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            text: String::from_str("A picocraft server! :D").expect("String is long enough"),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Default)]
// #[derive(JsonDeserialize,JsonSerialize)]
pub struct PlayerSample {
    name: String<16>,
    id: UUID,
}
