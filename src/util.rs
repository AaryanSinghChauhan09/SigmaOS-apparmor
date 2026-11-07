//! AppArmor model — application security profiles.

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub mode: String,
    pub attachments: Vec<String>,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub action: String,
    pub path: String,
    pub perms: String,
}

#[derive(Debug, Clone)]
pub struct AppArmor {
    pub enabled: bool,
    pub profiles: Vec<Profile>,
}

impl Default for AppArmor {
    fn default() -> Self { Self { enabled: false, profiles: vec![] } }
}

impl AppArmor {
    pub fn new() -> Self { Self::default() }

    pub fn load_profile(&self, profile: Profile) -> bool {
        !profile.name.is_empty()
    }

    pub fn list_profiles(&self) -> Vec<Profile> {
        vec![Profile { name: "docker-default".into(), mode: "enforce".into(), attachments: vec!["/usr/bin/docker".into()], rules: vec![] }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn profiles() {
        let a = AppArmor::new();
        assert_eq!(a.list_profiles().len(), 1);
    }
}
