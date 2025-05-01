use std::{
    fmt::Display,
    sync::{Arc, LazyLock, Mutex},
};

use tokio::sync::broadcast::{self, error::SendError, Receiver};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Severity {
    Info,
    Trace,
    Warning,
    Error,
    Fatal,
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Trace => write!(f, "TRACE"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Error => write!(f, "ERROR"),
            Severity::Fatal => write!(f, "FATAL"),
        }
    }
}

pub trait CompilationEvent: Sync + Send {
    fn type_name(&self) -> &'static str;
    fn severity(&self) -> Severity;
    fn message(&self) -> String;
    fn to_json(&self) -> String;
}

const MAX_QUEUE_SIZE: usize = 16;

pub struct CompilationEventQueue {
    sender: broadcast::Sender<Arc<dyn CompilationEvent>>,
    receiver: broadcast::Receiver<Arc<dyn CompilationEvent>>,
}

impl Default for CompilationEventQueue {
    fn default() -> Self {
        let (sender, receiver) = broadcast::channel(MAX_QUEUE_SIZE);
        Self { sender, receiver }
    }
}

impl CompilationEventQueue {
    pub fn send(
        &self,
        message: Arc<dyn CompilationEvent>,
    ) -> Result<usize, SendError<Arc<dyn CompilationEvent>>> {
        self.sender.send(message)
    }

    pub fn subscribe(&self) -> Receiver<Arc<dyn CompilationEvent>> {
        // We want to use resubscribe here to avoid capturing messages sent before the subscription
        // began. If we just cloned the Arc, it would have all the messages sent before the
        // subscription began.
        self.receiver.resubscribe()
    }
}
