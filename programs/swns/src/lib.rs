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
        // using require_gt, domain length should be strictly greater than 0
        require_gt!(domain.chars().count(), 0, WebServiceError::InvalidDomain);

        web_service_account.authority = ctx.accounts.authority.key();
        web_service_account.receiver = ctx.accounts.receiver.key();
        web_service_account.domain = domain;

        Ok(())
    }

    pub fn update_web_service_account(
        ctx: Context<UpdateWebServiceAccount>,
        domain: String,
    ) -> Result<()> {
        let web_service_account = &mut ctx.accounts.web_service_account;

        // todo: add valid domain checks
        require_gt!(domain.chars().count(), 0);

        // check whether the signer (authority) is the one who created the web_service_account
        // if this check isn't there anyone can sign the tx and update web_service_account
        require_keys_eq!(
            web_service_account.authority.key(),
            ctx.accounts.authority.key(),
            WebServiceError::AuthorityMismatch
        );

        // update the fields, i.e. reciever and domain
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

#[derive(Accounts)]
#[instruction(domain: String)]
pub struct UpdateWebServiceAccount<'info> {
    #[account(
        mut,
        has_one = authority,
        realloc = 8 + 32 + 32 + 4 + domain.len(),
        realloc::payer = authority,
        realloc::zero = false
    )]
    pub web_service_account: Account<'info, WebServiceAccount>,
    /// CHECK: any type
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    // system program is required to realloc space
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WebServiceAccount {
    pub receiver: Pubkey,
    pub authority: Pubkey,
    pub domain: String,
}

#[error_code]
pub enum WebServiceError {
    #[msg("Invalid domain specified.")]
    InvalidDomain,
    #[msg("Unauthorized, authority mismatch")]
    AuthorityMismatch,
}
