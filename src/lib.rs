
#[cfg(test)]
mod tests {
    use super::*;
    use bitcoincore_rpc::{Auth, Client};
    use borsh::{BorshDeserialize, BorshSerialize};
    use common::constants::*;
    use common::helper::*;
    use common::models::*;
    use sdk::{Pubkey, UtxoMeta};
    use serial_test::serial;
    use std::thread;
    use std::str::FromStr;

    
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

    // BorrowLoan Function
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

    
    #[test]
    fn test_borrow_loan() {
        let borrower_wallet = Wallet {
            wallet_address: "bc1qexyqtwyxrey8jkq9qmrnu4jtuwqkyusm5adgdn".to_string(),
            balance: 1000,
        };

        let params = BorrowLoanParams {
            borrower_wallet: borrower_wallet.clone(),
            nft_id: "nft123".to_string(),
            nft_price: 500,
            borrower_pay_amount: 100,
            loan_amount: 400,
            repay_duration: 30,
            interest: 10,
            fee: 20,
            tx_hex: vec![],
        };

        match BorrowLoan(params) {
            Ok(updated_wallet) => {
                assert_eq!(updated_wallet.balance, 880);
            }
            Err(err) => {
                panic!("BorrowLoan failed: {}", err);
            }
        }
    }
}
