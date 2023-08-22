#![no_std]

use catalog_io::*;
use gstd::{msg, prelude::*, ActorId};
use hashbrown::HashMap;
use resource_io::*;
use types::primitives::{PartId, ResourceId};

#[derive(Debug, Default)]
struct ResourceStorage {
    name: String,
    // the admin is the rmrk contract that initializes the storage contract
    admin: ActorId,
    resources: HashMap<ResourceId, Resource>,
}

static mut RESOURCE_STORAGE: Option<ResourceStorage> = None;

impl ResourceStorage {
    fn add_resource_entry(&mut self, resource_id: ResourceId, resource: Resource) {
        assert!(resource_id != 0, "Write to zero");
        assert!(msg::source() == self.admin, "Not admin");
        assert!(
            self.resources
                .insert(resource_id, resource.clone())
                .is_none(),
            "resource already exists"
        );
        msg::reply(
            ResourceEvent::ResourceEntryAdded {
                resource_id,
                resource,
            },
            0,
        )
        .expect("Error in reply `[ResourceEvent::ResourceEntryAdded]`");
    }

    async fn add_part_to_resource(&mut self, resource_id: ResourceId, part_id: PartId) {
        assert!(msg::source() == self.admin, "Not admin");
        let resource = self
            .resources
            .get_mut(&resource_id)
            .expect("Resource with indicated id does not exist");
        if let Resource::Composed(ComposedResource { base, parts, .. }) = resource {
            // check that part exist in base contract
            msg::send_for_reply_as::<_, CatalogReply>(
                *base,
                CatalogAction::CheckPart(part_id),
                0,
                0,
            )
            .expect("Error in sending async message `[BaseAction::CheckPart]` to base contract")
            .await
            .expect("Error in async message `[BaseAction::CheckPart]`");
            parts.push(part_id);
        } else {
            panic!("Resource must be composed");
        }

        msg::reply(ResourceEvent::PartIdAddedToResource(part_id), 0)
            .expect("Error in reply `[ResourceEvent::PartIdAddedToResource]`");
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let config: InitResource = msg::load().expect("Unable to decode InitResource");
    let resource = ResourceStorage {
        name: config.resource_name,
        admin: msg::source(),
        ..ResourceStorage::default()
    };
    RESOURCE_STORAGE = Some(resource);
}

#[gstd::async_main]
async unsafe fn main() {
    let action: ResourceAction = msg::load().expect("Could not load ResourceAction");
    let storage = unsafe { RESOURCE_STORAGE.get_or_insert(Default::default()) };
    match action {
        ResourceAction::AddResourceEntry {
            resource_id,
            resource,
        } => storage.add_resource_entry(resource_id, resource),
        ResourceAction::AddPartToResource {
            resource_id,
            part_id,
        } => storage.add_part_to_resource(resource_id, part_id).await,
        ResourceAction::GetResource { id } => {
            let resource = storage.resources.get(&id).expect("Resource is not found");
            msg::reply(ResourceEvent::Resource(resource.clone()), 0)
                .expect("Error in reply `[ResourceEvent::Resource]`");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let resource = unsafe {
        RESOURCE_STORAGE
            .as_ref()
            .expect("Resource is not initialized")
    };
    let resource_state = ResourceState {
        name: resource.name.clone(),
        admin: resource.admin,
        resources: resource
            .resources
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect(),
    };
    msg::reply(resource_state, 0).expect("Failed to share state");
}
