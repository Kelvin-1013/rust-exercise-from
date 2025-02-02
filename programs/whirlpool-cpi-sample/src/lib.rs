use anchor_lang::prelude::*;

declare_id!("Agr1zAhadb4mNd54LaJoqTLV79YfXmWrMrx6TGsw39H9");

pub mod instructions;
use instructions::*;

pub use whirlpool_cpi::state::OpenPositionBumps;

#[program]
pub mod whirlpool_cpi_sample {
  use super::*;

  pub fn verify_whirlpools_config_account(
    ctx: Context<VerifyWhirlpoolsConfigAccount>,
  ) -> Result<()> {
    return instructions::verify_account::handler_whirlpools_config(ctx);
  }

  pub fn verify_feetier_account(
    ctx: Context<VerifyFeeTierAccount>,
  ) -> Result<()> {
    return instructions::verify_account::handler_feetier(ctx);
  }

  pub fn verify_whirlpool_account(
    ctx: Context<VerifyWhirlpoolAccount>,
  ) -> Result<()> {
    return instructions::verify_account::handler_whirlpool(ctx);
  }

  pub fn verify_tickarray_account(
    ctx: Context<VerifyTickArrayAccount>,
    sampling1: u32,
    sampling2: u32,
    sampling3: u32,
    sampling4: u32,
    sampling5: u32,
    sampling6: u32,
    sampling7: u32,
    sampling8: u32,
  ) -> Result<()> {
    return instructions::verify_account::handler_tickarray(
      ctx,
      sampling1, sampling2, sampling3, sampling4,
      sampling5, sampling6, sampling7, sampling8,
    );
  }

  pub fn verify_position_account(
    ctx: Context<VerifyPositionAccount>,
  ) -> Result<()> {
    return instructions::verify_account::handler_position(ctx);
  }

  pub fn proxy_swap(
    ctx: Context<ProxySwap>,
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit: u128,
    amount_specified_is_input: bool,
    a_to_b: bool,
  ) -> Result<()> {
    return instructions::proxy_swap::proxy_swap(
      ctx,
      amount,
      other_amount_threshold,
      sqrt_price_limit,
      amount_specified_is_input,
      a_to_b,
    );
  }

  pub fn proxy_open_position(
    ctx: Context<ProxyOpenPosition>,
    tick_lower_index: i32,
    tick_upper_index: i32,
  ) -> Result<()> {
    return instructions::proxy_open_position::proxy_open_position(
      ctx,
      tick_lower_index,
      tick_upper_index,
    );
  }

  pub fn proxy_increase_liquidity(
    ctx: Context<ProxyIncreaseLiquidity>,
    liquidity: u128,
    token_max_a: u64,
    token_max_b: u64,
  ) -> Result<()> {
    return instructions::proxy_increase_liquidity::proxy_increase_liquidity(
      ctx,
      liquidity,
      token_max_a,
      token_max_b,
    );
  }

  pub fn proxy_decrease_liquidity(
    ctx: Context<ProxyDecreaseLiquidity>,
    liquidity: u128,
    token_min_a: u64,
    token_min_b: u64,
  ) -> Result<()> {
    return instructions::proxy_decrease_liquidity::proxy_decrease_liquidity(
      ctx,
      liquidity,
      token_min_a,
      token_min_b,
    );
  }

  pub fn proxy_update_fees_and_rewards(
    ctx: Context<ProxyUpdateFeesAndRewards>,
  ) -> Result<()> {
    return instructions::proxy_update_fees_and_rewards::proxy_update_fees_and_rewards(
      ctx,
    );
  }

  pub fn proxy_collect_fees(
    ctx: Context<ProxyCollectFees>,
  ) -> Result<()> {
    return instructions::proxy_collect_fees::proxy_collect_fees(
      ctx,
    );
  }

  pub fn proxy_collect_reward(
    ctx: Context<ProxyCollectReward>,
    reward_index: u8,
  ) -> Result<()> {
    return instructions::proxy_collect_reward::proxy_collect_reward(
      ctx,
      reward_index,
    );
  }

  pub fn proxy_close_position(
    ctx: Context<ProxyClosePosition>,
  ) -> Result<()> {
    return instructions::proxy_close_position::proxy_close_position(
      ctx,
    );
  }

  pub fn proxy_initialize_pool(
    ctx: Context<ProxyInitializePool>,
    tick_spacing: u16,
    initial_sqrt_price: u128,
  ) -> Result<()> {
    return instructions::proxy_initialize_pool::proxy_initialize_pool(
      ctx,
      tick_spacing,
      initial_sqrt_price,
    );
  }

  pub fn proxy_initialize_tick_array(
    ctx: Context<ProxyInitializeTickArray>,
    start_tick_index: i32,
  ) -> Result<()> {
    return instructions::proxy_initialize_tick_array::proxy_initialize_tick_array(
      ctx,
      start_tick_index,
    );
  }

}
