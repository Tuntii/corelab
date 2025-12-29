//! AI Interface
//! 
//! Provider-agnostic AI abstraction layer.
//! Supports both cloud (OpenAI) and local (Ollama) providers.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// AI Provider errors
#[derive(Error, Debug)]
pub enum AIError {
    #[error("Provider not configured")]
    NotConfigured,
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

/// AI request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

/// Structured AI response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub structured: Option<serde_json::Value>,
    pub tokens_used: Option<u32>,
}

/// Memory extraction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedMemory {
    pub key: String,
    pub value: String,
    pub importance: i32,
    pub confidence: f32,
}

/// AI Provider trait - implement for each provider
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &str;
    
    /// Check if provider is available
    async fn is_available(&self) -> bool;
    
    /// Send a completion request
    async fn complete(&self, request: AIRequest) -> Result<AIResponse, AIError>;
    
    /// Extract memories from conversation text
    async fn extract_memories(&self, conversation: &str) -> Result<Vec<ExtractedMemory>, AIError>;
}

/// OpenAI Provider
pub struct OpenAIProvider {
    api_key: Option<String>,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            model: "gpt-4o-mini".to_string(),
        }
    }
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "OpenAI"
    }
    
    async fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
    
    async fn complete(&self, _request: AIRequest) -> Result<AIResponse, AIError> {
        // TODO: Implement OpenAI API call
        Err(AIError::NotConfigured)
    }
    
    async fn extract_memories(&self, _conversation: &str) -> Result<Vec<ExtractedMemory>, AIError> {
        // TODO: Implement memory extraction
        Err(AIError::NotConfigured)
    }
}

/// Ollama Provider (local)
pub struct OllamaProvider {
    endpoint: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(endpoint: Option<String>, model: Option<String>) -> Self {
        Self {
            endpoint: endpoint.unwrap_or_else(|| "http://localhost:11434".to_string()),
            model: model.unwrap_or_else(|| "llama2".to_string()),
        }
    }
}

#[async_trait]
impl AIProvider for OllamaProvider {
    fn name(&self) -> &str {
        "Ollama"
    }
    
    async fn is_available(&self) -> bool {
        // TODO: Check if Ollama is running
        false
    }
    
    async fn complete(&self, _request: AIRequest) -> Result<AIResponse, AIError> {
        // TODO: Implement Ollama API call
        Err(AIError::NotConfigured)
    }
    
    async fn extract_memories(&self, _conversation: &str) -> Result<Vec<ExtractedMemory>, AIError> {
        // TODO: Implement memory extraction
        Err(AIError::NotConfigured)
    }
}

/// Mock Provider for testing
pub struct MockProvider;

#[async_trait]
impl AIProvider for MockProvider {
    fn name(&self) -> &str {
        "Mock"
    }
    
    async fn is_available(&self) -> bool {
        true
    }
    
    async fn complete(&self, request: AIRequest) -> Result<AIResponse, AIError> {
        Ok(AIResponse {
            content: format!("Mock response for: {}", request.prompt),
            structured: None,
            tokens_used: Some(10),
        })
    }
    
    async fn extract_memories(&self, conversation: &str) -> Result<Vec<ExtractedMemory>, AIError> {
        // Return mock memories
        Ok(vec![
            ExtractedMemory {
                key: "topic".to_string(),
                value: "Discussed in conversation".to_string(),
                importance: 3,
                confidence: 0.8,
            }
        ])
    }
}
