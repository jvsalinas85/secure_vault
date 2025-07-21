use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errores personalizados del Secure Vault
#[derive(Error, Debug, Copy, Clone)]
pub enum VaultError {
    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Vault is paused")]
    VaultPaused,

    #[error("Insufficient balance in vault")]
    InsufficientBalance,

    #[error("Unauthorized access - insufficient permissions")]
    Unauthorized,

    #[error("Invalid account provided")]
    InvalidAccount,

    #[error("Account not initialized")]
    AccountNotInitialized,

    #[error("Account already initialized")]
    AccountAlreadyInitialized,

    #[error("Daily withdrawal limit exceeded")]
    DailyLimitExceeded,

    #[error("Too many operations in short time - rate limited")]
    RateLimited,

    #[error("Operation requires multiple signatures")]
    InsufficientSignatures,

    #[error("Time lock period not elapsed")]
    TimeLockNotElapsed,

    #[error("Operation has expired")]
    OperationExpired,

    #[error("Maximum number of managers reached")]
    MaxManagersReached,

    #[error("Manager not found")]
    ManagerNotFound,

    #[error("Cannot remove the last manager")]
    CannotRemoveLastManager,

    #[error("Invalid security configuration")]
    InvalidSecurityConfig,

    #[error("Amount exceeds maximum allowed")]
    AmountTooLarge,

    #[error("Amount below minimum required")]
    AmountTooSmall,

    #[error("Invalid operation type for current context")]
    InvalidOperationType,

    #[error("Suspicious activity detected - operation blocked")]
    SuspiciousActivity,

    #[error("Emergency mode active - limited operations only")]
    EmergencyModeActive,

    #[error("Invalid timestamp")]
    InvalidTimestamp,

    #[error("Arithmetic overflow")]
    ArithmeticOverflow,

    #[error("Arithmetic underflow")]
    ArithmeticUnderflow,

    #[error("Invalid pubkey")]
    InvalidPubkey,

    #[error("Account data serialization failed")]
    SerializationError,

    #[error("Account data deserialization failed")]
    DeserializationError,

    #[error("Invalid account owner")]
    InvalidAccountOwner,

    #[error("Invalid account size")]
    InvalidAccountSize,

    #[error("Operation not found")]
    OperationNotFound,

    #[error("Operation already executed")]
    OperationAlreadyExecuted,

    #[error("Operation already signed by this account")]
    AlreadySigned,

    #[error("Invalid operation status")]
    InvalidOperationStatus,

    #[error("Audit log full - cannot add more entries")]
    AuditLogFull,

    #[error("Invalid audit log entry")]
    InvalidAuditLogEntry,

    #[error("System clock unavailable")]
    ClockUnavailable,

    #[error("Invalid program derived address")]
    InvalidPDA,

    #[error("Cross-program invocation failed")]
    CPIFailed,

    #[error("Rent exempt minimum not met")]
    NotRentExempt,

    #[error("Invalid token account")]
    InvalidTokenAccount,

    #[error("Token transfer failed")]
    TokenTransferFailed,

    #[error("Invalid mint account")]
    InvalidMintAccount,

    #[error("Slippage tolerance exceeded")]
    SlippageExceeded,

    #[error("Oracle price too stale")]
    StalePriceData,

    #[error("Price manipulation detected")]
    PriceManipulation,

    #[error("Invalid oracle account")]
    InvalidOracle,

    #[error("Emergency contact not authorized")]
    EmergencyContactUnauthorized,

    #[error("Invalid emergency operation")]
    InvalidEmergencyOperation,

    //  ERRORES ESPECÍFICOS PARA PDAs
    #[error("Invalid seeds provided for PDA generation")]
    InvalidSeeds,

    #[error("PDA address mismatch - expected different address")]
    PDAAddressMismatch,

    #[error("Incorrect program ID for PDA")]
    IncorrectProgramId,

    #[error("Invalid bump seed for PDA")]
    InvalidBumpSeed,

    #[error("PDA creation failed")]
    PDACreationFailed,

    #[error("PDA validation failed")]
    PDAValidationFailed,

    #[error("Seeds exceed maximum length")]
    SeedsExceedMaxLength,

    #[error("Too many seeds provided for PDA")]
    TooManySeeds,

    #[error("Empty seeds array provided")]
    EmptySeeds,

    #[error("PDA not found on curve - invalid bump")]
    PDANotFoundOnCurve,

    #[error("PDA account space insufficient")]
    PDASpaceInsufficient,

    #[error("PDA account already exists")]
    PDAAlreadyExists,

    #[error("PDA derivation failed")]
    PDADerivationFailed,

    #[error("Invalid PDA signer")]
    InvalidPDASigner,
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for VaultError {
    fn type_of() -> &'static str {
        "VaultError"
    }
}

impl VaultError {
    /// Obtener código de error numérico
    pub fn error_code(&self) -> u32 {
        *self as u32
    }

    /// Obtener descripción detallada del error
    pub fn detailed_description(&self) -> &'static str {
        match self {
            VaultError::InvalidInstruction => "La instruccion dada no es reconocida",
            VaultError::VaultPaused => "El vault esta pausado y no puede procesar transacciones",
            VaultError::InsufficientBalance => {
                "El vault no tiene suficiente balance para realizar la operacion"
            }
            VaultError::Unauthorized => {
                "Esta account no tiene permiso para realizar esta operacion"
            }
            VaultError::InvalidAccount => "La account dada no es validad para esta operacion",
            VaultError::AccountAlreadyInitialized => "La account ya a sido inicializada",
            VaultError::AccountNotInitialized => "La account no ha sido inicializada",

            // Errores del sistema de PDA
            //TODO
            _ => "Error ocurrido",
        }
    }

    /// Verificar si el error es recuperable
    pub fn is_recoverable(&self) -> bool {
        // TODO: Implementar lógica de recuperabilidad
        todo!()
    }

    /// Obtener nivel de severidad del error
    pub fn severity_level(&self) -> ErrorSeverity {
        match self {
            VaultError::SuspiciousActivity
            | VaultError::PriceManipulation
            | VaultError::EmergencyModeActive => ErrorSeverity::Critical,

            VaultError::Unauthorized
            | VaultError::InvalidAccount
            | VaultError::PDAValidationFailed
            | VaultError::IncorrectProgramId
            | VaultError::InvalidPDASigner => ErrorSeverity::High,

            _ => ErrorSeverity::Low,
        }
    }

    /// Verificar si el error requiere notificación de emergencia
    pub fn requires_emergency_notification(&self) -> bool {
        match self {
            VaultError::SuspiciousActivity
            | VaultError::PriceManipulation
            | VaultError::EmergencyModeActive
            | VaultError::Unauthorized => true,

            _ => false,
        }
    }
}

/// Niveles de severidad de errores
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Low,      // Errores de validación menor
    Medium,   // Errores de operación
    High,     // Errores de seguridad
    Critical, // Errores que requieren intervención inmediata
}

/// Contexto adicional para errores
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation_id: Option<u64>,
    pub account_key: Option<String>,
    pub timestamp: i64,
    pub additional_info: Option<String>,
}

impl ErrorContext {
    /// Crear nuevo contexto de error
    pub fn new() -> Self {
        Self {
            operation_id: None,
            account_key: None,
            timestamp: 0,
            additional_info: None,
        }
    }

    /// Agregar información de operación
    pub fn with_operation_id(mut self, operation_id: u64) -> Self {
        self.operation_id = Some(operation_id);
        self
    }

    /// Agregar información de cuenta
    pub fn with_account_key(mut self, account_key: String) -> Self {
        self.account_key = Some(account_key);
        self
    }

    /// Agregar información adicional
    pub fn with_additional_info(mut self, info: String) -> Self {
        self.additional_info = Some(info);
        self
    }
}
