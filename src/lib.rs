use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::serde::{Deserialize, Serialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;
use rand::Rng;
use sha2::{Sha256, Digest};
use ic_cdk::api::call::CallResult;
use ic_cdk::export::Principal;
use ic_cdk::caller;
use ic_cdk::id;
use std::time::{SystemTime, UNIX_EPOCH};

type Balance = u128; // ICP tokens are represented in e8s (1 ICP = 100_000_000 e8s)

#[app::state(emits = Event)]
#[derive(Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct LotteryState {
    tickets: UnorderedMap<String, String>, // Encrypted ticket ID -> Participant (as String)
    prize_pool: Balance,                   // Total prize pool in e8s
    lottery_state: LotteryStateEnum,       // Current state of the lottery
    winner: Option<String>,                // Encrypted ticket ID of the winner
    ticket_price: Balance,                 // Fixed price for each ticket in e8s
    max_tickets_per_user: u32,             // Maximum tickets a user can buy
    total_tickets_sold: u32,               // Total tickets sold
    owner: String,                         // Contract owner (as String)
    icp_token_canister: String,            // ICRC-1 token canister ID (as String)
    lottery_start_time: u64,               // Timestamp when the lottery starts
    lottery_end_time: u64,                 // Timestamp when the lottery ends
    claim_deadline: u64,                   // Timestamp for prize claim deadline
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub enum LotteryStateEnum {
    Open,
    Closed,
    WinnerSelected,
    PrizeClaimed,
}

#[app::event]
pub enum Event {
    LotteryOpened { start_time: u64, end_time: u64 },
    TicketPurchased { ticket_id: String, participant: String, amount: Balance },
    LotteryClosed,
    WinnerSelected { ticket_id: String, prize_pool: Balance },
    PrizeClaimed { winner: String, amount: Balance },
    LotteryReset,
}

#[app::logic]
impl LotteryState {
    #[app::init]
    pub fn init(owner: String, icp_token_canister: String, duration_days: u64) -> LotteryState {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let end_time = now + (duration_days * 86400); // Convert days to seconds
        let claim_deadline = end_time + (7 * 86400); // 7 days after lottery ends to claim prize

        LotteryState {
            tickets: UnorderedMap::new(),
            prize_pool: 0,
            lottery_state: LotteryStateEnum::Open,
            winner: None,
            ticket_price: 100_000_000, // 1 ICP in e8s
            max_tickets_per_user: 10,  // Max 10 tickets per user
            total_tickets_sold: 0,
            owner,
            icp_token_canister,
            lottery_start_time: now,
            lottery_end_time: end_time,
            claim_deadline,
        }
    }

    /// Purchase a ticket
    pub fn purchase_ticket(&mut self) -> Result<(), Error> {
        // Check if the lottery is open
        if self.lottery_state != LotteryStateEnum::Open {
            return Err(Error::msg("Lottery is not open"));
        }

        // Check if the lottery has ended
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if now >= self.lottery_end_time {
            self.lottery_state = LotteryStateEnum::Closed;
            return Err(Error::msg("Lottery is closed"));
        }

        // Check if the user has reached the maximum ticket limit
        let caller_id = caller().to_string();
        let user_tickets = self.tickets.entries()?
            .filter(|(_, account_id)| account_id == &caller_id)
            .count();
        if user_tickets >= self.max_tickets_per_user as usize {
            return Err(Error::msg("Maximum tickets per user reached"));
        }

        // Generate a unique ticket ID using SHA-256
        let mut hasher = Sha256::new();
        hasher.update(caller_id.as_bytes());
        let random_bytes: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
        hasher.update(random_bytes);
        let ticket_id = hex::encode(hasher.finalize());

        if self.tickets.get(&ticket_id)?.is_some() {
            return Err(Error::msg("Ticket ID already exists"));
        }

        // Add the ticket to the lottery
        self.tickets.insert(ticket_id.clone(), caller_id.clone())?;
        self.prize_pool += self.ticket_price; // Add ticket price to the prize pool
        self.total_tickets_sold += 1;

        env::emit(&Event::TicketPurchased {
            ticket_id: ticket_id.clone(),
            participant: caller_id,
            amount: self.ticket_price,
        });

        Ok(())
    }

    /// Close the lottery and select a winner
    pub fn close_lottery(&mut self) -> Result<(), Error> {
        // Check if the lottery has ended
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if now < self.lottery_end_time {
            return Err(Error::msg("Lottery is still open"));
        }

        if self.lottery_state != LotteryStateEnum::Open {
            return Err(Error::msg("Lottery is not open"));
        }

        self.lottery_state = LotteryStateEnum::Closed;

        // Select a random winner
        let ticket_ids: Vec<String> = self.tickets.entries()?
            .map(|(k, _)| k)
            .collect();
        if ticket_ids.is_empty() {
            return Err(Error::msg("No tickets purchased"));
        }

        let random_index = rand::thread_rng().gen_range(0..ticket_ids.len());
        let winner_ticket_id = ticket_ids[random_index].clone();
        self.winner = Some(winner_ticket_id.clone());

        env::emit(&Event::LotteryClosed);
        env::emit(&Event::WinnerSelected {
            ticket_id: winner_ticket_id.clone(),
            prize_pool: self.prize_pool,
        });

        Ok(())
    }

    /// Claim the prize
    pub fn claim_prize(&mut self) -> Result<(), Error> {
        if self.lottery_state != LotteryStateEnum::Closed {
            return Err(Error::msg("Lottery is not closed"));
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if now > self.claim_deadline {
            return Err(Error::msg("Prize claim deadline has passed"));
        }

        let caller_id = caller().to_string();
        let winner_ticket_id = self.winner.as_ref().ok_or_else(|| Error::msg("No winner selected"))?;
        let winner_account = self.tickets.get(winner_ticket_id)?
            .ok_or_else(|| Error::msg("Winner not found"))?;

        if caller_id != winner_account {
            return Err(Error::msg("You are not the winner"));
        }

        // Emit the prize claimed event
        env::emit(&Event::PrizeClaimed {
            winner: winner_ticket_id.clone(),
            amount: self.prize_pool,
        });

        self.prize_pool = 0;
        self.lottery_state = LotteryStateEnum::PrizeClaimed;

        Ok(())
    }

    /// Reset the lottery for a new round (only callable by the owner)
    pub fn reset_lottery(&mut self, duration_days: u64) -> Result<(), Error> {
        self.ensure_owner()?;
        if self.lottery_state != LotteryStateEnum::PrizeClaimed {
            return Err(Error::msg("Prize must be claimed before resetting the lottery"));
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let end_time = now + (duration_days * 86400);
        let claim_deadline = end_time + (7 * 86400);

        self.tickets.clear();
        self.prize_pool = 0;
        self.winner = None;
        self.total_tickets_sold = 0;
        self.lottery_state = LotteryStateEnum::Open;
        self.lottery_start_time = now;
        self.lottery_end_time = end_time;
        self.claim_deadline = claim_deadline;

        env::emit(&Event::LotteryReset);

        Ok(())
    }

    /// Ensure the caller is the contract owner
    fn ensure_owner(&self) -> Result<(), Error> {
        if caller().to_string() != self.owner {
            return Err(Error::msg("Only the owner can call this function"));
        }
        Ok(())
    }
}