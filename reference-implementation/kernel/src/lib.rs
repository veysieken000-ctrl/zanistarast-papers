use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use zanistarast_core::{
    RuntimeId,
    ScientificObject,
};
use zanistarast_runtime::{
    DeterministicRuntime,
    RuntimeExecutionResult,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelState {
    Created,
    Initialized,
    Executing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelTraceEntry {
    pub kernel_id: KernelId,
    pub runtime_id: Option<RuntimeId>,
    pub state: KernelState,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelTrace {
    pub kernel_id: KernelId,
    pub entries: Vec<KernelTraceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelExecutionResult {
    pub kernel_id: KernelId,
    pub runtime_result: RuntimeExecutionResult,
    pub kernel_trace: KernelTrace,
}

pub struct ScientificKernel {
    runtime: DeterministicRuntime,
}

impl ScientificKernel {
    pub fn new() -> Self {
        Self {
            runtime: DeterministicRuntime::new(),
        }
    }

    pub fn execute(
        &mut self,
        object: ScientificObject,
    ) -> KernelExecutionResult {
        let kernel_id = KernelId(Uuid::new_v4());

        let mut trace = KernelTrace {
            kernel_id: kernel_id.clone(),
            entries: Vec::new(),
        };

        Self::push_trace(
            &mut trace,
            None,
            KernelState::Created,
            "kernel session created",
        );

        Self::push_trace(
            &mut trace,
            None,
            KernelState::Initialized,
            "kernel initialized",
        );

        Self::push_trace(
            &mut trace,
            None,
            KernelState::Executing,
            "runtime execution requested",
        );

        let runtime_result = self.runtime.execute(object);

        Self::push_trace(
            &mut trace,
            Some(runtime_result.runtime_id.clone()),
            KernelState::Completed,
            "kernel execution completed",
        );

        KernelExecutionResult {
            kernel_id,
            runtime_result,
            kernel_trace: trace,
        }
    }

    fn push_trace(
        trace: &mut KernelTrace,
        runtime_id: Option<RuntimeId>,
        state: KernelState,
        message: impl Into<String>,
    ) {
        trace.entries.push(KernelTraceEntry {
            kernel_id: trace.kernel_id.clone(),
            runtime_id,
            state,
            message: message.into(),
            timestamp: Utc::now(),
        });
    }
}

impl Default for ScientificKernel {
    fn default() -> Self {
        Self::new()
    }
}



