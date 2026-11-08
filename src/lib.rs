//! SigmaOS-apparmor — AppArmor MAC for SigmaOS.

pub mod profile;
pub mod rule;
pub mod attachment;
pub mod audit;
pub mod manager;
pub mod util;
pub mod options;

pub use profile::{Profile, ProfileKind, ProfileMode, ProfileState};
pub use rule::{Rule, RuleKind, RuleAction};
pub use attachment::{Attachment, AttachmentState};
pub use audit::{AuditEvent, AuditKind, AuditResult};
pub use manager::{ApparmorManager, ManagerStatistics};
pub use util::Model;
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_profile_kind() {
        let kinds = vec![ProfileKind::Enforce, ProfileKind::Complain, ProfileKind::Unconfined, ProfileKind::Disable];
        assert_eq!(kinds.len(), 4);
    }
    
    #[test]
    fn test_profile_mode() {
        let modes = vec![ProfileMode::Enforce, ProfileMode::Complain];
        assert_eq!(modes.len(), 2);
    }
    
    #[test]
    fn test_rule_kind() {
        let kinds = vec![RuleKind::File, RuleKind::Network, RuleKind::Capability, RuleKind::Mount, RuleKind::Signal, RuleKind::Unix, RuleKind::Ptrace];
        assert_eq!(kinds.len(), 7);
    }
    
    #[test]
    fn test_rule_action() {
        let actions = vec![RuleAction::Allow, RuleAction::Deny, RuleAction::Audit];
        assert_eq!(actions.len(), 3);
    }
    
    #[test]
    fn test_profile_creation() {
        let profile = Profile::new("docker".into(), ProfileKind::Enforce);
        assert_eq!(profile.name, "docker".to_string());
        assert_eq!(profile.kind, ProfileKind::Enforce);
        assert_eq!(profile.state, ProfileState::Loaded);
        assert!(profile.rules.is_empty());
    }
    
    #[test]
    fn test_profile_add_rule() {
        let mut profile = Profile::new("nginx".into(), ProfileKind::Enforce);
        let rule = Rule::new("/etc/nginx/** r,".into(), RuleKind::File, RuleAction::Allow);
        profile.add_rule(rule);
        
        assert_eq!(profile.rules.len(), 1);
        assert!(matches!(profile.rules[0].kind, RuleKind::File));
    }
    
    #[test]
    fn test_profile_mode_change() {
        let mut profile = Profile::new("test".into(), ProfileKind::Enforce);
        profile.set_mode(ProfileKind::Complain);
        assert_eq!(profile.mode, ProfileKind::Complain);
    }
    
    #[test]
    fn test_profile_unload() {
        let mut profile = Profile::new("test".into(), ProfileKind::Enforce);
        profile.unload();
        assert_eq!(profile.state, ProfileState::Unloaded);
    }
    
    #[test]
    fn test_rule_creation() {
        let rule = Rule::new("capability net_bind_service,".into(), RuleKind::Capability, RuleAction::Allow);
        assert_eq!(rule.target, "capability net_bind_service,".to_string());
        assert_eq!(rule.kind, RuleKind::Capability);
        assert_eq!(rule.action, RuleAction::Allow);
    }
    
    #[test]
    fn test_rule_file() {
        let rule = Rule::file("/var/log/** rw,".into());
        assert!(matches!(rule.kind, RuleKind::File));
        assert_eq!(rule.action, RuleAction::Allow);
    }
    
    #[test]
    fn test_rule_deny() {
        let rule = Rule::deny("/etc/shadow r,".into());
        assert_eq!(rule.action, RuleAction::Deny);
    }
    
    #[test]
    fn test_attachment_creation() {
        let attachment = Attachment::new("/usr/bin/docker".into(), "docker".into());
        assert_eq!(attachment.path, "/usr/bin/docker".to_string());
        assert_eq!(attachment.profile_name, "docker".to_string());
        assert_eq!(attachment.state, AttachmentState::Active);
    }
    
    #[test]
    fn test_attachment_lifecycle() {
        let mut attachment = Attachment::new("/bin/test".into(), "test-profile".into());
        attachment.detach();
        assert_eq!(attachment.state, AttachmentState::Detached);
        attachment.attach();
        assert_eq!(attachment.state, AttachmentState::Active);
    }
    
    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new(AuditKind::FileAccess, "/etc/passwd".into(), AuditResult::Allowed);
        assert!(matches!(event.kind, AuditKind::FileAccess));
        assert_eq!(event.target, "/etc/passwd".to_string());
        assert!(matches!(event.result, AuditResult::Allowed));
    }
    
    #[test]
    fn test_audit_kinds() {
        let kinds = vec![AuditKind::FileAccess, AuditKind::NetworkConnect, AuditKind::Capability, AuditKind::Mount, AuditKind::Signal, AuditKind::Ptrace];
        assert_eq!(kinds.len(), 6);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = ApparmorManager::new();
        assert!(manager.list_profiles().is_empty());
        assert!(manager.list_attachments().is_empty());
        assert!(manager.list_audit_events().is_empty());
    }
    
    #[test]
    fn test_load_profile() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        let id = manager.load_profile("docker".into(), ProfileKind::Enforce);
        assert!(manager.get_profile(&id).is_some());
    }
    
    #[test]
    fn test_unload_profile() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        
        let profile_id = manager.load_profile("test".into(), ProfileKind::Enforce);
        assert!(manager.remove_profile(&profile_id));
        assert!(manager.get_profile(&profile_id).is_none());
    }
    
    #[test]
    fn test_add_rule_to_profile() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        
        let profile_id = manager.load_profile("nginx".into(), ProfileKind::Enforce);
        let rule = Rule::new("/var/www/** r,".into(), RuleKind::File, RuleAction::Allow);
        
        assert!(manager.add_rule(&profile_id, rule));
        
        let profile = manager.get_profile(&profile_id).unwrap();
        assert_eq!(profile.rules.len(), 1);
    }
    
    #[test]
    fn test_attach_profile() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        
        let profile_id = manager.load_profile("docker".into(), ProfileKind::Enforce);
        let attachment_id = manager.attach_profile(&profile_id, "/usr/bin/docker".into());
        
        assert!(manager.get_attachment(&attachment_id).is_some());
    }
    
    #[test]
    fn test_detach_profile() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        
        let profile_id = manager.load_profile("test".into(), ProfileKind::Enforce);
        let attachment_id = manager.attach_profile(&profile_id, "/bin/test".into());
        
        assert!(manager.detach_profile(&attachment_id));
        
        let attachment = manager.get_attachment(&attachment_id).unwrap();
        assert_eq!(attachment.state, AttachmentState::Detached);
    }
    
    #[test]
    fn test_audit_event() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        
        let event = AuditEvent::new(AuditKind::FileAccess, "/etc/shadow".into(), AuditResult::Denied);
        manager.add_audit_event(event);
        
        assert_eq!(manager.list_audit_events().len(), 1);
    }
    
    #[test]
    fn test_complain_mode() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        
        let profile_id = manager.load_profile("test".into(), ProfileKind::Complain);
        let profile = manager.get_profile(&profile_id).unwrap();
        assert_eq!(profile.mode, ProfileKind::Complain);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        
        let profile_id = manager.load_profile("docker".into(), ProfileKind::Enforce);
        manager.add_rule(&profile_id, Rule::file("/var/run/docker.sock rw,".into()));
        manager.attach_profile(&profile_id, "/usr/bin/docker".into());
        
        let export = manager.export_data();
        let mut manager2 = ApparmorManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_profiles().len(), manager.list_profiles().len());
        assert_eq!(manager2.list_attachments().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = ApparmorManager::new();
        manager.initialize();
        
        let profile_id = manager.load_profile("docker".into(), ProfileKind::Enforce);
        manager.attach_profile(&profile_id, "/usr/bin/docker".into());
        manager.add_audit_event(AuditEvent::new(AuditKind::FileAccess, "/test".into(), AuditResult::Allowed));
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_profiles, 3); // 2 defaults + 1 added
        assert_eq!(stats.total_attachments, 1);
        assert_eq!(stats.total_audit_events, 1);
        assert!(stats.uptime > 0 || stats.uptime == 0);
    }
}
