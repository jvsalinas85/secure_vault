use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::state::{OperationType, SecurityConfig};

/// Instrucciones del programa Secure Vault
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VaultInstruction {
    /// Inicializar un nuevo vault
    /// Accounts:
    /// 0. [signer] Admin del vault
    /// 1. [writable] Cuenta del vault (PDA)
    /// 2. [] System program
    InitializeVault {
        config: SecurityConfig,
    },

    /// Depositar fondos en el vault
    /// Accounts:
    /// 0. [signer] Depositante
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de origen de los fondos
    /// 3. [] System program
    Deposit {
        amount: u64,
    },

    /// Retirar fondos del vault
    /// Accounts:
    /// 0. [signer] Solicitante (debe ser manager o admin)
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta destino
    /// 3. [writable] Cuenta de audit log
    Withdraw {
        amount: u64,
        recipient: Pubkey,
    },

    /// Pausar el vault en caso de emergencia
    /// Accounts:
    /// 0. [signer] Admin o emergency contact
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de audit log
    EmergencyPause,

    /// Reanudar operaciones del vault
    /// Accounts:
    /// 0. [signer] Admin
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de audit log
    Resume,

    /// Agregar un nuevo manager
    /// Accounts:
    /// 0. [signer] Admin
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de audit log
    AddManager {
        new_manager: Pubkey,
    },

    /// Remover un manager
    /// Accounts:
    /// 0. [signer] Admin
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de audit log
    RemoveManager {
        manager_to_remove: Pubkey,
    },

    /// Actualizar configuración de seguridad
    /// Accounts:
    /// 0. [signer] Admin
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de audit log
    UpdateSecurityConfig {
        new_config: SecurityConfig,
    },

    /// Transferir administración del vault
    /// Accounts:
    /// 0. [signer] Admin actual
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de audit log
    /// 3. [signer] Nuevo admin (debe firmar para aceptar)
    TransferAdmin {
        new_admin: Pubkey,
    },

    /// Crear operación con delay (time-locked)
    /// Accounts:
    /// 0. [signer] Solicitante
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de operación pendiente
    /// 3. [writable] Cuenta de audit log
    CreateTimeLockOperation {
        operation_type: OperationType,
        amount: u64,
        target_account: Pubkey,
        delay_seconds: i64,
    },

    /// Firmar operación pendiente
    /// Accounts:
    /// 0. [signer] Firmante autorizado
    /// 1. [writable] Cuenta de operación pendiente
    /// 2. [writable] Cuenta de audit log
    SignPendingOperation {
        operation_id: u64,
    },

    /// Ejecutar operación pendiente
    /// Accounts:
    /// 0. [signer] Ejecutor
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta de operación pendiente
    /// 3. [writable] Cuenta destino (si aplica)
    /// 4. [writable] Cuenta de audit log
    ExecutePendingOperation {
        operation_id: u64,
    },

    /// Cancelar operación pendiente
    /// Accounts:
    /// 0. [signer] Admin o creador de la operación
    /// 1. [writable] Cuenta de operación pendiente
    /// 2. [writable] Cuenta de audit log
    CancelPendingOperation {
        operation_id: u64,
    },

    /// Retiro de emergencia (solo admin)
    /// Accounts:
    /// 0. [signer] Admin
    /// 1. [writable] Cuenta del vault
    /// 2. [writable] Cuenta destino de emergencia
    /// 3. [writable] Cuenta de audit log
    EmergencyWithdraw {
        amount: u64,
        emergency_recipient: Pubkey,
    },

    /// Obtener información del vault (read-only)
    /// Accounts:
    /// 0. [] Cuenta del vault
    GetVaultInfo,

    /// Obtener logs de auditoría
    /// Accounts:
    /// 0. [] Cuenta de audit log
    GetAuditLogs {
        from_operation_id: u64,
        limit: u32,
    },
}

impl VaultInstruction {
    /// Deserializar instrucción desde bytes
    pub fn unpack(input: &[u8]) -> Result<Self, std::io::Error> {
        // TODO: Implementar deserialización
        todo!()
    }

    /// Serializar instrucción a bytes
    pub fn pack(&self) -> Vec<u8> {
        // TODO: Implementar serialización
        todo!()
    }

    /// Obtener el tipo de operación de la instrucción
    pub fn get_operation_type(&self) -> Option<OperationType> {
        // TODO: Implementar
        todo!()
    }

    /// Verificar si la instrucción requiere múltiples firmas
    pub fn requires_multisig(&self) -> bool {
        // TODO: Implementar
        todo!()
    }

    /// Obtener el nivel de riesgo de la operación
    pub fn get_risk_level(&self) -> RiskLevel {
        // TODO: Implementar
        todo!()
    }
}

/// Niveles de riesgo para las operaciones
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,    // Depósitos, consultas
    Medium, // Retiros pequeños
    High,   // Retiros grandes, cambios de configuración
    Critical, // Transferencia de admin, emergencias
} 