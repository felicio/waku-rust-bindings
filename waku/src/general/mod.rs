//! Waku [general](https://rfc.vac.dev/spec/36/#general) types

// std
use std::fmt::{Display, Formatter};
use std::str::FromStr;
// crates
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use sscanf::{scanf, RegexRepresentation};
// internal

pub type WakuMessageVersion = usize;
/// Base58 encoded peer id
pub type PeerId = String;
pub type MessageId = String;

/// JsonResponse wrapper.
/// `go-waku` ffi returns this type as a `char *` as per the [specification](https://rfc.vac.dev/spec/36/#jsonresponse-type)
/// This is internal, as it is better to use rust plain `Result` type.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum JsonResponse<T> {
    Result(T),
    Error(String),
}

/// Waku response, just a `Result` with an `String` error.
/// Convenient we can transform a [`JsonResponse`] into a [`std::result::Result`]
pub type Result<T> = std::result::Result<T, String>;

impl<T> From<JsonResponse<T>> for Result<T> {
    fn from(response: JsonResponse<T>) -> Self {
        match response {
            JsonResponse::Result(t) => Ok(t),
            JsonResponse::Error(e) => Err(e),
        }
    }
}

/// JsonMessage, Waku message in JSON format.
/// as per the [specification](https://rfc.vac.dev/spec/36/#jsonmessage-type)
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WakuMessage {
    payload: Box<[u8]>,
    /// The content topic to be set on the message
    content_topic: WakuContentTopic,
    /// The Waku Message version number
    version: WakuMessageVersion,
    /// Unix timestamp in nanoseconds
    timestamp: usize,
}

/// A payload once decoded, used when a received Waku Message is encrypted
pub struct DecodedPayload {
    /// Public key that signed the message (optional), hex encoded with 0x prefix
    public_key: Option<String>,
    /// Message signature (optional), hex encoded with 0x prefix
    signature: Option<String>,
    /// Decrypted message payload base64 encoded
    data: String,
    /// Padding base64 encoded
    padding: String,
}

/// The content topic of a Waku message
/// as per the [specification](https://rfc.vac.dev/spec/36/#contentfilter-type)
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentFilter {
    /// The content topic of a Waku message
    content_topic: WakuContentTopic,
}

/// The criteria to create subscription to a light node in JSON Format
/// as per the [specification](https://rfc.vac.dev/spec/36/#filtersubscription-type)
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterSubscription {
    /// Array of [`ContentFilter`] being subscribed to / unsubscribed from
    content_filters: Vec<ContentFilter>,
    /// Optional pubsub topic
    pubsub_topic: Option<WakuPubSubTopic>,
}

/// Criteria used to retrieve historical messages
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreQuery {
    /// The pubsub topic on which messages are published
    pubsub_topic: Option<WakuPubSubTopic>,
    /// Array of [`ContentFilter`] to query for historical messages
    content_filters: Vec<ContentFilter>,
    /// The inclusive lower bound on the timestamp of queried messages.
    /// This field holds the Unix epoch time in nanoseconds
    start_time: Option<usize>,
    /// The inclusive upper bound on the timestamp of queried messages.
    /// This field holds the Unix epoch time in nanoseconds
    end_time: Option<usize>,
    /// Paging information in [`PagingOptions`] format
    paging_options: Option<PagingOptions>,
}

/// The response received after doing a query to a store node
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreResponse {
    /// Array of retrieved historical messages in [`WakuMessage`] format
    messages: Vec<WakuMessage>,
    /// Paging information in [`PagingOptions`] format from which to resume further historical queries
    paging_options: Option<PagingOptions>,
}

/// Paging information
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagingOptions {
    /// Number of messages to retrieve per page
    page_size: usize,
    /// Message Index from which to perform pagination.
    /// If not included and forward is set to `true`, paging will be performed from the beginning of the list.
    /// If not included and forward is set to `false`, paging will be performed from the end of the list
    cursor: Option<MessageIndex>,
    /// `true` if paging forward, `false` if paging backward
    forward: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageIndex {
    /// Hash of the message at this [`MessageIndex`]
    digest: String,
    /// UNIX timestamp in nanoseconds at which the message at this [`MessageIndex`] was received
    receiver_time: usize,
    /// UNIX timestamp in nanoseconds at which the message is generated by its sender
    sender_time: usize,
    /// The pubsub topic of the message at this [`MessageIndex`]
    pubsub_topic: WakuPubSubTopic,
}

#[derive(Copy, Clone)]
pub enum Encoding {
    Proto,
    Rlp,
    Rfc26,
}

impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Encoding::Proto => "proto",
            Encoding::Rlp => "rlp",
            Encoding::Rfc26 => "rfc26",
        };
        f.write_str(s)
    }
}

impl FromStr for Encoding {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "proto" => Ok(Self::Proto),
            "rlp" => Ok(Self::Rlp),
            "rfc26" => Ok(Self::Rfc26),
            encoding => Err(format!("Unrecognized encoding: {}", encoding)),
        }
    }
}

impl RegexRepresentation for Encoding {
    const REGEX: &'static str = r"\w";
}

#[derive(Clone)]
pub struct WakuContentTopic {
    application_name: String,
    version: usize,
    content_topic_name: String,
    encoding: Encoding,
}

impl FromStr for WakuContentTopic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok((application_name, version, content_topic_name, encoding)) =
            scanf!(s, "/{}/{}/{}/{}", String, usize, String, Encoding)
        {
            Ok(WakuContentTopic {
                application_name,
                version,
                content_topic_name,
                encoding,
            })
        } else {
            Err(
                format!(
                    "Wrong pub-sub topic format. Should be `/{{application-name}}/{{version-of-the-application}}/{{content-topic-name}}/{{encoding}}`. Got: {}", 
                    s
                )
            )
        }
    }
}

impl Display for WakuContentTopic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/{}/{}/{}/{}",
            self.application_name, self.version, self.content_topic_name, self.encoding
        )
    }
}

impl Serialize for WakuContentTopic {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WakuContentTopic {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let as_string: String = String::deserialize(deserializer)?;
        as_string
            .parse::<WakuContentTopic>()
            .map_err(D::Error::custom)
    }
}

#[derive(Clone)]
pub struct WakuPubSubTopic {
    topic_name: String,
    encoding: Encoding,
}

impl FromStr for WakuPubSubTopic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok((topic_name, encoding)) = scanf!(s, "/waku/v2/{}/{}", String, Encoding) {
            Ok(WakuPubSubTopic {
                topic_name,
                encoding,
            })
        } else {
            Err(
                format!(
                    "Wrong pub-sub topic format. Should be `/waku/2/{{topic-name}}/{{encoding}}`. Got: {}",
                    s
                )
            )
        }
    }
}

impl Display for WakuPubSubTopic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "/waku/2/{}/{}", self.topic_name, self.encoding)
    }
}

impl Serialize for WakuPubSubTopic {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WakuPubSubTopic {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let as_string: String = String::deserialize(deserializer)?;
        as_string
            .parse::<WakuPubSubTopic>()
            .map_err(D::Error::custom)
    }
}
