//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Status,
    ListProfiles,
    Load,
    Unload,
    Enforce,
    Complain,
    Disable,
}

#[derive(Debug, Clone)]
pub struct AppArmorOptions {
    pub mode: Mode,
    pub profile: Option<String>,
}

impl Default for AppArmorOptions {
    fn default() -> Self { Self { mode: Mode::Default, profile: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(AppArmorOptions::default().mode, Mode::Default); }
}
