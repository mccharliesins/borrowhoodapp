#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub symbol: String,
    pub name: String,
    pub lend_rate: i128,
    pub borrow_rate: i128,
    pub total_supplied: i128,
    pub total_borrowed: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loan {
    pub borrower: Address,
    pub token: String,
    pub amount: i128,
    pub interest_rate: i128,
    pub term_days: u32,
    pub status: LoanStatus,
    pub created_at: u64,
    pub repaid_at: Option<u64>,
    pub pool_type: PoolType,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoanStatus {
    Requested,
    Approved,
    Disbursed,
    Repaid,
    Defaulted,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PoolType {
    Global,
    Isolated,
    NativeStake,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LendingPool {
    pub owner: Address,
    pub token: String,
    pub pool_type: PoolType,
    pub total_liquidity: i128,
    pub available_liquidity: i128,
    pub min_loan_amount: i128,
    pub max_loan_amount: i128,
}

// Events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoanCreatedEvent {
    pub loan_id: u32,
    pub borrower: Address,
    pub token: String,
    pub amount: i128,
    pub interest_rate: i128,
    pub term_days: u32,
    pub pool_type: PoolType,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoanDisbursedEvent {
    pub loan_id: u32,
    pub borrower: Address,
    pub token: String,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoanRepaidEvent {
    pub loan_id: u32,
    pub borrower: Address,
    pub token: String,
    pub amount: i128,
    pub interest_paid: i128,
}

#[contract]
pub struct LendingContract;

#[contractimpl]
impl LendingContract {
    // Initialize the contract with supported tokens
    pub fn initialize(env: &Env, owner: Address) {
        env.storage().instance().set(&symbol_short!("owner"), &owner);
        
        // Initialize supported tokens
        let mut tokens = Map::<String, Token>::new(env);
        tokens.set(
            String::from_str(env, "XLM"),
            Token {
                symbol: String::from_str(env, "XLM"),
                name: String::from_str(env, "Stellar Lumens"),
                lend_rate: 520,  // 5.2%
                borrow_rate: 680, // 6.8%
                total_supplied: 0,
                total_borrowed: 0,
            },
        );
        tokens.set(
            String::from_str(env, "USDC"),
            Token {
                symbol: String::from_str(env, "USDC"),
                name: String::from_str(env, "USD Coin"),
                lend_rate: 810,  // 8.1%
                borrow_rate: 950, // 9.5%
                total_supplied: 0,
                total_borrowed: 0,
            },
        );
        
        env.storage().instance().set(&symbol_short!("tokens"), &tokens);
        env.storage().instance().set(&symbol_short!("pools"), &Map::<u32, LendingPool>::new(env));
        env.storage().instance().set(&symbol_short!("loans"), &Map::<u32, Loan>::new(env));
        env.storage().instance().set(&symbol_short!("nxt_loan"), &0u32);
    }

    // Create a new lending pool
    pub fn create_pool(
        env: &Env,
        token: String,
        pool_type: PoolType,
        min_loan_amount: i128,
        max_loan_amount: i128,
    ) -> u32 {
        let contract_owner: Address = env.storage().instance().get(&symbol_short!("owner")).unwrap();
        contract_owner.require_auth();

        let pool = LendingPool {
            owner: env.current_contract_address(),
            token: token.clone(),
            pool_type,
            total_liquidity: 0,
            available_liquidity: 0,
            min_loan_amount,
            max_loan_amount,
        };

        let pool_id = env.storage().instance().get(&symbol_short!("nxt_pool")).unwrap_or(0u32);
        let mut pools: Map<u32, LendingPool> = env.storage().instance().get(&symbol_short!("pools")).unwrap();
        
        pools.set(pool_id, pool);
        env.storage().instance().set(&symbol_short!("pools"), &pools);
        env.storage().instance().set(&symbol_short!("nxt_pool"), &(pool_id + 1));

        pool_id
    }

    // Supply assets to a pool
    pub fn supply(env: &Env, pool_id: u32, amount: i128) {
        let mut pools: Map<u32, LendingPool> = env.storage().instance().get(&symbol_short!("pools")).unwrap();
        let mut pool = pools.get(pool_id).unwrap();
        
        let mut tokens: Map<String, Token> = env.storage().instance().get(&symbol_short!("tokens")).unwrap();
        let mut token = tokens.get(pool.token.clone()).unwrap();
        
        pool.total_liquidity += amount;
        pool.available_liquidity += amount;
        token.total_supplied += amount;
        
        pools.set(pool_id, pool.clone());
        tokens.set(pool.token.clone(), token);
        
        env.storage().instance().set(&symbol_short!("pools"), &pools);
        env.storage().instance().set(&symbol_short!("tokens"), &tokens);
    }

    // Request a loan
    pub fn request_loan(
        env: &Env,
        pool_id: u32,
        amount: i128,
        term_days: u32,
    ) -> u32 {
        let pools: Map<u32, LendingPool> = env.storage().instance().get(&symbol_short!("pools")).unwrap();
        let pool = pools.get(pool_id).unwrap();
        
        let tokens: Map<String, Token> = env.storage().instance().get(&symbol_short!("tokens")).unwrap();
        let token = tokens.get(pool.token.clone()).unwrap();

        assert!(amount >= pool.min_loan_amount, "Loan amount too small");
        assert!(amount <= pool.max_loan_amount, "Loan amount too large");
        assert!(amount <= pool.available_liquidity, "Insufficient pool liquidity");

        let loan = Loan {
            borrower: env.current_contract_address(),
            token: pool.token.clone(),
            amount,
            interest_rate: token.borrow_rate,
            term_days,
            status: LoanStatus::Requested,
            created_at: env.ledger().timestamp(),
            repaid_at: None,
            pool_type: pool.pool_type.clone(),
        };

        let loan_id = env.storage().instance().get(&symbol_short!("nxt_loan")).unwrap_or(0u32);
        let mut loans: Map<u32, Loan> = env.storage().instance().get(&symbol_short!("loans")).unwrap();
        
        loans.set(loan_id, loan);
        env.storage().instance().set(&symbol_short!("loans"), &loans);
        env.storage().instance().set(&symbol_short!("nxt_loan"), &(loan_id + 1));

        env.events().publish(
            ("loan", "created"),
            LoanCreatedEvent {
                loan_id,
                borrower: env.current_contract_address(),
                token: pool.token,
                amount,
                interest_rate: token.borrow_rate,
                term_days,
                pool_type: pool.pool_type,
            },
        );

        loan_id
    }

    // View functions
    pub fn get_token_info(env: &Env, symbol: String) -> Token {
        let tokens: Map<String, Token> = env.storage().instance().get(&symbol_short!("tokens")).unwrap();
        tokens.get(symbol).unwrap()
    }

    pub fn get_pool(env: &Env, pool_id: u32) -> LendingPool {
        let pools: Map<u32, LendingPool> = env.storage().instance().get(&symbol_short!("pools")).unwrap();
        pools.get(pool_id).unwrap()
    }

    pub fn get_loan(env: &Env, loan_id: u32) -> Loan {
        let loans: Map<u32, Loan> = env.storage().instance().get(&symbol_short!("loans")).unwrap();
        loans.get(loan_id).unwrap()
    }
}