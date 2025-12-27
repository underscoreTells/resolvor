#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "A conversation containing multiple messages"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A conversation containing multiple messages\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"created_at\","]
#[doc = "    \"id\","]
#[doc = "    \"messages\","]
#[doc = "    \"updated_at\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"description\": \"When the conversation was created\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"Unique identifier for the conversation\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"description\": \"Messages in the conversation\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Message\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Optional title for the conversation\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"description\": \"When the conversation was last updated\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Conversation {
    #[doc = "When the conversation was created"]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Unique identifier for the conversation"]
    pub id: ::std::string::String,
    #[doc = "Messages in the conversation"]
    pub messages: ::std::vec::Vec<Message>,
    #[doc = "Optional title for the conversation"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "When the conversation was last updated"]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Conversation> for Conversation {
    fn from(value: &Conversation) -> Self {
        value.clone()
    }
}
impl Conversation {
    pub fn builder() -> builder::Conversation {
        Default::default()
    }
}
#[doc = "A message in the conversation thread"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A message in the conversation thread\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"id\","]
#[doc = "    \"role\","]
#[doc = "    \"status\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"description\": \"Array of content blocks in the message\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/MessageContent\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"Unique identifier for the message\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$ref\": \"#/definitions/MessageRole\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/MessageStatus\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"When the message was created\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Message {
    #[doc = "Array of content blocks in the message"]
    pub content: ::std::vec::Vec<MessageContent>,
    #[doc = "Unique identifier for the message"]
    pub id: ::std::string::String,
    pub role: MessageRole,
    pub status: MessageStatus,
    #[doc = "When the message was created"]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Message> for Message {
    fn from(value: &Message) -> Self {
        value.clone()
    }
}
impl Message {
    pub fn builder() -> builder::Message {
        Default::default()
    }
}
#[doc = "A content block within a message"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A content block within a message\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ToolUseContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ToolResultContent\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum MessageContent {
    TextContent(TextContent),
    ToolUseContent(ToolUseContent),
    ToolResultContent(ToolResultContent),
}
impl ::std::convert::From<&Self> for MessageContent {
    fn from(value: &MessageContent) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<TextContent> for MessageContent {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ToolUseContent> for MessageContent {
    fn from(value: ToolUseContent) -> Self {
        Self::ToolUseContent(value)
    }
}
impl ::std::convert::From<ToolResultContent> for MessageContent {
    fn from(value: ToolResultContent) -> Self {
        Self::ToolResultContent(value)
    }
}
#[doc = "The role of the message sender"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The role of the message sender\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"user\","]
#[doc = "    \"assistant\","]
#[doc = "    \"system\","]
#[doc = "    \"error\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "error")]
    Error,
}
impl ::std::convert::From<&Self> for MessageRole {
    fn from(value: &MessageRole) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
            Self::System => f.write_str("system"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for MessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The current status of the message"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The current status of the message\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"pending\","]
#[doc = "    \"streaming\","]
#[doc = "    \"complete\","]
#[doc = "    \"error\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessageStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "streaming")]
    Streaming,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "error")]
    Error,
}
impl ::std::convert::From<&Self> for MessageStatus {
    fn from(value: &MessageStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MessageStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
            Self::Streaming => f.write_str("streaming"),
            Self::Complete => f.write_str("complete"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for MessageStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            "streaming" => Ok(Self::Streaming),
            "complete" => Ok(Self::Complete),
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Shared message types for the Resolvor agent UI"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"messages.schema.json\","]
#[doc = "  \"title\": \"Message Types\","]
#[doc = "  \"description\": \"Shared message types for the Resolvor agent UI\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"Conversation\": {"]
#[doc = "      \"$ref\": \"#/definitions/Conversation\""]
#[doc = "    },"]
#[doc = "    \"Message\": {"]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"MessageContent\": {"]
#[doc = "      \"$ref\": \"#/definitions/MessageContent\""]
#[doc = "    },"]
#[doc = "    \"ToolCall\": {"]
#[doc = "      \"$ref\": \"#/definitions/ToolCall\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MessageTypes {
    #[serde(
        rename = "Conversation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conversation: ::std::option::Option<Conversation>,
    #[serde(
        rename = "Message",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub message: ::std::option::Option<Message>,
    #[serde(
        rename = "MessageContent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub message_content: ::std::option::Option<MessageContent>,
    #[serde(
        rename = "ToolCall",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tool_call: ::std::option::Option<ToolCall>,
}
impl ::std::convert::From<&MessageTypes> for MessageTypes {
    fn from(value: &MessageTypes) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MessageTypes {
    fn default() -> Self {
        Self {
            conversation: Default::default(),
            message: Default::default(),
            message_content: Default::default(),
            tool_call: Default::default(),
        }
    }
}
impl MessageTypes {
    pub fn builder() -> builder::MessageTypes {
        Default::default()
    }
}
#[doc = "A text content block"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A text content block\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"text\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The text content, may contain markdown\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"text\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TextContent {
    #[doc = "The text content, may contain markdown"]
    pub text: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&TextContent> for TextContent {
    fn from(value: &TextContent) -> Self {
        value.clone()
    }
}
impl TextContent {
    pub fn builder() -> builder::TextContent {
        Default::default()
    }
}
#[doc = "Represents a tool/function call made by the agent"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a tool/function call made by the agent\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"arguments\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"description\": \"Arguments passed to the tool\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"completed_at\": {"]
#[doc = "      \"description\": \"Timestamp when the tool call completed\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"Unique identifier for the tool call\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Name of the tool being called\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"Result returned from the tool execution\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"started_at\": {"]
#[doc = "      \"description\": \"Timestamp when the tool call started\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/definitions/ToolCallStatus\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ToolCall {
    #[doc = "Arguments passed to the tool"]
    pub arguments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Timestamp when the tool call completed"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub completed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Unique identifier for the tool call"]
    pub id: ::std::string::String,
    #[doc = "Name of the tool being called"]
    pub name: ::std::string::String,
    #[doc = "Result returned from the tool execution"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<::std::string::String>,
    #[doc = "Timestamp when the tool call started"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub started_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub status: ToolCallStatus,
}
impl ::std::convert::From<&ToolCall> for ToolCall {
    fn from(value: &ToolCall) -> Self {
        value.clone()
    }
}
impl ToolCall {
    pub fn builder() -> builder::ToolCall {
        Default::default()
    }
}
#[doc = "The current status of a tool call"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The current status of a tool call\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"pending\","]
#[doc = "    \"running\","]
#[doc = "    \"success\","]
#[doc = "    \"error\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ToolCallStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}
impl ::std::convert::From<&Self> for ToolCallStatus {
    fn from(value: &ToolCallStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ToolCallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
            Self::Running => f.write_str("running"),
            Self::Success => f.write_str("success"),
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for ToolCallStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            "running" => Ok(Self::Running),
            "success" => Ok(Self::Success),
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolCallStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A tool result content block"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A tool result content block\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"result\","]
#[doc = "    \"tool_call_id\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"is_error\": {"]
#[doc = "      \"description\": \"Whether the tool execution resulted in an error\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result of the tool execution\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tool_call_id\": {"]
#[doc = "      \"description\": \"ID of the tool call this result corresponds to\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tool_result\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ToolResultContent {
    #[doc = "Whether the tool execution resulted in an error"]
    #[serde(default)]
    pub is_error: bool,
    #[doc = "The result of the tool execution"]
    pub result: ::std::string::String,
    #[doc = "ID of the tool call this result corresponds to"]
    pub tool_call_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&ToolResultContent> for ToolResultContent {
    fn from(value: &ToolResultContent) -> Self {
        value.clone()
    }
}
impl ToolResultContent {
    pub fn builder() -> builder::ToolResultContent {
        Default::default()
    }
}
#[doc = "A tool use content block"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A tool use content block\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"tool_call\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"tool_call\": {"]
#[doc = "      \"$ref\": \"#/definitions/ToolCall\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tool_use\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ToolUseContent {
    pub tool_call: ToolCall,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&ToolUseContent> for ToolUseContent {
    fn from(value: &ToolUseContent) -> Self {
        value.clone()
    }
}
impl ToolUseContent {
    pub fn builder() -> builder::ToolUseContent {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Conversation {
        created_at:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        messages: ::std::result::Result<::std::vec::Vec<super::Message>, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        updated_at:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for Conversation {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                id: Err("no value supplied for id".to_string()),
                messages: Err("no value supplied for messages".to_string()),
                title: Ok(Default::default()),
                updated_at: Err("no value supplied for updated_at".to_string()),
            }
        }
    }
    impl Conversation {
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Message>>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn updated_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.updated_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Conversation> for super::Conversation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Conversation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                id: value.id?,
                messages: value.messages?,
                title: value.title?,
                updated_at: value.updated_at?,
            })
        }
    }
    impl ::std::convert::From<super::Conversation> for Conversation {
        fn from(value: super::Conversation) -> Self {
            Self {
                created_at: Ok(value.created_at),
                id: Ok(value.id),
                messages: Ok(value.messages),
                title: Ok(value.title),
                updated_at: Ok(value.updated_at),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Message {
        content:
            ::std::result::Result<::std::vec::Vec<super::MessageContent>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::MessageRole, ::std::string::String>,
        status: ::std::result::Result<super::MessageStatus, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for Message {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                id: Err("no value supplied for id".to_string()),
                role: Err("no value supplied for role".to_string()),
                status: Err("no value supplied for status".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl Message {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::MessageContent>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Message> for super::Message {
        type Error = super::error::ConversionError;
        fn try_from(value: Message) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                id: value.id?,
                role: value.role?,
                status: value.status?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::Message> for Message {
        fn from(value: super::Message) -> Self {
            Self {
                content: Ok(value.content),
                id: Ok(value.id),
                role: Ok(value.role),
                status: Ok(value.status),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageTypes {
        conversation: ::std::result::Result<
            ::std::option::Option<super::Conversation>,
            ::std::string::String,
        >,
        message:
            ::std::result::Result<::std::option::Option<super::Message>, ::std::string::String>,
        message_content: ::std::result::Result<
            ::std::option::Option<super::MessageContent>,
            ::std::string::String,
        >,
        tool_call:
            ::std::result::Result<::std::option::Option<super::ToolCall>, ::std::string::String>,
    }
    impl ::std::default::Default for MessageTypes {
        fn default() -> Self {
            Self {
                conversation: Ok(Default::default()),
                message: Ok(Default::default()),
                message_content: Ok(Default::default()),
                tool_call: Ok(Default::default()),
            }
        }
    }
    impl MessageTypes {
        pub fn conversation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Conversation>>,
            T::Error: ::std::fmt::Display,
        {
            self.conversation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conversation: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Message>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
        pub fn message_content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MessageContent>>,
            T::Error: ::std::fmt::Display,
        {
            self.message_content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_content: {}", e));
            self
        }
        pub fn tool_call<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ToolCall>>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_call = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_call: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageTypes> for super::MessageTypes {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageTypes,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                conversation: value.conversation?,
                message: value.message?,
                message_content: value.message_content?,
                tool_call: value.tool_call?,
            })
        }
    }
    impl ::std::convert::From<super::MessageTypes> for MessageTypes {
        fn from(value: super::MessageTypes) -> Self {
            Self {
                conversation: Ok(value.conversation),
                message: Ok(value.message),
                message_content: Ok(value.message_content),
                tool_call: Ok(value.tool_call),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextContent {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TextContent {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TextContent {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TextContent> for super::TextContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TextContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                text: value.text?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TextContent> for TextContent {
        fn from(value: super::TextContent) -> Self {
            Self {
                text: Ok(value.text),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolCall {
        arguments: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        completed_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        started_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::ToolCallStatus, ::std::string::String>,
    }
    impl ::std::default::Default for ToolCall {
        fn default() -> Self {
            Self {
                arguments: Err("no value supplied for arguments".to_string()),
                completed_at: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                result: Ok(Default::default()),
                started_at: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl ToolCall {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {}", e));
            self
        }
        pub fn completed_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.completed_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completed_at: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
        pub fn started_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.started_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for started_at: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToolCallStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolCall> for super::ToolCall {
        type Error = super::error::ConversionError;
        fn try_from(value: ToolCall) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                arguments: value.arguments?,
                completed_at: value.completed_at?,
                id: value.id?,
                name: value.name?,
                result: value.result?,
                started_at: value.started_at?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::ToolCall> for ToolCall {
        fn from(value: super::ToolCall) -> Self {
            Self {
                arguments: Ok(value.arguments),
                completed_at: Ok(value.completed_at),
                id: Ok(value.id),
                name: Ok(value.name),
                result: Ok(value.result),
                started_at: Ok(value.started_at),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolResultContent {
        is_error: ::std::result::Result<bool, ::std::string::String>,
        result: ::std::result::Result<::std::string::String, ::std::string::String>,
        tool_call_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ToolResultContent {
        fn default() -> Self {
            Self {
                is_error: Ok(Default::default()),
                result: Err("no value supplied for result".to_string()),
                tool_call_id: Err("no value supplied for tool_call_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ToolResultContent {
        pub fn is_error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_error: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
        pub fn tool_call_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_call_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_call_id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolResultContent> for super::ToolResultContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolResultContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                is_error: value.is_error?,
                result: value.result?,
                tool_call_id: value.tool_call_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ToolResultContent> for ToolResultContent {
        fn from(value: super::ToolResultContent) -> Self {
            Self {
                is_error: Ok(value.is_error),
                result: Ok(value.result),
                tool_call_id: Ok(value.tool_call_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ToolUseContent {
        tool_call: ::std::result::Result<super::ToolCall, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ToolUseContent {
        fn default() -> Self {
            Self {
                tool_call: Err("no value supplied for tool_call".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ToolUseContent {
        pub fn tool_call<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ToolCall>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_call = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_call: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ToolUseContent> for super::ToolUseContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ToolUseContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tool_call: value.tool_call?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ToolUseContent> for ToolUseContent {
        fn from(value: super::ToolUseContent) -> Self {
            Self {
                tool_call: Ok(value.tool_call),
                type_: Ok(value.type_),
            }
        }
    }
}
