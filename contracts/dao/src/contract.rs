use crate::ft_messages::*;
use dao_io::*;
use gstd::{exec, msg, prelude::*, ActorId, String};
use hashbrown::HashMap;

pub const BASE_PERCENT: u8 = 100;

static mut DAO: Option<Dao> = None;

#[derive(Debug, Default)]
pub struct Dao {
    pub admin: ActorId,
    pub approved_token_program_id: ActorId,
    pub period_duration: u64,
    pub voting_period_length: u64,
    pub grace_period_length: u64,
    pub dilution_bound: u8,
    pub abort_window: u64,
    pub total_shares: u128,
    pub balance: u128,
    pub members: HashMap<ActorId, Member>,
    pub member_by_delegate_key: HashMap<ActorId, ActorId>,
    pub proposal_id: u128,
    pub proposals: HashMap<u128, Proposal>,
    pub whitelist: Vec<ActorId>,
    pub transaction_id: u64,
    pub transactions: HashMap<u64, Option<DaoAction>>,
}

impl Dao {
    pub fn add_to_whitelist(&mut self, member: &ActorId) {
        self.assert_admin();
        Self::assert_not_zero_address(member);

        if self.whitelist.contains(member) {
            panic!("Member has already been added to the whitelist");
        }
        self.whitelist.push(*member);
        msg::reply(DaoEvent::MemberAddedToWhitelist(*member), 0)
            .expect("Error in a reply `DaoEvent::MemberAddedToWhitelist`");
    }

    pub async fn submit_membership_proposal(
        &mut self,
        transaction_id: Option<u64>,
        applicant: &ActorId,
        token_tribute: u128,
        shares_requested: u128,
        quorum: u128,
        details: String,
    ) {
        let current_transaction_id = self.get_transaction_id(transaction_id);
        self.check_for_membership();
        // check that applicant is either in whitelist or a DAO member
        if !self.whitelist.contains(applicant) && !self.members.contains_key(applicant) {
            panic!("Applicant must be either in whitelist or be a DAO member");
        }

        // transfer applicant tokens to DAO contract
        if transfer_tokens(
            current_transaction_id,
            &self.approved_token_program_id,
            applicant,
            &exec::program_id(),
            token_tribute,
        )
        .await
        .is_err()
        {
            self.transactions.remove(&current_transaction_id);
            msg::reply(DaoEvent::TransactionFailed(current_transaction_id), 0)
                .expect("Error in a reply `DaoEvent::TransactionFailed`");
            return;
        };

        let mut starting_period = exec::block_timestamp();
        let proposal_id = self.proposal_id;
        // compute startingPeriod for proposal
        // there should be a minimum time interval between proposals (period_duration) so that members have time to ragequit
        if proposal_id > 0 {
            let previous_starting_period = self
                .proposals
                .get(&(&proposal_id - 1))
                .expect("Error getting proposal")
                .starting_period;
            if starting_period < previous_starting_period + self.period_duration {
                starting_period = previous_starting_period + self.period_duration;
            }
        }
        let proposal = Proposal {
            proposer: msg::source(),
            applicant: *applicant,
            shares_requested,
            quorum: quorum * BASE_PERCENT as u128,
            is_membership_proposal: true,
            token_tribute,
            details,
            starting_period,
            ..Proposal::default()
        };
        self.proposals.insert(proposal_id, proposal);
        self.proposal_id = self.proposal_id.saturating_add(1);
        self.transactions.remove(&current_transaction_id);
        msg::reply(
            DaoEvent::SubmitMembershipProposal {
                proposer: msg::source(),
                applicant: *applicant,
                proposal_id,
                token_tribute,
            },
            0,
        )
        .expect("Error in a reply `DaoEvent::SubmitMembershipProposal`");
    }

    pub fn submit_funding_proposal(
        &mut self,
        applicant: &ActorId,
        amount: u128,
        quorum: u128,
        details: String,
    ) {
        self.check_for_membership();
        Self::assert_not_zero_address(applicant);

        let mut starting_period = exec::block_timestamp();
        let proposal_id = self.proposal_id;
        // compute startingPeriod for proposal
        // there should be a minimum time interval between proposals (period_duration) so that members have time to ragequit
        if proposal_id > 0 {
            let previous_starting_period = self
                .proposals
                .get(&(&proposal_id - 1))
                .expect("Can't be None")
                .starting_period;
            if starting_period < previous_starting_period + self.period_duration {
                starting_period = previous_starting_period + self.period_duration;
            }
        }

        let proposal = Proposal {
            proposer: msg::source(),
            applicant: *applicant,
            quorum,
            amount,
            details,
            starting_period,
            ..Proposal::default()
        };

        self.proposals.insert(proposal_id, proposal);
        self.proposal_id = self.proposal_id.saturating_add(1);
        msg::reply(
            DaoEvent::SubmitFundingProposal {
                proposer: msg::source(),
                applicant: *applicant,
                proposal_id,
                amount,
            },
            0,
        )
        .expect("Error in a reply `DaoEvent::SubmitFungingProposal");
    }

    pub fn submit_vote(&mut self, proposal_id: u128, vote: Vote) {
        let proposal = match self.proposals.get_mut(&proposal_id) {
            Some(proposal) => {
                if exec::block_timestamp() > proposal.starting_period + self.voting_period_length {
                    panic!("proposal voting period has expired");
                }
                if exec::block_timestamp() < proposal.starting_period {
                    panic!("voting period has not started");
                }
                if proposal
                    .votes_by_member
                    .iter()
                    .any(|(actor_id, _vote)| msg::source().eq(actor_id))
                {
                    panic!("account has already voted on this proposal");
                }
                if proposal.aborted {
                    panic!("The proposal has been aborted");
                }
                proposal
            }
            None => {
                panic!("proposal does not exist");
            }
        };

        let member_id = self
            .member_by_delegate_key
            .get(&msg::source())
            .expect("Account is not a delegate");
        let member = self
            .members
            .get_mut(member_id)
            .expect("Account is not a DAO member");

        match vote {
            Vote::Yes => {
                proposal.yes_votes = proposal.yes_votes.saturating_add(member.shares);
                if self.total_shares > proposal.max_total_shares_at_yes_vote {
                    proposal.max_total_shares_at_yes_vote = self.total_shares;
                }
                // it is necessary to save the highest id of the proposal - must be processed for member to ragequit
                if member.highest_index_yes_vote < proposal_id {
                    member.highest_index_yes_vote = proposal_id;
                }
            }
            Vote::No => {
                proposal.no_votes = proposal.no_votes.saturating_add(member.shares);
            }
        }
        proposal.votes_by_member.push((msg::source(), vote.clone()));

        msg::reply(
            DaoEvent::SubmitVote {
                account: msg::source(),
                proposal_id,
                vote,
            },
            0,
        )
        .expect("Error in a reply `DaoEvent::SubmitVote`");
    }

    pub async fn process_proposal(&mut self, transaction_id: Option<u64>, proposal_id: u128) {
        let current_transaction_id = self.get_transaction_id(transaction_id);
        if proposal_id > 0
            && !self
                .proposals
                .get(&(&proposal_id - 1))
                .expect("Proposal does not exist")
                .processed
        {
            panic!("Previous proposal must be processed");
        }
        let proposal = match self.proposals.get_mut(&proposal_id) {
            Some(proposal) => {
                if proposal.processed || proposal.aborted {
                    panic!("Proposal has already been processed or aborted");
                }
                if exec::block_timestamp()
                    < proposal.starting_period
                        + self.voting_period_length
                        + self.grace_period_length
                {
                    panic!("Proposal is not ready to be processed");
                }
                proposal
            }
            None => {
                panic!("proposal does not exist");
            }
        };

        proposal.passed = proposal.yes_votes > proposal.no_votes
            && proposal.yes_votes * 10000 / self.total_shares >= proposal.quorum
            && proposal.max_total_shares_at_yes_vote
                < (self.dilution_bound as u128) * self.total_shares;
        // if membership proposal has passed
        if proposal.passed && proposal.is_membership_proposal {
            let applicant = self.members.entry(proposal.applicant).or_insert(Member {
                delegate_key: proposal.applicant,
                shares: 0,
                highest_index_yes_vote: 0,
            });
            applicant.shares = applicant.shares.saturating_add(proposal.shares_requested);
            self.member_by_delegate_key
                .entry(proposal.applicant)
                .or_insert(proposal.applicant);
            self.total_shares = self.total_shares.saturating_add(proposal.shares_requested);
            self.balance = self.balance.saturating_add(proposal.token_tribute);
        } else if proposal.is_membership_proposal
            && transfer_tokens(
                current_transaction_id,
                &self.approved_token_program_id,
                &exec::program_id(),
                &proposal.applicant,
                proposal.token_tribute,
            )
            .await
            .is_err()
        {
            // the tokens are on the DAO balance
            // we have to rerun that transaction to return tokens to applicant
            msg::reply(DaoEvent::TransactionFailed(current_transaction_id), 0)
                .expect("Error in a reply `DaoEvent::TransactionFailed`");
            return;
        }

        // if funding propoposal has passed
        if proposal.passed
            && !proposal.is_membership_proposal
            && transfer_tokens(
                current_transaction_id,
                &self.approved_token_program_id,
                &exec::program_id(),
                &proposal.applicant,
                proposal.amount,
            )
            .await
            .is_err()
        {
            // the same is here: the tokens are on the DAO balance
            // we have to rerun that transaction to transfer tokens to applicant
            msg::reply(DaoEvent::TransactionFailed(current_transaction_id), 0)
                .expect("Error in a reply `DaoEvent::TransactionFailed`");
            return;
        }
        proposal.processed = true;
        self.transactions.remove(&current_transaction_id);
        msg::reply(
            DaoEvent::ProcessProposal {
                proposal_id,
                passed: proposal.passed,
            },
            0,
        )
        .expect("Error in a reply `DaoEvent::ProcessProposal`");
    }

    pub async fn ragequit(&mut self, transaction_id: Option<u64>, amount: u128) {
        let current_transaction_id = self.get_transaction_id(transaction_id);
        if self.admin == msg::source() {
            panic!("admin can not ragequit");
        }
        let funds = self.redeemable_funds(amount);
        let member = self
            .members
            .get_mut(&msg::source())
            .expect("Account is not a DAO member");

        if amount > member.shares {
            panic!("unsufficient shares");
        }

        let proposal_id = member.highest_index_yes_vote;
        if !self
            .proposals
            .get(&proposal_id)
            .expect("Cant be None")
            .processed
        {
            panic!("cant ragequit until highest index proposal member voted YES on is processed");
        }

        // the tokens are on the DAO balance
        // we have to rerun that transaction to withdraw tokens to applicant in case of error
        if transfer_tokens(
            current_transaction_id,
            &self.approved_token_program_id,
            &exec::program_id(),
            &msg::source(),
            funds,
        )
        .await
        .is_ok()
        {
            member.shares = member.shares.saturating_sub(amount);
            self.total_shares = self.total_shares.saturating_sub(amount);
            self.balance = self.balance.saturating_sub(funds);
            self.transactions.remove(&current_transaction_id);
            msg::reply(
                DaoEvent::RageQuit {
                    member: msg::source(),
                    amount: funds,
                },
                0,
            )
            .expect("Error in a reply `DaoEvent::RageQuit`");
        } else {
            msg::reply(DaoEvent::TransactionFailed(current_transaction_id), 0)
                .expect("Error in a reply `DaoEvent::TransactionFailed`");
        };
    }

    pub async fn abort(&mut self, transaction_id: Option<u64>, proposal_id: u128) {
        let current_transaction_id = self.get_transaction_id(transaction_id);
        let proposal = self
            .proposals
            .get_mut(&proposal_id)
            .expect("The proposal does not exist");

        if proposal.applicant != msg::source() {
            panic!("caller must be applicant");
        }

        if !proposal.is_membership_proposal {
            panic!("The proposal must be membership");
        }

        if proposal.aborted {
            panic!("Proposal has already been aborted");
        }

        if exec::block_timestamp() > proposal.starting_period + self.abort_window {
            panic!("The abort window is over");
        }

        let amount = proposal.token_tribute;

        // if transfer of tokens  fails
        // we have to rerun the transaction to return tokens to applicant
        if transfer_tokens(
            current_transaction_id,
            &self.approved_token_program_id,
            &exec::program_id(),
            &msg::source(),
            amount,
        )
        .await
        .is_ok()
        {
            proposal.token_tribute = 0;
            proposal.aborted = true;

            msg::reply(DaoEvent::Abort(proposal_id), 0)
                .expect("Error in a reply `DaoEvent::Abort`");
        } else {
            msg::reply(DaoEvent::TransactionFailed(current_transaction_id), 0)
                .expect("Error in a reply `DaoEvent::TransactionFailed`");
        };
    }

    pub fn set_admin(&mut self, new_admin: &ActorId) {
        self.assert_admin();
        Self::assert_not_zero_address(new_admin);
        self.admin = *new_admin;
        msg::reply(DaoEvent::AdminUpdated(*new_admin), 0)
            .expect("Error in a reply `DaoEvent::AdminUpdated`");
    }

    pub fn update_delegate_key(&mut self, new_delegate_key: &ActorId) {
        if self.member_by_delegate_key.contains_key(new_delegate_key) {
            panic!("cannot overwrite existing delegate keys");
        }
        Self::assert_not_zero_address(new_delegate_key);

        let member = self
            .members
            .get_mut(&msg::source())
            .expect("Account is not a DAO member");
        self.member_by_delegate_key
            .insert(*new_delegate_key, msg::source());
        member.delegate_key = *new_delegate_key;
        msg::reply(
            DaoEvent::DelegateKeyUpdated {
                member: msg::source(),
                delegate: *new_delegate_key,
            },
            0,
        )
        .expect("Error in a reply `DaoEvent::DelegateKeyUpdated`");
    }

    pub async fn continue_transaction(&mut self, transaction_id: u64) {
        let transactions = self.transactions.clone();
        let payload = &transactions
            .get(&transaction_id)
            .expect("Transaction does not exist");
        if let Some(action) = payload {
            match action {
                DaoAction::SubmitMembershipProposal {
                    applicant,
                    token_tribute,
                    shares_requested,
                    quorum,
                    details,
                } => {
                    self.submit_membership_proposal(
                        Some(transaction_id),
                        applicant,
                        *token_tribute,
                        *shares_requested,
                        *quorum,
                        details.clone(),
                    )
                    .await;
                }
                DaoAction::ProcessProposal(proposal_id) => {
                    self.process_proposal(Some(transaction_id), *proposal_id)
                        .await;
                }
                DaoAction::RageQuit(amount) => {
                    self.ragequit(Some(transaction_id), *amount).await;
                }
                DaoAction::Abort(proposal_id) => {
                    self.abort(Some(transaction_id), *proposal_id).await
                }
                _ => unreachable!(),
            }
        }
    }
}

impl From<&Dao> for DaoState {
    fn from(dao: &Dao) -> DaoState {
        DaoState {
            admin: dao.admin,
            approved_token_program_id: dao.approved_token_program_id,
            period_duration: dao.period_duration,
            voting_period_length: dao.voting_period_length,
            grace_period_length: dao.grace_period_length,
            dilution_bound: dao.dilution_bound,
            abort_window: dao.abort_window,
            total_shares: dao.total_shares,
            balance: dao.balance,
            members: dao
                .members
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            member_by_delegate_key: dao
                .member_by_delegate_key
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect(),
            proposal_id: dao.proposal_id,
            proposals: dao
                .proposals
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            whitelist: dao.whitelist.clone(),
            transaction_id: dao.transaction_id,
            transactions: dao
                .transactions
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: InitDao = msg::load().expect("Unable to decode InitDao");
    let mut dao = Dao {
        admin: config.admin,
        approved_token_program_id: config.approved_token_program_id,
        voting_period_length: config.voting_period_length,
        period_duration: config.period_duration,
        grace_period_length: config.grace_period_length,
        abort_window: config.abort_window,
        dilution_bound: config.dilution_bound,
        total_shares: 1,
        ..Dao::default()
    };
    dao.members.insert(
        config.admin,
        Member {
            delegate_key: config.admin,
            shares: 1,
            highest_index_yes_vote: 0,
        },
    );
    dao.member_by_delegate_key
        .insert(config.admin, config.admin);
    unsafe { DAO = Some(dao) };
}

#[gstd::async_main]
async fn main() {
    let action: DaoAction = msg::load().expect("Could not load Action");
    let dao: &mut Dao = unsafe { DAO.get_or_insert(Default::default()) };
    match action {
        DaoAction::AddToWhiteList(account) => dao.add_to_whitelist(&account),
        DaoAction::SubmitMembershipProposal {
            applicant,
            token_tribute,
            shares_requested,
            quorum,
            ref details,
        } => {
            dao.transactions
                .insert(dao.transaction_id, Some(action.clone()));
            dao.submit_membership_proposal(
                None,
                &applicant,
                token_tribute,
                shares_requested,
                quorum,
                details.to_string(),
            )
            .await;
        }
        DaoAction::SubmitFundingProposal {
            applicant,
            amount,
            quorum,
            details,
        } => {
            dao.submit_funding_proposal(&applicant, amount, quorum, details);
        }
        DaoAction::ProcessProposal(proposal_id) => {
            dao.transactions.insert(dao.transaction_id, Some(action));
            dao.process_proposal(None, proposal_id).await;
        }
        DaoAction::SubmitVote { proposal_id, vote } => {
            dao.submit_vote(proposal_id, vote);
        }
        DaoAction::RageQuit(amount) => {
            dao.transactions.insert(dao.transaction_id, Some(action));
            dao.ragequit(None, amount).await;
        }
        DaoAction::Abort(proposal_id) => {
            dao.transactions.insert(dao.transaction_id, Some(action));
            dao.abort(None, proposal_id).await
        }
        DaoAction::Continue(transaction_id) => dao.continue_transaction(transaction_id).await,
        DaoAction::UpdateDelegateKey(account) => dao.update_delegate_key(&account),
        DaoAction::SetAdmin(account) => dao.set_admin(&account),
    }
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(
        unsafe {
            let dao = DAO.as_ref().expect("Uninitialized dao state");
            let dao_state: DaoState = dao.into();
            dao_state
        },
        0,
    )
    .expect("Failed to share state");
}
