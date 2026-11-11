pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5yn2GC9iw2KmyEbk6ACZemUKu51MHrfpFHrKufi9NYs3");

#[program]
pub mod escrow {
    use super::*;

     #[instruction(discriminator = 0)]
    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit: u64,
        receive: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts
            .init_escrow(seed, receive, &ctx.bumps, expiration)?;
        ctx.accounts.deposit(deposit)
    }

    #[instruction(discriminator = 3)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.take()
    }
}
