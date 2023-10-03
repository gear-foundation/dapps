#![no_std]

use dao_light_io::*;
use ft_messages::*;
use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId, String};

mod ft_messages;

#[derive(Debug, Default)]
struct Dao {
    approved_token_program_id: ActorId,
    period_duration: u64,
    voting_period_length: u64,
    grace_period_length: u64,
    total_shares: u128,
    members: HashMap<ActorId, Member>,
    proposal_id: u128,
    locked_funds: u128,
    proposals: HashMap<u128, Proposal>,
}

impl From<Dao> for DaoState {
    fn from(value: Dao) -> Self {
        let Dao {
            approved_token_program_id,
            period_duration,
            voting_period_length,
            grace_period_length,
            total_shares,
            members,
            proposal_id,
            locked_funds,
            proposals,
        } = value;

        let members = members.into_iter().collect();
        let proposals = proposals.into_iter().collect();

        Self {
            approved_token_program_id,
            period_duration,
            voting_period_length,
            grace_period_length,
            total_shares,
            members,
            proposal_id,
            locked_funds,
            proposals,
        }
    }
}

static mut DAO: Option<Dao> = None;

impl Dao {
    /// Deposits tokens to DAO
    /// Arguments:
    /// * `amount`: the number of fungible tokens that user wants to deposit to DAO
    async fn deposit(&mut self, amount: u128) {
        let share = self.calculate_share(amount).await;
        transfer_tokens(
            &self.approved_token_program_id,
            &msg::source(),
            &exec::program_id(),
            amount,
        )
        .await;
        self.members
            .entry(msg::source())
            .and_modify(|member| member.shares += share)
            .or_insert(Member {
                shares: share,
                highest_index_yes_vote: None,
            });

        self.total_shares = self.total_shares.saturating_add(share);
        msg::reply(
            DaoEvent::Deposit {
                member: msg::source(),
                share,
            },
            0,
        )
        .unwrap();
    }

    /// The proposal of funding
    /// Requirements:
    /// * The proposal can be submitted only by the existing members or their delegate addresses
    /// * The applicant ID can't be the zero
    /// * The DAO must have enough funds to finance the proposal
    /// Arguments:
    /// * `applicant`: an actor that will be funded
    /// * `amount`: the number of fungible tokens that will be sent to the applicant
    /// * `quorum`: a certain threshold of YES votes in order for the proposal to pass
    /// * `details`: the proposal description
    async fn submit_funding_proposal(
        &mut self,
        applicant: &ActorId,
        amount: u128,
        quorum: u128,
        details: String,
    ) {
        self.check_for_membership();

        if applicant.is_zero() {
            panic!("Proposal for the zero address");
        }

        // check that DAO has sufficient funds
        let balance = balance(&self.approved_token_program_id, &exec::program_id()).await;
        if balance.saturating_sub(self.locked_funds) < amount {
            panic!("Not enough funds in DAO");
        }

        let mut starting_period = exec::block_timestamp();
        // compute startingPeriod for proposal
        // there should be a minimum time interval between proposals (period_duration) so that members have time to ragequit
        if self.proposal_id > 0 {
            let previous_starting_period = self.proposals[&(self.proposal_id - 1)].starting_period;
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
            ended_at: starting_period + self.voting_period_length,
            ..Default::default()
        };

        self.proposals.insert(self.proposal_id, proposal);

        msg::reply(
            DaoEvent::SubmitFundingProposal {
                proposer: msg::source(),
                applicant: *applicant,
                proposal_id: self.proposal_id,
                amount,
            },
            0,
        )
        .unwrap();
        self.proposal_id = self.proposal_id.saturating_add(1);
        self.locked_funds = self.locked_funds.saturating_add(amount);
    }

    /// The member submit his vote (YES or NO) on the proposal
    /// Requirements:
    /// * The proposal can be submitted only by the existing members or their delegate addresses
    /// * The member can vote on the proposal only once
    /// * Proposal must exist, the voting period must has started and not expired
    /// Arguments:
    /// * `proposal_id`: the proposal ID
    /// * `vote`: the member  a member vote (YES or NO)
    fn submit_vote(&mut self, proposal_id: u128, vote: Vote) {
        self.check_for_membership();

        // checks that proposal exists, the voting period has started, not expired and that member did not vote on the proposal
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
                    .any(|(actor, _vote)| msg::source().eq(actor))
                {
                    panic!("account has already voted on that proposal");
                }
                proposal
            }
            None => {
                panic!("proposal does not exist");
            }
        };

        let member = self.members.get_mut(&msg::source()).unwrap();

        match vote {
            Vote::Yes => {
                proposal.yes_votes = proposal.yes_votes.saturating_add(member.shares);
                // it is necessary to save the highest id of the proposal - must be processed for member to ragequit
                let id = member.highest_index_yes_vote.get_or_insert(proposal_id);
                *id = proposal_id.max(*id);
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
        .unwrap();
    }

    /// The proposal processing after the proposal completes during the grace period.
    /// If the proposal is accepted, the indicated amount of tokens are sent to the applicant.
    /// Requirements:
    /// * The previous proposal must be processed
    /// * The proposal must exist and be ready for processing
    /// * The proposal must not be already be processed
    /// Arguments:
    /// * `proposal_id`: the proposal ID
    async fn process_proposal(&mut self, proposal_id: u128) {
        if proposal_id > 0 && !self.proposals.get(&(&proposal_id - 1)).unwrap().processed {
            panic!("Previous proposal must be processed");
        }
        let proposal = match self.proposals.get_mut(&proposal_id) {
            Some(proposal) => {
                if proposal.processed {
                    panic!("Proposal has already been processed");
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

        proposal.processed = true;
        proposal.did_pass = proposal.yes_votes > proposal.no_votes
            && proposal.yes_votes * 10_000 / self.total_shares >= proposal.quorum * 100;

        // if funding propoposal has passed
        if proposal.did_pass {
            transfer_tokens(
                &self.approved_token_program_id,
                &exec::program_id(),
                &proposal.applicant,
                proposal.amount,
            )
            .await;
        }
        self.locked_funds = self.locked_funds.saturating_sub(proposal.amount);
        let balance = balance(&self.approved_token_program_id, &exec::program_id()).await;
        if balance == 0 {
            self.total_shares = 0;
            self.members = HashMap::new();
        }
        msg::reply(
            DaoEvent::ProcessProposal {
                applicant: proposal.applicant,
                proposal_id,
                did_pass: proposal.did_pass,
            },
            0,
        )
        .unwrap();
    }

    /// Withdraws the capital of the member
    /// Requirements:
    /// * `msg::source()` must be DAO member
    /// * The member must have sufficient amount of shares
    /// * The latest proposal the member voted YES must be processed
    /// Arguments:
    /// * `amount`: The amount of shares the member would like to withdraw
    async fn ragequit(&mut self, amount: u128) {
        if !self.members.contains_key(&msg::source()) {
            panic!("account is not a DAO member");
        }
        let member = self.members.get_mut(&msg::source()).unwrap();
        if amount > member.shares {
            panic!("unsufficient shares");
        }
        if let Some(proposal_id) = member.highest_index_yes_vote {
            if let Some(proposal) = self.proposals.get(&proposal_id) {
                if !proposal.processed {
                    panic!("cant ragequit until highest index proposal member voted YES on is processed");
                }
            }
        }
        member.shares = member.shares.saturating_sub(amount);
        let funds = self.redeemable_funds(amount).await;
        transfer_tokens(
            &self.approved_token_program_id,
            &exec::program_id(),
            &msg::source(),
            funds,
        )
        .await;
        self.total_shares = self.total_shares.saturating_sub(amount);
        msg::reply(
            DaoEvent::RageQuit {
                member: msg::source(),
                amount: funds,
            },
            0,
        )
        .unwrap();
    }

    // calculates the funds that the member can redeem based on his shares
    async fn redeemable_funds(&self, share: u128) -> u128 {
        let balance = balance(&self.approved_token_program_id, &exec::program_id()).await;
        (share * balance) / self.total_shares
    }

    // calculates a share a user can receive for his deposited tokens
    async fn calculate_share(&self, tokens: u128) -> u128 {
        let balance = balance(&self.approved_token_program_id, &exec::program_id()).await;
        if balance == 0 {
            return tokens;
        }
        (self.total_shares * tokens) / balance
    }

    // checks that account is DAO member
    fn is_member(&self, account: &ActorId) -> bool {
        matches!(self.members.get(account), Some(member) if member.shares > 0)
    }

    // check that `msg::source()` is either a DAO member or a delegate key
    fn check_for_membership(&self) {
        if !self.is_member(&msg::source()) {
            panic!("account is not a DAO member")
        }
    }
}

#[no_mangle]
extern fn init() {
    let config: InitDao = msg::load().expect("Unable to decode InitDao");
    let dao = Dao {
        approved_token_program_id: config.approved_token_program_id,
        voting_period_length: config.voting_period_length,
        period_duration: config.period_duration,
        ..Dao::default()
    };
    unsafe { DAO = Some(dao) };
}

#[gstd::async_main]
async fn main() {
    let action: DaoAction = msg::load().expect("Could not load Action");
    let dao: &mut Dao = unsafe { DAO.get_or_insert(Dao::default()) };
    match action {
        DaoAction::Deposit { amount } => dao.deposit(amount).await,
        DaoAction::SubmitFundingProposal {
            applicant,
            amount,
            quorum,
            details,
        } => {
            dao.submit_funding_proposal(&applicant, amount, quorum, details)
                .await;
        }
        DaoAction::ProcessProposal { proposal_id } => {
            dao.process_proposal(proposal_id).await;
        }
        DaoAction::SubmitVote { proposal_id, vote } => {
            dao.submit_vote(proposal_id, vote);
        }
        DaoAction::RageQuit { amount } => {
            dao.ragequit(amount).await;
        }
    }
}

#[no_mangle]
extern fn state() {
    let dao = unsafe { DAO.take().expect("Unexpected error in taking state") };
    msg::reply::<DaoState>(dao.into(), 0)
        .expect("Failed to encode or reply with `DaoState` from `state()`");
}
