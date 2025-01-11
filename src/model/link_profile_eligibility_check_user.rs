use serde::{Serialize, Deserialize};
///An object specifying information about the end user who will be sharing their profile in this Link session
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkProfileEligibilityCheckUser {
    ///The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format
    pub phone_number: String,
}
impl std::fmt::Display for LinkProfileEligibilityCheckUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
