pub mod core {
    pub mod errors;
    pub mod types;
    pub mod utils;
}

pub mod communication {
    mod batch;
    pub(crate) mod messaging;
    pub mod protocol;
}

pub mod eth;

pub mod sdk;
