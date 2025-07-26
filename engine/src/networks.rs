mod value_network;
mod wdl_score;

pub use wdl_score::{WDLScore, AtomicWDLScore};

use crate::networks::value_network::ValueNetwork;

#[allow(non_upper_case_globals)]
pub static ValueNetwork: ValueNetwork = ValueNetwork::new();