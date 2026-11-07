//! apparmor model — mandatory access control.

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub mode: String,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub profiles: Vec<Profile>,
}

impl Default for Model {
    fn default() -> Self { Self { profiles: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_profile(&mut self, name: &str, mode: &str) {
        if !self.profiles.iter().any(|p| p.name == name) {
            self.profiles.push(Profile { name: name.into(), mode: mode.into(), rules: vec![] });
        }
    }

    pub fn add_rule(&mut self, name: &str, rule: &str) -> bool {
        if let Some(p) = self.profiles.iter_mut().find(|p| p.name == name) {
            p.rules.push(rule.into());
            true
        } else { false }
    }

    pub fn set_mode(&mut self, name: &str, mode: &str) -> bool {
        if let Some(p) = self.profiles.iter_mut().find(|p| p.name == name) {
            p.mode = mode.into();
            true
        } else { false }
    }

    pub fn list(&self) -> &[Profile] { &self.profiles }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn profile_rules() {
        let mut a = Model::new();
        a.add_profile("usr.bin.firefox", "enforce");
        assert!(a.add_rule("usr.bin.firefox", "/etc/passwd r"));
        assert!(a.set_mode("usr.bin.firefox", "complain"));
        assert_eq!(a.profiles[0].mode, "complain");
    }
}
