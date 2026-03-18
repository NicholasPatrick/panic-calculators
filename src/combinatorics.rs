use std::mem::swap;

use malachite::{
    Natural,
    base::num::{
        arithmetic::traits::{
            ModAddAssign, ModMulPrecomputed, ModMulPrecomputedAssign, ModSubAssign, Square,
        },
        basic::traits::{One, Zero}, logic::traits::BitConvertible,
    },
};
use wasm_bindgen::prelude::*;

pub fn partition(a: u32, m: Natural) -> Natural {
    if m <= 1 {
        return Natural::ZERO;
    }
    if a <= 1 {
        return Natural::ONE;
    }
    let mut part_table = vec![];
    part_table.resize(a as usize + 1, Natural::ZERO);
    part_table[0] = Natural::ONE;
    part_table[1] = Natural::ONE;
    // naive implementation of the pentagonal number theorem
    // O(a sqrt(a))
    // there exists O(a log a) algorithms, but that is a lot more involved
    for i in 2..=a as usize {
        let mut j = 1usize;
        let mut k = 1usize;
        while k <= i {
            let pt = part_table[i - k].clone();
            if j % 4 % 3 == 0 {
                part_table[i] += &m - pt;
            } else {
                part_table[i] += pt;
            }
            part_table[i] %= &m;
            j += 1;
            k += if j % 2 == 0 { j / 2 } else { j };
        }
    }
    part_table[a as usize].clone()
}

pub fn factorial(a: u32, m: Natural) -> Natural {
    if m <= 1 {
        return Natural::ZERO;
    }
    if a <= 1 {
        return Natural::ONE;
    }
    let mut ans = Natural::ONE;
    let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);
    for i in 2..=a {
        ans.mod_mul_precomputed_assign(Natural::from(i), &m, &data);
    }
    ans
}

pub fn derangement(a: u32, m: Natural) -> Natural {
    if m <= 1 {
        return Natural::ZERO;
    }
    let mut ans = Natural::ONE;
    let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);
    for i in 1..=a {
        ans.mod_mul_precomputed_assign(Natural::from(i), &m, &data);
        if i % 2 == 0 {
            ans.mod_add_assign(Natural::ONE, &m);
        } else {
            ans.mod_sub_assign(Natural::ONE, &m);
        }
    }
    ans
}

pub fn permutation(mut a: Natural, b: u32, m: Natural) -> Natural {
    if m <= 1 {
        return Natural::ZERO;
    }
    a %= &m;
    if a < b {
        return Natural::ZERO;
    }
    // m > a >= b
    let mut ans = Natural::ONE;
    let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);
    for i in 0..b {
        ans.mod_mul_precomputed_assign(&a - Natural::from(i), &m, &data);
    }
    ans
}

pub fn fibonacci(a: Natural, m: Natural) -> Natural {
    if m <= 1 {
        return Natural::ZERO;
    }
    if a <= 1 {
        return a;
    }
    let mut x = Natural::ONE;
    let mut y = Natural::ONE;
    for i in (a-Natural::ONE).to_bits_desc().iter().skip(1) {
        let diff = (&x << 1) + &m - &y;
        x = (&x).square() + (&y).square();
        y *= diff;
        x %= &m;
        y %= &m;
        if *i {
            y += &x;
            y %= &m;
            swap(&mut x, &mut y);
        }
    }
    x
}

#[wasm_bindgen]
pub fn partition_string(a: String, m: String) -> String {
    partition(a.parse().unwrap(), m.parse().unwrap()).to_string()
}

#[wasm_bindgen]
pub fn factorial_string(a: String, m: String) -> String {
    factorial(a.parse().unwrap(), m.parse().unwrap()).to_string()
}

#[wasm_bindgen]
pub fn derangement_string(a: String, m: String) -> String {
    derangement(a.parse().unwrap(), m.parse().unwrap()).to_string()
}

#[wasm_bindgen]
pub fn permutation_string(a: String, b: String, m: String) -> String {
    permutation(a.parse().unwrap(), b.parse().unwrap(), m.parse().unwrap()).to_string()
}

#[wasm_bindgen]
pub fn fibonacci_string(a: String, m: String) -> String {
    fibonacci(a.parse().unwrap(), m.parse().unwrap()).to_string()
}
