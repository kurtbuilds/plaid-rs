use super::AuthUpdateTypes;
pub type AccountIdsWithUpdatedAuth = std::collections::HashMap<
    String,
    Vec<AuthUpdateTypes>,
>;
