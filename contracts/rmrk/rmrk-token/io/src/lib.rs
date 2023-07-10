#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;
use types::primitives::*;
pub type TokenEquipment = Vec<(PartId, Equipment)>;
pub struct RMRKMetadata;

impl Metadata for RMRKMetadata {
    type Init = In<InitRMRK>;
    type Handle = InOut<RMRKAction, Result<RMRKReply, RMRKError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = RMRKState;
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct RMRKState {
    pub name: String,
    pub symbol: String,
    pub admin: ActorId,
    pub token_approvals: Vec<(TokenId, Vec<ActorId>)>,
    pub rmrk_owners: Vec<(TokenId, RMRKOwner)>,
    pub pending_children: Vec<(TokenId, Vec<CollectionAndToken>)>,
    pub accepted_children: Vec<(TokenId, Vec<CollectionAndToken>)>,
    pub children_status: Vec<(CollectionAndToken, ChildStatus)>,
    pub balances: Vec<(ActorId, U256)>,
    pub assets: AssetsState,
}

#[derive(Default, Encode, Debug, Decode, TypeInfo)]
pub struct AssetsState {
    /// Mapping of uint64 Ids to asset metadata
    pub assets: Vec<(u64, String)>,
    /// Mapping of uint64 asset ID to corresponding catalog address.
    pub catalog_addresses: Vec<(u64, ActorId)>,
    /// Mapping of asset_id to equippable_group_ids.
    pub equippable_group_ids: Vec<(u64, u64)>,
    /// Mapping of asset_id to catalog parts applicable to this asset, both fixed and slot
    pub part_ids: Vec<(u64, Vec<PartId>)>,
    /// Mapping of tokenId to an array of pending assets
    pub pending_assets: Vec<(TokenId, Vec<u64>)>,
    /// Mapping of tokenId to an array of active assets
    pub active_assets: Vec<(TokenId, Vec<u64>)>,
    /// Mapping of tokenId to an array of priorities for active assets
    pub active_assets_priorities: Vec<(TokenId, Vec<u64>)>,
    /// Mapping of tokenId to new asset, to asset to be replaced
    pub asset_replacement: Vec<(TokenId, Vec<(u64, u64)>)>,
    /// Mapping of `equippable_group_id` to parent contract address and valid `slot_id`.
    pub valid_parent_slots: Vec<(u64, Vec<(ActorId, PartId)>)>,
    /// Mapping of token ID and catalog address to slot part ID to equipment information.
    /// Used to compose an NFT.
    pub equipments: Vec<((TokenId, ActorId), TokenEquipment)>,
}
#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Equipment {
    ///  The ID of the asset equipping a child
    pub asset_id: u64,
    /// The ID of the asset used as equipment
    pub child_asset_id: u64,
    /// The ID of token that is equipped
    pub child_token_id: TokenId,
    /// Address of the collection to which the child asset belongs to
    pub child_id: ActorId,
}
#[derive(Debug, Default, PartialEq, Eq, Encode, Decode, TypeInfo, Clone)]
pub struct RMRKOwner {
    pub token_id: Option<TokenId>,
    pub owner_id: ActorId,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct MultiResourceState {
    pub pending_resources: Vec<(TokenId, Vec<ResourceId>)>,
    pub active_resources: Vec<(TokenId, Vec<ResourceId>)>,
    pub resource_overwrites: Vec<(TokenId, Vec<(ResourceId, ResourceId)>)>,
    pub active_resources_priorities: Vec<(TokenId, Vec<u8>)>,
}

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

#[derive(Debug, Decode, Encode, TypeInfo, Clone)]
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

    /// Used to add an equippable asset entry.
    ///
    /// Arguments:
    /// * `equippable_group_id`: ID of the equippable group
    /// * `catalog_address`: Address of the `Catalog` smart contract this asset belongs to
    /// * `metadata_uri`: Metadata URI of the asset
    /// * `parts_ids`:  An array of IDs of fixed and slot parts to be included in the asset
    ///
    /// On success reply `[RMRKEvent::ResourceEntryAdded]`.
    AddEquippableAssetEntry {
        equippable_group_id: u64,
        catalog_address: Option<ActorId>,
        metadata_uri: String,
        part_ids: Vec<PartId>,
    },

    AddAssetToToken {
        token_id: TokenId,
        asset_id: u64,
        replaces_asset_with_id: u64,
    },

    AcceptAsset {
        token_id: TokenId,
        asset_id: u64,
    },

    /// Declares that the assets belonging to a given `equippable_group_id` are
    /// equippable into the `Slot` associated with the `part_id` of the collection
    ///  at the specified `parent_id`.
    SetValidParentForEquippableGroup {
        equippable_group_id: u64,
        slot_part_id: PartId,
        parent_id: ActorId,
    },

    /// Equips a child to a parent's slot.
    ///
    /// # Arguments:
    /// * `token_id`: the tokenId of the NFT to be equipped.
    /// * `child_token_id`:
    /// * `child_id`:
    /// * `asset_id`:ID of the asset that we are equipping into
    /// * `slot_part_id`: slotPartId ID of the slot part that we are using to equip
    /// * `child_asset_id`: childAssetId ID of the asset that we are equipping
    ///
    /// On success replies [`RMRKEvent::TokenEquipped`].
    Equip {
        token_id: TokenId,
        child_token_id: TokenId,
        child_id: CollectionId,
        asset_id: u64,
        slot_part_id: PartId,
        child_asset_id: u64,
    },

    CanTokenBeEquippedWithAssetIntoSlot {
        parent_id: ActorId,
        token_id: TokenId,
        asset_id: u64,
        slot_part_id: PartId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq)]
pub enum RMRKReply {
    MintedToNft,
    MintedToRootOwner,
    Burnt,
    Approved,
    PendingChildAdded,
    ChildAccepted,
    RootOwner(ActorId),
    ChildRejected,
    ChildRemoved,
    ChildAdded,
    ChildBurnt,
    ChildTransferred,
    TokenBurnt,
    Transferred,
    TransferredToNft,
    Owner {
        token_id: Option<TokenId>,
        owner_id: ActorId,
    },
    PrioritySet,
    SlotResourceIsOk,
    TokenEquipped,
    EquippableIsOk,
    EquippableAssetEntryAdded,
    AssetAddedToToken,
    AssetAccepted,
    ValidParentEquippableGroupIdSet,
    TokenBeEquippedWithAssetIntoSlot,
    ChildAssetEquipped,
    AssetSet,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq, Clone)]
pub enum RMRKError {
    ZeroIdForbidden,
    TokenDoesNotExist,
    TokenAlreadyExists,
    AssetAlreadyExists,
    ChildDoesNotExist,
    CatalogRequiredForParts,
    NoAssetMatchingId,
    MaxPendingAssetsReached,
    AssetDoesNotExistInPendingArray,
    EquippableNotFound,
    WrongSlotId,
    WrongPartFormat,
    NotRMRKParentContract,
    ActiveAssetNotFound,
    EquippableNotAllowedByCatalog,
    TargetAssetCannotReceiveSlot,
    SlotAlreadyUsed,
    GasIsOver,
    NotInEquippableList,
    WrongChildStatus,
    UnknownError,
    CatalogDoesNotExist,
    UnexpectedReply,
    ChildInPendingArray,
    ChildInAcceptedArray,
    NotApprovedAccount,
    NotRootOwner,
    ErrorInCatalog,
}
