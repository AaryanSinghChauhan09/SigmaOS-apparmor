//! AppArmor audit.

/// Audit kind
#[derive(Debug, Clone, PartialEq)]
pub enum AuditKind {
    /// File access audit
    FileAccess,
    /// Network connect audit
    NetworkConnect,
    /// Capability audit
    Capability,
    /// Mount audit
    Mount,
    /// Signal audit
    Signal,
    /// Ptrace audit
    Ptrace,
}

/// Audit result
#[derive(Debug, Clone, PartialEq)]
pub enum AuditResult {
    /// Allowed
    Allowed,
    /// Denied
    Denied,
    /// Audit only
    Audit,
}

/// Audit event
#[derive(Debug, Clone)]
pub struct AuditEvent {
    /// Event ID
    pub id: String,
    /// Event kind
    pub kind: AuditKind,
    /// Target
    pub target: String,
    /// Result
    pub result: AuditResult,
    /// Profile name
    pub profile_name: String,
    /// Timestamp
    pub timestamp: u64,
}

impl AuditEvent {
    /// Create new audit event
    pub fn new(kind: AuditKind, target: String, result: AuditResult) -> Self {
        Self {
            id: format!("audit-{}-{}", kind_name(&kind), std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            kind,
            target,
            result,
            profile_name: String::new(),
            timestamp: Self::now(),
        }
    }
    
    /// Set profile
    pub fn set_profile(&mut self, profile_name: String) {
        self.profile_name = profile_name;
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

fn kind_name(kind: &AuditKind) -> &'static str {
    match kind {
        AuditKind::FileAccess => "file",
        AuditKind::NetworkConnect => "net",
        AuditKind::Capability => "cap",
        AuditKind::Mount => "mount",
        AuditKind::Signal => "signal",
        AuditKind::Ptrace => "ptrace",
    }
}
