use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Serialize, Deserialize, Validate)]
pub struct CraeteWebsiteInput {
    #[validate(url)]
    pub url: String
}
