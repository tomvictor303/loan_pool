#![no_main]
use anyhow::Result;
use bitcoin::consensus;
use bitcoin::Transaction;
use borsh::{BorshDeserialize, BorshSerialize};
use sdk::{entrypoint, Pubkey, UtxoInfo};


// Wallet Structure
#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct Wallet {
    pub wallet_address: String,
    pub balance: u32,
}

// Wallet Instructions Enumeration
#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub enum WalletInstruction {
    BorrowLoan(BorrowLoanParams),
}

impl WalletInstruction {
    pub fn tx_hex(&self) -> Vec<u8> {
        match self {
            WalletInstruction::BorrowLoan(inner) => inner.tx_hex.clone(),
        }
    }
}

// Borrow Loan Parameters
#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct BorrowLoanParams {
    pub borrower_wallet: Wallet,
    pub nft_id: String,
    pub nft_price: u32,
    pub borrower_pay_amount: u32,
    pub loan_amount: u32,
    pub repay_duration: u32,
    pub interest: u32,
    pub fee: u32,
    pub tx_hex: Vec<u8>,
}

pub fn BorrowLoan(mut params: BorrowLoanParams) -> Result<Wallet, String> {
    let nft_price = params.nft_price;
    let fee = params.fee;
    let borrower_balance = params.borrower_wallet.balance;
    let borrower_total_payment = nft_price / 5 + fee;

    // Check if user has enough balance to cover the NFT price and fees
    if borrower_balance < borrower_total_payment {
        return Err("Insufficient balance to cover payment and fees.".to_string());
    }

    // Deduct the total payment from user's balance
    params.borrower_wallet.balance -= borrower_total_payment;
    
    // Return the updated borrower wallet wrapped in Ok
    Ok(params.borrower_wallet)
}



#[cfg(target_os = "zkvm")]
entrypoint!(handler);

#[cfg(target_os = "zkvm")]
fn handler(_program_id: &Pubkey, utxos: &[UtxoInfo], instruction_data: &[u8]) -> Result<Vec<u8>> {
    let wallet_instruction: WalletInstruction = borsh::from_slice(instruction_data)?;
    let tx_hex = wallet_instruction.tx_hex();
    let account = match wallet_instruction {
        WalletInstruction::BorrowLoan(params) => BorrowLoan(params),
    };

    for utxo in utxos {
        *utxo.data.borrow_mut() = borsh::to_vec(&account).expect("Account should be serializable");
    }

    let tx: Transaction = consensus::deserialize(&tx_hex).unwrap();
    Ok(consensus::serialize(&tx))
}
