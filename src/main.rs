mod instructions;
mod state;

use instructions::*;
use state::*;
use std::collections::HashMap;

fn main() {

    let mut history: Vec<&str> = Vec::new();

    let mut state = DexState {
        sol_to_usdc: 85,
        volumes: HashMap::new(),
    };

    println!("--- 1. Перевірка process_instruction ---");
    process_instruction(DexInstruction::Swap {
        from_token: String::from("SOL"),
        to_token: String::from("USDC"),
        amount: 5,
    });
    history.push("Swap");
    process_instruction(DexInstruction::AddLiquidity {
        token_pair: String::from("SOL/USDC"),
        amount: 50,
    });
    history.push("AddLiquidity");
    process_instruction(DexInstruction::Withdraw { amount: 100 });
    history.push("Withdraw");
    process_instruction(DexInstruction::CloseAccount);
    history.push("CloseAccount");

    println!("\n--- 2. Перевірка DexState::exchange ---");
    state.exchange(DexInstruction::Swap {
        from_token: String::from("SOL"),
        to_token: String::from("USDC"),
        amount: 10,
    });
    history.push("Swap");
    state.exchange(DexInstruction::Withdraw { amount: 20 });
    history.push("Withdraw");
    state.exchange(DexInstruction::CloseAccount); // Перевіримо спрацювання _ => println!("Error")
    history.push("CloseAccount");

    println!("\n--- 3. Перевірка apply_bonus ---");
    let bonus_result = apply_bonus(100, Some("SOLANA10"));
    println!("Сума 100 після apply_bonus: {}", bonus_result);
    history.push("apply bonus");
    println!("--- Історія операцій ---");
    for entry in &history {
        println!("Запис: {}", entry);
    }
}