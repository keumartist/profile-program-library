use anchor_lang::error_code;

#[error_code]
pub enum ContributionError {
    InactiveContribution,
    CompletedContribution,
    ExceededParticipants,
    DuplicatedParticipant,
    InvalidProposer,
    AlreadyClaimed,
    NotCompletedContribution,
    InvalidParticipant,
}
