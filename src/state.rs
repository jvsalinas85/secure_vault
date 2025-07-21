use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{clock::Clock, pubkey::Pubkey, sysvar::Sysvar};

pub const SECONDS_PER_DAY: i64 = 86400;
pub const MAX_MANAGER: usize = 10;
pub const VAULT_DISCRIMINATOR: [u8; 8] = *b"SECVAULT";
pub const MAX_EMERGENCY_CONTACTS: usize = 5;

pub const VAULT_SEED: &[u8] = b"secure_vault";
pub const AUDIT_SEED: &[u8] = b"audit";
pub const PENDING_OP_SEED: &[u8] = b"pending_op";

pub const MAX_SEED_LENGTH: usize = 32;
pub const MAX_SEED_COUNT: usize = 16;

/// Estructura principal del Secure Vault
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct SecureVault {
    pub discriminator: [u8; 8],
    pub admin: Pubkey,
    pub managers: Vec<Pubkey>,
    pub total_balance: u64,
    pub is_paused: bool,
    pub min_signatures_required: u8,
    pub last_operation_timestamp: i64,
    pub daily_withdrawal_limit: u64,
    pub daily_withdrawn_amount: u64,
    pub last_reset_day: i64,
    pub operation_count: u64,
    pub emergency_contacts: Vec<Pubkey>,
    pub created_at: i64,
    pub bump: u8,
}

/// Registro de auditoría para cada operación
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct AuditLog {
    pub operation_id: u64,
    pub operation_type: OperationType,
    pub performer: Pubkey,
    pub timestamp: i64,
    pub amount: Option<u64>,
    pub target_account: Option<Pubkey>,
    pub success: bool,
    pub error_code: Option<u32>,
    pub gas_used: u64,
    pub bump: u8,
}

/// Configuración de seguridad del vault
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct SecurityConfig {
    pub max_daily_operations: u32,
    pub min_time_between_operations: i64,
    pub require_multisig_for_large_amounts: u64,
    pub auto_pause_on_suspicious_activity: bool,
    pub max_managers: u8,
    pub withdrawal_delay_seconds: i64,
}

/// Operación pendiente (para time-locked operations)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct PendingOperation {
    pub id: u64,
    pub operation_type: OperationType,
    pub performer: Pubkey,
    pub amount: u64,
    pub target_account: Pubkey,
    pub scheduled_execution: i64,
    pub required_signatures: Vec<Pubkey>,
    pub received_signatures: Vec<Pubkey>,
    pub created_at: i64,
    pub bump: u8,
}

/// Tipos de operaciones del sistema
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum OperationType {
    Deposit,        // Todos
    Withdraw,       // un Mananger , Admin
    EmergencyPause, // Admin, emergencia
    Resume,         // Solo Admin
    AddManager,
    RemoveManager,
    UpdateConfig,
    TransferAdmin,
    EmergencyWithdraw,
}

/// Roles del sistema
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum Role {
    Admin,
    Manager,
    User,
    EmergencyContact,
}

/// Estado de una operación
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum OperationStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Expired,
}

impl SecureVault {
    pub const MAX_SIZE: usize = 1024; // Tamaño máximo en bytes

    /// Crear un nuevo vault
    pub fn new(admin: Pubkey) -> Self {
        let current_time = Clock::get().unwrap().unix_timestamp;

        Self {
            discriminator: VAULT_DISCRIMINATOR,
            admin,
            managers: Vec::new(),
            total_balance: 0,
            is_paused: false,
            min_signatures_required: 1,
            last_operation_timestamp: current_time,
            daily_withdrawal_limit: u64::MAX,
            daily_withdrawn_amount: 0,
            last_reset_day: current_time,
            operation_count: 0,
            emergency_contacts: Vec::new(),
            created_at: current_time,
            bump: 0,
        }
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }

    pub fn get_seeds(&self) -> Vec<Vec<u8>> {
        vec![b"secure_vault".to_vec(), self.admin.to_bytes().to_vec()]
    }

    /// Verificar si una cuenta tiene el rol especificado
    pub fn has_role(&self, account: &Pubkey, role: Role) -> bool {
        match role {
            Role::Admin => self.admin == *account,
            Role::Manager => self.managers.contains(account),
            Role::User => true, // Todos los usuarios pueden realizar operaciones
            Role::EmergencyContact => self.emergency_contacts.contains(account),
        }
    }

    /// Obtener el rol de una cuenta
    pub fn get_role(&self, account: &Pubkey) -> Option<Role> {
        if self.admin == *account {
            Some(Role::Admin)
        } else if self.managers.contains(account) {
            Some(Role::Manager)
        } else if self.emergency_contacts.contains(account) {
            Some(Role::EmergencyContact)
        } else {
            Some(Role::User)
        }
    }

    /// Verificar si el vault está en estado válido
    pub fn is_valid_state(&self) -> bool {
        self.discriminator == VAULT_DISCRIMINATOR
            && self.admin != Pubkey::default()
            && self.managers.len() <= MAX_MANAGER
            && self.emergency_contacts.len() <= MAX_EMERGENCY_CONTACTS
            && self.daily_withdrawn_amount <= self.daily_withdrawal_limit
            && self.min_signatures_required > 0
    }

    /// Resetear límites diarios si es necesario
    pub fn reset_daily_limits_if_needed(&mut self, current_timestamp: i64) {
        let current_day = current_timestamp / SECONDS_PER_DAY;
        let last_reset_day = self.last_reset_day / SECONDS_PER_DAY;

        if current_day > last_reset_day {
            self.daily_withdrawn_amount = 0;
            self.last_reset_day = current_timestamp;
        }
    }

    pub fn check_authorization(&self, perfomer: &Pubkey, operation: &OperationType) -> bool {
        match operation {
            OperationType::Deposit => true,

            OperationType::Withdraw => self.admin == *perfomer || self.managers.contains(perfomer),

            OperationType::EmergencyPause => {
                self.admin == *perfomer || self.emergency_contacts.contains(perfomer)
            }

            OperationType::Resume
            | OperationType::AddManager
            | OperationType::RemoveManager
            | OperationType::UpdateConfig
            | OperationType::TransferAdmin
            | OperationType::EmergencyWithdraw => self.admin == *perfomer,
        }
    }

    pub fn can_withdraw(&self, amount: u64, current_timestamp: i64) -> Result<(), &'static str> {
        // Capa 1
        if self.is_paused {
            return Err("Vault is paused");
        }

        // Capa 2
        if amount > self.total_balance {
            return Err("Insufficient balance");
        }

        // Capa 3

        let mut temp_vault = self.clone();
        temp_vault.reset_daily_limits_if_needed(current_timestamp);
        if temp_vault.daily_withdrawn_amount + amount > temp_vault.daily_withdrawal_limit {
            return Err("Daily withdrwal limit exceeded");
        }

        Ok(())
    }

    pub fn add_manager(&mut self, new_manager: Pubkey) -> Result<(), &'static str> {
        //capa 1
        if self.managers.len() >= MAX_MANAGER {
            return Err("Maximo numero de manager alcanzado");
        }

        //Capa 2
        if self.managers.contains(&new_manager) {
            return Err("Ya existe este manager");
        }

        //Capa 3
        if new_manager == self.admin {
            return Err("El admin no puede ser manager");
        }
        self.managers.push(new_manager);
        Ok(())
    }

    pub fn remove_manager(&mut self, manager_to_remove: Pubkey) -> Result<(), &'static str> {
        if let Some(pos) = self.managers.iter().position(|&x| x == manager_to_remove) {
            self.managers.remove(pos);
            Ok(())
        } else {
            Err("Manager not found")
        }
    }
}

impl AuditLog {
    pub const MAX_SIZE: usize = 257;

    /// Crear un nuevo log de auditoría
    pub fn new(
        operation_id: u64,
        operation_type: OperationType,
        performer: Pubkey,
        timestamp: i64,
    ) -> Self {
        Self {
            operation_id,
            operation_type,
            performer,
            timestamp,
            amount: None,
            target_account: None,
            success: false,
            error_code: None,
            gas_used: 0,
            bump: 0,
        }
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }

    pub fn get_seeds(&self, vault_pubkey: &Pubkey) -> Vec<Vec<u8>> {
        let day: u32 = { self.timestamp / SECONDS_PER_DAY } as u32;
        vec![
            b"audit".to_vec(),
            vault_pubkey.to_bytes().to_vec(),
            day.to_le_bytes().to_vec(),
            self.operation_id.to_le_bytes().to_vec(),
        ]
    }

    /// Marcar operación como exitosa
    pub fn mark_success(&mut self, gas_used: u64) {
        self.success = true;
        self.gas_used = gas_used;
        self.error_code = None;
    }

    /// Marcar operación como fallida
    pub fn mark_failure(&mut self, error_code: u32, gas_used: u64) {
        self.success = false;
        self.gas_used = gas_used;
        self.error_code = Some(error_code);
    }

    pub fn with_amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn with_target_account(mut self, target: Pubkey) -> Self {
        self.target_account = Some(target);
        self
    }
}

impl SecurityConfig {
    /// Configuración por defecto
    pub fn default() -> Self {
        Self {
            max_daily_operations: 100,
            min_time_between_operations: 60,
            require_multisig_for_large_amounts: 10_000_000_00, // 10 SOL
            max_managers: 5,
            withdrawal_delay_seconds: 3600,
            auto_pause_on_suspicious_activity: false,
        }
    }

    /// Configuración de alta seguridad
    pub fn high_security() -> Self {
        Self {
            max_daily_operations: 10,
            min_time_between_operations: 600,
            require_multisig_for_large_amounts: 10_000_000, // 1 SOL
            max_managers: 3,
            withdrawal_delay_seconds: 3600,
            auto_pause_on_suspicious_activity: true,
        }
    }

    /// Validar que la configuración es válida
    pub fn is_valid(&self) -> bool {
        if self.max_daily_operations <= 5000 {
            return false;
        }
        true
    }
}

impl PendingOperation {
    pub const MAX_SIZE: usize = 513;

    /// Crear nueva operación pendiente
    pub fn new(
        id: u64,
        operation_type: OperationType,
        performer: Pubkey,
        amount: u64,
        target_account: Pubkey,
        delay_seconds: i64,
        required_signatures: Vec<Pubkey>,
    ) -> Self {
        let current_timestamp = Clock::get().unwrap().unix_timestamp;
        Self {
            id,
            operation_type,
            performer,
            amount,
            target_account,
            scheduled_execution: current_timestamp + delay_seconds,
            required_signatures,
            received_signatures: Vec::new(),
            created_at: current_timestamp,
            bump: 0,
        }
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }

    pub fn get_seeds(&self, vault_pubkey: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            b"pending_op".to_vec(),
            vault_pubkey.to_bytes().to_vec(),
            self.id.to_le_bytes().to_vec(),
        ]
    }

    /// Verificar si la operación está lista para ejecutar
    pub fn is_ready_to_execute(&self, current_timestamp: i64) -> bool {
        current_timestamp >= self.scheduled_execution && self.has_sufficient_signatures()
    }

    /// Agregar firma a la operación
    pub fn add_signature(&mut self, signer: Pubkey) -> bool {
        if !self.required_signatures.contains(&signer) {
            return false;
        }

        if self.received_signatures.contains(&signer) {
            return false;
        }

        self.received_signatures.push(signer);
        true
    }

    /// Verificar si tiene suficientes firmas
    pub fn has_sufficient_signatures(&self) -> bool {
        self.received_signatures.len() >= self.required_signatures.len()
    }

    /// Verificar si la operación ha expirado
    pub fn is_expired(&self, current_timestamp: i64, expiry_hours: i64) -> bool {
        let expiry_time: i64 = self.created_at + expiry_hours * 3600;
        current_timestamp > expiry_time
    }
}
