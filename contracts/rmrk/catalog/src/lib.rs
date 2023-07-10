#![no_std]

use catalog_io::*;
use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;
use types::primitives::*;
pub mod catalog;
use catalog::Catalog;

static mut CATALOG: Option<Catalog> = None;
static mut ADMIN: Option<ActorId> = None;

#[no_mangle]
extern "C" fn init() {
    let config: InitCatalog = msg::load().expect("Unable to decode InitBase");
    let catalog = Catalog {
        base_type: config.catalog_type,
        symbol: config.symbol,
        ..Default::default()
    };
    unsafe {
        CATALOG = Some(catalog);
        ADMIN = Some(msg::source());
    }
}
#[no_mangle]
extern "C" fn handle() {
    let action: CatalogAction = msg::load().expect("Could not load BaseAction");
    let catalog = unsafe { CATALOG.as_mut().expect("The contract is not initialized") };
    let reply = process_handle(&action, catalog);
    msg::reply(reply, 0).expect("Error in sending a reply from Catalog contract");
}

fn process_handle(
    action: &CatalogAction,
    catalog: &mut Catalog,
) -> Result<CatalogReply, CatalogError> {
    match action {
        CatalogAction::AddParts(parts) => {
            only_admin()?;
            catalog.add_parts(parts.clone())
        }
        CatalogAction::AddEquippableAddresses {
            part_id,
            collection_ids,
        } => {
            only_admin()?;
            catalog.add_equippable_addresses(*part_id, collection_ids.clone())
        }
        CatalogAction::RemoveParts(parts) => {
            only_admin()?;
            catalog.remove_parts(parts.clone())
        }
        CatalogAction::RemoveEquippable {
            part_id,
            collection_id,
        } => {
            only_admin()?;
            catalog.remove_equippable(*part_id, *collection_id)
        }
        CatalogAction::CheckPart(part_id) => catalog.check_part(*part_id),
        CatalogAction::CheckEquippable {
            part_id,
            collection_id,
        } => catalog.check_equippable(*part_id, *collection_id),
        CatalogAction::SetEquippableToAll { part_id } => catalog.set_equippable_to_all(*part_id),
        CatalogAction::ResetEquippableAddress { part_id } => {
            catalog.reset_equippable_addresses(*part_id)
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let catalog = unsafe { CATALOG.as_ref().expect("Base is not initialized") };
    let admin = unsafe { ADMIN.as_ref().expect("The contract is not initialized") };
    let catalog_state = CatalogState {
        admin: *admin,
        base_type: catalog.base_type.clone(),
        symbol: catalog.symbol.clone(),
        parts: catalog
            .parts
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect(),
        is_equippable_to_all: catalog.is_equippable_to_all.clone(),
    };
    msg::reply(catalog_state, 0).expect("Failed to share state");
}

fn only_admin() -> Result<(), CatalogError> {
    let admin = unsafe { ADMIN.as_ref().expect("The contract is not initialized") };
    if admin != &msg::source() {
        return Err(CatalogError::NotAllowedToCall);
    }
    Ok(())
}
