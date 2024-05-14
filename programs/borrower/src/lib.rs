use anchor_lang::prelude::*;

declare_id!("ABg9Lfz6k9C1G9THHsaxjgPSr6vt4FywcGXzPgVPLvEm");

#[program]
mod borrower {
    use super::*;

    pub fn use_loan(ctx: Context<UseLoan>, amount: u64) -> Result<()> {
        // let borrower = &ctx.accounts.borrower;
        // let lender = &ctx.accounts.lender;

        // Here you would perform operations using the loan amount
        // For simplicity, we just log and immediately return the funds

        msg!("Using loan of amount: {}", amount);
        msg!("Using lender: {:?}", &ctx.accounts.lender);
        msg!("Using borrowew: {:?}", &ctx.accounts.borrower);

        // Ensure the funds are returned
        // invoke(
        //     &transfer(&borrower.key(), &lender.key(), amount),
        //     &[
        //         borrower.to_account_info(),
        //         lender.to_account_info(),
        //     ],
        // )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UseLoan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub lender: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
