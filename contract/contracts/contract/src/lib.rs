#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contract]
pub struct TokenBurn;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    Admin,
    TotalSupply,
}

#[contractimpl]
impl TokenBurn {

    // Initialize contract with admin
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }

    // Mint tokens (admin only)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        if amount <= 0 {
            panic!("Invalid amount");
        }

        let key = DataKey::Balance(to.clone());
        let balance: i128 = env.storage().instance().get(&key).unwrap_or(0);
        env.storage().instance().set(&key, &(balance + amount));

        let total: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(total + amount));
    }

    // Check balance
    pub fn balance(env: Env, user: Address) -> i128 {
        let key = DataKey::Balance(user);
        env.storage().instance().get(&key).unwrap_or(0)
    }

    // Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    // Burn tokens (user burns their own tokens)
    pub fn burn(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Invalid burn amount");
        }

        let key = DataKey::Balance(user.clone());
        let balance: i128 = env.storage().instance().get(&key).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient balance");
        }

        // Reduce user balance
        env.storage().instance().set(&key, &(balance - amount));

        // Reduce total supply
        let total: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(total - amount));
    }
}