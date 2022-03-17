use anchor_lang::prelude::*;

declare_id!("AoKm5PqnPdzrEZPcgEWvBnbaAN8G9DMCULcWTGRGqPFw");


// keypair created with cli : 8c87kAXKRCopVJwtK1V8v2cpsZ3jPCc7XzpHFiyCAGbr
// seed
// Save this seed phrase and your BIP39 passphrase to recover your new keypair:
// slender cruel celery slender forum response reveal track uphold observe audit fringe


// deployed id : 4ET9e6ng5U3c9qchjNZoryo8HNQj2UEo6vxLbgpES264

#[program]
mod mysolanaapp {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count -= 1;
        Ok(())
    }
}

// Transaction instructions
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}