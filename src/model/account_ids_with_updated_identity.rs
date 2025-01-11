use super::IdentityUpdateTypes;
pub type AccountIdsWithUpdatedIdentity = std::collections::HashMap<
    String,
    Vec<IdentityUpdateTypes>,
>;
