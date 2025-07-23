// #![no_std]

// use soroban_sdk::{
//     contractimpl, contracttype,
//     Address, Env, Symbol, Vec, panic_with_error, sha256,
// };

// #[derive(Clone)]
// #[contracttype]
// pub struct TransferRecord {
//     previous_owner: Address,
//     transfer_date: u64,
// }

// #[derive(Clone)]
// #[contracttype]
// pub struct Land {
//     first_name: Vec<u8>,
//     last_name: Vec<u8>,
//     country: Vec<u8>,
//     district: Vec<u8>,
//     subcounty: Vec<u8>,
//     parish: Vec<u8>,
//     village: Vec<u8>,
//     date_registered: u64,
//     plot_number: Vec<u8>,
//     nin_hash: Vec<u8>,
//     owner: Address,
//     transfer_history: Vec<TransferRecord>, // Track all previous owners
//     is_verified: bool,
//     // LC1 Chairman attestation fields
//     lc1_chairman_name: Vec<u8>,
//     lc1_chairman_nin_hash: Vec<u8>,
//     lc1_signature_hash: Vec<u8>,
//     lc1_attestation_date: u64,
//     lc1_verified: bool,
// }

// #[derive(Debug)]
// #[contracttype]
// pub enum Error {
//     Unauthorized,
//     LandAlreadyRegistered,
//     ContractPaused,
//     InvalidInput,
//     LandNotFound,
//     InvalidTimestamp,
//     AdminTransferInProgress,
//     InvalidTransferRequest,
//     LC1ChairmanNotRegistered,
//     InvalidLC1Attestation,
//     LC1AlreadyVerified,
//     LC1ChairmanAlreadyRegistered,
//     CannotTransferToSelf,
//     InvalidNINHash,
// }

// #[contracttype]
// pub struct AdminTransfer {
//     proposed_admin: Address,
//     timestamp: u64,
// }

// #[contracttype]
// pub struct LC1Chairman {
//     name: Vec<u8>,
//     nin_hash: Vec<u8>,
//     village: Vec<u8>,
//     parish: Vec<u8>,
//     subcounty: Vec<u8>,
//     district: Vec<u8>,
//     wallet_address: Address,
//     date_registered: u64,
//     is_active: bool,
//     verified_by_admin: bool,
// }

// #[contracttype]
// pub struct LandRegistryContract;

// // FIXED: Use `static` with Symbol::short() for compile-time symbols
// static ADMIN_KEY: Symbol = Symbol::short("ADMIN");
// static PAUSED_KEY: Symbol = Symbol::short("PAUSED");
// static PENDING_ADMIN_KEY: Symbol = Symbol::short("PENDING_ADMIN");
// static LAND_PREFIX: Symbol = Symbol::short("LAND");
// static LC1_PREFIX: Symbol = Symbol::short("LC1");

// const TRANSFER_TIMEOUT: u64 = 86400; // 24 hours in seconds

// impl LandRegistryContract {
//     // Generate secure unique key using hash to prevent collisions
//     fn get_land_key(env: &Env, plot_number: &Vec<u8>, village: &Vec<u8>, district: &Vec<u8>) -> Vec<u8> {
//         let mut combined = Vec::new();
//         combined.extend_from_slice(plot_number);
//         combined.push(0u8); // null separator
//         combined.extend_from_slice(village);
//         combined.push(0u8); // null separator
//         combined.extend_from_slice(district);
//         // Hash the combined key for security and uniqueness
//         sha256(env, &combined)
//     }

//     // Generate LC1 Chairman key
//     fn get_lc1_key(env: &Env, village: &Vec<u8>, parish: &Vec<u8>, subcounty: &Vec<u8>, district: &Vec<u8>) -> Vec<u8> {
//         let mut combined = Vec::new();
//         combined.extend_from_slice(village);
//         combined.push(0u8);
//         combined.extend_from_slice(parish);
//         combined.push(0u8);
//         combined.extend_from_slice(subcounty);
//         combined.push(0u8);
//         combined.extend_from_slice(district);
//         sha256(env, &combined)
//     }

//     fn validate_inputs(env: &Env, land: &Land) -> Result<(), Error> {
//         // Validate all required fields are not empty
//         if land.first_name.is_empty()
//             || land.last_name.is_empty()
//             || land.country.is_empty()
//             || land.district.is_empty()
//             || land.subcounty.is_empty()
//             || land.parish.is_empty()
//             || land.village.is_empty()
//             || land.plot_number.is_empty()
//             || land.nin_hash.is_empty()
//         {
//             return Err(Error::InvalidInput);
//         }

//         // Validate timestamp is not in the future
//         let current_time = env.ledger().timestamp();
//         if land.date_registered > current_time {
//             return Err(Error::InvalidTimestamp);
//         }

//         // Validate NIN hash length (SHA256 produces 32 bytes)
//         if land.nin_hash.len() != 32 {
//             return Err(Error::InvalidNINHash);
//         }

//         Ok(())
//     }

//     fn validate_lc1_chairman(chairman: &LC1Chairman) -> Result<(), Error> {
//         if chairman.name.is_empty()
//             || chairman.nin_hash.is_empty()
//             || chairman.village.is_empty()
//             || chairman.parish.is_empty()
//             || chairman.subcounty.is_empty()
//             || chairman.district.is_empty()
//         {
//             return Err(Error::InvalidInput);
//         }

//         // Validate NIN hash length
//         if chairman.nin_hash.len() != 32 {
//             return Err(Error::InvalidNINHash);
//         }

//         Ok(())
//     }

//     fn get_admin(env: &Env) -> Address {
//         env.storage().persistent().get(&ADMIN_KEY).unwrap()
//     }

//     fn is_paused(env: &Env) -> bool {
//         env.storage().persistent().get(&PAUSED_KEY).unwrap_or(false)
//     }

//     fn require_admin(env: &Env) -> Result<(), Error> {
//         let admin = Self::get_admin(env);
//         if env.invoker() != admin {
//             return Err(Error::Unauthorized);
//         }
//         Ok(())
//     }

//     fn require_not_paused(env: &Env) -> Result<(), Error> {
//         if Self::is_paused(env) {
//             return Err(Error::ContractPaused);
//         }
//         Ok(())
//     }

//     fn require_land_owner(env: &Env, land: &Land) -> Result<(), Error> {
//         if env.invoker() != land.owner {
//             return Err(Error::Unauthorized);
//         }
//         Ok(())
//     }
// }

// #[contractimpl]
// impl LandRegistryContract {
//     /// Initialize the contract with an admin
//     pub fn init(env: Env, admin: Address) -> Self {
//         admin.require_auth();
        
//         env.storage().persistent().set(&ADMIN_KEY, &admin);
//         env.storage().persistent().set(&PAUSED_KEY, &false);
        
//         env.events().publish(
//             (Symbol::short("ContractInitialized"),),
//             admin.clone(),
//         );
        
//         Self
//     }

//     /// Pause the contract (admin only)
//     pub fn pause(env: Env) -> Result<(), Error> {
//         Self::require_admin(&env)?;
        
//         env.storage().persistent().set(&PAUSED_KEY, &true);
        
//         env.events().publish(
//             (Symbol::short("ContractPaused"),),
//             Self::get_admin(&env),
//         );
        
//         Ok(())
//     }

//     /// Unpause the contract (admin only)
//     pub fn unpause(env: Env) -> Result<(), Error> {
//         Self::require_admin(&env)?;
        
//         env.storage().persistent().set(&PAUSED_KEY, &false);
        
//         env.events().publish(
//             (Symbol::short("ContractUnpaused"),),
//             Self::get_admin(&env),
//         );
        
//         Ok(())
//     }

//     /// Propose a new admin (current admin only)
//     pub fn propose_admin_transfer(env: Env, new_admin: Address) -> Result<(), Error> {
//         Self::require_admin(&env)?;
        
//         let transfer = AdminTransfer {
//             proposed_admin: new_admin.clone(),
//             timestamp: env.ledger().timestamp(),
//         };
        
//         env.storage().persistent().set(&PENDING_ADMIN_KEY, &transfer);
        
//         env.events().publish(
//             (Symbol::short("AdminTransferProposed"),),
//             new_admin,
//         );
        
//         Ok(())
//     }

//     /// Accept admin transfer (proposed admin only)
//     pub fn accept_admin_transfer(env: Env) -> Result<(), Error> {
//         let transfer: AdminTransfer = env.storage().persistent()
//             .get(&PENDING_ADMIN_KEY)
//             .ok_or(Error::InvalidTransferRequest)?;
        
//         if env.invoker() != transfer.proposed_admin {
//             return Err(Error::Unauthorized);
//         }
        
//         // Check if transfer hasn't expired
//         let current_time = env.ledger().timestamp();
//         if current_time > transfer.timestamp + TRANSFER_TIMEOUT {
//             env.storage().persistent().remove(&PENDING_ADMIN_KEY);
//             return Err(Error::InvalidTransferRequest);
//         }
        
//         // Transfer admin rights
//         env.storage().persistent().set(&ADMIN_KEY, &transfer.proposed_admin);
//         env.storage().persistent().remove(&PENDING_ADMIN_KEY);
        
//         env.events().publish(
//             (Symbol::short("AdminTransferCompleted"),),
//             transfer.proposed_admin.clone(),
//         );
        
//         Ok(())
//     }

//     /// Register land with enhanced security and validation
//     pub fn register_land(
//         env: Env,
//         first_name: Vec<u8>,
//         last_name: Vec<u8>,
//         country: Vec<u8>,
//         district: Vec<u8>,
//         subcounty: Vec<u8>,
//         parish: Vec<u8>,
//         village: Vec<u8>,
//         plot_number: Vec<u8>,
//         nin_plain: Vec<u8>,
//     ) -> Result<(), Error> {
//         Self::require_not_paused(&env)?;
        
//         // Validate NIN is provided
//         if nin_plain.is_empty() {
//             return Err(Error::InvalidInput);
//         }
        
//         // Hash the NIN for privacy
//         let nin_hash = sha256(&env, &nin_plain);
        
//         // Create land record with invoker as owner
//         let land = Land {
//             first_name: first_name.clone(),
//             last_name: last_name.clone(),
//             country: country.clone(),
//             district: district.clone(),
//             subcounty: subcounty.clone(),
//             parish: parish.clone(),
//             village: village.clone(),
//             date_registered: env.ledger().timestamp(),
//             plot_number: plot_number.clone(),
//             nin_hash,
//             owner: env.invoker(),
//             transfer_history: Vec::new(), // Initialize empty transfer history
//             is_verified: false,
//             // LC1 Chairman fields - initially empty
//             lc1_chairman_name: Vec::new(),
//             lc1_chairman_nin_hash: Vec::new(),
//             lc1_signature_hash: Vec::new(),
//             lc1_attestation_date: 0,
//             lc1_verified: false,
//         };
        
//         Self::validate_inputs(&env, &land)?;
        
//         let key = Self::get_land_key(&env, &plot_number, &village, &district);
        
//         // Check if land already exists
//         if env.storage().persistent().has(&key) {
//             return Err(Error::LandAlreadyRegistered);
//         }
        
//         // Store land record in persistent storage
//         env.storage().persistent().set(&key, &land);
        
//         // Publish comprehensive event
//         env.events().publish(
//             (Symbol::short("LandRegistered"),),
//             (
//                 land.owner.clone(),
//                 sha256(&env, &key),
//                 land.date_registered,
//             ),
//         );
        
//         Ok(())
//     }

//     /// Get land information with full details
//     pub fn get_land(
//         env: Env, 
//         plot_number: Vec<u8>, 
//         village: Vec<u8>, 
//         district: Vec<u8>
//     ) -> Option<Land> {
//         let key = Self::get_land_key(&env, &plot_number, &village, &district);
//         env.storage().persistent().get(&key)
//     }

//     /// Verify land (admin only)
//     pub fn verify_land(
//         env: Env,
//         plot_number: Vec<u8>,
//         village: Vec<u8>,
//         district: Vec<u8>,
//     ) -> Result<(), Error> {
//         Self::require_admin(&env)?;
        
//         let key = Self::get_land_key(&env, &plot_number, &village, &district);
//         let mut land: Land = env.storage().persistent()
//             .get(&key)
//             .ok_or(Error::LandNotFound)?;
        
//         land.is_verified = true;
//         env.storage().persistent().set(&key, &land);
        
//         env.events().publish(
//             (Symbol::short("LandVerified"),),
//             (
//                 land.owner.clone(),
//                 sha256(&env, &key),
//                 env.ledger().timestamp(),
//             ),
//         );
        
//         Ok(())
//     }

//     /// Transfer land ownership with history tracking
//     pub fn transfer_land(
//         env: Env,
//         plot_number: Vec<u8>,
//         village: Vec<u8>,
//         district: Vec<u8>,
//         new_owner: Address,
//     ) -> Result<(), Error> {
//         Self::require_not_paused(&env)?;
        
//         let key = Self::get_land_key(&env, &plot_number, &village, &district);
//         let mut land: Land = env.storage().persistent()
//             .get(&key)
//             .ok_or(Error::LandNotFound)?;
        
//         // Only current owner can transfer
//         Self::require_land_owner(&env, &land)?;
        
//         // Cannot transfer to self
//         if land.owner == new_owner {
//             return Err(Error::CannotTransferToSelf);
//         }
        
//         // Add current owner to transfer history
//         let transfer_record = TransferRecord {
//             previous_owner: land.owner.clone(),
//             transfer_date: env.ledger().timestamp(),
//         };
        
//         land.transfer_history.push_back(transfer_record);
        
//         let old_owner = land.owner.clone();
//         land.owner = new_owner.clone();
        
//         // Store updated land record
//         env.storage().persistent().set(&key, &land);
        
//         // Emit detailed transfer event
//         env.events().publish(
//             (Symbol::short("LandTransferred"),),
//             (
//                 old_owner,
//                 new_owner,
//                 sha256(&env, &key),
//                 env.ledger().timestamp(),
//             ),
//         );
        
//         Ok(())
//     }

//     /// Update land information (owner only, non-core fields)
//     pub fn update_land_info(
//         env: Env,
//         plot_number: Vec<u8>,
//         village: Vec<u8>,
//         district: Vec<u8>,
//         first_name: Vec<u8>,
//         last_name: Vec<u8>,
//         country: Vec<u8>,
//         subcounty: Vec<u8>,
//         parish: Vec<u8>,
//     ) -> Result<(), Error> {
//         Self::require_not_paused(&env)?;
        
//         let key = Self::get_land_key(&env, &plot_number, &village, &district);
//         let mut land: Land = env.storage().persistent()
//             .get(&key)
//             .ok_or(Error::LandNotFound)?;
        
//         // Only owner can update
//         Self::require_land_owner(&env, &land)?;
        
//         // Validate inputs are not empty
//         if first_name.is_empty() || last_name.is_empty() || country.is_empty() 
//             || subcounty.is_empty() || parish.is_empty() {
//             return Err(Error::InvalidInput);
//         }
        
//         // Update only non-core fields
//         land.first_name = first_name;
//         land.last_name = last_name;
//         land.country = country;
//         land.subcounty = subcounty;
//         land.parish = parish;
//         // Note: Core location data (district, village, plot_number) cannot be changed
        
//         env.storage().persistent().set(&key, &land);
        
//         env.events().publish(
//             (Symbol::short("LandInfoUpdated"),),
//             (
//                 land.owner.clone(),
//                 sha256(&env, &key),
//                 env.ledger().timestamp(),
//             ),
//         );
        
//         Ok(())
//     }

//     /// Get contract admin
//     pub fn get_admin_address(env: Env) -> Address {
//         Self::get_admin(&env)
//     }

//     /// Check if contract is paused
//     pub fn is_contract_paused(env: Env) -> bool {
//         Self::is_paused(&env)
//     }

//     /// Get pending admin transfer info
//     pub fn get_pending_admin_transfer(env: Env) -> Option<AdminTransfer> {
//         env.storage().persistent().get(&PENDING_ADMIN_KEY)
//     }

//     /// Register LC1 Chairman (admin only)
//     pub fn register_lc1_chairman(
//         env: Env,
//         name: Vec<u8>,
//         nin_partial: Vec<u8>,
//         village: Vec<u8>,
//         parish: Vec<u8>,
//         subcounty: Vec<u8>,
//         district: Vec<u8>,
//         chairman_wallet: Address,
//     ) -> Result<(), Error> {
//         Self::require_admin(&env)?;
        
//         // Validate inputs
//         if name.is_empty() || nin_partial.is_empty() || village.is_empty() 
//             || parish.is_empty() || subcounty.is_empty() || district.is_empty() {
//             return Err(Error::InvalidInput);
//         }
        
//         // Hash the partial NIN for privacy
//         let nin_hash = sha256(&env, &nin_partial);
        
//         let chairman = LC1Chairman {
//             name: name.clone(),
//             nin_hash,
//             village: village.clone(),
//             parish: parish.clone(),
//             subcounty: subcounty.clone(),
//             district: district.clone(),
//             wallet_address: chairman_wallet.clone(),
//             date_registered: env.ledger().timestamp(),
//             is_active: true,
//             verified_by_admin: true,
//         };
        
//         Self::validate_lc1_chairman(&chairman)?;
        
//         let key = Self::get_lc1_key(&env, &village, &parish, &subcounty, &district);
        
//         // Check if LC1 chairman already exists for this area
//         if env.storage().persistent().has(&key) {
//             return Err(Error::LC1ChairmanAlreadyRegistered);
//         }
        
//         env.storage().persistent().set(&key, &chairman);
        
//         env.events().publish(
//             (Symbol::short("LC1ChairmanRegistered"),),
//             (
//                 chairman_wallet,
//                 sha256(&env, &key),
//                 env.ledger().timestamp(),
//             ),
//         );
        
//         Ok(())
//     }

//     /// LC1 Chairman attestation for land registration
//     pub fn lc1_attest_land(
//         env: Env,
//         plot_number: Vec<u8>,
//         village: Vec<u8>,
//         district: Vec<u8>,
//         signature_data: Vec<u8>,
//     ) -> Result<(), Error> {
//         Self::require_not_paused(&env)?;
        
//         // Validate signature data
//         if signature_data.is_empty() {
//             return Err(Error::InvalidInput);
//         }
        
//         // Get the land record
//         let land_key = Self::get_land_key(&env, &plot_number, &village, &district);
//         let mut land: Land = env.storage().persistent()
//             .get(&land_key)
//             .ok_or(Error::LandNotFound)?;
        
//         // Check if already verified by LC1
//         if land.lc1_verified {
//             return Err(Error::LC1AlreadyVerified);
//         }
        
//         // Get LC1 Chairman for this area
//         let lc1_key = Self::get_lc1_key(&env, &land.village, &land.parish, &land.subcounty, &land.district);
//         let lc1: LC1Chairman = env.storage().persistent()
//             .get(&lc1_key)
//             .ok_or(Error::LC1ChairmanNotRegistered)?;
        
//         // Only verified and active LC1 Chairman can attest
//         if !lc1.is_active || !lc1.verified_by_admin || env.invoker() != lc1.wallet_address {
//             return Err(Error::Unauthorized);
//         }
        
//         // Attach LC1 details to land record
//         land.lc1_chairman_name = lc1.name.clone();
//         land.lc1_chairman_nin_hash = lc1.nin_hash.clone();
//         land.lc1_signature_hash = sha256(&env, &signature_data);
//         land.lc1_attestation_date = env.ledger().timestamp();
//         land.lc1_verified = true;
        
//         env.storage().persistent().set(&land_key, &land);
        
//         env.events().publish(
//             (Symbol::short("LC1LandAttested"),),
//             (
//                 lc1.wallet_address.clone(),
//                 sha256(&env, &land_key),
//                 env.ledger().timestamp(),
//             ),
//         );
        
//         Ok(())
//     }
// }


































#![no_std]
use soroban_sdk::{
    contractimpl, contracttype,
    Address, Env, Symbol, Vec, panic_with_error,
    sha256,
};

#[derive(Clone)]
#[contracttype]
pub struct TransferRecord {
    previous_owner: Address,
    transfer_date: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Land {
    first_name: Vec<u8>,
    last_name: Vec<u8>,
    country: Vec<u8>,
    district: Vec<u8>,
    subcounty: Vec<u8>,
    parish: Vec<u8>,
    village: Vec<u8>,
    date_registered: u64,
    plot_number: Vec<u8>,
    nin_hash: Vec<u8>,
    owner: Address,
    transfer_history: Vec<TransferRecord>, // Track all previous owners
    is_verified: bool,
    // LC1 Chairman attestation fields
    lc1_chairman_name: Vec<u8>,
    lc1_chairman_nin_hash: Vec<u8>,
    lc1_signature_hash: Vec<u8>,
    lc1_attestation_date: u64,
    lc1_verified: bool,
}

#[derive(Debug)]
#[contracttype]
pub enum Error {
    Unauthorized,
    LandAlreadyRegistered,
    ContractPaused,
    InvalidInput,
    LandNotFound,
    InvalidTimestamp,
    AdminTransferInProgress,
    InvalidTransferRequest,
    LC1ChairmanNotRegistered,
    InvalidLC1Attestation,
    LC1AlreadyVerified,
    LC1ChairmanAlreadyRegistered,
    CannotTransferToSelf,
    InvalidNINHash,
}

#[contracttype]
pub struct AdminTransfer {
    proposed_admin: Address,
    timestamp: u64,
}

#[contracttype]
pub struct LC1Chairman {
    name: Vec<u8>,
    nin_hash: Vec<u8>,
    village: Vec<u8>,
    parish: Vec<u8>,
    subcounty: Vec<u8>,
    district: Vec<u8>,
    wallet_address: Address,
    date_registered: u64,
    is_active: bool,
    verified_by_admin: bool,
}

#[contracttype]
pub struct LandRegistryContract;

const ADMIN_KEY: Symbol = symbol!("ADMIN");
const PAUSED_KEY: Symbol = symbol!("PAUSED");
const PENDING_ADMIN_KEY: Symbol = symbol!("PENDING_ADMIN");
const LAND_PREFIX: Symbol = symbol!("LAND");
const LC1_PREFIX: Symbol = symbol!("LC1");
const TRANSFER_TIMEOUT: u64 = 86400; // 24 hours in seconds

impl LandRegistryContract {
    // Generate secure unique key using hash to prevent collisions
    fn get_land_key(env: &Env, plot_number: &Vec<u8>, village: &Vec<u8>, district: &Vec<u8>) -> Vec<u8> {
        let mut combined = Vec::new();
        combined.extend_from_slice(plot_number);
        combined.push(0u8); // null separator
        combined.extend_from_slice(village);
        combined.push(0u8); // null separator
        combined.extend_from_slice(district);
        
        // Hash the combined key for security and uniqueness
        sha256(env, &combined)
    }

    // Generate LC1 Chairman key
    fn get_lc1_key(env: &Env, village: &Vec<u8>, parish: &Vec<u8>, subcounty: &Vec<u8>, district: &Vec<u8>) -> Vec<u8> {
        let mut combined = Vec::new();
        combined.extend_from_slice(village);
        combined.push(0u8);
        combined.extend_from_slice(parish);
        combined.push(0u8);
        combined.extend_from_slice(subcounty);
        combined.push(0u8);
        combined.extend_from_slice(district);
        
        sha256(env, &combined)
    }

    fn validate_inputs(env: &Env, land: &Land) -> Result<(), Error> {
        // Validate all required fields are not empty
        if land.first_name.is_empty()
            || land.last_name.is_empty()
            || land.country.is_empty()
            || land.district.is_empty()
            || land.subcounty.is_empty()
            || land.parish.is_empty()
            || land.village.is_empty()
            || land.plot_number.is_empty()
            || land.nin_hash.is_empty()
        {
            return Err(Error::InvalidInput);
        }

        // Validate timestamp is not in the future
        let current_time = env.ledger().timestamp();
        if land.date_registered > current_time {
            return Err(Error::InvalidTimestamp);
        }

        // Validate NIN hash length (SHA256 produces 32 bytes)
        if land.nin_hash.len() != 32 {
            return Err(Error::InvalidNINHash);
        }

        Ok(())
    }

    fn validate_lc1_chairman(chairman: &LC1Chairman) -> Result<(), Error> {
        if chairman.name.is_empty()
            || chairman.nin_hash.is_empty()
            || chairman.village.is_empty()
            || chairman.parish.is_empty()
            || chairman.subcounty.is_empty()
            || chairman.district.is_empty()
        {
            return Err(Error::InvalidInput);
        }

        // Validate NIN hash length
        if chairman.nin_hash.len() != 32 {
            return Err(Error::InvalidNINHash);
        }

        Ok(())
    }

    fn get_admin(env: &Env) -> Address {
        env.storage().persistent().get(&ADMIN_KEY).unwrap()
    }

    fn is_paused(env: &Env) -> bool {
        env.storage().persistent().get(&PAUSED_KEY).unwrap_or(false)
    }

    fn require_admin(env: &Env) -> Result<(), Error> {
        let admin = Self::get_admin(env);
        if env.invoker() != admin {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    fn require_not_paused(env: &Env) -> Result<(), Error> {
        if Self::is_paused(env) {
            return Err(Error::ContractPaused);
        }
        Ok(())
    }

    fn require_land_owner(env: &Env, land: &Land) -> Result<(), Error> {
        if env.invoker() != land.owner {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }
}

#[contractimpl]
impl LandRegistryContract {
    /// Initialize the contract with an admin
    pub fn init(env: Env, admin: Address) -> Self {
        admin.require_auth();
        
        env.storage().persistent().set(&ADMIN_KEY, &admin);
        env.storage().persistent().set(&PAUSED_KEY, &false);
        
        env.events().publish(
            (symbol!("ContractInitialized"),),
            admin.clone(),
        );
        
        Self
    }

    /// Pause the contract (admin only)
    pub fn pause(env: Env) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        env.storage().persistent().set(&PAUSED_KEY, &true);
        
        env.events().publish(
            (symbol!("ContractPaused"),),
            Self::get_admin(&env),
        );
        
        Ok(())
    }

    /// Unpause the contract (admin only)
    pub fn unpause(env: Env) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        env.storage().persistent().set(&PAUSED_KEY, &false);
        
        env.events().publish(
            (symbol!("ContractUnpaused"),),
            Self::get_admin(&env),
        );
        
        Ok(())
    }

    /// Propose a new admin (current admin only)
    pub fn propose_admin_transfer(env: Env, new_admin: Address) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        let transfer = AdminTransfer {
            proposed_admin: new_admin.clone(),
            timestamp: env.ledger().timestamp(),
        };
        
        env.storage().persistent().set(&PENDING_ADMIN_KEY, &transfer);
        
        env.events().publish(
            (symbol!("AdminTransferProposed"),),
            new_admin,
        );
        
        Ok(())
    }

    /// Accept admin transfer (proposed admin only)
    pub fn accept_admin_transfer(env: Env) -> Result<(), Error> {
        let transfer: AdminTransfer = env.storage().persistent()
            .get(&PENDING_ADMIN_KEY)
            .ok_or(Error::InvalidTransferRequest)?;
        
        if env.invoker() != transfer.proposed_admin {
            return Err(Error::Unauthorized);
        }
        
        // Check if transfer hasn't expired
        let current_time = env.ledger().timestamp();
        if current_time > transfer.timestamp + TRANSFER_TIMEOUT {
            env.storage().persistent().remove(&PENDING_ADMIN_KEY);
            return Err(Error::InvalidTransferRequest);
        }
        
        // Transfer admin rights
        env.storage().persistent().set(&ADMIN_KEY, &transfer.proposed_admin);
        env.storage().persistent().remove(&PENDING_ADMIN_KEY);
        
        env.events().publish(
            (symbol!("AdminTransferCompleted"),),
            transfer.proposed_admin.clone(),
        );
        
        Ok(())
    }

    /// Register land with enhanced security and validation
    pub fn register_land(
        env: Env,
        first_name: Vec<u8>,
        last_name: Vec<u8>,
        country: Vec<u8>,
        district: Vec<u8>,
        subcounty: Vec<u8>,
        parish: Vec<u8>,
        village: Vec<u8>,
        plot_number: Vec<u8>,
        nin_plain: Vec<u8>,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;
        
        // Validate NIN is provided
        if nin_plain.is_empty() {
            return Err(Error::InvalidInput);
        }
        
        // Hash the NIN for privacy
        let nin_hash = sha256(&env, &nin_plain);
        
        // Create land record with invoker as owner
        let land = Land {
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            country: country.clone(),
            district: district.clone(),
            subcounty: subcounty.clone(),
            parish: parish.clone(),
            village: village.clone(),
            date_registered: env.ledger().timestamp(),
            plot_number: plot_number.clone(),
            nin_hash,
            owner: env.invoker(),
            transfer_history: Vec::new(), // Initialize empty transfer history
            is_verified: false,
            // LC1 Chairman fields - initially empty
            lc1_chairman_name: Vec::new(),
            lc1_chairman_nin_hash: Vec::new(),
            lc1_signature_hash: Vec::new(),
            lc1_attestation_date: 0,
            lc1_verified: false,
        };
        
        Self::validate_inputs(&env, &land)?;
        
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        
        // Check if land already exists
        if env.storage().persistent().has(&key) {
            return Err(Error::LandAlreadyRegistered);
        }
        
        // Store land record in persistent storage
        env.storage().persistent().set(&key, &land);
        
        // Publish comprehensive event
        env.events().publish(
            (symbol!("LandRegistered"),),
            (
                land.owner.clone(),
                sha256(&env, &key),
                land.date_registered,
            ),
        );
        
        Ok(())
    }

    /// Get land information with full details
    pub fn get_land(
        env: Env, 
        plot_number: Vec<u8>, 
        village: Vec<u8>, 
        district: Vec<u8>
    ) -> Option<Land> {
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        env.storage().persistent().get(&key)
    }

    /// Verify land (admin only)
    pub fn verify_land(
        env: Env,
        plot_number: Vec<u8>,
        village: Vec<u8>,
        district: Vec<u8>,
    ) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        let mut land: Land = env.storage().persistent()
            .get(&key)
            .ok_or(Error::LandNotFound)?;
        
        land.is_verified = true;
        env.storage().persistent().set(&key, &land);
        
        env.events().publish(
            (symbol!("LandVerified"),),
            (
                land.owner.clone(),
                sha256(&env, &key),
                env.ledger().timestamp(),
            ),
        );
        
        Ok(())
    }

    /// Transfer land ownership with history tracking
    pub fn transfer_land(
        env: Env,
        plot_number: Vec<u8>,
        village: Vec<u8>,
        district: Vec<u8>,
        new_owner: Address,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;
        
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        let mut land: Land = env.storage().persistent()
            .get(&key)
            .ok_or(Error::LandNotFound)?;
        
        // Only current owner can transfer
        Self::require_land_owner(&env, &land)?;
        
        // Cannot transfer to self
        if land.owner == new_owner {
            return Err(Error::CannotTransferToSelf);
        }
        
        // Add current owner to transfer history
        let transfer_record = TransferRecord {
            previous_owner: land.owner.clone(),
            transfer_date: env.ledger().timestamp(),
        };
        
        land.transfer_history.push_back(transfer_record);
        
        let old_owner = land.owner.clone();
        land.owner = new_owner.clone();
        
        // Store updated land record
        env.storage().persistent().set(&key, &land);
        
        // Emit detailed transfer event
        env.events().publish(
            (symbol!("LandTransferred"),),
            (
                old_owner,
                new_owner,
                sha256(&env, &key),
                env.ledger().timestamp(),
            ),
        );
        
        Ok(())
    }

    /// Update land information (owner only, non-core fields)
    pub fn update_land_info(
        env: Env,
        plot_number: Vec<u8>,
        village: Vec<u8>,
        district: Vec<u8>,
        first_name: Vec<u8>,
        last_name: Vec<u8>,
        country: Vec<u8>,
        subcounty: Vec<u8>,
        parish: Vec<u8>,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;
        
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        let mut land: Land = env.storage().persistent()
            .get(&key)
            .ok_or(Error::LandNotFound)?;
        
        // Only owner can update
        Self::require_land_owner(&env, &land)?;
        
        // Validate inputs are not empty
        if first_name.is_empty() || last_name.is_empty() || country.is_empty() 
            || subcounty.is_empty() || parish.is_empty() {
            return Err(Error::InvalidInput);
        }
        
        // Update only non-core fields
        land.first_name = first_name;
        land.last_name = last_name;
        land.country = country;
        land.subcounty = subcounty;
        land.parish = parish;
        // Note: Core location data (district, village, plot_number) cannot be changed
        
        env.storage().persistent().set(&key, &land);
        
        env.events().publish(
            (symbol!("LandInfoUpdated"),),
            (
                land.owner.clone(),
                sha256(&env, &key),
                env.ledger().timestamp(),
            ),
        );
        
        Ok(())
    }

    /// Get contract admin
    pub fn get_admin_address(env: Env) -> Address {
        Self::get_admin(&env)
    }

    /// Check if contract is paused
    pub fn is_contract_paused(env: Env) -> bool {
        Self::is_paused(&env)
    }

    /// Get pending admin transfer info
    pub fn get_pending_admin_transfer(env: Env) -> Option<AdminTransfer> {
        env.storage().persistent().get(&PENDING_ADMIN_KEY)
    }

    /// Register LC1 Chairman (admin only)
    pub fn register_lc1_chairman(
        env: Env,
        name: Vec<u8>,
        nin_partial: Vec<u8>,
        village: Vec<u8>,
        parish: Vec<u8>,
        subcounty: Vec<u8>,
        district: Vec<u8>,
        chairman_wallet: Address,
    ) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        // Validate inputs
        if name.is_empty() || nin_partial.is_empty() || village.is_empty() 
            || parish.is_empty() || subcounty.is_empty() || district.is_empty() {
            return Err(Error::InvalidInput);
        }
        
        // Hash the partial NIN for privacy
        let nin_hash = sha256(&env, &nin_partial);
        
        let chairman = LC1Chairman {
            name: name.clone(),
            nin_hash,
            village: village.clone(),
            parish: parish.clone(),
            subcounty: subcounty.clone(),
            district: district.clone(),
            wallet_address: chairman_wallet.clone(),
            date_registered: env.ledger().timestamp(),
            is_active: true,
            verified_by_admin: true,
        };
        
        Self::validate_lc1_chairman(&chairman)?;
        
        let key = Self::get_lc1_key(&env, &village, &parish, &subcounty, &district);
        
        // Check if LC1 chairman already exists for this area
        if env.storage().persistent().has(&key) {
            return Err(Error::LC1ChairmanAlreadyRegistered);
        }
        
        env.storage().persistent().set(&key, &chairman);
        
        env.events().publish(
            (symbol!("LC1ChairmanRegistered"),),
            (
                chairman_wallet,
                sha256(&env, &key),
                env.ledger().timestamp(),
            ),
        );
        
        Ok(())
    }

    /// LC1 Chairman attestation for land registration
    pub fn lc1_attest_land(
        env: Env,
        plot_number: Vec<u8>,
        village: Vec<u8>,
        district: Vec<u8>,
        signature_data: Vec<u8>,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;
        
        // Validate signature data
        if signature_data.is_empty() {
            return Err(Error::InvalidInput);
        }
        
        // Get the land record
        let land_key = Self::get_land_key(&env, &plot_number, &village, &district);
        let mut land: Land = env.storage().persistent()
            .get(&land_key)
            .ok_or(Error::LandNotFound)?;
        
        // Check if already verified by LC1
        if land.lc1_verified {
            return Err(Error::LC1AlreadyVerified);
        }
        
        // Get LC1 Chairman for this area
        let lc1_key = Self::get_lc1_key(&env, &village, &land.parish, &land.subcounty, &district);
        let chairman: LC1Chairman = env.storage().persistent()
            .get(&lc1_key)
            .ok_or(Error::LC1ChairmanNotRegistered)?;
        
        // Verify the chairman is active and matches the invoker
        if !chairman.is_active || env.invoker() != chairman.wallet_address {
            return Err(Error::Unauthorized);
        }
        
        // Update land record with LC1 attestation
        land.lc1_chairman_name = chairman.name.clone();
        land.lc1_chairman_nin_hash = chairman.nin_hash.clone();
        land.lc1_signature_hash = sha256(&env, &signature_data);
        land.lc1_attestation_date = env.ledger().timestamp();
        land.lc1_verified = true;
        
        // Store updated land record
        env.storage().persistent().set(&land_key, &land);
        
        env.events().publish(
            (symbol!("LC1LandAttested"),),
            (
                chairman.wallet_address.clone(),
                land.owner.clone(),
                sha256(&env, &land_key),
                env.ledger().timestamp(),
            ),
        );
        
        Ok(())
    }

    /// Get LC1 Chairman information
    pub fn get_lc1_chairman(
        env: Env,
        village: Vec<u8>,
        parish: Vec<u8>,
        subcounty: Vec<u8>,
        district: Vec<u8>,
    ) -> Option<LC1Chairman> {
        let key = Self::get_lc1_key(&env, &village, &parish, &subcounty, &district);
        env.storage().persistent().get(&key)
    }

    /// Deactivate LC1 Chairman (admin only)
    pub fn deactivate_lc1_chairman(
        env: Env,
        village: Vec<u8>,
        parish: Vec<u8>,
        subcounty: Vec<u8>,
        district: Vec<u8>,
    ) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        let key = Self::get_lc1_key(&env, &village, &parish, &subcounty, &district);
        let mut chairman: LC1Chairman = env.storage().persistent()
            .get(&key)
            .ok_or(Error::LC1ChairmanNotRegistered)?;
        
        chairman.is_active = false;
        env.storage().persistent().set(&key, &chairman);
        
        env.events().publish(
            (symbol!("LC1ChairmanDeactivated"),),
            (
                chairman.wallet_address,
                sha256(&env, &key),
                env.ledger().timestamp(),
            ),
        );
        
        Ok(())
    }

    /// Check if land has LC1 verification
    pub fn is_land_lc1_verified(
        env: Env,
        plot_number: Vec<u8>,
        village: Vec<u8>,
        district: Vec<u8>,
    ) -> bool {
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        if let Some(land) = env.storage().persistent().get::<Vec<u8>, Land>(&key) {
            land.lc1_verified
        } else {
            false
        }
    }

    /// Get transfer history for a land parcel
    pub fn get_transfer_history(
        env: Env,
        plot_number: Vec<u8>,
        village: Vec<u8>,
        district: Vec<u8>,
    ) -> Vec<TransferRecord> {
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        if let Some(land) = env.storage().persistent().get::<Vec<u8>, Land>(&key) {
            land.transfer_history
        } else {
            Vec::new()
        }
    }

    /// Get land count for statistics (admin only)
    pub fn get_land_statistics(env: Env) -> Result<u32, Error> {
        Self::require_admin(&env)?;
        
        // Note: This is a placeholder - actual implementation would require
        // maintaining a separate counter or iterating through storage
        // For now, return 0 as this would need additional storage design
        Ok(0)
    }
}