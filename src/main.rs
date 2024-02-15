use std::{time::Instant, u32};
fn eratos(limit: u32) -> Vec<u32> {
    let mut primes = vec![true; (limit + 1) as usize];
    primes[0] = false;
    primes[1] = false;

    let mut p = 2;
    while p * p <= limit {
        if primes[p as usize] {
            let mut i = p * p;
            while i <= limit {
                primes[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }

    let result: Vec<u32> = primes
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(num, _)| num as u32)
        .collect();

    result
}

fn pritchard(n: u32) -> Vec<u32> {
    // init variables
    let mut wheel: Vec<u32> = vec![1];
    let mut p = 3;
    let mut k = 1;

    while p * p <= n {
        if wheel.len() < n as usize {
            // extend the wheel and the length to a minimum of p * length and n
        }
        // delete multis
        // put p into primes
        k += 1;
        // set p to next p tf?
        while wheel.len() < n as usize {
            if wheel[k] + (wheel.len() as u32) > n {
                break;
            } else {
                wheel.push(wheel[k] + (wheel.len() as u32));
                k += 1;
            }
        }
    }
    wheel
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let num_total: u32 = 10000000; /* u32 = args[1].parse().unwrap(); */

    println!("eratos:");
    let start_time = Instant::now();
    let primes = eratos(num_total);
    // for prime in &primes {
    //     println!("{}", prime)
    // }
    let end_time = start_time.elapsed();
    println!("Time taken: {:?}", end_time);
    println!("Number of primes: {}", primes.len());

    println!("pritchard:");
    let start_time = Instant::now();
    let primes = pritchard(num_total);
    let end_time = start_time.elapsed();
    println!("Time taken: {:?}", end_time);
    println!("Number of primes: {}", primes.len());
}
