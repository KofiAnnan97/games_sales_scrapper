use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SaleInfo{
    pub icon_link: String,
    pub title: String,
    pub original_price: String,
    pub current_price: String,
    pub discount_percentage: String,
    pub store_page_link: String,
}