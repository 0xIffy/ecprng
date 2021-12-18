use anchor_lang::prelude::*;

pub mod helper;

use helper::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ecprng {
  use super::*;
  pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;

    base_account.curve = Curve::default();
    base_account.num = 0;

    Ok(())
  }

  pub fn set_curve(
    ctx: Context<SetCurve>, 
    a: u64, b: u64,
    prime: u64,
    x: u64, y: u64,
  ) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;

    base_account.curve = Curve::new(a, b, prime, Point { x, y });

    Ok(())
  }

  pub fn get_num(ctx: Context<GetNum>, max: u64) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;

    let point: Point = base_account.curve.add().unwrap();

    let num = point.x + point.y;

    base_account.num = num % max;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 64 + 64 + 64)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetCurve<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct GetNum<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}


#[account]
pub struct BaseAccount {
  curve: Curve,
  num: u64,
}



#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Point {
  pub x: u64,
  pub y: u64
}


/*
  Struct for the elliptic curve (ec) representing the equation y^2 = x^3 + ax + b
  a and b are coefficients in the equation
  max is a PRIME modulus for the field, all vaules are (mod max)
  start and curr represent the state of the curve/rng
  At all times curr = n * start for some int n 
*/
#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Curve {
  pub a: u64,
  pub b: u64,
  pub max: u64,
  pub start: Option<Point>,
  pub curr: Option<Point>
}

impl Curve {
  fn default() -> Curve {
    Curve { a: 2, b: 3, max: 2u64.pow(20) - 3, start: Some(Point { x: 12407, y: 12189 }), curr: None }
  }


  fn new(a: u64, b: u64, max: u64, start: Point) -> Curve {
    Curve { a, b, max, start: Some(start), curr: None }
  }

  /*
    Gets the y value of the point given the x value according to the equation y^2 = x^3 + ax + b
    Only returns successful if the y value is an integer
  */
  // fn at(&self, x: u64) -> Result<u64, &'static str> {
  //   let val = (x.pow(3) + (self.a * x) + self.b) % self.max;

  //   if jacobi_symbol::<u64>(val, self.max) == 1 {
  //     let y = helper::sqrt_mod_p(val, &self.max);
  //     // println!("{:?}", y);

  //     Ok(y.unwrap())
  //   } else {
  //     Err("No integer y value.")
  //   }
  // }

  /*
    Adds start pnt and curr pnt and sets the new curr pnt
    Refer to the equations file to see how the new curr pnt is calculated
  */
  fn add(&mut self) -> Result<Point, Point> {
    let p1 = self.start.clone().unwrap();
    let p2 = match self.curr.clone() {
      None => p1.clone(),
      Some(p) => p
    };

    let denom1 = (2 * p1.y) % self.max;
    let denom2 = (self.max + p2.x - p1.x) % self.max;

    // println!("{:?}", helper::invrs_mod_p(denom1, self.max));
    // println!("{:?}", helper::xgcd(denom1, self.max));

    let d = match denom2 {
      0 => ((3 * (p1.x.pow(2) % self.max) + self.a) * helper::xgcd(denom1, self.max)) % self.max,
      _ => ((self.max + p2.y - p1.y) * helper::xgcd(denom2, self.max)) % self.max
    };

    let x = (2 * self.max + d.pow(2) - p1.x - p2.x) % self.max;
    let y = (self.max + (d * ((self.max + p1.x - x) % self.max)) - p1.y) % self.max;


    self.curr = Some(Point { x, y });

    Ok(self.curr.clone().unwrap())
  }

  fn mult(&mut self, n: u64) -> Point {
    for _i in 0..n {
      self.add();
    }

    self.curr.clone().unwrap()
  }
}