use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "pizza name required"))]
    pub pizza_name: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdatePizzaURL{
    pub uuid: String,
}
