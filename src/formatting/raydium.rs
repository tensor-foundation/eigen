use super::{pad_label, CustomFormat};
use crate::types::{
    raydium_clmm::{PoolState as ClmmPoolState, RewardInfo},
    raydium_cp::PoolState as CpPoolState,
    raydium_v4::{AmmInfo, StateData},
};
use console::Style;

const AMM_LABEL_LENGTH: usize = 25;
const STATE_LABEL_LENGTH: usize = AMM_LABEL_LENGTH - 2;
const LINE_BREAK: &str = "-------------------------";

impl CustomFormat for AmmInfo {
    fn custom_format(&self) -> String {
        let color = Style::new();

        format!(
            "{}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}",
            color.apply_to("Raydium AMM Info---------"),
            pad_label("status", AMM_LABEL_LENGTH),
            color.apply_to(self.status),
            pad_label("nonce", AMM_LABEL_LENGTH),
            color.apply_to(self.nonce),
            pad_label("order_num", AMM_LABEL_LENGTH),
            color.apply_to(self.order_num),
            pad_label("depth", AMM_LABEL_LENGTH),
            color.apply_to(self.depth),
            pad_label("coin_decimals", AMM_LABEL_LENGTH),
            color.apply_to(self.coin_decimals),
            pad_label("pc_decimals", AMM_LABEL_LENGTH),
            color.apply_to(self.pc_decimals),
            pad_label("state", AMM_LABEL_LENGTH),
            color.apply_to(self.state),
            pad_label("reset_flag", AMM_LABEL_LENGTH),
            color.apply_to(self.reset_flag),
            pad_label("min_size", AMM_LABEL_LENGTH),
            color.apply_to(self.min_size),
            pad_label("coin_lot_size", AMM_LABEL_LENGTH),
            color.apply_to(self.coin_lot_size),
            pad_label("pc_lot_size", AMM_LABEL_LENGTH),
            color.apply_to(self.pc_lot_size),
            pad_label("min_price_multiplier", AMM_LABEL_LENGTH),
            color.apply_to(self.min_price_multiplier),
            pad_label("max_price_multiplier", AMM_LABEL_LENGTH),
            color.apply_to(self.max_price_multiplier),
            self.state_data.custom_format(),
            pad_label("coin_vault", AMM_LABEL_LENGTH),
            color.apply_to(self.coin_vault),
            pad_label("pc_vault", AMM_LABEL_LENGTH),
            color.apply_to(self.pc_vault),
            pad_label("coin_vault_mint", AMM_LABEL_LENGTH),
            color.apply_to(self.coin_vault_mint),
            pad_label("pc_vault_mint", AMM_LABEL_LENGTH),
            color.apply_to(self.pc_vault_mint),
            pad_label("lp_mint", AMM_LABEL_LENGTH),
            color.apply_to(self.lp_mint),
            pad_label("open_orders", AMM_LABEL_LENGTH),
            color.apply_to(self.open_orders),
            pad_label("market", AMM_LABEL_LENGTH),
            color.apply_to(self.market),
            pad_label("market_program", AMM_LABEL_LENGTH),
            color.apply_to(self.market_program),
            pad_label("target_orders", AMM_LABEL_LENGTH),
            color.apply_to(self.target_orders),
            pad_label("amm_owner", AMM_LABEL_LENGTH),
            color.apply_to(self.amm_owner),
            pad_label("lp_amount", AMM_LABEL_LENGTH),
            color.apply_to(self.lp_amount)
        )
    }
}

impl CustomFormat for StateData {
    fn custom_format(&self) -> String {
        let color = Style::new();
        format!(
            "{}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
{}",
            color.apply_to("--State Data-------------"),
            pad_label("need_take_pnl_coin", STATE_LABEL_LENGTH),
            color.apply_to(self.need_take_pnl_coin),
            pad_label("need_take_pnl_pc", STATE_LABEL_LENGTH),
            color.apply_to(self.need_take_pnl_pc),
            pad_label("total_pnl_pc", STATE_LABEL_LENGTH),
            color.apply_to(self.total_pnl_pc),
            pad_label("total_pnl_coin", STATE_LABEL_LENGTH),
            color.apply_to(self.total_pnl_coin),
            pad_label("pool_open_time", STATE_LABEL_LENGTH),
            color.apply_to(self.pool_open_time),
            pad_label("swap_coin_in_amount", STATE_LABEL_LENGTH),
            color.apply_to(self.swap_coin_in_amount),
            pad_label("swap_pc_out_amount", STATE_LABEL_LENGTH),
            color.apply_to(self.swap_pc_out_amount),
            pad_label("swap_acc_pc_fee", STATE_LABEL_LENGTH),
            color.apply_to(self.swap_acc_pc_fee),
            color.apply_to(LINE_BREAK)
        )
    }
}

impl CustomFormat for RewardInfo {
    fn custom_format(&self) -> String {
        let color = Style::new();

        format!(
            "{}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}",
            color.apply_to("--Reward Info-------------"),
            pad_label("reward_state", STATE_LABEL_LENGTH),
            color.apply_to(self.reward_state),
            pad_label("open_time", STATE_LABEL_LENGTH),
            color.apply_to(self.open_time),
            pad_label("end_time", STATE_LABEL_LENGTH),
            color.apply_to(self.end_time),
            pad_label("last_update_time", STATE_LABEL_LENGTH),
            color.apply_to(self.last_update_time),
            pad_label("emissions_per_second", STATE_LABEL_LENGTH),
            color.apply_to(self.emissions_per_second_x64),
            pad_label("total_emissioned", STATE_LABEL_LENGTH),
            color.apply_to(self.reward_total_emissioned),
            pad_label("reward_claimed", STATE_LABEL_LENGTH),
            color.apply_to(self.reward_claimed),
            pad_label("token_mint", STATE_LABEL_LENGTH),
            color.apply_to(self.token_mint),
            pad_label("token_vault", STATE_LABEL_LENGTH),
            color.apply_to(self.token_vault),
            pad_label("authority", STATE_LABEL_LENGTH),
            color.apply_to(self.authority)
        )
    }
}

impl CustomFormat for ClmmPoolState {
    fn custom_format(&self) -> String {
        let color = Style::new();

        let mut output = format!(
            "{}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}",
            color.apply_to("--CLMM Pool State---------"),
            pad_label("amm_config", AMM_LABEL_LENGTH),
            color.apply_to(self.amm_config),
            pad_label("owner", AMM_LABEL_LENGTH),
            color.apply_to(self.owner),
            pad_label("token_mint_0", AMM_LABEL_LENGTH),
            color.apply_to(self.token_mint_0),
            pad_label("token_mint_1", AMM_LABEL_LENGTH),
            color.apply_to(self.token_mint_1),
            pad_label("token_vault_0", AMM_LABEL_LENGTH),
            color.apply_to(self.token_vault_0),
            pad_label("token_vault_1", AMM_LABEL_LENGTH),
            color.apply_to(self.token_vault_1),
            pad_label("tick_spacing", AMM_LABEL_LENGTH),
            color.apply_to(self.tick_spacing),
            pad_label("liquidity", AMM_LABEL_LENGTH),
            color.apply_to(self.liquidity),
            pad_label("sqrt_price_x64", AMM_LABEL_LENGTH),
            color.apply_to(self.sqrt_price_x64),
            pad_label("tick_current", AMM_LABEL_LENGTH),
            color.apply_to(self.tick_current),
            pad_label("protocol_fees_0", AMM_LABEL_LENGTH),
            color.apply_to(self.protocol_fees_token_0),
            pad_label("protocol_fees_1", AMM_LABEL_LENGTH),
            color.apply_to(self.protocol_fees_token_1),
            pad_label("status", AMM_LABEL_LENGTH),
            color.apply_to(self.status),
            pad_label("open_time", AMM_LABEL_LENGTH),
            color.apply_to(self.open_time),
            pad_label("recent_epoch", AMM_LABEL_LENGTH),
            color.apply_to(self.recent_epoch)
        );

        // Add reward info formatting
        for (i, reward_info) in self.reward_infos.iter().enumerate() {
            if reward_info.initialized() {
                output.push_str(&format!(
                    "\n{}",
                    color.apply_to(format!("--Reward #{}--------------", i + 1))
                ));
                output.push_str(&format!("\n{}", reward_info.custom_format()));
            }
        }

        output.push_str(&format!("\n{}", color.apply_to(LINE_BREAK)));
        output
    }
}

impl CustomFormat for CpPoolState {
    fn custom_format(&self) -> String {
        let color = Style::new();

        format!(
            "{}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}",
            color.apply_to("--CPSwap Pool State-------"),
            pad_label("amm_config", AMM_LABEL_LENGTH),
            color.apply_to(self.amm_config),
            pad_label("pool_creator", AMM_LABEL_LENGTH),
            color.apply_to(self.pool_creator),
            pad_label("token_0_vault", AMM_LABEL_LENGTH),
            color.apply_to(self.token_0_vault),
            pad_label("token_1_vault", AMM_LABEL_LENGTH),
            color.apply_to(self.token_1_vault),
            pad_label("lp_mint", AMM_LABEL_LENGTH),
            color.apply_to(self.lp_mint),
            pad_label("token_0_mint", AMM_LABEL_LENGTH),
            color.apply_to(self.token_0_mint),
            pad_label("token_1_mint", AMM_LABEL_LENGTH),
            color.apply_to(self.token_1_mint),
            pad_label("token_0_program", AMM_LABEL_LENGTH),
            color.apply_to(self.token_0_program),
            pad_label("token_1_program", AMM_LABEL_LENGTH),
            color.apply_to(self.token_1_program),
            pad_label("observation_key", AMM_LABEL_LENGTH),
            color.apply_to(self.observation_key),
            pad_label("status", AMM_LABEL_LENGTH),
            color.apply_to(self.status),
            pad_label("lp_supply", AMM_LABEL_LENGTH),
            color.apply_to(self.lp_supply),
            pad_label("protocol_fees_0", AMM_LABEL_LENGTH),
            color.apply_to(self.protocol_fees_token_0),
            pad_label("protocol_fees_1", AMM_LABEL_LENGTH),
            color.apply_to(self.protocol_fees_token_1),
            pad_label("fund_fees_0", AMM_LABEL_LENGTH),
            color.apply_to(self.fund_fees_token_0),
            pad_label("fund_fees_1", AMM_LABEL_LENGTH),
            color.apply_to(self.fund_fees_token_1),
            pad_label("open_time", AMM_LABEL_LENGTH),
            color.apply_to(self.open_time),
            pad_label("recent_epoch", AMM_LABEL_LENGTH),
            color.apply_to(self.recent_epoch),
            color.apply_to(LINE_BREAK)
        )
    }
}
