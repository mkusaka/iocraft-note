use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================
// Base Types
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseMessage {
    pub parent_uuid: Option<String>,
    pub is_sidechain: bool,
    pub user_type: String,
    pub cwd: String,
    pub session_id: String,
    pub version: String,
    pub uuid: String,
    pub timestamp: String,
}

// ============================================
// Content Types
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Content {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: HashMap<String, serde_json::Value>,
    },
    ToolResult {
        tool_use_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<ToolResultContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },
    Thinking {
        thinking: String,
        signature: String,
    },
    Image {
        source: ImageSource,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolResultContent {
    String(String),
    TextArray(Vec<TextContent>),
    ImageArray(Vec<ImageContent>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub source: ImageSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

// ============================================
// Usage Schema
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Usage {
    pub input_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
    pub output_tokens: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tool_use: Option<ServerToolUse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerToolUse {
    pub web_search_requests: u64,
}

// ============================================
// Message Types
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SessionMessage {
    Summary {
        summary: String,
        #[serde(rename = "leafUuid")]
        leaf_uuid: String,
    },
    System {
        #[serde(flatten)]
        base: BaseMessage,
        content: String,
        #[serde(rename = "isMeta")]
        is_meta: bool,
        #[serde(rename = "toolUseID", skip_serializing_if = "Option::is_none")]
        tool_use_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        level: Option<String>,
        #[serde(rename = "gitBranch", skip_serializing_if = "Option::is_none")]
        git_branch: Option<String>,
        #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
        request_id: Option<String>,
    },
    User {
        #[serde(flatten)]
        base: BaseMessage,
        message: UserMessage,
        #[serde(rename = "gitBranch", skip_serializing_if = "Option::is_none")]
        git_branch: Option<String>,
        #[serde(rename = "isMeta", skip_serializing_if = "Option::is_none")]
        is_meta: Option<bool>,
        #[serde(rename = "isCompactSummary", skip_serializing_if = "Option::is_none")]
        is_compact_summary: Option<bool>,
        #[serde(rename = "toolUseResult", skip_serializing_if = "Option::is_none")]
        tool_use_result: Option<serde_json::Value>,
    },
    Assistant {
        #[serde(flatten)]
        base: BaseMessage,
        message: AssistantMessage,
        #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
        request_id: Option<String>,
        #[serde(rename = "gitBranch", skip_serializing_if = "Option::is_none")]
        git_branch: Option<String>,
        #[serde(rename = "isApiErrorMessage", skip_serializing_if = "Option::is_none")]
        is_api_error_message: Option<bool>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    pub role: String,
    pub content: UserContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserContent {
    String(String),
    Array(Vec<Content>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub role: String,
    pub model: String,
    pub content: Vec<Content>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

// ============================================
// Helper Functions
// ============================================

impl SessionMessage {
    pub fn get_type(&self) -> &str {
        match self {
            SessionMessage::Summary { .. } => "summary",
            SessionMessage::System { .. } => "system",
            SessionMessage::User { .. } => "user",
            SessionMessage::Assistant { .. } => "assistant",
        }
    }

    pub fn get_timestamp(&self) -> Option<&str> {
        match self {
            SessionMessage::Summary { .. } => None,
            SessionMessage::System { base, .. } => Some(&base.timestamp),
            SessionMessage::User { base, .. } => Some(&base.timestamp),
            SessionMessage::Assistant { base, .. } => Some(&base.timestamp),
        }
    }


    pub fn get_text_content(&self) -> Vec<String> {
        match self {
            SessionMessage::Summary { summary, .. } => vec![summary.clone()],
            SessionMessage::System { content, .. } => vec![content.clone()],
            SessionMessage::User { message, .. } => match &message.content {
                UserContent::String(s) => vec![s.clone()],
                UserContent::Array(contents) => contents
                    .iter()
                    .filter_map(|c| match c {
                        Content::Text { text } => Some(text.clone()),
                        _ => None,
                    })
                    .collect(),
            },
            SessionMessage::Assistant { message, .. } => message
                .content
                .iter()
                .filter_map(|c| match c {
                    Content::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect(),
        }
    }
}

// ============================================
// Project File Structure
// ============================================

#[derive(Debug, Clone)]
pub struct ProjectFile {
    #[allow(dead_code)]
    pub path: String,
    pub project_name: String,
    pub messages: Vec<SessionMessage>,
}

impl ProjectFile {
    pub fn new(path: String) -> Self {
        let project_name = Self::extract_project_name(&path);
        Self {
            path,
            project_name,
            messages: Vec::new(),
        }
    }

    fn extract_project_name(path: &str) -> String {
        // Extract project name from path like:
        // /Users/.../projects/-Users-masatomokusaka-src-github-com-mkusaka-ccsearch/uuid.jsonl
        path.split('/').rev().nth(1)
            .unwrap_or("unknown")
            .to_string()
    }
}