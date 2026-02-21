use crate::instructions::*;
pub struct DexState{
    pub sol_to_usdc: u64,
}

impl DexState {
    pub fn exchange(&self, instruction:DexInstruction){
        match instruction {
            DexInstruction::Swap { from_token, to_token, amount} => {
                let recieved = amount * &self.sol_to_usdc;
                println!("🔄 Успіх! Віддано: {} {}, Отримано: {} {}", amount, from_token, recieved, to_token);
            }
            DexInstruction::Withdraw { amount } => {
                println!("💸 Виведено: {}", amount);
            }
            _ => println!("Error"),
        }
    }
}

pub fn apply_bonus(amount: u64, code: Option<String>) -> u64 {

    if let Some(c) = code {

        if c == "SOLANA10" {
            let percent = amount / 10;
            return amount + percent;
        }
    }

    amount
}