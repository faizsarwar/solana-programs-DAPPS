use anchor_lang::prelude::*;

declare_id!("C4hazKmhxLEiGWKcpZvBAE2vd7DQQmjVNmkM3SJZhF9");

#[program]
pub mod gm_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account= &mut ctx.accounts.base_account;
        base_account.gm_count=0;
        Ok(())
    }

    pub fn saymessage(ctx: Context<SayMessage>,message:String) -> Result<()> {
        let base_account= &mut ctx.accounts.base_account;
        let message= message.clone();
        let timestamp = Clock::get().unwrap().unix_timestamp;

        //get the key of account of the user
        let user = *ctx.accounts.user.to_account_info().key;
        let message =Message {
         message,
         user,
         timestamp   
        };
        base_account.gm_list.push(message);
        base_account.gm_count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 1024)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SayMessage<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
}

#[account]
pub struct BaseAccount {
    pub gm_count: u64,
    pub gm_list: Vec<Message>,
}



// define a struct called GmMessage that contains a message, sender, and timestamp
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Message{
    pub message: String,
    pub user : Pubkey,
    pub timestamp: i64
}