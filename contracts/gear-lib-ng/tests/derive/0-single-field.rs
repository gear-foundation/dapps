use gear_lib::StorageProvider;

struct Storage;

#[derive(StorageProvider)]
struct Contract {
    #[storage_field]
    storage: Storage,
}

fn main() {}
