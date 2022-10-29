use gear_lib::StorageProvider;

struct Storage1;
struct Storage2;

#[derive(StorageProvider)]
struct Contract {
    #[storage_field]
    storage1: Storage1,
    #[storage_field]
    storage2: Storage2,
}

fn main() {}
