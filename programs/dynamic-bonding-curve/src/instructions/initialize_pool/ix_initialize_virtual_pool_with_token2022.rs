use super::InitializePoolParameters;
use super::{max_key, min_key};
use crate::{
    activation_handler::get_current_point,
    const_pda,
    constants::seeds::{POOL_PREFIX, TOKEN_VAULT_PREFIX},
    state::fee::VolatilityTracker,
    state::{PoolConfig, PoolType, TokenType, VirtualPool},
    token::update_account_lamports_to_minimum_balance,
    EvtInitializePool, PoolError,
};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;
use anchor_spl::token_interface::spl_pod::optional_keys::OptionalNonZeroPubkey;
use anchor_spl::{
    token_2022::{mint_to, MintTo, Token2022},
    token_interface::{
        token_metadata_initialize, token_metadata_update_authority, Mint, TokenAccount,
        TokenInterface, TokenMetadataInitialize,
    },
};

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeVirtualPoolWithToken2022Ctx<'info> {
    /// Which config the pool belongs to.
    #[account(has_one = quote_mint)]
    pub config: AccountLoader<'info, PoolConfig>,

    /// CHECK: pool authority
    #[account(
        address = const_pda::pool_authority::ID
    )]
    pub pool_authority: AccountInfo<'info>,

    pub creator: Signer<'info>,

    /// Unique token mint address, initialize in contract
    #[account(
        init,
        signer,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = config.load()?.token_decimal,
        mint::authority = pool_authority,
        extensions::metadata_pointer::authority = pool_authority,
        extensions::metadata_pointer::metadata_address = base_mint,
    )]
    pub base_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mint::token_program = token_quote_program,
    )]
    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Initialize an account to store the pool state
    #[account(
        init,
        seeds = [
            POOL_PREFIX.as_ref(),
            config.key().as_ref(),
            &max_key(&base_mint.key(), &quote_mint.key()),
            &min_key(&base_mint.key(), &quote_mint.key()),
        ],
        bump,
        payer = payer,
        space = 8 + VirtualPool::INIT_SPACE
    )]
    pub pool: AccountLoader<'info, VirtualPool>,

    /// CHECK: Token base vault for the pool
    #[account(
        init,
        seeds = [
            TOKEN_VAULT_PREFIX.as_ref(),
            base_mint.key().as_ref(),
            pool.key().as_ref(),
        ],
        token::mint = base_mint,
        token::authority = pool_authority,
        token::token_program = token_program,
        payer = payer,
        bump,
    )]
    pub base_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Token quote vault for the pool
    #[account(
        init,
        seeds = [
            TOKEN_VAULT_PREFIX.as_ref(),
            quote_mint.key().as_ref(),
            pool.key().as_ref(),
        ],
        token::mint = quote_mint,
        token::authority = pool_authority,
        token::token_program = token_quote_program,
        payer = payer,
        bump,
    )]
    pub quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Program to create mint account and mint tokens
    pub token_quote_program: Interface<'info, TokenInterface>,
    /// token program for base mint
    pub token_program: Program<'info, Token2022>,
    // Sysvar for program account
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_virtual_pool_with_token2022<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, InitializeVirtualPoolWithToken2022Ctx<'info>>,
    params: InitializePoolParameters,
) -> Result<()> {
    Ok(())
}
