
enum DexInstruction{
    Swap{from_token:String, to_token:String, amount:u64},
    AddLiquidity{token_pair:String, amount:u64},
    Withdraw{amount:u64},
    CloseAccount,
}

struct DexState{
    sol_to_usdc: u64,
}

impl DexState {
    fn exchange(&self, instruction:DexInstruction){
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

fn process_instruction(instruction:DexInstruction){
    match instruction{
        DexInstruction::Swap { from_token, to_token, amount} => println!("🔄 Обмін {} {} на {}", amount, from_token, to_token),
        DexInstruction::AddLiquidity {amount, token_pair} => println!("💰 Додавання ліквідності: {} у пару {}", amount, token_pair),
        DexInstruction::Withdraw {amount} => println!("💸 Виведення коштів: {} lamports", amount),
        DexInstruction::CloseAccount => println!("🚫 Закриття торгового акаунта"),
    }
}

fn apply_bonus(amount: u64, code: Option<String>) -> u64 {

    if let Some(c) = code {

        if c == "SOLANA10" {
            let percent = amount / 10;
            return amount + percent;
        }
    }


    amount
}

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