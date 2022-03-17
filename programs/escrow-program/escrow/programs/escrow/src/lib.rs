use anchor_lang::prelude::*;

declare_id!("6hFX1XNJQ9M9QHbTo8959QAt9NywQ3GpDcZagYGXb5mC");


#[program]
pub mod escrow {
    use super::*;


    pub fn collect(ctx: Context<Accept>,amount_b: f64) -> Result<()>{
        // recieve money from reciever
        let escrow = &mut ctx.accounts.escrow;
        let buyer = *ctx.accounts.initializer.to_account_info().key;

        // check caller is the buyer ?
        require!(escrow.buyer == *ctx.accounts.initializer.to_account_info().key, MyError::SomeError);
        // check the amount given was correct
        require!(amount_b == escrow.transaction_amount_b, MyError::SomeError); 
        
        // send saved money to receiver
        anchor_lang::solana_program::system_instruction::transfer(
            &escrow.to_account_info().key, 
            &escrow.buyer,
            escrow.transaction_amount_a as u64,  //amount send from  escrow to  reciever
        );

        // send recivers money to sender 
        anchor_lang::solana_program::system_instruction::transfer(
            &buyer, 
            &escrow.seller,
            escrow.transaction_amount_a as u64,  //amount send from  reciever to sender
        );

        Ok(())
    }

    //escrow program instruction
    pub fn initialize(ctx: Context<Initialize>,amount_a: f64, amount_b: f64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.seller = *ctx.accounts.initializer.to_account_info().key; 
        escrow.buyer = *ctx.accounts.buyer.to_account_info().key; 
        escrow.transaction_amount_a =amount_a;  // amount the sender have sent
        escrow.transaction_amount_b = amount_b;  // amount the sender wants to recieve

        // doing transfer
        anchor_lang::solana_program::system_instruction::transfer(
            &escrow.seller, 
            &escrow.to_account_info().key,
            amount_a as u64,  //amount recieved from sender to escrow
        );
        Ok(())
    }
}


// account signature that we need in to call the instrutcion 
#[derive(Accounts)]
pub struct Initialize<'info> {
    pub initializer: Signer<'info>,        //seller
    pub token_account: Account<'info,TokenAccount>,   //seller token account

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer: UncheckedAccount<'info>,    //buyer
    pub buyer_token_account: Account<'info,TokenAccount>,  // buyer token account    
    #[account(init, payer = user, space = 64 + 1024)]
    pub escrow: Account<'info,Escrow>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    // pub token_program: Program<'info,Token>
}



#[derive(Accounts)]
pub struct Accept<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,    
    pub escrow: Account<'info,Escrow>,
}


#[account]
pub struct Escrow { //saving transaction info of seller and buyer
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub transaction_amount_a: f64,
    pub transaction_amount_b: f64
}

#[account]
pub struct TokenAccount {
    pub pubkey : Pubkey
}

#[error_code]
pub enum MyError {
    SomeError
}