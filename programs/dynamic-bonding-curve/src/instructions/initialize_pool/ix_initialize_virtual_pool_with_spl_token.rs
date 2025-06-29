use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, MintTo, Token, TokenAccount},
    token_2022::spl_token_2022::instruction::AuthorityType,
    token_interface::{
        Mint as MintInterface, TokenAccount as TokenAccountInterface, TokenInterface,
    },
};
use std::cmp::{max, min};

use crate::{
    activation_handler::get_current_point,
    const_pda,
    constants::seeds::{POOL_PREFIX, TOKEN_VAULT_PREFIX},
    process_create_token_metadata,
    state::{fee::VolatilityTracker, PoolConfig, PoolType, TokenType, VirtualPool},
    EvtInitializePool, PoolError, ProcessCreateTokenMetadataParams,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializePoolParameters {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

// To fix IDL generation: https://github.com/coral-xyz/anchor/issues/3209
pub fn max_key(left: &Pubkey, right: &Pubkey) -> [u8; 32] {
    max(left, right).to_bytes()
}

pub fn min_key(left: &Pubkey, right: &Pubkey) -> [u8; 32] {
    min(left, right).to_bytes()
}

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeVirtualPoolWithSplTokenCtx<'info> {}

pub fn handle_initialize_virtual_pool_with_spl_token<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, InitializeVirtualPoolWithSplTokenCtx<'info>>,
    params: InitializePoolParameters,
) -> Result<()> {
    Ok(())
}
