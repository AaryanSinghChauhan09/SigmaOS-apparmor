//! AppArmor manager.

use std::collections::HashMap;
use crate::profile::{Profile, ProfileKind, ProfileState};
use crate::rule::Rule;
use crate::attachment::{Attachment, AttachmentState};
use crate::audit::{AuditEvent, AuditKind, AuditResult};

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total profiles
    pub total_profiles: usize,
    /// Loaded profiles
    pub loaded_profiles: usize,
    /// Enforcing profiles
    pub enforcing_profiles: usize,
    /// Total attachments
    pub total_attachments: usize,
    /// Active attachments
    pub active_attachments: usize,
    /// Total audit events
    pub total_audit_events: usize,
    /// Denied events
    pub denied_events: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// AppArmor manager
#[derive(Debug, Clone)]
pub struct ApparmorManager {
    /// Profiles
    profiles: HashMap<String, Profile>,
    /// Attachments
    attachments: HashMap<String, Attachment>,
    /// Audit events
    audit_events: Vec<AuditEvent>,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for ApparmorManager {
    fn default() -> Self {
        Self {
            profiles: HashMap::new(),
            attachments: HashMap::new(),
            audit_events: Vec::new(),
            statistics: ManagerStatistics {
                total_profiles: 0,
                loaded_profiles: 0,
                enforcing_profiles: 0,
                total_attachments: 0,
                active_attachments: 0,
                total_audit_events: 0,
                denied_events: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl ApparmorManager {
    /// Create new apparmor manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Load default profiles
        self.load_profile("docker-default".into(), ProfileKind::Enforce);
        self.load_profile("unprivileged_userns".into(), ProfileKind::Enforce);
        
        self.update_statistics();
    }
    
    // Profile methods
    
    /// Load profile
    pub fn load_profile(&mut self, name: String, kind: ProfileKind) -> String {
        let profile = Profile::new(name.clone(), kind);
        let id = profile.id.clone();
        
        self.profiles.insert(id.clone(), profile);
        self.update_statistics();
        id
    }
    
    /// Get profile
    pub fn get_profile(&self, profile_id: &str) -> Option<&Profile> {
        self.profiles.get(profile_id)
    }
    
    /// Get profile mutably
    pub fn get_profile_mut(&mut self, profile_id: &str) -> Option<&mut Profile> {
        self.profiles.get_mut(profile_id)
    }
    
    /// Get all profiles
    pub fn list_profiles(&self) -> Vec<&Profile> {
        self.profiles.values().collect()
    }
    
    /// Remove profile
    pub fn remove_profile(&mut self, profile_id: &str) -> bool {
        if self.profiles.remove(profile_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Unload profile
    pub fn unload_profile(&mut self, profile_id: &str) -> bool {
        if let Some(profile) = self.profiles.get_mut(profile_id) {
            profile.unload();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add rule to profile
    pub fn add_rule(&mut self, profile_id: &str, rule: Rule) -> bool {
        if let Some(profile) = self.profiles.get_mut(profile_id) {
            profile.add_rule(rule);
            return true;
        }
        false
    }
    
    /// Set profile mode
    pub fn set_profile_mode(&mut self, profile_id: &str, mode: ProfileKind) -> bool {
        if let Some(profile) = self.profiles.get_mut(profile_id) {
            profile.set_mode(mode);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get profiles by kind
    pub fn get_profiles_by_kind(&self, kind: &ProfileKind) -> Vec<&Profile> {
        self.profiles.values()
            .filter(|p| p.kind == *kind)
            .collect()
    }
    
    /// Get enforcing profiles
    pub fn get_enforcing_profiles(&self) -> Vec<&Profile> {
        self.profiles.values()
            .filter(|p| p.is_enforcing())
            .collect()
    }
    
    /// Search profiles
    pub fn search_profiles(&self, query: &str) -> Vec<&Profile> {
        let query_lower = query.to_lowercase();
        self.profiles.values()
            .filter(|p| p.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Attachment methods
    
    /// Attach profile
    pub fn attach_profile(&mut self, profile_id: &str, path: String) -> String {
        let profile_name = self.profiles.get(profile_id)
            .map(|p| p.name.clone())
            .unwrap_or_default();
        
        let attachment = Attachment::new(path.clone(), profile_name);
        let id = attachment.id.clone();
        
        self.attachments.insert(id.clone(), attachment);
        self.update_statistics();
        id
    }
    
    /// Get attachment
    pub fn get_attachment(&self, attachment_id: &str) -> Option<&Attachment> {
        self.attachments.get(attachment_id)
    }
    
    /// Get attachment mutably
    pub fn get_attachment_mut(&mut self, attachment_id: &str) -> Option<&mut Attachment> {
        self.attachments.get_mut(attachment_id)
    }
    
    /// Get all attachments
    pub fn list_attachments(&self) -> Vec<&Attachment> {
        self.attachments.values().collect()
    }
    
    /// Remove attachment
    pub fn remove_attachment(&mut self, attachment_id: &str) -> bool {
        if self.attachments.remove(attachment_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Detach attachment
    pub fn detach_profile(&mut self, attachment_id: &str) -> bool {
        if let Some(attachment) = self.attachments.get_mut(attachment_id) {
            attachment.detach();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get active attachments
    pub fn get_active_attachments(&self) -> Vec<&Attachment> {
        self.attachments.values()
            .filter(|a| a.state == AttachmentState::Active)
            .collect()
    }
    
    // Audit methods
    
    /// Add audit event
    pub fn add_audit_event(&mut self, event: AuditEvent) {
        self.audit_events.push(event);
        self.update_statistics();
    }
    
    /// Get audit events
    pub fn list_audit_events(&self) -> &[AuditEvent] {
        &self.audit_events
    }
    
    /// Get audit events by kind
    pub fn get_audit_events_by_kind(&self, kind: &AuditKind) -> Vec<&AuditEvent> {
        self.audit_events.iter()
            .filter(|e| e.kind == *kind)
            .collect()
    }
    
    /// Get denied events
    pub fn get_denied_events(&self) -> Vec<&AuditEvent> {
        self.audit_events.iter()
            .filter(|e| e.result == AuditResult::Denied)
            .collect()
    }
    
    /// Clear audit events
    pub fn clear_audit_events(&mut self) {
        self.audit_events.clear();
        self.update_statistics();
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_profiles = self.profiles.len();
        self.statistics.loaded_profiles = self.profiles.values()
            .filter(|p| p.state == ProfileState::Loaded)
            .count();
        self.statistics.enforcing_profiles = self.profiles.values()
            .filter(|p| p.is_enforcing())
            .count();
        self.statistics.total_attachments = self.attachments.len();
        self.statistics.active_attachments = self.attachments.values()
            .filter(|a| a.state == AttachmentState::Active)
            .count();
        self.statistics.total_audit_events = self.audit_events.len();
        self.statistics.denied_events = self.audit_events.iter()
            .filter(|e| e.result == AuditResult::Denied)
            .count();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            profiles: self.profiles.values().cloned().collect(),
            attachments: self.attachments.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.profiles.clear();
        for profile in export.profiles {
            self.profiles.insert(profile.id.clone(), profile);
        }
        
        self.attachments.clear();
        for attachment in export.attachments {
            self.attachments.insert(attachment.id.clone(), attachment);
        }
        
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Profiles
    pub profiles: Vec<Profile>,
    /// Attachments
    pub attachments: Vec<Attachment>,
}
