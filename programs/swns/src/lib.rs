use anchor_lang::prelude::*;

declare_id!("webrAiEmdFtimWnVBjbYbsezPiQAThwBmNvtN4agEH2");

#[program]
pub mod swns {
    use super::*;

    pub fn create_web_service_account(
        ctx: Context<CreateWebServiceAccount>,
        domain: String,
    ) -> Result<()> {
        let web_service_account = &mut ctx.accounts.web_service_account;

        // todo: add valid domain checks
        if domain.chars().count() < 1 {
            return err!(Errors::InvalidDomain);
        }

        web_service_account.authority = ctx.accounts.authority.key();
        web_service_account.receiver = ctx.accounts.receiver.key();
        web_service_account.domain = domain;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(domain: String)]
pub struct CreateWebServiceAccount<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 4 + domain.len()
    )]
    pub web_service_account: Account<'info, WebServiceAccount>,
    /// CHECK: any type
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WebServiceAccount {
    pub receiver: Pubkey,
    pub authority: Pubkey,
    pub domain: String,
}

#[error_code]
pub enum Errors {
    #[msg("Invalid domain specified.")]
    InvalidDomain,
}
