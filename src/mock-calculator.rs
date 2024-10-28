// use spinners::{Spinner, Spinners, Stream};
// use std::io;
// use std::thread::sleep;
// use std::time::{Duration, Instant};
// use rand::prelude::*;


// fn main() {
//     println!("Cli calculater using Rust. Welcome!\n");
//     println!("Write any operation you want:");

//     let mut input = String::new();

//     io::stdin()
//         .read_line(&mut input)
//         .expect("something gone wrong");
//     let mut sp = Spinner::with_timer_and_stream(Spinners::Dots, "Calculating".into(), Stream::Stderr);

//     let max_wait = Duration::new(rand::thread_rng().gen_range(1..=4), 0);
//     let start = Instant::now();

//     while Instant::now() - start < max_wait {
//         sleep(Duration::from_secs(1));
//     }



//     sp.stop_and_persist(
//         "\x1b[32m🗸\x1b[0m",
//         "Operation Successful\nResult: Hello World!".into(),
//     );

//     println!("\n\nPress Enter to exit...");
//     let _ = io::stdin().read_line(&mut String::new()).unwrap();

// }
