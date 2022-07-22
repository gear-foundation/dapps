#![no_std]

use gstd::{prelude::*, ActorId};
use resource_io::Resource;
use types::primitives::*;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitRMRK {
    pub name: String,
    pub symbol: String,
    pub resource_name: String,
    pub resource_hash: Option<[u8; 32]>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Copy, Eq, PartialEq)]
pub enum ChildStatus {
    Pending,
    Accepted,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum RMRKAction {
    /// Mints token that will belong to another token in another RMRK contract.
    ///
    /// # Requirements:
    /// * The `parent_id` must be a deployed RMRK contract.
    /// * The token with id `parent_token_id` must exist in `parent_id` contract.
    /// * The `token_id` must not exist.
    ///
    /// # Arguments:
    /// * `parent_id`: is the address of RMRK parent contract.
    /// * `parent_token_id`: is the parent RMRK token.
    /// * `token_id`: is the tokenId of new RMRK token.
    ///
    /// On success replies [`RMRKEvent::MintToNft`].
    MintToNft {
        parent_id: ActorId,
        parent_token_id: TokenId,
        token_id: TokenId,
    },

    /// Mints token to the user or program.
    ///
    /// # Requirements:
    /// * The `token_id` must not exist.
    /// * The `root_owner` address should be a non-zero address.
    ///
    /// # Arguments:
    /// * `root_owner`: is the address who will own the token.
    /// * `token_id`: is the tokenId of new RMRK token.
    ///
    /// On success replies [`RMRKEvent::MintToRootOwner`].
    MintToRootOwner {
        root_owner: ActorId,
        token_id: TokenId,
    },

    /// That message is designed to be send from another RMRK contracts
    /// when minting an NFT(child_token_id) to another NFT(parent_token_id).
    /// It adds a child to the NFT with tokenId `parent_token_id`
    /// The status of added child is `Pending`.
    ///
    /// # Requirements:
    /// * Token with TokenId `parent_token_id` must exist.
    /// * There cannot be two identical children.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// On success replies [`RMRKEvent::PendingChild`].
    AddChild {
        parent_token_id: TokenId,
        child_token_id: TokenId,
    },

    /// Accepts an RMRK child being in the `Pending` status.
    /// Removes RMRK child from `pending_children` and adds to `accepted_children`.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be an RMRK owner of NFT with tokenId `parent_token_id` or an approved account.
    /// * The indicated NFT with tokenId `child_token_id` must exist in the pending array of `parent_token_id`.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT
    /// * `child_token_id`: is the tokenId of the child instance
    ///
    /// On success replies [`RMRKEvent::AcceptedChild`].
    AcceptChild {
        parent_token_id: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
    },

    /// Rejects an RMRK child being in the `Pending` status.
    /// It sends message to the child NFT contract to burn NFT token from it.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be an RMRK owner or an approved account.
    /// * The indicated NFT with tokenId `child_token_id` must exist in the pending array of `parent_token_id`.
    ///
    /// Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_contract_id`: is the address of the child RMRK contract.
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// On success replies [`RMRKEvent::RejectedChild`].
    RejectChild {
        parent_token_id: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
    },

    /// Removes an RMRK child being in the `Accepted` status.
    /// It sends message to the child NFT contract to burn NFT token from it.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be an RMRK owner or an approved account.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_contract_id`: is the address of the child RMRK contract.
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// On success replies [`RMRKEvent::RemovedChild`].
    RemoveChild {
        parent_token_id: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
    },

    /// Burns RMRK token.
    /// It recursively burn all the children NFTs.
    /// It checks whether the token is a child of another token.
    /// If so, it sends a message to the parent NFT  to remove the child.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the root owner of the token.
    ///
    /// # Arguments:
    /// * `token_id`: is the tokenId of the burnt token.
    ///
    /// On success replies [`RMRKEvent::Transfer`].
    Burn(TokenId),

    /// Burns RMRK tokens. It must be called from the RMRK parent contract when the root owner removes or rejects child NFTs.
    /// The input argument is an `BTreeSet<TokenId>` since a parent contract can have multiple children that must be burnt.
    /// It also recursively send messages [`RMRKAction::BurnFromParent`] to children of burnt tokens if any.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be RMRK parent contract.
    /// * All tokens in `BTreeSet<TokenId>` must exist.
    ///
    /// # Arguments:
    /// * `token_ids`: is the tokenIds of the burnt tokens.
    ///
    /// On success replies [`RMRKEvent::TokensBurnt`].
    BurnFromParent {
        child_token_id: TokenId,
        root_owner: ActorId,
    },

    /// Burns a child of NFT.
    /// That function must be called from the child RMRK contract during `transfer`, `transfer_to_nft` and `burn` functions.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be a child RMRK contract.
    /// * The indicated child must exist the children list of `parent_token_id`.
    ///
    /// # Arguments:
    /// * `parent_token_id`: is the tokenId of the parent NFT.
    /// * `child_token_id`: is the tokenId of the child instance.
    ///
    /// On success replies [`RMRKEvent::ChildBurnt`].
    BurnChild {
        parent_token_id: TokenId,
        child_token_id: TokenId,
    },

    /// Transfers NFT to another account.
    /// If the previous owner is another RMRK contract, it sends the message [`RMRKAction::BurnChild`] to the parent conract.
    ///
    /// # Requirements:
    /// * The `token_id` must exist.
    /// * The `msg::source()` must be approved or owner of the token.
    /// * The `to` address should be a non-zero address.
    ///
    /// # Arguments:
    /// * `to`: is the receiving address.
    /// * `token_id`: is the tokenId of the transfered token.
    ///
    /// On success replies [`RMRKEvent::ChildBurnt`].
    Transfer {
        to: ActorId,
        token_id: TokenId,
    },

    /// Transfers NFT to another NFT.
    ///
    /// # Requirements:
    /// * The `token_id` must exist.
    /// * The `msg::source()` must be approved or root owner of the token.
    /// * The `to` address should be a non-zero address
    ///
    /// # Arguments:
    /// * `to`: is the address of new parent RMRK contract.
    /// * `destination_id: is the tokenId of the parent RMRK token.
    /// * `token_id`: is the tokenId of the transfered token.
    ///
    /// On success replies [`RMRKEvent::TransferToNft`].
    TransferToNft {
        to: ActorId,
        token_id: TokenId,
        destination_id: TokenId,
    },

    /// That message is designed to be sent from another RMRK contracts
    /// when root owner transfers his child to another parent token within one contract.
    /// If root owner transfers child token from NFT to another his NFT
    /// it adds a child to the NFT  with a status that child had before.
    /// If root owner transfers child token from NFT to another NFT that he does not own
    /// it adds a child to the NFT  with a status `Pending`.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be a child RMRK contract.
    /// * The `to` must be an existing RMRK token
    /// * The `root_owner` of `to` and `from` must be the same.
    ///
    /// # Arguments:
    /// * `from`: RMRK token from which the child token will be transferred.
    /// * `to`: RMRK token to which the child token will be transferred.
    /// * `child_token_id`: is the tokenId of the child in the RMRK child contract.
    ///
    /// On success replies [`RMRKEvent::ChildTransferred`].
    TransferChild {
        from: TokenId,
        to: TokenId,
        child_token_id: TokenId,
    },
    RootOwner(TokenId),

    /// Approves an account to transfer NFT.
    ///
    /// # Requirements:
    /// * The `token_id` must exist.
    /// * The `msg::source()` must be approved or root owner of the token.
    /// * The `to` address must be a non-zero address
    ///
    /// # Arguments:
    /// * `to`: is the address of approved account.
    /// * `token_id`: is the tokenId of the token.
    ///
    /// On success replies [`RMRKEvent::Approval`].
    Approve {
        to: ActorId,
        token_id: TokenId,
    },

    /// That function is designed to be called from another RMRK contracts
    /// when root owner transfers his child NFT to another his NFT in another contract.
    /// It adds a child to the RMRK token with tokenId `parent_token_id` with status `Accepted`.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be a child RMRK contract.
    /// * The `parent_token_id` must be an existing RMRK token that must have `child_token_id` in its `accepted_children`.
    ///
    /// # Arguments:
    /// * `parent_token_id`: RMRK token to which the child token will be transferred.
    /// * `child_token_id`: is the tokenId of the child of the RMRK child contract.
    ///
    /// On success replies [`RMRKEvent::AcceptedChild`].
    AddAcceptedChild {
        parent_token_id: TokenId,
        child_token_id: TokenId,
    },

    /// Adds resource entry on resource storage contract.
    /// It sends a message to resource storage contract with information about new resource.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the contract admin.
    ///
    /// Arguments:
    /// * `resource_id`: is a resource identifier
    /// * `resource`: Resource (Basic, Slot or Composable)
    ///
    /// On success reply `[RMRKEvent::ResourceEntryAdded]`.
    AddResourceEntry {
        resource_id: ResourceId,
        resource: Resource,
    },

    /// Adds resource to an existing token.
    /// Checks that the resource with indicated id exists in the resource storage contract.
    /// Proposed resource is placed in the "Pending" array.
    /// A pending resource can be also proposed to overwrite an existing resource.
    ///
    /// # Requirements
    /// Token with indicated `token_id` must exist.
    /// The proposed resource must not already exist for the token.
    /// The resource that is proposed to be overwritten must exist for the token.
    /// The length of resources in pending status must be less or equal to `MAX_RESOURCE_LEN`.
    ///
    /// # Arguments:
    /// * `token_id`: an id of the token.
    /// * `resource_id`: a proposed resource.
    /// * `overwrite_id`: a resource to be overwritten.
    ///
    /// On success reply `[RMRKEvent::ResourceAdded]`.
    AddResource {
        token_id: TokenId,
        resource_id: u8,
        overwrite_id: u8,
    },

    /// Accepts resource from pending list.
    /// Moves the resource from the pending array to the accepted array.
    ///
    /// # Requirements
    /// Only root owner or approved account can accept a resource.
    /// `resource_id` must exist for the token in the pending array.
    ///
    /// # Arguments:
    /// * `token_id`: an id of the token.
    /// * `resource_id`: a resource to be accepted.
    ///
    /// On success reply `[RMRKEvent::ResourceAccepted]`.
    AcceptResource {
        token_id: TokenId,
        resource_id: u8,
    },

    /// Rejects a resource, dropping it from the pending array.
    ///
    /// # Requirements
    /// Only root owner or approved account can reject a resource.
    /// `resource_id` must exist for the token in the pending array.
    ///
    /// # Arguments:
    /// * `token_id`: an id of the token.
    /// * `resource_id`: a resource to be rejected.
    ///
    /// On success reply `[RMRKEvent::ResourceRejected]`.
    RejectResource {
        token_id: TokenId,
        resource_id: u8,
    },

    /// Sets the priority of the active resources array
    /// Priorities have a 1:1 relationship with their corresponding index in
    /// the active resources array. E.G, a priority array of [1, 3, 2] indicates
    ///  that the the active resource at index 1 of the active resource array
    ///  has a priority of 1, index 2 has a priority of 3, and index 3 has a priority
    ///  of 2. There is no validation on priority value input; out of order indexes
    ///  must be handled by the frontend.
    ///
    /// # Requirements
    /// Only root owner or approved account can set priority
    /// The length of the priorities array must be equal to the present length of the active resources array
    ///
    /// # Arguments:
    /// * `token_id`: an id of the token.
    /// * `priorities`: An array of priorities to set.
    ///
    /// On success reply `[RMRKEvent::PrioritySet]`.
    SetPriority {
        token_id: TokenId,
        priorities: Vec<u8>,
    },

    /// Equips a child NFT's resource to a parent's slot.
    /// It sends message to the parent contract checking the child status.
    /// and the parent's resource.
    /// to check whether the child token has the indicated slot resource.
    ///
    /// # Requirements:
    /// * The `msg::source()` must be the root owner.
    /// * The child token must have the slot resource with indicated `base_id` and `slot_id`.
    /// * The parent token must have composed resource with indicated `base_id`.
    ///
    /// # Arguments:
    /// * `token_id`: the tokenId of the NFT to be equipped.
    /// * `resource_id`: the id of the slot resource.
    /// * `equippable`: parent's contract and token.
    /// * `equippable_resource_id`: the id of the composed resource.
    ///
    /// On success replies [`RMRKEvent::TokenEquipped`].
    Equip {
        token_id: TokenId,
        resource_id: ResourceId,
        equippable: CollectionAndToken,
        equippable_resource_id: ResourceId,
    },

    /// That message is designed to be sent from another RMRK contracts
    /// when equipping  `child_token_id` to `parent_token_id`.
    /// It checks that `parent_token_id` has the child with `child_token_id` in its accepted children.
    /// It checks that `parent_token_id` has composed resource.
    /// It sends a message to base contract to check whether the `parent_token_id` is in equippable list.
    /// It sends a message to resource contract to add part id to composed resource.
    ///
    /// # Requirements:
    /// * `msg::source()` must be the child contract.
    /// * The resource with indicated id must exist in the token resources and must be Composed.
    /// * The `parent_token_id` must be in the equippable list.
    ///
    /// # Arguments:
    /// * `parent_token_id`: the id of the equippable token.
    /// * `child_token_id`: the id of the token to be equipped.
    /// * `resource_id`: the id of the composed resource.
    /// * `slot_id`: the id of the slot part.
    ///
    /// On success replies [`RMRKEvent::EquippableIsOk`].
    CheckEquippable {
        parent_token_id: TokenId,
        child_token_id: TokenId,
        resource_id: ResourceId,
        slot_id: PartId,
    },
    CheckSlotResource {
        token_id: TokenId,
        resource_id: ResourceId,
        base_id: BaseId,
        slot_id: PartId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum RMRKEvent {
    MintToNft {
        parent_id: ActorId,
        parent_token_id: TokenId,
        token_id: TokenId,
    },
    MintToRootOwner {
        root_owner: ActorId,
        token_id: TokenId,
    },
    Approval {
        root_owner: ActorId,
        approved_account: ActorId,
        token_id: TokenId,
    },
    PendingChild {
        child_token_address: ActorId,
        child_token_id: TokenId,
        parent_token_id: TokenId,
    },
    AcceptedChild {
        child_contract_id: ActorId,
        child_token_id: TokenId,
        parent_token_id: TokenId,
    },
    RootOwner(ActorId),
    RejectedChild {
        child_contract_id: ActorId,
        child_token_id: TokenId,
        parent_token_id: TokenId,
    },
    RemovedChild {
        child_contract_id: ActorId,
        child_token_id: TokenId,
        parent_token_id: TokenId,
    },
    ChildAdded {
        parent_token_id: TokenId,
        child_token_id: TokenId,
        child_status: ChildStatus,
    },
    ChildBurnt {
        parent_token_id: TokenId,
        child_token_id: TokenId,
    },
    ChildTransferred {
        from: TokenId,
        to: TokenId,
        child_contract_id: ActorId,
        child_token_id: TokenId,
    },
    TokenBurnt(TokenId),
    Transfer {
        to: ActorId,
        token_id: TokenId,
    },
    TransferToNft {
        to: ActorId,
        token_id: TokenId,
        destination_id: TokenId,
    },
    Owner {
        token_id: Option<TokenId>,
        owner_id: ActorId,
    },
    ResourceEntryAdded(Resource),
    ResourceAdded {
        token_id: TokenId,
        resource_id: ResourceId,
        overwrite_id: ResourceId,
    },
    ResourceAccepted {
        token_id: TokenId,
        resource_id: ResourceId,
    },
    ResourceRejected {
        token_id: TokenId,
        resource_id: ResourceId,
    },
    PrioritySet {
        token_id: TokenId,
        priorities: Vec<u8>,
    },
    ResourceInited {
        resource_id: ActorId,
    },
    SlotResourceIsOk,
    TokenEquipped {
        token_id: TokenId,
        resource_id: ResourceId,
        slot_id: PartId,
        equippable: CollectionAndToken,
    },
    EquippableIsOk,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum RMRKState {
    RMRKInfo,
    Owner(TokenId),
    Balance(ActorId),
    PendingChildren(TokenId),
    AcceptedChildren(TokenId),
    PendingResources(TokenId),
    ActiveResources(TokenId),
}

#[derive(Debug, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub enum RMRKStateReply {
    RMRKInfo {
        name: String,
        symbol: String,
        admin: ActorId,
        resource_id: ActorId,
    },
    Owner {
        token_id: Option<TokenId>,
        owner_id: ActorId,
    },
    Balance(TokenId),
    PendingChildren(BTreeSet<CollectionAndToken>),
    AcceptedChildren(BTreeSet<CollectionAndToken>),
    PendingResources(BTreeSet<ResourceId>),
    ActiveResources(BTreeSet<ResourceId>),
}
