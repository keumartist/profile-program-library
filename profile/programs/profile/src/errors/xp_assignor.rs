use anchor_lang::error_code;

#[error_code]
pub enum XpAssignorError {
    AlreadyAssignable,
    NotAssignable,
    UnauthorizedAssignee,
    InvalidAssignee,
    InvalidAssignor,
    EmptyAssginors,
    EmptyXpAssigns,
    AlreadyClaimed,
}
