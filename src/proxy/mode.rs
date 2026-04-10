#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyMode {
    Rule,
    Global,
    Direct,
}

impl ProxyMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rule" => Some(Self::Rule),
            "global" => Some(Self::Global),
            "direct" => Some(Self::Direct),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rule => "rule",
            Self::Global => "global",
            Self::Direct => "direct",
        }
    }
}
