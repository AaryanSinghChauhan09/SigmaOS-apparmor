//! AppArmor attachment.

/// Attachment state
#[derive(Debug, Clone, PartialEq)]
pub enum AttachmentState {
    /// Active attachment
    Active,
    /// Detached attachment
    Detached,
    /// Error attachment
    Error,
}

/// AppArmor attachment
#[derive(Debug, Clone)]
pub struct Attachment {
    /// Attachment ID
    pub id: String,
    /// Executable path
    pub path: String,
    /// Profile name
    pub profile_name: String,
    /// Attachment state
    pub state: AttachmentState,
    /// Created timestamp
    pub created_at: u64,
}

impl Attachment {
    /// Create new attachment
    pub fn new(path: String, profile_name: String) -> Self {
        Self {
            id: format!("attach-{}-{}", profile_name, std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            path,
            profile_name,
            state: AttachmentState::Active,
            created_at: Self::now(),
        }
    }
    
    /// Attach
    pub fn attach(&mut self) {
        self.state = AttachmentState::Active;
    }
    
    /// Detach
    pub fn detach(&mut self) {
        self.state = AttachmentState::Detached;
    }
    
    /// Set error
    pub fn set_error(&mut self) {
        self.state = AttachmentState::Error;
    }
    
    /// Get attachment age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
