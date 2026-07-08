pub mod provider;
pub mod openai_provider;
pub mod anthropic_provider;
pub mod gemini_provider;
pub mod ollama_provider;
pub mod provider_registry;
pub mod provider_factory;
pub mod llamacpp_provider;
pub mod api_key_manager;
pub mod provider_config;
pub mod provider_loader;
pub mod provider_selector;


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use zanistarast_core::ScientificObject;
use zanistarast_kernel::{
    KernelExecutionResult,
    ScientificKernel,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSessionId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiState {
    Created,
    ContextLoaded,
    Reasoning,
    Planning,
    Executing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTraceEntry {
    pub ai_session_id: AiSessionId,
    pub state: AiState,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTrace {
    pub ai_session_id: AiSessionId,
    pub entries: Vec<AiTraceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiExecutionResult {
    pub ai_session_id: AiSessionId,
    pub kernel_result: KernelExecutionResult,
    pub ai_trace: AiTrace,
}

pub struct NativeAiRuntime {
    kernel: ScientificKernel,
}

impl NativeAiRuntime {
    pub fn new() -> Self {
        Self {
            kernel: ScientificKernel::new(),
        }
    }

    pub fn execute_scientific_request(
        &mut self,
        object: ScientificObject,
    ) -> AiExecutionResult {
        let ai_session_id = AiSessionId(Uuid::new_v4());

        let mut trace = AiTrace {
            ai_session_id: ai_session_id.clone(),
            entries: Vec::new(),
        };

        Self::push_trace(&mut trace, AiState::Created, "AI session created");
        Self::push_trace(&mut trace, AiState::ContextLoaded, "certified context loaded");
        Self::push_trace(&mut trace, AiState::Reasoning, "deterministic reasoning started");
        Self::push_trace(&mut trace, AiState::Planning, "execution plan generated");
        Self::push_trace(&mut trace, AiState::Executing, "scientific kernel execution requested");

        let kernel_result = self.kernel.execute(object);

        let final_state = if kernel_result.runtime_result.certification.verified {
            AiState::Completed
        } else {
            AiState::Failed
        };

        Self::push_trace(&mut trace, final_state, "AI scientific execution completed");

        AiExecutionResult {
            ai_session_id,
            kernel_result,
            ai_trace: trace,
        }
    }

    fn push_trace(
        trace: &mut AiTrace,
        state: AiState,
        message: impl Into<String>,
    ) {
        trace.entries.push(AiTraceEntry {
            ai_session_id: trace.ai_session_id.clone(),
            state,
            message: message.into(),
            timestamp: Utc::now(),
        });
    }
}

impl Default for NativeAiRuntime {
    fn default() -> Self {
        Self::new()
    }
}


