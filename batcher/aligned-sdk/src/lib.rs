pub mod core {
    pub mod errors;
    pub mod types;
}

pub mod communication {
    mod batch;
    pub(crate) mod messaging;
    pub mod protocol;
}

mod eth;

pub mod sdk;
