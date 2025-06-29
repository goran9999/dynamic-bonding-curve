use anchor_lang::prelude::*;
use mpl_token_metadata::types::DataV2;

use crate::state::TokenAuthorityOption;
pub struct ProcessCreateTokenMetadataParams<'a, 'info> {
    pub system_program: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub pool_authority: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub metadata_program: AccountInfo<'info>,
    pub mint_metadata: AccountInfo<'info>,
    pub creator: AccountInfo<'info>,
    pub name: &'a str,
    pub symbol: &'a str,
    pub uri: &'a str,
    pub pool_authority_bump: u8,
    pub token_authority: TokenAuthorityOption,
    pub partner: Pubkey,
}

pub fn process_create_token_metadata(params: ProcessCreateTokenMetadataParams) -> Result<()> {
    // create token metadata
    msg!("create token metadata");

    Ok(())
}
