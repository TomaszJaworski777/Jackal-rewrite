mod value_network;
mod wdl_score;
mod layers;
mod inputs;

pub use wdl_score::{WDLScore, AtomicWDLScore};

use crate::networks::value_network::ValueNetwork;

#[allow(non_upper_case_globals)]
pub static ValueNetwork: ValueNetwork = unsafe {
    std::mem::transmute(*include_bytes!("../../resources/networks/v600cos3072WDL-TD-OB-007b-Q.network"))
};