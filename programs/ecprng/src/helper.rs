use anchor_lang::prelude::*;


pub fn sqrt_mod_p(n: u64, p: u64) -> Option<u64> {
  let mod_n = n % p;

  for i in 2..p {
    if i.pow(2) % p == mod_n {
      return Some(i); 
    }
  }

  None
}

pub fn invrs_mod_p(n: u64, p: u64) -> u64 {
  let mod_n = n % p;

  for x in 1..p {
    if (mod_n * x) % p == 1 {
      return x;
    }
  }

  0
}