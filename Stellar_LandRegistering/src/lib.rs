#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    Address, Vec, Bytes, Env,
    symbol_short
};

#[derive(Clone)]
#[contracttype]
pub struct TransferRecord {
    pub previous_owner: Address,
    pub transfer_date: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Land {
    pub first_name: Bytes,
    pub last_name: Bytes,
    pub country: Bytes,
    pub district: Bytes,
    pub subcounty: Bytes,
    pub parish: Bytes,
    pub village: Bytes,
    pub date_registered: u64,
    pub plot_number: Bytes,
    pub nin_hash: Bytes,
    pub owner: Address,
    pub transfer_history: Vec<TransferRecord>,
    pub is_verified: bool,
    pub lc1_chairman_name: Bytes,
    pub lc1_chairman_nin_hash: Bytes,
    pub lc1_signature_hash: Bytes,
    pub lc1_attestation_date: u64,
    pub lc1_verified: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct AdminTransfer {
    pub proposed_admin: Address,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct LC1Chairman {
    pub name: Bytes,
    pub nin_hash: Bytes,
    pub village: Bytes,
    pub parish: Bytes,
    pub subcounty: Bytes,
    pub district: Bytes,
    pub wallet_address: Address,
    pub date_registered: u64,
    pub is_active: bool,
    pub verified_by_admin: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Paused,
    PendingAdmin,
    Land(Bytes),
    LC1(Bytes),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Unauthorized = 1,
    LandAlreadyRegistered = 2,
    ContractPaused = 3,
    InvalidInput = 4,
    LandNotFound = 5,
    InvalidTimestamp = 6,
    AdminTransferInProgress = 7,
    InvalidTransferRequest = 8,
    LC1ChairmanNotRegistered = 9,
    InvalidLC1Attestation = 10,
    LC1AlreadyVerified = 11,
    LC1ChairmanAlreadyRegistered = 12,
    CannotTransferToSelf = 13,
    InvalidNINHash = 14,
}

const TRANSFER_TIMEOUT: u64 = 86400; // 24 hours in seconds

#[contract]
pub struct LandRegistryContract;

impl Land {
    fn validate(&self, env: &Env) -> Result<(), Error> {
        if self.first_name.is_empty()
            || self.last_name.is_empty()
            || self.country.is_empty()
            || self.district.is_empty()
            || self.subcounty.is_empty()
            || self.parish.is_empty()
            || self.village.is_empty()
            || self.plot_number.is_empty()
            || self.nin_hash.is_empty()
        {
            return Err(Error::InvalidInput);
        }

        if self.date_registered > env.ledger().timestamp() {
            return Err(Error::InvalidTimestamp);
        }

        if self.nin_hash.len() != 32 {
            return Err(Error::InvalidNINHash);
        }

        Ok(())
    }
}

impl LC1Chairman {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty()
            || self.nin_hash.is_empty()
            || self.village.is_empty()
            || self.parish.is_empty()
            || self.subcounty.is_empty()
            || self.district.is_empty()
        {
            return Err(Error::InvalidInput);
        }

        if self.nin_hash.len() != 32 {
            return Err(Error::InvalidNINHash);
        }

        Ok(())
    }
}

#[contractimpl]
impl LandRegistryContract {
    fn get_admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Admin)
            .unwrap()
    }

    fn is_paused(env: &Env) -> bool {
        env.storage()
            .instance()
            .get::<DataKey, bool>(&DataKey::Paused)
            .unwrap_or(false)
    }

    fn require_admin(env: &Env) -> Result<(), Error> {
        let admin = Self::get_admin(env);
        admin.require_auth();
        Ok(())
    }

    fn require_not_paused(env: &Env) -> Result<(), Error> {
        if Self::is_paused(env) {
            return Err(Error::ContractPaused);
        }
        Ok(())
    }

    fn require_land_owner(_env: &Env, land: &Land) -> Result<(), Error> {
        land.owner.require_auth();
        Ok(())
    
}

    fn get_land_key(env: &Env, plot_number: &Bytes, village: &Bytes, district: &Bytes) -> Bytes {
        let mut combined = Bytes::new(env);
        combined.append(&plot_number);
        combined.append(&Bytes::from_slice(env, &[0]));
        combined.append(&village);
        combined.append(&Bytes::from_slice(env, &[0]));
        combined.append(&district);
        env.crypto().sha256(&combined).into()
    }

    fn get_lc1_key(env: &Env, village: &Bytes, parish: &Bytes, subcounty: &Bytes, district: &Bytes) -> Bytes {
        let mut combined = Bytes::new(env);
        combined.append(&village);
        combined.append(&Bytes::from_slice(env, &[0]));
        combined.append(&parish);
        combined.append(&Bytes::from_slice(env, &[0]));
        combined.append(&subcounty);
        combined.append(&Bytes::from_slice(env, &[0]));
        combined.append(&district);
        env.crypto().sha256(&combined).into()
    }

    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Paused, &false);
        
        env.events().publish(
            (symbol_short!("init"),),
            admin,
        );
    }

    pub fn pause(env: Env) -> Result<(), Error> {
        Self::require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &true);
        env.events().publish(
            (symbol_short!("paused"),),
            true,
        );
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), Error> {
        Self::require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &false);
        env.events().publish(
            (symbol_short!("unpaused"),),
            false,
        );
        Ok(())
    }

    pub fn propose_admin_transfer(env: Env, new_admin: Address) -> Result<(), Error> {
        Self::require_admin(&env)?;
        let transfer = AdminTransfer {
            proposed_admin: new_admin.clone(),
            timestamp: env.ledger().timestamp(),
        };
        env.storage().instance().set(&DataKey::PendingAdmin, &transfer);
        env.events().publish(
            (symbol_short!("admn_tran"),),
            new_admin,
        );
        Ok(())
    }

    pub fn accept_admin_transfer(env: Env, new_admin: Address) -> Result<(), Error> {
        let transfer: AdminTransfer = env.storage().instance()
            .get::<DataKey, AdminTransfer>(&DataKey::PendingAdmin)
            .ok_or(Error::InvalidTransferRequest)?;
        
        new_admin.require_auth();
        
        if new_admin != transfer.proposed_admin {
            return Err(Error::Unauthorized);
        }
        
        let current_time = env.ledger().timestamp();
        if current_time > transfer.timestamp + TRANSFER_TIMEOUT {
            env.storage().instance().remove(&DataKey::PendingAdmin);
            return Err(Error::InvalidTransferRequest);
        }
        
        env.storage().instance().set(&DataKey::Admin, &transfer.proposed_admin);
        env.storage().instance().remove(&DataKey::PendingAdmin);
        
        env.events().publish(
            (symbol_short!("admn_acpt"),),
            transfer.proposed_admin,
        );
        
        Ok(())
    }

    pub fn register_land(
        env: Env,
        owner: Address,
        first_name: Bytes,
        last_name: Bytes,
        country: Bytes,
        district: Bytes,
        subcounty: Bytes,
        parish: Bytes,
        village: Bytes,
        plot_number: Bytes,
        nin_plain: Bytes,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;
        owner.require_auth();
        
        if nin_plain.is_empty() {
            return Err(Error::InvalidInput);
        }
        
        let nin_hash = env.crypto().sha256(&nin_plain).into();
        
        let land = Land {
            first_name,
            last_name,
            country,
            district,
            subcounty,
            parish,
            village,
            date_registered: env.ledger().timestamp(),
            plot_number,
            nin_hash,
            owner: owner.clone(),
            transfer_history: Vec::new(&env),
            is_verified: false,
            lc1_chairman_name: Bytes::new(&env),
            lc1_chairman_nin_hash: Bytes::new(&env),
            lc1_signature_hash: Bytes::new(&env),
            lc1_attestation_date: 0,
            lc1_verified: false,
        };
        
        land.validate(&env)?;
        
        let key = Self::get_land_key(&env, &land.plot_number, &land.village, &land.district);
        
        if env.storage().persistent().has(&DataKey::Land(key.clone())) {
            return Err(Error::LandAlreadyRegistered);
        }
        
        env.storage().persistent().set(&DataKey::Land(key), &land);
        
        env.events().publish(
            (symbol_short!("land_reg"),),
            land.owner,
        );
        
        Ok(())
    }

    pub fn get_land(
        env: Env, 
        plot_number: Bytes, 
        village: Bytes, 
        district: Bytes
    ) -> Option<Land> {
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        env.storage().persistent().get::<DataKey, Land>(&DataKey::Land(key))
    }

    pub fn verify_land(
        env: Env,
        plot_number: Bytes,
        village: Bytes,
        district: Bytes,
    ) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        let mut land: Land = env.storage().persistent()
            .get::<DataKey, Land>(&DataKey::Land(key.clone()))
            .ok_or(Error::LandNotFound)?;
        
        land.is_verified = true;
        env.storage().persistent().set(&DataKey::Land(key), &land);
        
        env.events().publish(
            (symbol_short!("land_ver"),),
            land.owner,
        );
        
        Ok(())
    }

    pub fn transfer_land(
        env: Env,
        plot_number: Bytes,
        village: Bytes,
        district: Bytes,
        new_owner: Address,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;
        
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        let mut land: Land = env.storage().persistent()
            .get::<DataKey, Land>(&DataKey::Land(key.clone()))
            .ok_or(Error::LandNotFound)?;
        
        Self::require_land_owner(&env, &land)?;
        
        if land.owner == new_owner {
            return Err(Error::CannotTransferToSelf);
        }
        
        let transfer_record = TransferRecord {
            previous_owner: land.owner.clone(),
            transfer_date: env.ledger().timestamp(),
        };
        
        land.transfer_history.push_back(transfer_record);
        let old_owner = land.owner.clone();
        land.owner = new_owner.clone();
        
        env.storage().persistent().set(&DataKey::Land(key), &land);
        
        env.events().publish(
            (symbol_short!("land_tran"),),
            (old_owner, new_owner),
        );
        
        Ok(())
    }

    pub fn update_land_info(
        env: Env,
        plot_number: Bytes,
        village: Bytes,
        district: Bytes,
        first_name: Bytes,
        last_name: Bytes,
        country: Bytes,
        subcounty: Bytes,
        parish: Bytes,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;
        
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        let mut land: Land = env.storage().persistent()
            .get::<DataKey, Land>(&DataKey::Land(key.clone()))
            .ok_or(Error::LandNotFound)?;
        
        Self::require_land_owner(&env, &land)?;
        
        if first_name.is_empty() || last_name.is_empty() || country.is_empty() 
            || subcounty.is_empty() || parish.is_empty() {
            return Err(Error::InvalidInput);
        }
        
        land.first_name = first_name;
        land.last_name = last_name;
        land.country = country;
        land.subcounty = subcounty;
        land.parish = parish;
        
        env.storage().persistent().set(&DataKey::Land(key), &land);
        
        env.events().publish(
            (symbol_short!("land_upd"),),
            land.owner,
        );
        
        Ok(())
    }

    pub fn get_admin_address(env: Env) -> Address {
        Self::get_admin(&env)
    }

    pub fn is_contract_paused(env: Env) -> bool {
        Self::is_paused(&env)
    }

    pub fn get_pending_admin_transfer(env: Env) -> Option<AdminTransfer> {
        env.storage().instance().get::<DataKey, AdminTransfer>(&DataKey::PendingAdmin)
    }

    pub fn register_lc1_chairman(
        env: Env,
        name: Bytes,
        nin_partial: Bytes,
        village: Bytes,
        parish: Bytes,
        subcounty: Bytes,
        district: Bytes,
        chairman_wallet: Address,
    ) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        if name.is_empty() || nin_partial.is_empty() || village.is_empty() 
            || parish.is_empty() || subcounty.is_empty() || district.is_empty() {
            return Err(Error::InvalidInput);
        }
        
        let nin_hash = env.crypto().sha256(&nin_partial).into();
        
        let chairman = LC1Chairman {
            name,
            nin_hash,
            village,
            parish,
            subcounty,
            district,
            wallet_address: chairman_wallet.clone(),
            date_registered: env.ledger().timestamp(),
            is_active: true,
            verified_by_admin: true,
        };
        
        chairman.validate()?;
        
        let key = Self::get_lc1_key(&env, &chairman.village, &chairman.parish, &chairman.subcounty, &chairman.district);
        
        if env.storage().persistent().has(&DataKey::LC1(key.clone())) {
            return Err(Error::LC1ChairmanAlreadyRegistered);
        }
        
        env.storage().persistent().set(&DataKey::LC1(key), &chairman);
        
        env.events().publish(
            (symbol_short!("lc1_reg"),),
            chairman_wallet,
        );
        
        Ok(())
    }

    pub fn lc1_attest_land(
        env: Env,
        plot_number: Bytes,
        village: Bytes,
        district: Bytes,
        signature_data: Bytes,
        chairman_wallet: Address,
    ) -> Result<(), Error> {
        Self::require_not_paused(&env)?;
        chairman_wallet.require_auth();
        
        if signature_data.is_empty() {
            return Err(Error::InvalidInput);
        }
        
        let land_key = Self::get_land_key(&env, &plot_number, &village, &district);
        let mut land: Land = env.storage().persistent()
            .get::<DataKey, Land>(&DataKey::Land(land_key.clone()))
            .ok_or(Error::LandNotFound)?;
        
        if land.lc1_verified {
            return Err(Error::LC1AlreadyVerified);
        }
        
        let lc1_key = Self::get_lc1_key(&env, &village, &land.parish, &land.subcounty, &district);
        let chairman: LC1Chairman = env.storage().persistent()
            .get::<DataKey, LC1Chairman>(&DataKey::LC1(lc1_key))
            .ok_or(Error::LC1ChairmanNotRegistered)?;
        
        if !chairman.is_active || chairman_wallet != chairman.wallet_address {
            return Err(Error::Unauthorized);
        }
        
        land.lc1_chairman_name = chairman.name;
        land.lc1_chairman_nin_hash = chairman.nin_hash;
        land.lc1_signature_hash = env.crypto().sha256(&signature_data).into();
        land.lc1_attestation_date = env.ledger().timestamp();
        land.lc1_verified = true;
        
        env.storage().persistent().set(&DataKey::Land(land_key), &land);
        
        env.events().publish(
            (symbol_short!("lc1_attst"),),
            chairman.wallet_address,
        );
        
        Ok(())
    }

    pub fn get_lc1_chairman(
        env: Env,
        village: Bytes,
        parish: Bytes,
        subcounty: Bytes,
        district: Bytes,
    ) -> Option<LC1Chairman> {
        let key = Self::get_lc1_key(&env, &village, &parish, &subcounty, &district);
        env.storage().persistent().get::<DataKey, LC1Chairman>(&DataKey::LC1(key))
    }

    pub fn deactivate_lc1_chairman(
        env: Env,
        village: Bytes,
        parish: Bytes,
        subcounty: Bytes,
        district: Bytes,
    ) -> Result<(), Error> {
        Self::require_admin(&env)?;
        
        let key = Self::get_lc1_key(&env, &village, &parish, &subcounty, &district);
        let mut chairman: LC1Chairman = env.storage().persistent()
            .get::<DataKey, LC1Chairman>(&DataKey::LC1(key.clone()))
            .ok_or(Error::LC1ChairmanNotRegistered)?;
        
        chairman.is_active = false;
        env.storage().persistent().set(&DataKey::LC1(key), &chairman);
        
        env.events().publish(
            (symbol_short!("lc1_deact"),),
            chairman.wallet_address,
        );
        
        Ok(())
    }

    pub fn is_land_lc1_verified(
        env: Env,
        plot_number: Bytes,
        village: Bytes,
        district: Bytes,
    ) -> bool {
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        if let Some(land) = env.storage().persistent().get::<DataKey, Land>(&DataKey::Land(key)) {
            land.lc1_verified
        } else {
            false
        }
    }

    pub fn get_transfer_history(
        env: Env,
        plot_number: Bytes,
        village: Bytes,
        district: Bytes,
    ) -> Vec<TransferRecord> {
        let key = Self::get_land_key(&env, &plot_number, &village, &district);
        if let Some(land) = env.storage().persistent().get::<DataKey, Land>(&DataKey::Land(key)) {
            land.transfer_history
        } else {
            Vec::new(&env)
        }
    }

    pub fn get_land_statistics(env: Env) -> Result<u32, Error> {
        Self::require_admin(&env)?;
        Ok(0) // Placeholder - would need storage redesign
    }
}