use std::fmt;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub enum SnackSeverity {
    Success,
    Info,
    Warning,
    Error,
}
impl fmt::Display for SnackSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnackSeverity::Success => write!(f, "success"),
            SnackSeverity::Info => write!(f, "info"),
            SnackSeverity::Warning => write!(f, "warning"),
            SnackSeverity::Error => write!(f, "error"),
        }
    }
}

pub enum SnackVertical {
    Top,
    Bottom,
}
impl fmt::Display for SnackVertical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnackVertical::Top => write!(f, "top"),
            SnackVertical::Bottom => write!(f, "bottom"),
        }
    }
}

pub enum SnackHorizontal {
    Left,
    Center,
    Right,
}
impl fmt::Display for SnackHorizontal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnackHorizontal::Left => write!(f, "left"),
            SnackHorizontal::Center => write!(f, "center"),
            SnackHorizontal::Right => write!(f, "right"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, Hash, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Snack {
    pub severity: String,
    pub message: String,
    pub open: bool,
    pub vertical: String,
    pub horizontal: String,
}
