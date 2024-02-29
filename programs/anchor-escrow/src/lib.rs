use anchor_lang::prelude::*;

declare_id!("5R7pWP5Ho263MdpcW92Md3EVSpH99gLcPZ9Ereod1mAu");

pub mod state;
pub mod contexts;

pub use contexts::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
