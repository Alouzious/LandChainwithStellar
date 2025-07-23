#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_init_and_admin() {
    let env = Env::default();
    let admin = Address::random(&env);

    LandRegistryContract::init(env.clone(), admin.clone());
    assert_eq!(LandRegistryContract::get_admin_address(env.clone()), admin);
    assert!(!LandRegistryContract::is_contract_paused(env.clone()));
}

#[test]
fn test_register_land_and_get() {
    let env = Env::default();
    let admin = Address::random(&env);
    LandRegistryContract::init(env.clone(), admin.clone());

    let user = Address::random(&env);
    env.set_invoker(user.clone());

    let result = LandRegistryContract::register_land(
        env.clone(),
        b"John".to_vec(),
        b"Doe".to_vec(),
        b"CountryX".to_vec(),
        b"DistrictA".to_vec(),
        b"SubcountyA".to_vec(),
        b"ParishA".to_vec(),
        b"VillageA".to_vec(),
        b"Plot123".to_vec(),
        b"NIN123456789".to_vec(),
    );

    assert!(result.is_ok());

    let land = LandRegistryContract::get_land(
        env.clone(),
        b"Plot123".to_vec(),
        b"VillageA".to_vec(),
        b"DistrictA".to_vec(),
    )
    .unwrap();

    assert_eq!(land.first_name, b"John".to_vec());
    assert_eq!(land.last_name, b"Doe".to_vec());
    assert_eq!(land.owner, user);
}

#[test]
fn test_pause_and_unpause() {
    let env = Env::default();
    let admin = Address::random(&env);
    LandRegistryContract::init(env.clone(), admin.clone());
    env.set_invoker(admin.clone());

    assert!(!LandRegistryContract::is_contract_paused(env.clone()));
    assert!(LandRegistryContract::pause(env.clone()).is_ok());
    assert!(LandRegistryContract::is_contract_paused(env.clone()));
    assert!(LandRegistryContract::unpause(env.clone()).is_ok());
    assert!(!LandRegistryContract::is_contract_paused(env.clone()));
}

#[test]
fn test_transfer_land() {
    let env = Env::default();
    let admin = Address::random(&env);
    LandRegistryContract::init(env.clone(), admin.clone());

    let user1 = Address::random(&env);
    env.set_invoker(user1.clone());

    LandRegistryContract::register_land(
        env.clone(),
        b"Jane".to_vec(),
        b"Doe".to_vec(),
        b"CountryX".to_vec(),
        b"DistrictA".to_vec(),
        b"SubcountyA".to_vec(),
        b"ParishA".to_vec(),
        b"VillageA".to_vec(),
        b"Plot999".to_vec(),
        b"NIN999999999".to_vec(),
    ).unwrap();

    let user2 = Address::random(&env);
    env.set_invoker(user1.clone()); // Owner must call transfer
    let result = LandRegistryContract::transfer_land(
        env.clone(),
        b"Plot999".to_vec(),
        b"VillageA".to_vec(),
        b"DistrictA".to_vec(),
        user2.clone(),
    );

    assert!(result.is_ok());

    let land = LandRegistryContract::get_land(
        env.clone(),
        b"Plot999".to_vec(),
        b"VillageA".to_vec(),
        b"DistrictA".to_vec(),
    )
    .unwrap();

    assert_eq!(land.owner, user2);
    assert_eq!(land.transfer_history.len(), 1);
    assert_eq!(land.transfer_history.get(0).unwrap().previous_owner, user1);
}

#[test]
fn test_unauthorized_transfer() {
    let env = Env::default();
    let admin = Address::random(&env);
    LandRegistryContract::init(env.clone(), admin.clone());

    let owner = Address::random(&env);
    env.set_invoker(owner.clone());

    LandRegistryContract::register_land(
        env.clone(),
        b"Alex".to_vec(),
        b"Smith".to_vec(),
        b"CountryX".to_vec(),
        b"DistrictB".to_vec(),
        b"SubcountyB".to_vec(),
        b"ParishB".to_vec(),
        b"VillageB".to_vec(),
        b"Plot456".to_vec(),
        b"NIN456789012".to_vec(),
    ).unwrap();

    // Try to transfer from non-owner
    let attacker = Address::random(&env);
    env.set_invoker(attacker.clone());

    let result = LandRegistryContract::transfer_land(
        env.clone(),
        b"Plot456".to_vec(),
        b"VillageB".to_vec(),
        b"DistrictB".to_vec(),
        Address::random(&env),
    );

    assert_eq!(result.unwrap_err(), Error::Unauthorized);
}

