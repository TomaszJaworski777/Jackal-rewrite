mod value_network;
mod policy_network;
mod wdl_score;
mod layers;
mod inputs;

pub use wdl_score::{WDLScore, AtomicWDLScore};

use crate::networks::value_network::ValueNetwork;
use crate::networks::policy_network::PolicyNetwork;

#[allow(non_upper_case_globals)]
pub static ValueNetwork: ValueNetwork = unsafe {
    std::mem::transmute(*include_bytes!("../../resources/networks/v600cos3072WDL-TD-OB-007b-Q.network"))
};

#[allow(non_upper_case_globals)]
pub static PolicyNetwork: PolicyNetwork = unsafe {
    std::mem::transmute(*include_bytes!("../../resources/networks/p300cos32x32see005.network"))
};