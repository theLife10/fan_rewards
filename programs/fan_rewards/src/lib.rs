use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod fan_rewards {
    use super::*;

    // Function to mint loyalty tokens
    pub fn mint_loyalty_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn redeem_rewards(ctx: Context<RedeemRewards>, amount: u64) -> Result<()> {
        let cpi_accounts = token::Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Perform the burn operation
        token::burn(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn get_loyalty_tier(ctx: Context<LoyaltyTier>) -> Result<String> {
        let balance = ctx.accounts.user_account.amount; // Token balance
        let tier = if balance <= 100 {
            "Bronze".to_string()
        } else if balance <= 500 {
            "Silver".to_string()
        } else {
            "Gold".to_string()
        };
        msg!("User Loyalty Tier: {}", tier);
        Ok(tier)
    }

    pub fn mint_achievement_nft(ctx: Context<MintAchievement>, metadata_uri: String) -> Result<()> {
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.nft_mint.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, 1)?; // Mint 1 NFT
        msg!("Minted Achievement NFT with metadata URI: {}", metadata_uri);
        Ok(())
    }




    pub fn redeem_perk(ctx: Context<RedeemPerk>, nft_id: String) -> Result<()> {
        let user_wallet = &ctx.accounts.user.key;
        let nft_owner = ctx.accounts.nft_account.owner;

        require_keys_eq!(**user_wallet, nft_owner, CustomError::Unauthorized);
        msg!("Perk redeemed with NFT ID: {}", nft_id);
        Ok(())
    }
}


// Contexts for loyalty token minting
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    #[account(signer)]
    pub mint_authority: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RedeemRewards<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct LoyaltyTier<'info> {
    pub user_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct MintAchievement<'info> {
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    #[account(signer)]
    pub mint_authority: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RedeemPerk<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>, // Validate user's NFT account
}

// Contexts for NFT access verification
#[derive(Accounts)]
pub struct VerifyAccess<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>,
}

// Custom error definitions
#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access. User does not own the required NFT.")]
    Unauthorized,
}