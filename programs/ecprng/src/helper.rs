use anchor_lang::prelude::*;

pub fn xgcd(a: u64, b: u64) -> u64 {
  let mut old_r = a;
  let mut r = b;
  let mut old_s: i64 = 1;
  let mut s: i64 = 0;

  let mut q;
  let mut prev: i64;

  while r != 0 {
    q = old_r / r;

    prev = r as i64;
    r = old_r - q * prev as u64;
    old_r = prev as u64;

    prev = s;
    s = old_s - (q as i64 * prev) as i64;
    old_s = prev;
  }

  match old_s > 0 {
    true => old_s as u64,
    false => (b as i64 + old_s) as u64 
  }
}