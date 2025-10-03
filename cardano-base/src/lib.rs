//! Cardano feature flag primitives.
//!
//! This crate mirrors the original Haskell module `Cardano.Base.FeatureFlags`
//! and exposes a strongly typed representation of experimental Cardano
//! protocol features. Flags can be parsed from and serialised to JSON strings
//! matching the historical names used across the Cardano ecosystem.

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A finite set of experimental Cardano features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CardanoFeatureFlag {
    /// Ouroboros Leios (higher throughput).
    #[serde(alias = "LeiosFlag")]
    Leios,
    /// Ouroboros Peras (faster settlement).
    #[serde(alias = "PerasFlag")]
    Peras,
    /// Ouroboros Phalanx (anti-grinding).
    #[serde(alias = "PhalanxFlag")]
    Phalanx,
}

impl CardanoFeatureFlag {
    /// Return all known feature flags in declaration order.
    #[must_use] 
    pub const fn all() -> &'static [CardanoFeatureFlag; 3] {
        &[
            CardanoFeatureFlag::Leios,
            CardanoFeatureFlag::Peras,
            CardanoFeatureFlag::Phalanx,
        ]
    }

    /// An ergonomic iterator over all feature flags.
    pub fn iter() -> impl Iterator<Item = CardanoFeatureFlag> {
        CardanoFeatureFlag::all().iter().copied()
    }
}

impl fmt::Display for CardanoFeatureFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            CardanoFeatureFlag::Leios => "Leios",
            CardanoFeatureFlag::Peras => "Peras",
            CardanoFeatureFlag::Phalanx => "Phalanx",
        };
        f.write_str(text)
    }
}

impl FromStr for CardanoFeatureFlag {
    type Err = ParseFeatureFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Leios" | "LeiosFlag" => Ok(CardanoFeatureFlag::Leios),
            "Peras" | "PerasFlag" => Ok(CardanoFeatureFlag::Peras),
            "Phalanx" | "PhalanxFlag" => Ok(CardanoFeatureFlag::Phalanx),
            _ => Err(ParseFeatureFlagError::UnknownFlag(s.to_owned())),
        }
    }
}

/// Parse a collection of feature flags from textual names.
pub fn parse_flags<I, S>(names: I) -> Result<Vec<CardanoFeatureFlag>, ParseFeatureFlagError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    names
        .into_iter()
        .map(|name| CardanoFeatureFlag::from_str(name.as_ref()))
        .collect()
}

/// Error raised when decoding a [`CardanoFeatureFlag`].
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ParseFeatureFlagError {
    #[error("unknown Cardano feature flag: {0}")]
    UnknownFlag(String),
}

/// Lazily initialised lookup table for quick case-insensitive parsing.
static LOOKUP_LOWER: Lazy<HashMap<String, CardanoFeatureFlag>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(CardanoFeatureFlag::all().len());
    for flag in CardanoFeatureFlag::iter() {
        map.insert(flag.to_string().to_lowercase(), flag);
    }

    map
});

/// Case-insensitive parsing convenience.
pub fn parse_flag_case_insensitive(
    value: &str,
) -> Result<CardanoFeatureFlag, ParseFeatureFlagError> {
    let lower = value.to_lowercase();
    if let Some(flag) = LOOKUP_LOWER.get(&lower) {
        return Ok(*flag);
    }

    CardanoFeatureFlag::from_str(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        for flag in CardanoFeatureFlag::iter() {
            let json = serde_json::to_string(&flag).expect("serialize");
            assert_eq!(json, format!("\"{}\"", flag));

            let back: CardanoFeatureFlag = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, flag);
        }
    }

    #[test]
    fn from_str_variants() {
        assert_eq!(
            "Leios".parse::<CardanoFeatureFlag>().unwrap(),
            CardanoFeatureFlag::Leios
        );
        assert_eq!(
            "PerasFlag".parse::<CardanoFeatureFlag>().unwrap(),
            CardanoFeatureFlag::Peras
        );
        assert!("Unknown".parse::<CardanoFeatureFlag>().is_err());
    }

    #[test]
    fn parse_flags_list() {
        let parsed = parse_flags(["Leios", "Phalanx"]).expect("parse flags");
        assert_eq!(
            parsed,
            vec![CardanoFeatureFlag::Leios, CardanoFeatureFlag::Phalanx]
        );
    }

    #[test]
    fn case_insensitive_lookup() {
        assert_eq!(
            parse_flag_case_insensitive("leios").unwrap(),
            CardanoFeatureFlag::Leios
        );
        assert!(parse_flag_case_insensitive("leioss").is_err());
    }
}
