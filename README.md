# ICP Jackpot

## Project Front Page

Here is a preview of the project's front page:

![LotteryX](/Users/anish/Documents/calimero_icp1/lottery/app/lottery-frontend/src/assets/frontimageICPjackpot.png)

## Introduction

ICP Jackpot is a decentralized lottery protocol built on the ICP blockchain using the Calimero SDK. The contract allows users to participate in a lottery by purchasing tickets, with the prize pool funded by ticket sales. Once the lottery ends, a winner is randomly selected from the participants. The winner can claim their prize, and after the prize is claimed, the lottery can be reset for a new round.

## Features

### Ticket Purchase:

- Users can buy lottery tickets during an active lottery round.
- Each ticket has a fixed price in ICP (represented in e8s).
- A user can buy a limited number of tickets per round.

### Prize Pool:

- The prize pool is funded by ticket purchases.
- The total prize pool is accumulated over the lottery round.

### Lottery Closure & Winner Selection:

- Once the lottery ends, the contract selects a winner randomly from the ticket holders.
- The winner is announced along with the prize pool.
- A lottery can only be closed once the lottery period has expired.

### Prize Claiming:

- The winner can claim their prize after the lottery is closed.
- There is a claim deadline, after which the prize cannot be claimed.
  
### Lottery Reset:

- Only the owner can reset the lottery.
- The lottery can be reset after a round, and a new round can start with fresh ticket purchases.

## Contract Details

### Ticket Purchase

- Users purchase tickets by paying a fixed amount in ICP.
- A unique ticket ID is generated for each user.
- The tickets are encrypted to ensure privacy.

### Prize Pool

- The prize pool accumulates as users buy tickets.
- The winner receives the full prize pool upon claiming.

### Winner Selection

- The lottery contract selects a winner randomly after the lottery ends.
- The random selection is done using a secure method to ensure fairness.

### Prize Claim

- The winner can claim the prize pool in ICP tokens.
- Prize claims can only be made within the defined claim deadline.

### Lottery Reset

- After the prize is claimed, the lottery can be reset for a new round by the owner.
- The contract clears all tickets and resets the prize pool for the next round.

## Contract Variables

- **Ticket Price**: Fixed at 1 ICP (in e8s).
- **Max Tickets per User**: Limited to 10 tickets per user.
- **Lottery Duration**: Defined by the owner (in seconds).
- **Claim Deadline**: 7 days after the lottery ends for prize claims.
- **Owner**: The address of the contract owner.

## Functions Overview

### User Functions

- `purchase_ticket()`: Purchase a ticket for the lottery.
- `close_lottery()`: Close the lottery and select a random winner.
- `claim_prize()`: Claim the prize after winning the lottery.
- `reset_lottery(duration_days: u64)`: Reset the lottery for a new round (only callable by the owner).

### Admin Functions

- `ensure_owner()`: Ensures that only the contract owner can call specific functions.

## Usage Instructions

### Deploy the Contract

1. Deploy the Lottery contract on the ICP blockchain using the Calimero SDK.
2. Initialize the contract with the owner's address and ICP token canister address.

### Purchase Ticket

1. Call `purchase_ticket()` to buy a lottery ticket.
2. A unique ticket ID is generated, and the prize pool is updated.

### Close Lottery

1. After the lottery duration has expired, call `close_lottery()` to select a random winner.
2. The winner is selected, and the lottery state is updated.

### Claim Prize

1. Once the lottery is closed, the winner can claim the prize by calling `claim_prize()`.
2. Ensure that the claim is made within the claim deadline.

### Reset Lottery

1. After the prize is claimed, the owner can call `reset_lottery(duration_days)` to reset the lottery.
2. A new lottery round will begin, allowing new ticket purchases.

## Events

- `TicketPurchased(address indexed participant, uint256 amount)`: Emitted when a user purchases a ticket.
- `LotteryClosed(address indexed winner, uint256 prizePool)`: Emitted when the lottery is closed and a winner is selected.
- `PrizeClaimed(address indexed winner, uint256 amount)`: Emitted when the winner claims the prize.
- `LotteryReset`: Emitted when the lottery is reset for a new round.

## Security Considerations

- Ensure the contract is audited before deployment.
- Only deposit funds you can afford to lose.
- Monitor the lottery duration to avoid missing deadlines.
- Ensure that only the owner can call the reset function.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Support

For any inquiries or support, please open an issue on the GitHub repository or contact us via email.

## Links

- GitHub Repository: [LotteryX](GitHub_Link_Here)
- Demo Video: [Watch Here](Demo_Link_Here)
- Project Website: [Visit Here](Website_Link_Here)
