use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::transfer as spl_transfer;
use anchor_spl::token::Transfer as SplTransfer;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod wba_vault {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.state_bump = *ctx.bumps.get("state").unwrap();
        ctx.accounts.state.auth_bump = *ctx.bumps.get("auth").unwrap();
        ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap();
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let ctx_accounts = Transfer {
            from : ctx.accounts.owner.to_account_info(),
            to : ctx.accounts.vault.to_account_info(),
        };
        let cpi = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            ctx_accounts
        );
        transfer(cpi, amount)
    }
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let ctx_accounts = Transfer {
            to : ctx.accounts.owner.to_account_info(),
            from : ctx.accounts.vault.to_account_info(),
        };
        let seeds : &[&[u8]; 3] = &[
            b"vault",
            ctx.accounts.owner.to_account_info().key.as_ref(),
            &[ctx.accounts.state.vault_bump],
        ];
        let pda_signer : &[&[&[u8]];1] = &[&seeds[..]];
        let cpi = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(), 
            ctx_accounts,
            pda_signer,
        );
        transfer(cpi, amount)
    }

    pub fn spl_deposit(ctx: Context<SplDeposit>, amount: u64) -> Result<()> {
        let ctx_accounts = SplTransfer {
            from : ctx.accounts.owner_ata.to_account_info(),
            to : ctx.accounts.vault.to_account_info(),
            authority:ctx.accounts.owner.to_account_info(),
        };
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            ctx_accounts
        );
        spl_transfer(cpi, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = VaultState::LEN,
        seeds = [b"state", owner.key().as_ref()],
        bump,
    )]
    state : Account<'info, VaultState>,
    #[account(
        seeds = [b"auth", state.key().as_ref()],
        bump
    )]
    // Check: This is safe
    auth : UncheckedAccount<'info>,
    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    vault : SystemAccount<'info>,
    system_program : Program<'info,System>,

}

#[derive(Accounts)]
pub struct Payment <'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump,
    )]
    state : Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    vault : SystemAccount<'info>,
    system_program : Program<'info,System>,

}

#[derive(Accounts)]
pub struct SplDeposit <'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump,
    )]
    state : Account<'info, VaultState>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    owner_ata : Account<'info, TokenAccount>,
    mint : Account<'info, Mint>,
    #[account(
        seeds = [b"auth", state.key().as_ref()],
        bump = state.auth_bump,
    )]
    // Check: This is safe
    auth : UncheckedAccount<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"spl_vault", state.key().as_ref()],
        token::mint = mint,
        token::authority = owner,
        bump
    )]
    vault : Account<'info, TokenAccount>,
    token_program : Program<'info, Token>,
    associated_token_program : Program<'info, AssociatedToken>,
    system_program : Program<'info, System>

}

#[account]

pub struct VaultState {
    state_bump : u8,
    auth_bump : u8,
    vault_bump : u8,
}

impl VaultState {
    const LEN : usize = 8 + 1*3;
}