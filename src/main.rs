use panic_calculators::{
    combinatorics::{derangement_string, factorial_string, fibonacci_string, partition_string, permutation_string},
    number_theory::{factor_string, gcd_string, geometric_series_string, is_prime_string, kronecker_symbol_string, lcm_string, modex_string, sum}, statistics::binomial_greater_string,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 && args[1] == "sum" {
        let result = sum(args[2].clone(), args[3].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 4 && args[1] == "gcd" {
        let result = gcd_string(args[2].clone(), args[3].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 4 && args[1] == "lcm" {
        let result = lcm_string(args[2].clone(), args[3].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 4 && args[1] == "kronecker" {
        let result = kronecker_symbol_string(args[2].clone(), args[3].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 5 && args[1] == "modex" {
        println!(
            "{}",
            modex_string(args[2].clone(), args[3].clone(), args[4].clone())
        );
        return;
    }

    if args.len() == 6 && args[1] == "geom_series" {
        println!(
            "{}",
            geometric_series_string(args[2].clone(), args[3].clone(), args[4].clone(), args[5].clone())
        );
        return;
    }

    if args.len() == 3 && args[1] == "is_prime" {
        println!("{}", is_prime_string(args[2].clone()));
        return;
    }

    if args.len() == 3 && args[1] == "factor" {
        println!("{}", factor_string(args[2].clone()));
        return;
    }

    if args.len() == 4 && args[1] == "partition" {
        let result = partition_string(args[2].clone(), args[3].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 4 && args[1] == "factorial" {
        let result = factorial_string(args[2].clone(), args[3].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 4 && args[1] == "derangement" {
        let result = derangement_string(args[2].clone(), args[3].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 5 && args[1] == "permutation" {
        let result = permutation_string(args[2].clone(), args[3].clone(), args[4].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 4 && args[1] == "fibonacci" {
        let result = fibonacci_string(args[2].clone(), args[3].clone());
        println!("{}", result);
        return;
    }

    if args.len() == 5 && args[1] == "binomial_greater" {
        let result = binomial_greater_string(args[2].clone(), args[3].clone(), args[4].clone());
        println!("{}", result);
        return;
    }

    eprintln!("Usage: sum <a> <b>");
    eprintln!("Usage: gcd <a> <b>");
    eprintln!("Usage: lcm <a> <b>");
    eprintln!("Usage: kronecker <a> <b>");
    eprintln!("Usage: modpow <a> <b> <m>");
    eprintln!("Usage: geom_series <a> <b> <m>");
    eprintln!("Usage: is_prime <a>");
    eprintln!("Usage: factor <a>");
    eprintln!("Usage: partition <a> <m>");
    eprintln!("Usage: factorial <a> <m>");
    eprintln!("Usage: derangement <a> <m>");
    eprintln!("Usage: permutation <a> <b> <m>");
    eprintln!("Usage: fibonacci <a> <m>");
    eprintln!("Usage: binomial_greater <n> <k> <p>");
}
