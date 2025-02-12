use solana_program::program_error::ProgramError;
use std::fmt;

pub type ElusivResult = Result<(), ElusivError>;

#[derive(Copy, Clone)]
#[cfg_attr(any(test, feature = "elusiv-client"), derive(Debug))]
pub enum ElusivError {
    InvalidInstructionData,
    InputsMismatch,
    InvalidOtherInstruction,
    InvalidAmount,
    InsufficientFunds,
    InvalidAccount,
    InvalidRecipient,
    InvalidAccountState,
    NonScalarValue,
    MissingSubAccount,
    FeatureNotAvailable,
    UnsupportedToken,
    OracleError,
    DuplicateValue,

    // Merkle tree
    InvalidMerkleRoot,

    // Nullifier
    CouldNotInsertNullifier,

    // Commitment
    NoRoomForCommitment,
    InvalidBatchingRate,

    // Proof
    InvalidPublicInputs,
    CouldNotProcessProof,

    // Queue
    QueueIsEmpty,
    QueueIsFull,
    InvalidQueueAccess,

    // Archiving
    UnableToArchiveNullifierAccount,
    MerkleTreeIsNotFullYet,

    // Partial computations
    PartialComputationError,
    AccountCannotBeReset,
    ComputationIsNotYetStarted,
    ComputationIsNotYetFinished,
    ComputationIsAlreadyFinished,

    // Fee
    InvalidFee,
    InvalidFeeVersion,

    // Accounts
    SubAccountAlreadyExists,
    SubAccouttDoesNotExists,
}

#[cfg(not(tarpaulin_include))]
impl From<ElusivError> for ProgramError {
    fn from(e: ElusivError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for ElusivError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}
