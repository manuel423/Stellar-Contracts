use soroban_sdk::{contract, contractimpl, Address, Env, symbol_short, Vec, vec, TryFromVal, IntoVal};

#[contract]
pub struct MDTContract;

#[contractimpl]
impl MDTContract {
    pub fn initialize(env: Env, sac: Address, cwt_address: Address) {
        env.storage().persistent().set(&symbol_short!("SAC"), &sac);
        env.storage().persistent().set(&symbol_short!("CWT"), &cwt_address);
    }

    pub fn distribute(env: Env, amount: i128, holders: Vec<Address>) {
        let sac: Address = env.storage().persistent().get(&symbol_short!("SAC")).unwrap();
        sac.require_auth();
        assert!(amount > 0, "Amount must be positive");
        assert!(!holders.is_empty(), "Holders list cannot be empty");

        let cwt_address: Address = env.storage().persistent().get(&symbol_short!("CWT")).unwrap();
        let cwt_total: i128 = i128::try_from_val(&env, &env.invoke_contract::<i128>(
            &cwt_address,
            &symbol_short!("tot_sup"),
            vec![&env],
        )).unwrap_or(0);
        assert!(cwt_total > 0, "CWT total supply must be greater than zero");

        let mut total_distributed: i128 = 0;
        for holder in holders.iter() {
            let cwt_balance: i128 = i128::try_from_val(&env, &env.invoke_contract::<i128>(
                &cwt_address,
                &symbol_short!("bal_of"),
                vec![&env, holder.into_val(&env)],
            )).unwrap_or(0);
            if cwt_balance > 0 {
                let mdt_amount = (cwt_balance * amount) / cwt_total;
                if mdt_amount > 0 {
                    let mut balance: i128 = env.storage().persistent().get(&holder).unwrap_or(0);
                    balance += mdt_amount;
                    env.storage().persistent().set(&holder, &balance);

                    let mut total_supply: i128 = env.storage().persistent().get(&symbol_short!("TOTAL")).unwrap_or(0);
                    total_supply += mdt_amount;
                    env.storage().persistent().set(&symbol_short!("TOTAL"), &total_supply);
                    
                    total_distributed += mdt_amount;
                }
            }
        }

        // Publish event with topics and data
        let topics = (symbol_short!("dist"), total_distributed);
        env.events().publish(topics, holders);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        assert!(amount >= 0, "Amount must be non-negative");

        let mut from_balance: i128 = env.storage().persistent().get(&from).unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");

        if to == env.current_contract_address() {
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

    pub fn balance_of(env: Env, address: Address) -> i128 {
        env.storage().persistent().get(&address).unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().persistent().get(&symbol_short!("TOTAL")).unwrap_or(0)
    }
}