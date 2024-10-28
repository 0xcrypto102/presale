pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("8wDUqAZzTJ1ZHbGDMMU15yjiifdCyasAnNtX9CZqUBYg");

#[program]
pub mod presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64, token_price: u64) -> Result<()> {
        instructions::initialize(ctx, amount, token_price)
    }

    pub fn toggle_status(ctx: Context<ManagePresale>) -> Result<()> {
        instructions::toggle_status(ctx)
    }

    pub fn update_sale_type(ctx: Context<ManagePresale>) -> Result<()> {
        instructions::update_sale_type(ctx)
    }

    pub fn update_token_price(ctx: Context<ManagePresale>, new_price: u64) -> Result<()> {
        instructions::update_token_price(ctx, new_price)
    }

    pub fn update_owner(ctx: Context<ManagePresale>, new_owner: Pubkey) -> Result<()> {
        instructions::update_owner(ctx, new_owner)
    }
}
