use gstd::{prelude::*, ActorId};

#[derive(Debug, Decode, Encode, TypeInfo, Clone)]
pub enum DaoAction {
    /// Adds members to whitelist.
    ///
    /// Requirements:
    /// * Only admin can add actors to whitelist;
    /// * Member ID cant be zero;
    /// * Member can not be added to whitelist more than once;
    ///
    /// On success replies with [`DaoEvent::MemberAddedToWhitelist`]
    AddToWhiteList(
        /// valid actor ID
        ActorId,
    ),

    /// The proposal of joining the DAO.
    ///
    /// Requirements:
    /// * The proposal can be submitted only by the existing members or their delegate addresses;
    /// * The applicant account must be either a DAO member or is in the whitelist.
    ///
    /// On success replies with [`DaoEvent::SubmitMembershipProposal`]
    SubmitMembershipProposal {
        /// an actor who wishes to become a DAO member
        applicant: ActorId,
        /// the number of tokens the applicant offered for shares in DAO
        token_tribute: u128,
        /// the amount of shares the applicant is requesting for his token tribute
        shares_requested: u128,
        /// a certain threshold of YES votes in order for the proposal to pass
        quorum: u128,
        /// the proposal description
        details: String,
    },

    /// The proposal of funding.
    ///
    /// Requirements:
    /// * The proposal can be submitted only by the existing members or their delegate addresses;
    /// * The receiver ID can't be the zero;
    /// * The DAO must have enough funds to finance the proposal;
    ///
    /// On success replies with [`DaoEvent::SubmitFundingProposal`]
    SubmitFundingProposal {
        /// an actor that will be funded
        applicant: ActorId,
        /// the number of fungible tokens that will be sent to the receiver
        amount: u128,
        /// a certain threshold of YES votes in order for the proposal to pass
        quorum: u128,
        /// the proposal description
        details: String,
    },

    /// The proposal processing after the proposal completes during the grace period.
    /// If the membership proposal is accepted, the tribute tokens are deposited into the contract
    /// and new shares are minted and issued to the applicant.
    /// If the membership proposal is rejected, the tribute tokens are returned to the applicant.
    /// If the funding proposal is accepted, the indicated amount of tokens is transfered to the applicant;
    /// If the funging proposal is rejected, the indicated amount of tokens remains in the contract.
    ///
    /// Requirements:
    /// * The previous proposal must be processed;
    /// * The proposal must exist and be ready for processing;
    /// * The proposal must not be aborted or already be processed.
    ///
    /// On success replies with [`DaoEvent::ProcessProposal`]
    ProcessProposal(
        /// the proposal ID
        u128,
    ),

    /// The member (or the delegate address of the member) submits his vote (YES or NO) on the proposal.
    ///
    /// Requirements:
    /// * The proposal can be submitted only by the existing members or their delegate addresses;
    /// * The member can vote on the proposal only once;
    /// * Proposal must exist, the voting period must has started and not expired;
    /// * Proposal must not be aborted.
    ///
    /// On success replies with [`DaoEvent::SubmitVote`]
    SubmitVote {
        /// the proposal ID
        proposal_id: u128,
        /// the member  a member vote (YES or NO)
        vote: Vote,
    },

    /// Withdraws the capital of the member.
    ///
    /// Requirements:
    /// * `msg::source()` must be DAO member;
    /// * The member must have sufficient amount;
    /// * The latest proposal the member voted YES must be processed;
    /// * Admin can ragequit only after transferring his role to another actor.
    ///
    /// On success replies with [`DaoEvent::RageQuit`]
    RageQuit(
        /// The amount of shares the member would like to withdraw (the shares are converted to fungible tokens)
        u128,
    ),

    /// Aborts the membership proposal.
    /// It can be used in case when applicant is disagree with the requested shares
    /// or the details the proposer indicated by the proposer.
    ///
    /// Requirements:
    /// * `msg::source()` must be the applicant;
    /// * The proposal must be membership proposal;
    /// * The proposal can be aborted during only the abort window
    /// * The proposal has not be already aborted.
    ///
    /// On success replies with [`DaoEvent::Abort`]
    Abort(
        /// the proposal ID
        u128,
    ),

    /// Sets the delegate key that is responsible for submitting proposals and voting;
    /// The deleagate key defaults to member address unless updated.
    ///
    /// Requirements:
    /// * `msg::source()` must be DAO member;
    /// * The delegate key must not be zero address;
    /// * A delegate key can be assigned only to one member.
    ///
    /// On success replies with [`DaoEvent::DelegateKeyUpdated`]
    UpdateDelegateKey(
        /// New delegate account
        ActorId,
    ),

    /// Assigns the admin position to new actor.
    ///
    /// Requirements:
    /// * Only admin can assign new admin.
    ///
    /// On success replies with [`DaoEvent::AdminUpdated`]
    SetAdmin(
        /// New admin account
        ActorId,
    ),

    /// Continues the transaction if it fails due to lack of gas
    /// or due to an error in the token contract.
    ///
    /// Requirements:
    /// * Transaction must exist.
    ///
    /// On success replies with the payload of continued transaction.
    Continue(
        /// the transaction ID
        u64,
    ),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum DaoEvent {
    MemberAddedToWhitelist(ActorId),
    SubmitMembershipProposal {
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        token_tribute: u128,
    },
    SubmitFundingProposal {
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        amount: u128,
    },
    SubmitVote {
        account: ActorId,
        proposal_id: u128,
        vote: Vote,
    },
    ProcessProposal {
        proposal_id: u128,
        passed: bool,
    },
    RageQuit {
        member: ActorId,
        amount: u128,
    },
    Abort(u128),
    AdminUpdated(ActorId),
    DelegateKeyUpdated {
        member: ActorId,
        delegate: ActorId,
    },
    TransactionFailed(u64),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitDao {
    pub admin: ActorId,
    pub approved_token_program_id: ActorId,
    pub period_duration: u64,
    pub voting_period_length: u64,
    pub grace_period_length: u64,
    pub dilution_bound: u8,
    pub abort_window: u64,
}

#[derive(Debug, Encode, Decode, Clone, TypeInfo)]
pub enum Vote {
    Yes,
    No,
}
