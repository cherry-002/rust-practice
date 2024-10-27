use std::io::{self, Write};

fn main() {
    let mut input: String = String::new();

    print!("Enter your local Time and I tell you when to sleep:\nexpected format: 18:35\n");
    io::stdout().flush().unwrap();  

    

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.contains(":") {
        let mut time: Vec<String> = input.trim().split(':').map(|s: &str| s.to_string()).collect();
    
        for t  in &mut time {
            if t.is_empty() {
                eprintln!("wrong format: empty Time")
            } else {
                t.replace_range(2..,"")
            }
            
        }
    
        if time.len() != 2 {
            eprintln!("wrong time format")
        }
        let hour: u8 = time[0].parse().expect("invalid hour format");
        let min: u8 = time[1].parse().expect("invalid format for minutes");
        let sleep_time = format!("spare time before going to sleep: {}:{}", 24 - hour , 60 - min);
        println!("{}", sleep_time);
    } else {
        let time: u8 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("format not supported!");
                return;
            }
        };
        let sleep_time: u8 = 24 - time;
        println!("spare time before going to sleep: {}", sleep_time);
    }

}
