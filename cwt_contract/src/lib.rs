use soroban_sdk::{contract, contractimpl, Address, Env, symbol_short};

#[contract]
pub struct CWTContract;

#[contractimpl]
impl CWTContract {
    pub fn initialize(env: Env, sac: Address) {
        env.storage().persistent().set(&symbol_short!("SAC"), &sac);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let sac: Address = env.storage().persistent().get(&symbol_short!("SAC")).unwrap();
        sac.require_auth();
        assert!(amount > 0, "Amount must be positive");

        let mut balance: i128 = env.storage().persistent().get(&to).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&to, &balance);

        let mut total_supply: i128 = env.storage().persistent().get(&symbol_short!("TOTAL")).unwrap_or(0);
        total_supply += amount;
        env.storage().persistent().set(&symbol_short!("TOTAL"), &total_supply);

        // Publish event with topics and data
        let topics = (symbol_short!("mint"), to, amount);
        env.events().publish(topics, ());
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        assert!(amount >= 0, "Amount must be non-negative");

        let mut from_balance: i128 = env.storage().persistent().get(&from).unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");

        if to == env.current_contract_address() {
            // Only SAC can burn tokens
            let sac: Address = env.storage().persistent().get(&symbol_short!("SAC")).unwrap();
            sac.require_auth();

            from_balance -= amount;
            env.storage().persistent().set(&from, &from_balance);

            let mut total_supply: i128 = env.storage().persistent().get(&symbol_short!("TOTAL")).unwrap();
            total_supply -= amount;
            env.storage().persistent().set(&symbol_short!("TOTAL"), &total_supply);

            // Publish event with topics and data
            let topics = (symbol_short!("burn"), from, amount);
            env.events().publish(topics, ());
        } else {
            from_balance -= amount;
            env.storage().persistent().set(&from, &from_balance);

            let mut to_balance: i128 = env.storage().persistent().get(&to).unwrap_or(0);
            to_balance += amount;
            env.storage().persistent().set(&to, &to_balance);
        }
    }

    pub fn bal_of(env: Env, address: Address) -> i128 {
        env.storage().persistent().get(&address).unwrap_or(0)
    }

    pub fn tot_sup(env: Env) -> i128 {
        env.storage().persistent().get(&symbol_short!("TOTAL")).unwrap_or(0)
    }
}