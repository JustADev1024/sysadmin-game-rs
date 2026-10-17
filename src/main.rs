use std::io; // Импортируем модуль ввода/вывода

fn main() {
    let mut input = String::new();
    
    println!("Hello! This is simple text game about sysadmins.");
    println!("Now, I ask you one question.");
    println!("Your servers are in kernel panic. Clients are flooding support chats: \"Service isn't working!\"");
    println!("Your select:");
    println!("1. Fixing servers all the night.");
    println!("2. Go sleep.");
    println!("3. Do nothing.");

    // Добавили обработку ошибки
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "1" => {
            println!("Your heart isn't stopped. It is just executed `systemctl suspend`. Goodbye my dear gamer.")
        },
        "2" => {
            println!("On morning, with fresh mind, you fixed all the servers in five minutes. Success!")
        },
        "3" => {
            println!("Your PC is exploded by user's messages.")
        },
        _ => println!("I don't know this command.") // Изменил print! на println!
    }
}