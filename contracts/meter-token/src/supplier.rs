use crate::storage_types::{DataKey};
use soroban_sdk::{Address, Env};

pub fn read_supplier(e: &Env, consumer: Address) -> Option<Address> {
    let key = DataKey::Supplier(consumer);

    e.storage().persistent().get::<DataKey, Address>(&key)
}

pub fn write_supplier(e: &Env, consumer: Address, supplier: Address) {
    
    let key = DataKey::Supplier(consumer);

    e.storage().persistent().set(&key.clone(), &supplier);
}