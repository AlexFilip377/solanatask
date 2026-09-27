use std::env;


#[derive(Debug)]
enum Instruction {
    Balance(String),
    Transfer(String, u64),
    Unknown,
}


#[derive(Debug)]
struct Transaction {
    instruction: Instruction,
    timestamp: u64,
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let command = if args.len() > 1 {
        args[1].as_str()
    } else {
        ""
    };

    let instruction = match command {
        "balance" => {
            let account = args.get(2).unwrap_or(&String::from("default_account")).clone();
            Instruction::Balance(account)
        }
        "transfer" => {
            let target = args.get(2).unwrap_or(&String::from("unknown_target")).clone();
            let amount: u64 = args.get(3).unwrap_or(&String::from("0")).parse().unwrap_or(0);
            Instruction::Transfer(target, amount)
        }
        _ => Instruction::Unknown,
    };

    let tx = Transaction {
        instruction,
        timestamp: 1690000000,
    };

    println!("Обработка транзакции...");
    match tx.instruction {
        Instruction::Balance(acc) => println!("Запрос баланса для аккаунта: {}", acc),
        Instruction::Transfer(target, amount) => {
            if amount > 0 {
                println!("Успешный перевод {} SOL на адрес {}", amount, target);
            } else {
                println!("Ошибка: сумма перевода должна быть больше 0");
            }
        }
        Instruction::Unknown => {
            println!("Доступные команды:");
            println!("  cargo run -- balance <адрес>");
            println!("  cargo run -- transfer <адрес> <сумма>");
        }
    }
}