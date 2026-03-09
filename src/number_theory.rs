use malachite::base::num::arithmetic::traits::*;
use malachite::base::num::basic::traits::{One, Two, Zero};
use malachite::base::num::conversion::traits::SaturatingFrom;
use malachite::base::num::factorization::traits::Primes;
use malachite::base::num::logic::traits::*;
use malachite::*;
use std::cmp::min;
use std::iter::Product;
use std::ops::*;
use wasm_bindgen::prelude::*;

pub fn modex(mut a: Natural, mut b: Natural, m: Natural) -> Natural {
    if m == 0 {
        return m;
    }
    a.mod_assign(&m);
    let mut r = Natural::from(1u32) % &m;
    let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);
    while b > 0 {
        if b.odd() {
            r.mod_mul_precomputed_assign(&a, &m, &data);
        }
        a.mod_mul_precomputed_assign(a.clone(), &m, &data);
        b >>= 1;
    }
    r
}

pub fn modex_integer(a: Integer, b: Integer, m: Natural) -> Natural {
    if m <= 1 {
        return Natural::ZERO;
    }
    let mut nat_a = if a < 0 {
        &m - Natural::saturating_from(-a) % &m
    } else {
        Natural::saturating_from(a) % &m
    };
    if b < 0 {
        match nat_a.mod_inverse(&m) {
            Some(inv) => nat_a = inv,
            None => return m,
        }
    }
    modex(nat_a, b.unsigned_abs_ref().clone(), m)
}

pub fn geometric_series(a: Natural, r: Natural, n: Natural, m: Natural) -> Natural {
    if m <= 1 || n == 0 {
        return Natural::ZERO;
    }
    if n == 1 {
        return a;
    }
    if n.even() {
        geometric_series(a * (&r + Natural::ONE) % &m, &r * &r % &m, n >> 1, m)
    } else {
        (&a + geometric_series(&a * &r % &m, r, n - Natural::ONE, m.clone())) % m
    }
}

pub fn geometric_series_integer(a: Integer, r: Integer, n: Natural, m: Natural) -> Natural {
    if m <= 1 {
        return Natural::ZERO;
    }
    let a = if a < 0 {&m - Natural::saturating_from(-a) % &m} else {Natural::saturating_from(a) % &m};
    let r = if r < 0 {&m - Natural::saturating_from(-r) % &m} else {Natural::saturating_from(r) % &m};
    geometric_series(a, r, n, m)
}

pub fn is_prime(n: Natural) -> bool {
    if n < 4 {
        return n > 1;
    }
    if n.even() {
        return false;
    }
    let mut q = &n - Natural::ONE;
    let r = q.trailing_zeros().unwrap();
    q >>= r;
    let isqrt = (&n).floor_sqrt();
    for i in 0u32..16u32 {
        let mut a = &isqrt * Natural::from(i) + Natural::TWO;
        a.mod_assign(&n);
        if a.significant_bits() == 0 {
            continue;
        }
        a = modex(a, q.clone(), n.clone());
        if a == 1 {
            continue;
        }
        for _ in 1..r {
            if a == &n - Natural::ONE {
                break;
            }
            a.square_assign();
            a.mod_assign(&n);
        }
        if a != &n - Natural::ONE {
            return false;
        }
    }
    true
}

pub fn kronecker_symbol(n: Integer, k: Integer) -> i8 {
    return n.kronecker_symbol(k);
}

pub fn trial_divide(mut n: Natural, bound: Natural) -> (Vec<(Natural, u32)>, Natural) {
    if n == 0 {
        return (vec![], Natural::ZERO);
    }
    let mut factors = vec![];
    for p in Natural::primes_less_than_or_equal_to(&bound) {
        if (&n).divisible_by(&p) {
            let mut e = 0;
            loop {
                n.div_exact_assign(&p);
                e += 1;
                if !(&n).divisible_by(&p) {
                    break;
                }
            }
            factors.push((p, e));
        }
    }
    (factors, n)
}

pub fn pollard_rho(n: &Natural) -> Natural {
    // Richard Brent's modification of Pollard's rho
    let mut c = 0u32;
    let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&n);
    loop {
        c += 1;
        let f = |x: &Natural| {
            x.mod_mul_precomputed(x.clone(), n, &data)
                .mod_add(Natural::from(c), n)
        };
        let mut x = Natural::ONE;
        let mut y = x.clone();
        let mut r = 1;
        let mut q = Natural::ONE;
        let mut ys = y.clone();
        let m = 4096;
        let mut g = Natural::ONE;
        while r < 1048576 && g == 1 {
            x = y.clone();
            for _ in 0..r {
                y = f(&y)
            }
            ys = y.clone();
            let mut k = 0;
            while k < r && g == Natural::ONE {
                for _ in 0..min(m, r - k) {
                    y = f(&y);
                    q.mul_assign(x.clone().abs_diff(&y));
                    q.mod_assign(n);
                }
                g = Natural::from(q.clone().gcd(n));
                k += m;
            }
            r <<= 1;
        }
        if g == 1 {
            // there seems to be no small factors
            return Natural::ZERO;
        }
        if g == *n {
            g = Natural::ONE;
            while g == 1 {
                ys = f(&ys);
                g = x.clone().abs_diff(&ys).gcd(n);
            }
        }
        if g != *n {
            return g;
        }
    }
}

pub fn smooth(b: u32) -> Natural {
    let mut powers = Vec::new();
    let isqrt = b.floor_sqrt();
    for p in Natural::primes_less_than_or_equal_to(&Natural::from(isqrt)) {
        let mut a = p.clone();
        loop {
            let b = &a * &p;
            if b > b {
                break;
            }
            a = b;
        }
        powers.push(a);
    }
    for p in Natural::primes_less_than_or_equal_to(&Natural::from(b)) {
        if p <= isqrt {
            continue;
        }
        powers.push(p);
    }
    Natural::product(powers.iter())
}

pub fn factor(mut n: Natural) -> (Vec<(Natural, u32)>, Natural) {
    if n == 0 {
        return (vec![], Natural::ZERO);
    }
    // 3 digit factors
    let trial_division_result = trial_divide(n, Natural::from(1000u32));
    let mut ret = trial_division_result.0;
    n = trial_division_result.1;
    let mut to_factor = if n == 1 { vec![] } else { vec![n] };
    let mut given_up = vec![];
    while !to_factor.is_empty() {
        let n = to_factor.pop().unwrap();
        if is_prime(n.clone()) {
            ret.push((n, 1));
            continue;
        }
        // 12 digit factors
        let g = pollard_rho(&n);
        if g != 0 {
            to_factor.push(n.clone().div_exact(&g));
            to_factor.push(g);
            continue;
        }
        // give up
        given_up.push(n);
        // for i in 0..100 {
        //     let mut px = Integer::from(0);
        //     let mut pz = Integer::from(1);
        //     let mut qx = Integer::from(i * 123456 ^ 789);
        //     let mut qz = Integer::from(1);
        //     let mut a24 = Integer::from(i * 456789 ^ 123);
        // }
    }
    ret.sort();
    (ret, Natural::product(given_up.iter()))
}

#[wasm_bindgen]
pub fn sum(a: String, b: String) -> String {
    let a: Integer = a.parse().unwrap();
    let b: Integer = b.parse().unwrap();
    (a + b).to_string()
}

#[wasm_bindgen]
pub fn gcd_string(a: String, b: String) -> String {
    let a: Natural = a.parse().unwrap();
    let b: Natural = b.parse().unwrap();
    a.gcd(b).to_string()
}

#[wasm_bindgen]
pub fn lcm_string(a: String, b: String) -> String {
    let a: Natural = a.parse().unwrap();
    let b: Natural = b.parse().unwrap();
    a.lcm(b).to_string()
}

#[wasm_bindgen]
pub fn kronecker_symbol_string(a: String, b: String) -> String {
    kronecker_symbol(a.parse().unwrap(), b.parse().unwrap()).to_string()
}

#[wasm_bindgen]
pub fn modex_string(a: String, b: String, m: String) -> String {
    modex_integer(a.parse().unwrap(), b.parse().unwrap(), m.parse().unwrap()).to_string()
}

#[wasm_bindgen]
pub fn geometric_series_string(a: String, r: String, n: String, m: String) -> String {
    geometric_series_integer(a.parse().unwrap(), r.parse().unwrap(), n.parse().unwrap(), m.parse().unwrap()).to_string()
}

#[wasm_bindgen]
pub fn is_prime_string(n: String) -> String {
    is_prime(n.parse().unwrap()).to_string()
}

#[wasm_bindgen]
pub fn factor_string(n: String) -> String {
    let factors = factor(n.parse().unwrap());
    let mut ret = "".to_string();
    for (p, e) in factors.0 {
        if !ret.is_empty() {
            ret += " x ";
        }
        if e == 1 {
            ret += &p.to_string();
        } else {
            ret += &p.to_string();
            ret += "^";
            ret += &e.to_string();
        }
    }
    if factors.1 != 1 {
        if !ret.is_empty() {
            ret += " x ";
        }
        ret += &factors.1.to_string();
        ret += " (failed to factor)"
    }
    ret
    // for some reason, an implementation using map and collect didn't produce good wasm code.
    // keeping this in case it fixes itself
    // if factors.1 == 1 {
    //     factors.0
    //         .iter()
    //         .map(|x| if x.1 == 1 {x.0.to_string()} else {x.0.to_string() + "^" + &x.1.to_string()} + " x ")
    //         .collect::<String>()
    //         .trim_end_matches(" x ")
    //         .to_owned()
    // } else {
    //     factors.0
    //         .iter()
    //         .map(|x| if x.1 == 1 {x.0.to_string()} else {x.0.to_string() + "^" + &x.1.to_string()} + " x ")
    //         .collect::<String>()
    //         .to_owned() + &factors.1.to_string() + "(failed to factor)"
    // }
}
