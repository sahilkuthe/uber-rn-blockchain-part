use anchor_lang::prelude::*;

declare_id!("7CQXHffkfxxQ2nkBcWPL8MGXWeab7byMdYBa5Qv1rzsg");

#[program]
pub mod solana_ride_hailing {
    use super::*;

    pub fn register_user(ctx: Context<RegisterUser>, role: String, ipfs_cid: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.role = role;
        user_account.ipfs_cid = ipfs_cid;
        Ok(())
    }

    pub fn create_ride(ctx: Context<CreateRide>, ipfs_cid: String, amount: u64) -> Result<()> {
        let ride = &mut ctx.accounts.ride;
        ride.rider = ctx.accounts.user.key();
        ride.ipfs_cid = ipfs_cid;
        ride.amount = amount;
        Ok(())
    }
}

#[account]
pub struct UserAccount {
    pub role: String,
    pub ipfs_cid: String,
}

#[account]
pub struct Ride {
    pub rider: Pubkey,
    pub ipfs_cid: String,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(init, payer = user, space = 64)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateRide<'info> {
    #[account(init, payer = user, space = 64)]
    pub ride: Account<'info, Ride>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

