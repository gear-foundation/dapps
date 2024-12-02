## Proxy example

Proxy programs are a design pattern in blockchain development used to delegate function calls to a separate logic program. This separation of responsibilities allows developers to modify the business logic of a program without changing its state or address. Proxy programs act as intermediaries, forwarding user interactions to a logic program, making the system both modular and upgradeable.

To understand how proxy programs function, let’s explore a practical implementation. Below, we detail a proxy program designed to forward calls to a logic program. This proxy program manages two key responsibilities:

1. Forwarding **state-modifying messages** (e.g., to update data) while preserving the original sender's address.
2. Handling **state queries** to retrieve data from the logic program without making changes.
This implementation also includes mechanisms for updating the logic program address, ensuring compatibility, and maintaining administrative control. Let's break down the code and its functionality.

#### Message Forwarding
1. **execute_msg**:
- Forwards state-modifying messages to the logic program.
- Includes the original sender (`msg::source()`) in the forwarded message payload to preserve accountability.
```rust
pub async fn execute_msg(&mut self, bytes: Vec<u8>) -> Vec<u8> {
    let original_sender = Some(msg::source());
    let sender_encoded = original_sender.encode();
    let mut new_payload = bytes.clone();
    new_payload.extend(sender_encoded);
    msg::send_bytes_for_reply(
        self.get().logic_address,
        new_payload,
        msg::value(),
        REPLY_DEPOSIT,
    )
    .expect("Error during message sending")
    .await
    .expect("Error during getting the reply")
}
```
2. **read_state**:
- Handles state queries by forwarding the request to the logic program without modifying state.
```rust
pub async fn read_state(&self, bytes: Vec<u8>) -> Vec<u8> {
    msg::send_bytes_for_reply(self.get().logic_address, bytes, 0, 0)
        .expect("Error during message sending")
        .await
        .expect("Error during getting the reply")
}
```
#### Logic Program Updates
The **update_logic** method allows the proxy's admin to change the logic program address. 
```rust
pub async fn read_state(&self, bytes: Vec<u8>) -> Vec<u8> {
    msg::send_bytes_for_reply(self.get().logic_address, bytes, 0, 0)
        .expect("Error during message sending")
        .await
        .expect("Error during getting the reply")
}
```
### Creating a Proxy-Compatible Program
To make a program compatible with a proxy, you need to implement certain features that allow the proxy to forward messages effectively while preserving the original sender's address. Below is an explanation of the key adjustments and features added to the `Counter` program to support a proxy.

#### Key Features of a Proxy-Compatible Program
1. Proxy Address Registration:
The program must allow an admin to set the address of the proxy, which is responsible for forwarding messages.
2. Forwarded Sender (`msg_source()`):

    When the proxy forwards a message, it includes the original sender's address (`msg_source()`). The program verifies this sender and distinguishes between direct and proxied calls.
```rust
fn get_msg_source(&self, msg_source: Option<ActorId>) -> ActorId {
    if self.get().proxy_address.is_some() {
        msg_source.expect("msg_source must be set through proxy")
    } else {
        msg::source()
    }
}
```
3. Forwarding Logic in Proxy-Compatible Methods:

    To ensure compatibility with a proxy, key methods in the program include an additional parameter of type `Option<ActorId>`. This parameter (`msg_source`) specifies the actual sender of the message:
    - If the program is called via a proxy:
The proxy forwards the `ActorId` of the original sender, and `msg_source` will be `Some(ActorId)`.
    - If the program is called directly:
`msg_source` will be `None`, and the program defaults to using `msg::source()` to determine the sender.
```rust 
pub fn contribute(&mut self, msg_source: Option<ActorId>) -> u128 {
    self.check_if_proxy();
    let msg_source = self.get_msg_source(msg_source); 
    let amount = msg::value();
    assert!(amount > 0, "Contribution must be greater than zero");
    
    let state = self.get_mut();
    let contribution = state.contributions.entry(msg_source).or_insert(0);
    *contribution += amount;
    state.value += amount;
    state.value
}
```

#### State Migration
State migration allows the transfer of critical program data from an old program to a new one, enabling smooth upgrades while preserving important state variables. In this example, the migration involves exporting specific fields (`value`, `limit`, and `contributions`) and importing them into a new instance of the program.
1. Exporting State
The `export_migration_state` function serializes only the necessary fields from the program's state and encodes them for transfer.
```rust
/// Exports the essential state for migration.
/// Only includes `value`, `limit`, and `contributions` fields.
pub fn export_migration_state(&self) -> Vec<u8> {
    let state = self.get();
    let export_data = (state.value, state.limit, state.contributions.clone());
    export_data.encode()
}
```
2. Importing State
The `import_migration_state` function allows a new program to decode and load the state exported from the old program.
```rust
/// Imports the state from the previous program.
/// Decodes and applies `value`, `limit`, and `contributions` fields.
pub fn import_migration_state(&mut self, encoded_state: Vec<u8>) {
    let (value, limit, contributions) =
        <(u128, u128, BTreeMap<ActorId, u128>)>::decode(&mut encoded_state.as_ref())
            .expect("Failed to decode migration state");

    let state = self.get_mut();

    state.value = value;
    state.limit = limit;
    state.contributions = contributions;
}
```
### Kill Function
The kill function is used to stop the execution of the current program and transfer its remaining balance to a specified inheritor (e.g., a newly deployed program). This function is typically invoked by the admin as part of an upgrade process.
```rust
/// Stops the execution of the current program and transfers its remaining balance
/// to the specified inheritor (e.g., a new program).
pub async fn kill(&mut self, inheritor: ActorId, msg_source: Option<ActorId>) {
        self.check_if_proxy();
        let msg_source = self.get_msg_source(msg_source);
        self.only_admin(msg_source);
        exec::exit(inheritor);
    }

```
#### Considerations for State Migration
While state migration is essential for upgrading programs, developers must address the following challenge:

1. **Storage Size Limitations**:
    - Programs with large or complex states may exceed the gas limit for a single transaction, making it impossible to migrate all data in one go:

To address this, developers can implement partial migration, which enables the state to be migrated in smaller, manageable chunks.
