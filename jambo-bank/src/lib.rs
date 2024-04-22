use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use solana_program::{pubkey, pubkey::Pubkey};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("");

// Derived PDAs seed
pub const JAMBO_VAULT_SEED: &[u8] = b"jambo_vault";
pub const JAMBO_VAULT_AUTHORITY_SEED: &[u8] = b"jambo_vault_authority";

pub const JAMBO_BANK_ACCOUNT_SEED: &[u8] = b"jambo_bank_account";

// Jambo Bank Owner
pub const JAMBO_BANK_OWNER: Pubkey = pubkey!("");

// owner only
#[program]
pub mod jambo_bank {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_account(ctx: Context<CreateAccount>, id: Vec<u8>) -> Result<()> {
        let bank_account = &mut ctx.accounts.bank_account;

        bank_account.id = id.clone();
        bank_account.balance = 0;

        msg!(
            "Jambo Bank Account: {}, Will be created",
            String::from_utf8_lossy(&id).to_string()
        );

        Ok(())
    }

    pub fn remove_account(ctx: Context<RemoveAccount>, id: Vec<u8>) -> Result<()> {
        let bank_account = &mut ctx.accounts.bank_account;

        if bank_account.balance > 0 {
            return Err(error!(ErrorCode::BalanceTooBig));
        }

        msg!(
            "Jambo Bank Account: {}, Will be deleted",
            String::from_utf8_lossy(&id).to_string()
        );

        Ok(())
    }

    pub fn transfer_balance(
        ctx: Context<TransferBalance>,
        fid: Vec<u8>,
        tid: Vec<u8>,
        amount: u64,
    ) -> Result<()> {
        let f_bank_account = &mut ctx.accounts.f_bank_account;
        let t_bank_account = &mut ctx.accounts.t_bank_account;

        if amount > f_bank_account.balance {
            return Err(error!(ErrorCode::BalanceTooSmall));
        };

        f_bank_account.balance = f_bank_account.balance.checked_sub(amount).unwrap();
        t_bank_account.balance = t_bank_account.balance.checked_add(amount).unwrap();

        msg!(
            "{} transfer {} amount to {}.",
            String::from_utf8_lossy(&fid).to_string(),
            amount,
            String::from_utf8_lossy(&tid).to_string()
        );

        Ok(())
    }

    pub fn deposit(ctx: Context<TransferSpl>, id: Vec<u8>, amount: u64) -> Result<()> {
        if amount == 0 {
            return Err(error!(ErrorCode::AmountTooSmall));
        };

        let bank_account = &mut ctx.accounts.bank_account;

        bank_account.balance = bank_account.balance.checked_add(amount).unwrap();

        let amount_decimals = amount
            .checked_mul(10u64.pow(ctx.accounts.mint.decimals as u32))
            .unwrap();
        let spl_amount = amount_decimals.checked_div(100u64).unwrap();

        let transfer_instruction = Transfer {
            from: ctx.accounts.wallet.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.wallet_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        anchor_spl::token::transfer(cpi_ctx, spl_amount)?;

        msg!(
            "{} deposit {} amount.",
            String::from_utf8_lossy(&id).to_string(),
            amount,
        );

        Ok(())
    }

    pub fn withdraw(ctx: Context<TransferSpl>, id: Vec<u8>, amount: u64) -> Result<()> {
        let bank_account = &mut ctx.accounts.bank_account;

        if amount > bank_account.balance {
            return Err(error!(ErrorCode::AmountTooBig));
        };

        bank_account.balance = bank_account.balance.checked_sub(amount).unwrap();

        let amount_decimals = amount
            .checked_mul(10u64.pow(ctx.accounts.mint.decimals as u32))
            .unwrap();
        let spl_amount = amount_decimals.checked_div(100u64).unwrap();

        let transfer_instruction = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.wallet.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };

        let bump = ctx.bumps.vault_authority;
        let seeds = &[
            JAMBO_VAULT_AUTHORITY_SEED,
            ctx.accounts.owner.key.as_ref(),
            &[bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer,
        );

        anchor_spl::token::transfer(cpi_ctx, spl_amount)?;

        msg!(
            "{} withdraw {} amount.",
            String::from_utf8_lossy(&id).to_string(),
            amount,
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Derived PDAs
    #[account(
        init_if_needed,
        payer=payer,
        seeds=[JAMBO_VAULT_AUTHORITY_SEED, owner.key().as_ref()],
        bump,
        space=8
    )]
    pub vault_authority: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer=payer,
        seeds=[JAMBO_VAULT_SEED, mint.key().as_ref()],
        bump,
        token::mint=mint,
        token::authority=vault_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(address=JAMBO_BANK_OWNER, signer)]
    pub owner: AccountInfo<'info>,

    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: Vec<u8>)]
pub struct CreateAccount<'info> {
    // Derived PDAs
    #[account(
        init_if_needed,
        payer=payer,
        seeds=[JAMBO_BANK_ACCOUNT_SEED, owner.key().as_ref(), id.as_ref()],
        bump,
        space=8+std::mem::size_of::<BankAccount>()
    )]
    pub bank_account: Account<'info, BankAccount>,

    #[account(address=JAMBO_BANK_OWNER, signer)]
    pub owner: AccountInfo<'info>,

    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: Vec<u8>)]
pub struct RemoveAccount<'info> {
    #[account(
        mut,
        seeds=[JAMBO_BANK_ACCOUNT_SEED, owner.key().as_ref(), id.as_ref()],
        bump,
        close=payer
    )]
    pub bank_account: Account<'info, BankAccount>,

    #[account(address=JAMBO_BANK_OWNER, signer)]
    pub owner: AccountInfo<'info>,

    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(fid: Vec<u8>, tid: Vec<u8>)]
pub struct TransferBalance<'info> {
    // from
    #[account(
        mut,
        seeds=[JAMBO_BANK_ACCOUNT_SEED, owner.key().as_ref(), fid.as_ref()],
        bump
    )]
    pub f_bank_account: Account<'info, BankAccount>,

    // to
    #[account(
        mut,
        seeds=[JAMBO_BANK_ACCOUNT_SEED, owner.key().as_ref(), tid.as_ref()],
        bump
    )]
    pub t_bank_account: Account<'info, BankAccount>,

    #[account(address=JAMBO_BANK_OWNER, signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: Vec<u8>)]
pub struct TransferSpl<'info> {
    #[account(
        mut,
        seeds=[JAMBO_VAULT_AUTHORITY_SEED, owner.key().as_ref()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds=[JAMBO_VAULT_SEED, mint.key().as_ref()],
        bump,
        token::mint=mint,
        token::authority=vault_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(signer)]
    pub wallet_authority: AccountInfo<'info>,

    #[account(mut)]
    pub wallet: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[JAMBO_BANK_ACCOUNT_SEED, owner.key().as_ref(), id.as_ref()],
        bump
    )]
    pub bank_account: Account<'info, BankAccount>,

    #[account(address=JAMBO_BANK_OWNER, signer)]
    pub owner: AccountInfo<'info>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct BankAccount {
    pub id: Vec<u8>,
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero")]
    AmountTooSmall,

    #[msg("Withdraw amount cannot be less than deposit")]
    AmountTooBig,

    #[msg("Balance amount cannot be less than transfer")]
    BalanceTooSmall,

    #[msg("Balance amount must be less than or equal to zero")]
    BalanceTooBig,
}
