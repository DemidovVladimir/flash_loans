use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction, program_error::ProgramError};
use borrower::{self, cpi::accounts::UseLoan};

declare_id!("7dr3UkepCLptxHhpkVpmJr1Xt3habA5Mov8nmfiDQ67z");

#[program]
mod flash_loans {
    use super::*;

    pub fn execute_flash_loan(ctx: Context<ExecuteFlashLoan>, amount: u64) -> Result<()> {
        let lender = &ctx.accounts.lender;
        let borrower = &ctx.accounts.borrower;

        // Transfer funds from lender to borrower
        invoke(
            &system_instruction::transfer(&lender.key(), &borrower.key(), amount),
            &[
                lender.to_account_info(),
                borrower.to_account_info(),
            ],
        )?;

        // Call the borrower program
        let cpi_accounts = UseLoan {
            borrower: borrower.to_account_info(),
            lender: lender.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_program = ctx.accounts.borrower_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        borrower::cpi::use_loan(cpi_ctx, amount)?;

        // Ensure funds are returned to lender
        if borrower.to_account_info().lamports() < amount {
            return Err(ProgramError::InsufficientFunds.into());
        }

        invoke(
            &system_instruction::transfer(&borrower.key(), &lender.key(), amount),
            &[
                borrower.to_account_info(),
                lender.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteFlashLoan<'info> {
    #[account(mut)]
    pub lender: Signer<'info>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub borrower_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
