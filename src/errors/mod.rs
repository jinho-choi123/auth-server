
#[derive(Debug, PartialEq, Eq)]
pub enum VerificationStatus {
    Success,
    Fail(String),
}