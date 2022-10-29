use gear_lib::StorageProvider;

#[derive(Clone, Copy)]
struct Storage;

#[derive(StorageProvider)]
enum ContractEnum {
    A {
        #[storage_field]
        storage: Storage,
    },
    B,
}

#[derive(StorageProvider)]
union ContractUnion {
    #[storage_field]
    storage: Storage,
}

fn main() {}
