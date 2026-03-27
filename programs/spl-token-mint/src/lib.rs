use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod spl_token_mint {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mint_authority: Pubkey, metadata: String) -> ProgramResult {
        // Initialize the mint and configure authority
        let mint = &mut ctx.accounts.mint;
        mint.init(&ctx.accounts.token_program, &mint_authority, None, &metadata)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub mint: Account<'info, Mint>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

// Metadata structure
#[account]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint_authority: Pubkey,
}

impl Metadata {
    pub fn new(name: String, symbol: String, uri: String, mint_authority: Pubkey) -> Self {
        Self {
            name,
            symbol,
            uri,
            mint_authority,
        }
    }
}