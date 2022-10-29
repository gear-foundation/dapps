use gear_lib::StorageProvider;

struct Storage;

#[derive(StorageProvider)]
struct Contract {
    storage: Storage,
}

fn main() {}
