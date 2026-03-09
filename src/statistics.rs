use core::f64;
use std::f64::consts::{E, TAU};
use libm::{erfc, sqrt};
use wasm_bindgen::prelude::*;


pub fn stirling(n: u32) -> f64 {
    if n <= 1 {
        return 1.;
    }
    return (n as f64 / E).powf(n as f64) * (TAU * n as f64).sqrt() * (1. / 12. / n as f64).exp();
}

// straight from ChatGPT
fn betacf(a: f64, b: f64, x: f64) -> f64 {
    const MAXIT: usize = 200;
    const EPS: f64 = 3.0e-14;
    const FPMIN: f64 = 1e-300;

    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;

    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < FPMIN {
        d = FPMIN;
    }
    d = 1.0 / d;

    let mut h = d;

    for m in 1..=MAXIT {
        let m2 = 2 * m;

        let mut aa =
            (m as f64) * (b - m as f64) * x /
            ((qam + m2 as f64) * (a + m2 as f64));

        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        h *= d * c;

        aa = -(a + m as f64) * (qab + m as f64) * x /
            ((a + m2 as f64) * (qap + m2 as f64));

        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;

        let del = d * c;
        h *= del;

        if (del - 1.0).abs() < EPS {
            break;
        }
    }

    h
}

fn betai(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }

    let bt = ((libm::lgamma(a + b)
        - libm::lgamma(a)
        - libm::lgamma(b))
        + a * x.ln()
        + b * (1.0 - x).ln())
        .exp();

    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

fn normal_tail(z: f64) -> f64 {
    0.5 * erfc(z / std::f64::consts::SQRT_2)
}

pub fn binomial_greater(n: u32, k: u32, p: f64) -> f64 {
    if k == 0 {
        return 1.0;
    }
    if k > n {
        return 0.0;
    }

    let n = n as f64;
    let k = k as f64;

    let mean = n * p;
    let var = n * p * (1.0 - p);

    // Use normal approximation when variance is large
    if var > 50.0 {
        let z = (k - 0.5 - mean) / sqrt(var);
        return normal_tail(z);
    }

    // fallback to incomplete beta for small n
    betai(k, n - k + 1.0, p)
}

#[wasm_bindgen]
pub fn binomial_greater_string(n: String, k: String, p: String) -> String {
    binomial_greater(n.parse().unwrap(), k.parse().unwrap(), p.parse().unwrap()).to_string()
}
