# Snake AI Backend - Enhanced Patron Framework

A comprehensive Solana program implementing advanced tokenomics, role-based governance, and DeFi utilities for the Snake AI ecosystem.

## 🎯 Program Overview

**Program ID**: `GkRRA3Jhds6sxDr89wMneCjNDDmHof2zqnHdjaqP7kGU`

### Core Architecture

The Snake AI program implements a sophisticated token ecosystem with multiple interconnected systems:

- **🎭 Role-Based System**: Tiered user roles with unique privileges
- **🏦 Advanced Vesting**: Time-locked tokens with yield rewards  
- **🔄 Enhanced OTC Trading**: Multi-type swap marketplace
- **👑 Patron Framework**: Elite user tier with DAO preparation
- **⚖️ Governance Foundation**: DAO-ready infrastructure

## 🏗️ System Components

### 1. User Role Registry

#### Role Types
```rust
pub enum UserRole {
    None,    // Default state
    Staker,  // 3-month commitment, 5% APY
    Patron,  // 6-month commitment, DAO eligible
}
```

#### Role Selection Process
- Mining history validation (Phase 1 participation required)
- Automatic role-based benefit calculation
- Lock duration enforcement (3 months Staker, 6 months Patron)

### 2. Enhanced Vesting System

#### Vesting Types
```rust
pub enum VestingType {
    Staker,  // 3 months with 5% APY yield
    Patron,  // 6 months commitment for DAO qualification
}
```

#### Key Features
- **Linear Vesting**: Gradual token release over time
- **Yield Rewards**: Continuous 5% APY for Stakers
- **Cliff Periods**: Configurable unlock schedules
- **Early Exit Penalties**: Burn mechanisms for violations

#### Vesting Account Structure
```rust
pub struct VestingSchedule {
    pub beneficiary: Pubkey,
    pub total_amount: u64,
    pub vested_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub duration_months: u8,
    pub vesting_type: VestingType,
    pub is_active: bool,
    pub last_claim_time: i64,
    pub yield_rate: u16, // APY in basis points
}
```

### 3. Enhanced OTC Trading

#### Swap Types
```rust
pub enum SwapType {
    NormalToPatron,     // Basic users → Patrons
    NormalToStaker,     // Basic users → Stakers  
    PatronToPatron,     // Patron → Patron (with penalties)
    TreasuryBuyback,    // Project liquidity management
    AnyToAny,           // Open market trading
}
```

#### OTC Swap Structure
```rust
pub struct OtcSwap {
    pub seller: Pubkey,
    pub token_amount: u64,
    pub sol_rate: u64,
    pub buyer_rebate: u64,
    pub swap_type: SwapType,
    pub is_active: bool,
    pub created_at: i64,
    pub expires_at: i64,
}
```

### 4. Patron Framework

#### Application System
```rust
pub struct UserClaim {
    // ... base fields
    pub patron_status: PatronStatus,
    pub patron_qualification_score: u32,
    pub wallet_age_days: u32,
    pub community_score: u32,
    pub patron_approval_timestamp: i64,
}

pub enum PatronStatus {
    None,
    Applied,
    Approved,
    Revoked,
}
```

#### Qualification Scoring
- **Mining History**: Phase 1 participation weight
- **Wallet Age**: Account creation verification  
- **Community Score**: Contribution assessment (0-30 points)
- **Total Score**: Combined qualification metric

### 5. Governance Infrastructure

#### DAO Registry
```rust
pub struct DaoRegistry {
    pub total_seats: u32,
    pub occupied_seats: u32,
    pub min_stake_amount: u64,
    pub month6_timestamp: i64,
    pub is_active: bool,
}
```

#### DAO Eligibility Rules
- Must be approved Patron
- Complete 6-month vesting commitment
- No early exit violations
- Minimum stake requirements met

## 🚀 Key Instructions

### Role Management
```rust
// Select user role (requires mining history)
pub fn select_role(ctx: Context<SelectRole>, role: UserRole) -> Result<()>

// Update user statistics for patron qualification
pub fn update_user_stats(ctx: Context<UpdateUserStats>, params: UpdateUserStatsParams) -> Result<()>
```

### Patron System
```rust
// Apply for patron status
pub fn apply_for_patron(ctx: Context<ApplyForPatron>, wallet_age_days: u32, community_score: u32) -> Result<()>

// Admin approval of patron application
pub fn approve_patron_application(ctx: Context<ApprovePatronApplication>, min_qualification_score: u32) -> Result<()>

// Revoke patron status for violations
pub fn revoke_patron_status(ctx: Context<ApprovePatronApplication>) -> Result<()>
```

### Vesting Operations
```rust
// Create role-based vesting schedule
pub fn create_vesting_schedule(ctx: Context<CreateVesting>, vesting_amount: u64) -> Result<()>

// Claim vested tokens and yield
pub fn claim_vested_tokens(ctx: Context<WithdrawVesting>) -> Result<()>
```

### Enhanced OTC Trading
```rust
// Create enhanced swap with type specification
pub fn initiate_otc_swap_enhanced(
    ctx: Context<InitiateOtcSwapEnhanced>, 
    token_amount: u64, 
    sol_rate: u64,
    buyer_rebate: u64,
    swap_type: SwapType
) -> Result<()>

// Accept patron-to-patron swap (with burn penalty)
pub fn accept_otc_swap_patron_to_patron(ctx: Context<AcceptOtcSwapPatronToPatron>) -> Result<()>

// Treasury buyback mechanism
pub fn accept_treasury_buyback(ctx: Context<AcceptTreasuryBuyback>) -> Result<()>
```

### Governance Functions
```rust
// Initialize DAO registry
pub fn initialize_dao_registry(
    ctx: Context<InitializeDAORegistry>,
    max_seats: u32,
    min_dao_stake: u64,
    month6_timestamp: i64,
) -> Result<()>

// Allocate DAO seat to qualified patron
pub fn allocate_dao_seat(ctx: Context<AllocateDAOSeat>, current_balance: u64) -> Result<()>
```

## 🎯 Economic Model

### Token Flow
```
Phase 1 Mining → Role Selection → Vesting/Staking → DAO Participation
     ↓              ↓              ↓                    ↓
  Mining Rewards → Yield Earning → OTC Trading → Governance Rights
```

### Yield Calculation
- **Staker APY**: 5% annually on vested tokens
- **Yield Formula**: `(amount × rate × time) / (10000 × seconds_in_year)`
- **Compound Interest**: Continuous accrual with claim flexibility

### Penalty System
- **Early Patron Exit**: 20% burn penalty
- **Role Violations**: Potential patron status revocation
- **DAO Seat Loss**: Failure to maintain minimum stake

## 📊 Account Structure

### Program Derived Addresses (PDAs)

#### User Accounts
```rust
// User claim account
seeds = [b"user_claim", user.key()]

// Vesting schedule  
seeds = [b"vesting", user.key()]

// Escrow for vested tokens
seeds = [b"escrow", user.key()]
```

#### Trading Accounts
```rust
// OTC swap
seeds = [b"otc_swap", seller.key()]

// OTC swap tracker
seeds = [b"otc_tracker", user.key()]
```

#### System Accounts
```rust
// Reward pool
seeds = [b"reward_pool"]

// DAO registry
seeds = [b"dao_registry"]
```

## 🔐 Security Features

### Access Control
- **Role-based permissions**: Function access by user role
- **Admin-only operations**: Critical system functions
- **Time-locked commitments**: Enforced lock periods
- **Mining history validation**: Phase 1 participation required

### Economic Security
- **Burn mechanisms**: Penalty enforcement
- **Lock period validation**: Prevents early exits
- **Qualification scoring**: Merit-based patron selection
- **Stake requirements**: DAO participation barriers

### Technical Security
- **PDA derivation**: Deterministic account generation
- **Constraint validation**: Comprehensive input checking
- **Arithmetic safety**: Overflow/underflow protection
- **Account initialization**: Proper state management

## 🧪 Testing

### Test Coverage
- **Unit Tests**: Individual instruction testing
- **Integration Tests**: Cross-feature interactions
- **Role Tests**: Permission and access validation
- **Economic Tests**: Yield calculations and penalties
- **Security Tests**: Attack vector prevention

### Running Tests
```bash
# Build program
anchor build

# Run all tests
anchor test

# Run specific test file
anchor test --skip-deploy tests/vesting_OTCswap.ts
```

### Test Results Summary
- ✅ **25 passing tests**: Core functionality verified
- ✅ **Role system**: Complete validation
- ✅ **Vesting mechanics**: Yield and time-lock testing
- ✅ **OTC trading**: All swap types functional
- ✅ **Patron framework**: Application and approval flow

## 🛠️ Development Setup

### Prerequisites
- Rust 1.70+
- Solana CLI 1.16+
- Anchor Framework 0.28+
- Node.js 16+ (for tests)

### Installation
```bash
# Clone repository
git clone <repository_url>
cd snakeAI_BE

# Install Anchor
npm install -g @coral-xyz/anchor-cli

# Build program
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

### Project Structure
```
programs/
└── snake_contract/
    ├── src/
    │   ├── lib.rs              # Main program entry
    │   ├── state.rs            # Account structures
    │   ├── errors.rs           # Custom error definitions
    │   ├── events.rs           # Event emissions
    │   ├── constants.rs        # System constants
    │   └── instructions/       # Instruction implementations
    │       ├── mod.rs
    │       ├── role_selection.rs
    │       ├── vesting.rs
    │       ├── otc_swap_enhanced.rs
    │       ├── patron_application.rs
    │       └── dao_governance.rs
    └── Cargo.toml

tests/                          # Comprehensive test suite
├── user_role_registry_tests.ts
├── patron_framework.ts
├── vesting_OTCswap.ts
└── simple_role_test.ts

frontend/                       # React frontend
backend/                        # API backend (if applicable)
```

## 🔄 Integration Guide

### Frontend Integration
See [frontend/README.md](frontend/README.md) for detailed frontend integration guide.

### API Integration
The program integrates with RESTful APIs for:
- Transaction construction
- Account state queries
- Real-time updates
- User interface data

### Wallet Integration
- **Supported Wallets**: Phantom, Solflare, Backpack
- **Transaction Signing**: Client-side signature requests
- **Account Detection**: Automatic wallet connection

## 📈 Performance Metrics

### Transaction Costs
- **Role Selection**: ~0.001 SOL
- **Vesting Creation**: ~0.002 SOL  
- **OTC Swap**: ~0.0015 SOL
- **Claim Operations**: ~0.001 SOL

### Compute Units
- **Average**: 5,000-15,000 CU per instruction
- **Complex Operations**: Up to 50,000 CU
- **Optimization**: Efficient PDA derivation

## 🎯 Roadmap

### Phase 1: ✅ Core Infrastructure
- [x] Role-based system
- [x] Basic vesting mechanics
- [x] OTC trading foundation

### Phase 2: ✅ Enhanced Features  
- [x] Advanced vesting with yield
- [x] Enhanced OTC swap types
- [x] Patron framework
- [x] Qualification scoring

### Phase 3: 🔄 DAO Integration
- [x] DAO registry infrastructure
- [x] Seat allocation mechanics
- [ ] Governance proposal system
- [ ] Voting mechanisms

### Phase 4: 🔮 Advanced Features
- [ ] Cross-program integrations
- [ ] Advanced DeFi utilities
- [ ] Mobile wallet support
- [ ] Analytics dashboard

## 📚 Documentation

- **Instruction Guide**: Detailed instruction documentation
- **Account Reference**: Complete account structure guide  
- **Integration Examples**: Code samples and tutorials
- **Security Best Practices**: Development guidelines

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Built with ⚡ on Solana using Anchor Framework