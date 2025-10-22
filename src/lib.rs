#![no_std]
use reflector::{Asset as ReflectorAsset, ReflectorClient};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

mod reflector;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env) {
        let oracle_address = Address::from_str(
            &env,
            "CCYOZJCOPG34LLQQ7N24YXBM7LL62R7ONMZ3G6WZAAYPB5OYKOMJRN63",
        );
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "oracle"), &oracle_address);
    }

    pub fn hello_xlm(env: Env) -> i128 {
        let oracle_address: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "oracle"))
            .unwrap();

        let reflector_client = ReflectorClient::new(&env, &oracle_address);
        let xlm_asset = ReflectorAsset::Other(Symbol::new(&env, "XLM"));
        let price_data = reflector_client.lastprice(&xlm_asset);

        match price_data {
            Some(data) => data.price,
            None => -1,
        }
    }
}
