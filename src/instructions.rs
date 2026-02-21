pub enum DexInstruction{
    Swap{from_token:String, to_token:String, amount:u64},
    AddLiquidity{token_pair:String, amount:u64},
    Withdraw{amount:u64},
    CloseAccount,
}

pub fn process_instruction(instruction:DexInstruction){
    match instruction{
        DexInstruction::Swap { from_token, to_token, amount} => println!("🔄 Обмін {} {} на {}", amount, from_token, to_token),
        DexInstruction::AddLiquidity {amount, token_pair} => println!("💰 Додавання ліквідності: {} у пару {}", amount, token_pair),
        DexInstruction::Withdraw {amount} => println!("💸 Виведення коштів: {} lamports", amount),
        DexInstruction::CloseAccount => println!("🚫 Закриття торгового акаунта"),
    }
}