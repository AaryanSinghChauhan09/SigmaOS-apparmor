//! AppArmor rule.

/// Rule kind
#[derive(Debug, Clone, PartialEq)]
pub enum RuleKind {
    /// File rule
    File,
    /// Network rule
    Network,
    /// Capability rule
    Capability,
    /// Mount rule
    Mount,
    /// Signal rule
    Signal,
    /// Unix rule
    Unix,
    /// Ptrace rule
    Ptrace,
}

/// Rule action
#[derive(Debug, Clone, PartialEq)]
pub enum RuleAction {
    /// Allow
    Allow,
    /// Deny
    Deny,
    /// Audit
    Audit,
}

/// AppArmor rule
#[derive(Debug, Clone)]
pub struct Rule {
    /// Rule ID
    pub id: String,
    /// Rule target
    pub target: String,
    /// Rule kind
    pub kind: RuleKind,
    /// Rule action
    pub action: RuleAction,
    /// Created timestamp
    pub created_at: u64,
}

impl Rule {
    /// Create new rule
    pub fn new(target: String, kind: RuleKind, action: RuleAction) -> Self {
        Self {
            id: format!("rule-{}-{}", kind_name(&kind), std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            target,
            kind,
            action,
            created_at: Self::now(),
        }
    }
    
    /// Create file allow rule
    pub fn file(target: String) -> Self {
        Self::new(target, RuleKind::File, RuleAction::Allow)
    }
    
    /// Create file deny rule
    pub fn deny(target: String) -> Self {
        Self::new(target, RuleKind::File, RuleAction::Deny)
    }
    
    /// Get rule age in seconds
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

fn kind_name(kind: &RuleKind) -> &'static str {
    match kind {
        RuleKind::File => "file",
        RuleKind::Network => "net",
        RuleKind::Capability => "cap",
        RuleKind::Mount => "mount",
        RuleKind::Signal => "signal",
        RuleKind::Unix => "unix",
        RuleKind::Ptrace => "ptrace",
    }
}
