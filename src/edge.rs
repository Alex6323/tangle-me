use crate::id::TransactionId;

#[derive(Debug, Eq, PartialEq)]
pub enum Edge {
    With { id: TransactionId },
    None,
}
