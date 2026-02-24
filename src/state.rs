use crate::instructions::*;
use std::collections::HashMap;
pub struct DexState{
    pub sol_to_usdc: u64,
    pub volumes: HashMap<String, u64>,
}

impl DexState {
    pub fn exchange(&mut self, instruction:DexInstruction){
        match instruction {
            DexInstruction::Swap { from_token, to_token, amount} => {
                let received_amount = amount * self.sol_to_usdc;
                let total_volume = self.volumes.entry(to_token.clone()).or_insert(0);
                *total_volume += received_amount;
                println!("🔄 Успіх! Віддано: {} {}, Отримано: {} {}", amount, &from_token, total_volume, &to_token);
            }
            DexInstruction::Withdraw { amount } => {
                println!("💸 Виведено: {}", amount);
            }
            _ => println!("Error"),
        }
    }
}

pub fn apply_bonus(amount: u64, code: Option<&str>) -> u64 {

    if let Some("SOLANA10") = code {
            let percent = amount / 10;
            return amount + percent;
    }

    amount
}