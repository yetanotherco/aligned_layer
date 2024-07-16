pub mod core {
    pub mod errors;
    pub mod types;
}

pub mod communication {
    pub(crate) mod batch;
    pub(crate) mod messaging;
    pub mod protocol;
}

pub(crate) mod eth {
    pub(crate) mod aligned_service_manager;
    pub(crate) mod batcher_payment_service;
}

pub mod sdk;
