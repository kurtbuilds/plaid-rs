use serde::{Serialize, Deserialize};
///A value from a MISMO defined list that identifies the role that the party plays in the transaction. Parties may be either a person or legal entity. A party may play multiple roles in a transaction.A value from a MISMO defined list that identifies the role that the party plays in the transaction. Parties may be either a person or legal entity. A party may play multiple roles in a transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PartyRoleType {
    Borrower,
}
