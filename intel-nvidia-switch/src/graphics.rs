#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GraphicsMode {
    Nvidia,
    Intel,
    Hybrid
}

impl std::fmt::Display for GraphicsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
        Self::Hybrid => "hybrid",
        Self::Nvidia => "nvidia",
        Self::Intel => "intel",
        };
        s.fmt(f)
    }
}

impl std::str::FromStr for GraphicsMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
        "hybrid" => Ok(Self::Hybrid),
        "nvidia" => Ok(Self::Nvidia),
        "intel" => Ok(Self::Intel),
        _ => Err(format!("Unknown Graphics Mode: {s}")),
        }
    }
}
