mod instructions;
mod state;

use instructions::*;
use state::*;

fn main() {

    let state = DexState {
        sol_to_usdc: 85,
    };

    println!("--- 1. Перевірка process_instruction ---");
    process_instruction(DexInstruction::Swap {
        from_token: String::from("SOL"),
        to_token: String::from("USDC"),
        amount: 5,
    });
    process_instruction(DexInstruction::AddLiquidity {
        token_pair: String::from("SOL/USDC"),
        amount: 50,
    });
    process_instruction(DexInstruction::Withdraw { amount: 100 });
    process_instruction(DexInstruction::CloseAccount);

    println!("\n--- 2. Перевірка DexState::exchange ---");
    state.exchange(DexInstruction::Swap {
        from_token: String::from("SOL"),
        to_token: String::from("USDC"),
        amount: 10,
    });
    state.exchange(DexInstruction::Withdraw { amount: 20 });
    state.exchange(DexInstruction::CloseAccount); // Перевіримо спрацювання _ => println!("Error")

    println!("\n--- 3. Перевірка apply_bonus ---");
    let bonus_result = apply_bonus(100, Some(String::from("SOLANA10")));
    println!("Сума 100 після apply_bonus: {}", bonus_result);

}