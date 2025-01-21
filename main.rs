use solana_sdk::signature::{Keypair, Signer};
use bs58;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::{
    Arc, 
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::thread;
use std::time::Instant;
use num_format::{Locale, ToFormattedString};
use num_cpus;
fn generate_keypair() -> Keypair {
    
    use rand::rngs::OsRng;
    Keypair::generate(&mut OsRng)
}
/
fn matches_prefix_postfix(
    public_key: &str,
    prefix: &str,
    postfix: &str,
    case_sensitive: bool,
) -> bool {
    if prefix.is_empty() && postfix.is_empty() {
        
        return true;
    }
    let candidate = if case_sensitive {
        public_key.to_string()
    } else {
        public_key.to_lowercase()
    };
    let prefix_check = if case_sensitive {
        prefix.to_string()
    } else {
        prefix.to_lowercase()
    };
    let postfix_check = if case_sensitive {
        postfix.to_string()
    } else {
        postfix.to_lowercase()
    };
    
    if !prefix_check.is_empty() && !candidate.starts_with(&prefix_check) {
        return false;
    }
    
    if !postfix_check.is_empty() && !candidate.ends_with(&postfix_check) {
        return false;
    }
    true
}
fn main() {
    
    println!("\n\x1b[1mHow many matching wallets do you want to generate?\x1b[0m");
    println!("\x1b[3m[0 = infinite]\x1b[0m");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let desired_wallet_count: usize = input.trim().parse().unwrap_or(1);
    
    
    let available_cpus = num_cpus::get();
    println!(
        "\n\x1b[1mYou have {} CPU threads available.\nHow many threads do you want to use?\x1b[0m",
        available_cpus
    );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_threads: usize = input.trim().parse().unwrap_or(1);
    let num_threads = std::cmp::min(num_threads, available_cpus);
    println!(
        "\nUsing {} thread(s) for generation.\n",
        num_threads
    );
    
    println!("\x1b[1mEnter desired prefix (leave blank for none):\x1b[0m");
    let mut prefix = String::new();
    io::stdin().read_line(&mut prefix).expect("Failed to read line");
    let prefix = prefix.trim().to_string();
    println!("\n\x1b[1mEnter desired postfix (leave blank for none):\x1b[0m");
    let mut postfix = String::new();
    io::stdin().read_line(&mut postfix).expect("Failed to read line");
    let postfix = postfix.trim().to_string();
    
    println!("\n\x1b[1mCase sensitive search? [y/n]\x1b[0m");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let case_sensitive = input.trim().eq_ignore_ascii_case("y");
    
    println!(
        "\n\x1b[1m\x1b[42m\x1b[30mSearching for public keys that start with \"{}\" and end with \"{}\"...\x1b[0m\n",
        prefix, postfix
    );
    
    let found_count = Arc::new(AtomicUsize::new(0));
    let found_flag = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(AtomicUsize::new(0));
    
    let milestone_check = 1_000_000;
    
    let start_time = Instant::now();
    
    let mut handles = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let found_count = Arc::clone(&found_count);
        let found_flag = Arc::clone(&found_flag);
        let attempts = Arc::clone(&attempts);
        let prefix_clone = prefix.clone();
        let postfix_clone = postfix.clone();
        let start_time_clone = start_time.clone();
        
        let desired_wallet_count_clone = desired_wallet_count;
        let case_sensitive_clone = case_sensitive;
        let handle = thread::spawn(move || {
            while !found_flag.load(Ordering::Relaxed) {
                
                let keypair = generate_keypair();
                
                let public_key = bs58::encode(keypair.pubkey().to_bytes()).into_string();
                
                let current_attempts = attempts.fetch_add(1, Ordering::Relaxed) + 1;
                
                if matches_prefix_postfix(
                    &public_key,
                    &prefix_clone,
                    &postfix_clone,
                    case_sensitive_clone,
                ) {
                    
                    let total_found_so_far = found_count.fetch_add(1, Ordering::Relaxed) + 1;
                    
                    let elapsed = start_time_clone.elapsed().as_secs_f64();
                    let speed = (current_attempts as f64) / elapsed; 
                    
                    let private_key = bs58::encode(keypair.to_bytes()).into_string();
                    println!(
                        "\x1b[32m\x1b[1mFound\x1b[0m #{}: {} (attempts: ~{}, elapsed: {:.2}s, ~{:.2} wallets/s)",
                        total_found_so_far,
                        public_key,
                        current_attempts.to_formatted_string(&Locale::en),
                        elapsed,
                        speed
                    );
                    
                    let mut file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open("keys.txt")
                        .unwrap();
                    writeln!(
                        file,
                        "Found #{}:\n  Public Key: {}\n  Private Key: {}\n",
                        total_found_so_far, public_key, private_key
                    ).unwrap();
                    
                    if desired_wallet_count_clone != 0 && total_found_so_far >= desired_wallet_count_clone {
                        found_flag.store(true, Ordering::Relaxed);
                        break;
                    }
                }
                
                if current_attempts % milestone_check == 0 {
                    println!(
                        "\x1b[3mChecked \x1b[35m\x1b[1m{}\x1b[0m \x1b[3maddresses...\x1b[0m",
                        current_attempts.to_formatted_string(&Locale::en)
                    );
                }
                
                if found_flag.load(Ordering::Relaxed) {
                    break;
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let total_found = found_count.load(Ordering::Relaxed);
    if desired_wallet_count == 0 {
        
        println!(
            "\n\x1b[32m\x1b[1mStopped.\x1b[0m Total wallets found: {}\n",
            total_found
        );
    } else if total_found >= desired_wallet_count {
        println!(
            "\n\x1b[32m\x1b[1mSuccess!\x1b[0m Found {} matching wallet(s).\n",
            total_found
        );
    } else {
        println!(
            "\n\x1b[31m\x1b[1mStopped.\x1b[0m Only found {} out of {} requested.\n",
            total_found, desired_wallet_count
        );
    }
}
