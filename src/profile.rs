//! AppArmor profile.

use crate::rule::Rule;

/// Profile kind
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileKind {
    /// Enforce mode
    Enforce,
    /// Complain mode
    Complain,
    /// Unconfined
    Unconfined,
    /// Disabled
    Disable,
}

/// Profile mode
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileMode {
    /// Enforce mode
    Enforce,
    /// Complain mode
    Complain,
}

/// Profile state
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileState {
    /// Loaded profile
    Loaded,
    /// Unloaded profile
    Unloaded,
    /// Error profile
    Error,
}

/// AppArmor profile
#[derive(Debug, Clone)]
pub struct Profile {
    /// Profile ID
    pub id: String,
    /// Profile name
    pub name: String,
    /// Profile kind
    pub kind: ProfileKind,
    /// Profile mode
    pub mode: ProfileKind,
    /// Profile state
    pub state: ProfileState,
    /// Rules
    pub rules: Vec<Rule>,
    /// Created timestamp
    pub created_at: u64,
}

impl Profile {
    /// Create new profile
    pub fn new(name: String, kind: ProfileKind) -> Self {
        Self {
            id: format!("profile-{}-{}", name, std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            mode: kind.clone(),
            name,
            kind,
            state: ProfileState::Loaded,
            rules: Vec::new(),
            created_at: Self::now(),
        }
    }
    
    /// Add rule
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
    
    /// Remove rule
    pub fn remove_rule(&mut self, index: usize) -> Option<Rule> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }
    
    /// Set mode
    pub fn set_mode(&mut self, mode: ProfileKind) {
        self.mode = mode;
    }
    
    /// Unload profile
    pub fn unload(&mut self) {
        self.state = ProfileState::Unloaded;
    }
    
    /// Set error
    pub fn set_error(&mut self) {
        self.state = ProfileState::Error;
    }
    
    /// Check if enforcing
    pub fn is_enforcing(&self) -> bool {
        self.mode == ProfileKind::Enforce
    }
    
    /// Get profile age in seconds
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
