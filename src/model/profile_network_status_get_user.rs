use serde::{Serialize, Deserialize};
///An object specifying information about the end user for the network status check
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileNetworkStatusGetUser {
    ///The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format
    pub phone_number: String,
}
impl std::fmt::Display for ProfileNetworkStatusGetUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
