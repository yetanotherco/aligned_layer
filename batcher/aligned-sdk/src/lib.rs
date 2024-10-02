pub mod core {
    pub mod constants;
    pub mod errors;
    pub mod types;
}

pub mod communication {
    pub(crate) mod batch;
    pub(crate) mod messaging;
    pub mod protocol;
    pub mod serialization;
}

pub mod eth {
    pub mod aligned_service_manager;
    pub mod batcher_payment_service;
}

pub mod sdk;
