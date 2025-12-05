pub mod alerting {
    pub mod email; 
}

pub mod stores {
    pub mod steam;
    pub mod gog;
    pub mod microsoft_store;
}

pub mod file_ops {
    pub mod csv;
    pub mod json;
    pub mod settings;
    pub mod thresholds;
}

pub mod structs {
    pub mod data;
    pub mod steam_response;
    pub mod gog_response;
    pub mod microsoft_store_response;
}

pub mod tests {
    //pub mod threshold_data_test;
    pub mod steam_api_test;
    pub mod gog_api_test;
    pub mod microsoft_store_api_test;
}

pub use alerting::email;
pub use stores::{steam, gog, microsoft_store};
pub use file_ops::{csv, json, settings, thresholds};
pub use structs::{data, steam_response, gog_response, microsoft_store_response};
pub use tests::{steam_api_test, gog_api_test, microsoft_store_api_test};//threshold_data_test,