use std::collections::hash_map::{Entry, RandomState};
use std::collections::{HashMap, HashSet};
use std::marker::{Send, Sync};
use std::sync::{Arc, Mutex, RwLock};

use crate::client::client::{ConnStatus, EClient};
use crate::client::common::{
    BarData, CommissionReport, DepthMktDataDescription, FaDataType, FamilyCode, HistogramData,
    HistoricalTick, HistoricalTickBidAsk, NewsProvider, PriceIncrement, SmartComponent, TickAttrib,
    TickAttribBidAsk, TickAttribLast, TickType,
};
use crate::client::contract::{
    Contract, ContractDescription, ContractDetails, DeltaNeutralContract,
};
use crate::client::execution::Execution;
use crate::client::order::{Order, OrderState, SoftDollarTier};
use crate::client::wrapper::Wrapper;

//==================================================================================================

pub struct DefaultWrapper {
    pub client: Option<Arc<Mutex<EClient<DefaultWrapper>>>>,
}

impl DefaultWrapper {
    pub fn new() -> Self {
        DefaultWrapper { client: None }
    }
}
impl Wrapper for DefaultWrapper {
    fn error(&self, req_id: i32, error_code: i32, error_string: &str) {
        error!(
            "req_id: {} ,error_code: {} , error_string:{}",
            req_id, error_code, error_string
        );
    }

    fn win_error(&self, text: &str, last_error: i32) {
        error!("text: {} , last_error:{}", text, last_error);
    }

    fn connect_ack(&self) {
        info!("Connected.");
    }

    fn market_data_type(&self, req_id: i32, market_data_type: i32) {
        info!(
            "market_data_type -- req_id: {}, market_data_type: {}",
            req_id, market_data_type
        );
    }

    fn tick_price(&self, req_id: i32, tick_type: TickType, price: f64, attrib: TickAttrib) {
        info!(
            "tick_size -- req_id: {}, tick_type: {}, price: {}, attrib: {}",
            req_id, tick_type, price, attrib
        );
    }

    fn tick_size(&self, req_id: i32, tick_type: TickType, size: i32) {
        info!(
            "tick_size -- req_id: {}, tick_type: {}, size: {}",
            req_id, tick_type, size
        );
    }

    fn tick_snapshot_end(&self, req_id: i32) {
        info!("tick_snapshot_end -- req_id: {}", req_id);
    }

    fn tick_generic(&self, req_id: i32, tick_type: TickType, value: f64) {
        info!(
            "tick_generic -- req_id: {}, tick_type: {}, value: {}",
            req_id, tick_type, value
        );
    }

    fn tick_string(&self, req_id: i32, tick_type: TickType, value: &str) {
        info!(
            "tick_string -- req_id: {}, tick_type: {}, value: {}",
            req_id, tick_type, value
        );
    }

    fn tick_efp(
        &self,
        req_id: i32,
        tick_type: TickType,
        basis_points: f64,
        formatted_basis_points: &str,
        total_dividends: f64,
        hold_days: i32,
        future_last_trade_date: &str,
        dividend_impact: f64,
        dividends_to_last_trade_date: f64,
    ) {
        info!(
            "tick_efp -- req_id: {},
             tick_type: {},
             basis_points: {},
             formatted_basis_points: {},
             total_dividends: {},
             hold_days: {},
             future_last_trade_date: {},
             dividend_impact: {},
             dividends_to_last_trade_date: {},",
            req_id,
            tick_type,
            basis_points,
            formatted_basis_points,
            total_dividends,
            hold_days,
            future_last_trade_date,
            dividend_impact,
            dividends_to_last_trade_date,
        );
    }

    fn order_status(
        &self,
        order_id: i32,
        status: &str,
        filled: f64,
        remaining: f64,
        avg_fill_price: f64,
        perm_id: i32,
        parent_id: i32,
        last_fill_price: f64,
        client_id: i32,
        why_held: &str,
        mkt_cap_price: f64,
    ) {
        info!(
            "order_status -- order_id: {}, status: {}, filled: {}, remaining: {}, avg_fill_price: {}, \
            perm_id: {}, parent_id: {}, last_fill_price: {}, client_id: {}, why_held: {}, mkt_cap_price: {}",
            order_id, status, filled, remaining, avg_fill_price, perm_id, parent_id, last_fill_price,
            client_id, why_held, mkt_cap_price
        );
    }

    fn open_order(&self, order_id: i32, contract: Contract, order: Order, order_state: OrderState) {
        info!(
            "open_order -- order_id: {}, contract: {}, order: {}, order_state: {}",
            order_id, contract, order, order_state
        );
    }

    fn open_order_end(&self) {
        info!("connection_closed. (no parmeters passed)");
    }

    fn connection_closed(&self) {
        info!("connection_closed. (no parmeters passed)");
    }

    fn update_account_value(&self, key: &str, val: &str, currency: &str, account_name: &str) {
        info!(
            "key: {}, value: {}, ccy: {}, account: {}.",
            key, val, currency, account_name
        );
    }

    fn update_portfolio(
        &self,
        contract: Contract,
        position: f64,
        market_price: f64,
        market_value: f64,
        average_cost: f64,
        unrealized_pnl: f64,
        realized_pnl: f64,
        account_name: &str,
    ) {
        info!(
            "update_portfolio -- contract: {}, position: {}, market_price: {}, market_value: {}, 
             average_cost: {}, unrealized_pnl: {},  realized_pnl: {},  account_name: {}",
            contract,
            position,
            market_price,
            market_value,
            average_cost,
            unrealized_pnl,
            realized_pnl,
            account_name
        );
    }

    fn update_account_time(&self, time_stamp: &str) {
        info!("update_account_time: {}.", time_stamp);
    }

    fn account_download_end(&self, account_name: &str) {
        info!("account_download_end: {}.", account_name);
    }

    fn next_valid_id(&self, order_id: i32) {
        info!("next_valid_id -- order_id: {}", order_id);
    }

    fn contract_details(&self, req_id: i32, contract_details: ContractDetails) {
        info!(
            "contract_details -- req_id: {}, contract_details: {}",
            req_id, contract_details
        );
    }

    fn bond_contract_details(&self, req_id: i32, contract_details: ContractDetails) {
        info!(
            "bond_contract_details -- req_id: {}, contract_details: {}",
            req_id, contract_details
        );
    }

    fn contract_details_end(&self, req_id: i32) {
        info!("contract_details_end -- req_id: {}", req_id);
    }

    fn exec_details(&self, req_id: i32, contract: Contract, execution: Execution) {
        info!(
            "exec_details -- req_id: {}, contract: {}, execution: {}",
            req_id, contract, execution
        );
    }

    fn exec_details_end(&self, req_id: i32) {
        info!("exec_details_end -- req_id: {}", req_id);
    }

    fn update_mkt_depth(
        &self,
        req_id: i32,
        position: i32,
        operation: i32,
        side: i32,
        price: f64,
        size: i32,
    ) {
        info!(
            "update_mkt_depth -- req_id: {}, position: {}, operation: {}, side: {}, price: {}, size: {}",
            req_id, position, operation, side, price, size
        );
    }

    fn update_mkt_depth_l2(
        &self,
        req_id: i32,
        position: i32,
        market_maker: &str,
        operation: i32,
        side: i32,
        price: f64,
        size: i32,
        is_smart_depth: bool,
    ) {
        info!(
            "update_mkt_depth_l2 -- req_id: {}, position: {}, market_maker: {}, operation: {}, side: {}, price: {}, size: {}, is_smart_depth: {},",
            req_id, position, market_maker, operation, side, price, size, is_smart_depth
        );
    }

    fn update_news_bulletin(
        &self,
        msg_id: i32,
        msg_type: i32,
        news_message: &str,
        origin_exch: &str,
    ) {
        info!(
            "update_news_bulletin -- msg_id: {}, msg_type: {}, news_message: {}, origin_exch: {}",
            msg_id, msg_type, news_message, origin_exch
        );
    }

    fn managed_accounts(&self, accounts_list: &str) {
        info!("managed_accounts -- accounts_list: {}", accounts_list);
    }

    fn receive_fa(&self, fa_data: FaDataType, cxml: &str) {
        info!("receive_fa -- fa_data: {}, cxml: {}", fa_data, cxml);
    }

    fn historical_data(&self, req_id: i32, bar: BarData) {
        info!("historical_data -- req_id: {}, bar: {}", req_id, bar);
    }

    fn historical_data_end(&self, req_id: i32, start: &str, end: &str) {
        info!(
            "historical_data_end -- req_id: {}, start: {}, end: {}",
            req_id, start, end
        );
    }

    fn scanner_parameters(&self, xml: &str) {
        info!("scanner_parameters -- xml: {}", xml);
    }

    fn scanner_data(
        &self,
        req_id: i32,
        rank: i32,
        contract_details: ContractDetails,
        distance: &str,
        benchmark: &str,
        projection: &str,
        legs_str: &str,
    ) {
        info!(
            "scanner_data -- req_id: {}, rank: {},
             contract_details: {},
             distance: {},
             benchmark: {},
             projection: {},
             legs_str: {}",
            req_id, rank, contract_details, distance, benchmark, projection, legs_str
        );
    }

    fn scanner_data_end(&self, req_id: i32) {
        info!("scanner_data_end -- req_id: {}", req_id);
    }

    fn realtime_bar(
        &self,
        req_id: i32,
        time: i32,
        open_: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: i32,
        wap: f64,
        count: i32,
    ) {
        info!(
            "realtime_bar -- req_id: {}, time: {}, open_: {}, high: {}, low: {}, close: {}, volume: {}, wap: {}, count: {}",
            req_id,
            time,
            open_,
            high,
            low,
            close,
            volume,
            wap,
            count,
        );
    }

    fn current_time(&self, time: i32) {
        info!("current_time -- time: {}", time);
    }

    fn fundamental_data(&self, req_id: i32, data: &str) {
        info!(
            "fundamental_data -- req_id: {}, delta_neutral_contract: {}",
            req_id, data
        );
    }

    fn delta_neutral_validation(&self, req_id: i32, delta_neutral_contract: DeltaNeutralContract) {
        info!(
            "delta_neutral_validation -- req_id: {}, delta_neutral_contract: {}",
            req_id, delta_neutral_contract
        );
    }

    fn commission_report(&self, commission_report: CommissionReport) {
        info!(
            "commission_report -- commission_report: {}",
            commission_report
        );
    }

    fn position(&self, account: &str, contract: Contract, position: f64, avg_cost: f64) {
        info!(
            "position -- account: {}, contract: [{}], position: {}, avg_cost: {}",
            account, contract, position, avg_cost
        );
    }

    fn position_end(&self) {
        info!("position_end -- (no params are passed in this one)");
    }

    fn account_summary(&self, req_id: i32, account: &str, tag: &str, value: &str, currency: &str) {
        info!(
            "account_summary -- req_id: {}, account: {}, tag: {}, value: {}, currency: {}",
            req_id, account, tag, value, currency
        );
        {
            self.client
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .req_current_time();
        }
    }

    fn account_summary_end(&self, req_id: i32) {
        info!("account_summary_end -- req_id: {}", req_id);
    }

    fn verify_message_api(&self, api_data: &str) {
        info!("verify_message_api -- api_data: {}", api_data);
    }

    fn verify_completed(&self, is_successful: bool, error_text: &str) {
        info!(
            "verify_completed -- is_successful: {}, error_text: {}",
            is_successful, error_text
        );
    }

    fn verify_and_auth_message_api(&self, api_data: &str, xyz_challange: &str) {
        info!(
            "verify_and_auth_message_api -- api_data: {}, xyz_challange: {}",
            api_data, xyz_challange
        );
    }

    fn verify_and_auth_completed(&self, is_successful: bool, error_text: &str) {
        info!(
            "verify_and_auth_completed -- is_successful: {}, error_text: {}",
            is_successful, error_text
        );
    }

    fn display_group_list(&self, req_id: i32, groups: &str) {
        info!(
            "display_group_list -- req_id: {}, error_text: {}",
            req_id, groups
        );
    }

    fn display_group_updated(&self, req_id: i32, contract_info: &str) {
        info!(
            "display_group_updated -- req_id: {}, contract_info: {}",
            req_id, contract_info
        );
    }

    fn position_multi(
        &self,
        req_id: i32,
        account: &str,
        model_code: &str,
        contract: Contract,
        pos: f64,
        avg_cost: f64,
    ) {
        info!(
            "position_multi -- req_id: {}, account: {}, model_code: {}, contract: {}, pos: {}, \
             avg_cost: {}",
            req_id, account, model_code, contract, pos, avg_cost
        );
    }

    fn position_multi_end(&self, req_id: i32) {
        info!("position_multi_end -- req_id: {}", req_id);
    }

    fn account_update_multi(
        &self,
        req_id: i32,
        account: &str,
        model_code: &str,
        key: &str,
        value: &str,
        currency: &str,
    ) {
        info!(
            "account_update_multi -- req_id: {}, account: {}, model_code: {}, key: {}, value: {}, currency: {}",
            req_id, account, model_code, key, value, currency
        );
    }

    fn account_update_multi_end(&self, req_id: i32) {
        info!("account_update_multi_end -- req_id: {}", req_id);
    }

    fn tick_option_computation(
        &self,
        req_id: i32,
        tick_type: TickType,
        implied_vol: f64,
        delta: f64,
        opt_price: f64,
        pv_dividend: f64,
        gamma: f64,
        vega: f64,
        theta: f64,
        und_price: f64,
    ) {
        info!(
            "tick_option_computation -- req_id: {}, tick_type: {}, implied_vol: {}, delta: {}, \
             opt_price: {}, pv_dividend: {},  gamma: {}, vega: {}, theta: {}, und_price: {}",
            req_id,
            tick_type,
            implied_vol,
            delta,
            opt_price,
            pv_dividend,
            gamma,
            vega,
            theta,
            und_price
        );
    }

    fn security_definition_option_parameter(
        &self,
        req_id: i32,
        exchange: &str,
        underlying_con_id: i32,
        trading_class: &str,
        multiplier: &str,
        expirations: HashSet<String>,
        strikes: HashSet<f64>,
    ) {
        info!(
            "tick_option_computation -- req_id: {}, exchange: {}, underlying_con_id: {}, \
             trading_class: {}, multiplier: {}, expirations: {:?},  strikes: {:?}",
            req_id,
            exchange,
            underlying_con_id,
            trading_class,
            multiplier,
            expirations
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>(),
            strikes.iter().map(|x| *x).collect::<Vec<f64>>()
        );
    }

    fn security_definition_option_parameter_end(&self, req_id: i32) {
        info!(
            "security_definition_option_parameter_end -- req_id: {}",
            req_id
        );
    }

    fn soft_dollar_tiers(&self, req_id: i32, tiers: Vec<SoftDollarTier>) {
        info!(
            "soft_dollar_tiers -- req_id: {}, tiers: {:?}",
            req_id, tiers
        );
    }

    fn family_codes(&self, family_codes: Vec<FamilyCode>) {
        info!("family_codes -- family_codes: {:?}", family_codes);
    }

    fn symbol_samples(&self, req_id: i32, contract_descriptions: Vec<ContractDescription>) {
        info!(
            "symbol_samples -- req_id: {}, contract_descriptions: {:?}",
            req_id, contract_descriptions
        );
    }

    fn mkt_depth_exchanges(&self, depth_mkt_data_descriptions: Vec<DepthMktDataDescription>) {
        info!(
            "mkt_depth_exchanges -- depth_mkt_data_descriptions: {:?}",
            depth_mkt_data_descriptions
        );
    }

    fn tick_news(
        &self,
        ticker_id: i32,
        time_stamp: i32,
        provider_code: &str,
        article_id: &str,
        headline: &str,
        extra_data: &str,
    ) {
        info!(
            "tick_news -- ticker_id: {}, time_stamp: {}, provider_code: {}, article_id: {}, \
             headline: {}, extra_data: {},",
            ticker_id, time_stamp, provider_code, article_id, headline, extra_data
        );
    }

    fn smart_components(&self, req_id: i32, smart_components: Vec<SmartComponent>) {
        info!(
            "smart_components -- req_id: {}, smart_components: {:?}",
            req_id, smart_components
        );
    }

    fn tick_req_params(
        &self,
        ticker_id: i32,
        min_tick: f64,
        bbo_exchange: &str,
        snapshot_permissions: i32,
    ) {
        info!(
            "tick_req_params -- ticker_id: {}, min_tick: {}, bbo_exchange: {}, snapshot_permissions: {}",
            ticker_id, min_tick, bbo_exchange, snapshot_permissions
        );
    }

    fn news_providers(&self, news_providers: Vec<NewsProvider>) {
        info!("news_providers -- news_providers: {:?}", news_providers);
    }

    fn news_article(&self, request_id: i32, article_type: i32, article_text: &str) {
        info!(
            "news_article -- request_id: {}, article_type: {}, article_text: {}",
            request_id, article_type, article_text
        );
    }

    fn historical_news(
        &self,
        request_id: i32,
        time: &str,
        provider_code: &str,
        article_id: &str,
        headline: &str,
    ) {
        info!(
            "historical_news -- request_id: {}, time: {}, provider_code: {}, article_id: {}, headline: {}",
            request_id, time, provider_code, article_id, headline
        );
    }

    fn historical_news_end(&self, request_id: i32, has_more: bool) {
        info!(
            "historical_news_end -- request_id: {}, has_more: {}",
            request_id, has_more
        );
    }

    fn head_timestamp(&self, req_id: i32, head_timestamp: &str) {
        info!(
            "head_timestamp -- req_id: {}, head_timestamp: {}",
            req_id, head_timestamp
        );
    }

    fn histogram_data(&self, req_id: i32, items: HistogramData) {
        info!("histogram_data -- req_id: {}, items: {}", req_id, items);
    }

    fn historical_data_update(&self, req_id: i32, bar: BarData) {
        info!("historical_data_update -- req_id: {}, bar: {}", req_id, bar);
    }

    fn reroute_mkt_data_req(&self, req_id: i32, con_id: i32, exchange: &str) {
        info!(
            "reroute_mkt_data_req -- req_id: {}, con_id: {}, exchange: {}",
            req_id, con_id, exchange
        );
    }

    fn reroute_mkt_depth_req(&self, req_id: i32, con_id: i32, exchange: &str) {
        info!(
            "reroute_mkt_depth_req -- req_id: {}, con_id: {}, exchange: {}",
            req_id, con_id, exchange
        );
    }

    fn market_rule(&self, market_rule_id: i32, price_increments: Vec<PriceIncrement>) {
        info!(
            "market_rule -- market_rule_id: {}, price_increments: {:?}",
            market_rule_id, price_increments
        );
    }

    fn pnl(&self, req_id: i32, daily_pn_l: f64, unrealized_pn_l: f64, realized_pn_l: f64) {
        info!(
            "pnl -- req_id: {}, daily_pn_l: {}, unrealized_pn_l: {}, realized_pn_l: {})",
            req_id, daily_pn_l, unrealized_pn_l, realized_pn_l
        );
    }

    fn pnl_single(
        &self,
        req_id: i32,
        pos: i32,
        daily_pn_l: f64,
        unrealized_pn_l: f64,
        realized_pn_l: f64,
        value: f64,
    ) {
        info!(
            "pnl_single -- req_id: {}, pos: {}, daily_pn_l: {}, unrealized_pn_l: {}, realized_pn_l: {}, value: {})",
            req_id, pos, daily_pn_l, unrealized_pn_l, realized_pn_l, value
        );
    }

    fn historical_ticks(&self, req_id: i32, ticks: Vec<HistoricalTick>, done: bool) {
        info!(
            "historical_ticks -- req_id: {}, ticks: {:?}, done: {}",
            req_id, ticks, done
        );
    }

    fn historical_ticks_bid_ask(&self, req_id: i32, ticks: Vec<HistoricalTickBidAsk>, done: bool) {
        info!(
            "historical_ticks_bid_ask -- req_id: {}, ticks: {:?}, done: {}",
            req_id, ticks, done
        );
    }

    fn tick_by_tick_all_last(
        &self,
        req_id: i32,
        tick_type: TickType,
        time: i32,
        price: f64,
        size: i32,
        tick_attrib_last: TickAttribLast,
        exchange: &str,
        special_conditions: &str,
    ) {
        info!(
            "tick_by_tick_all_last -- req_id: {}, tick_type: {}, time: {}, price: {}, size: {}, \
             tick_attrib_last: {}, exchange: {}, special_conditions: {}",
            req_id, tick_type, time, price, size, tick_attrib_last, exchange, special_conditions
        );
    }

    fn tick_by_tick_bid_ask(
        &self,
        req_id: i32,
        time: i32,
        bid_price: f64,
        ask_price: f64,
        bid_size: i32,
        ask_size: i32,
        tick_attrib_bid_ask: TickAttribBidAsk,
    ) {
        info!(
            "tick_by_tick_bid_ask -- req_id: {}, time: {}, bid_price: {}, ask_price: {}, bid_size: {}, \
             ask_size: {}, tick_attrib_last: {}",
            req_id, time, bid_price, ask_price, bid_size, ask_size, tick_attrib_bid_ask
        );
    }

    fn tick_by_tick_mid_point(&self, req_id: i32, time: i32, mid_point: f64) {
        info!(
            "tick_by_tick_mid_point -- req_id: {}, time: {}, mid_point: {}",
            req_id, time, mid_point
        );
    }

    fn order_bound(&self, req_id: i32, api_client_id: i32, api_order_id: i32) {
        info!(
            "order_bound -- req_id: {}, api_client_id: {}, api_order_id: {}",
            req_id, api_client_id, api_order_id
        );
    }

    fn completed_order(&self, contract: Contract, order: Order, order_state: OrderState) {
        info!(
            "completed_order -- contract: [{}], order: [{}], order_state: [{}]",
            contract, order, order_state
        );
    }

    fn completed_orders_end(&self) {
        info!("completed_orders_end -- (no parameters for this message)");
    }
}

unsafe impl Send for DefaultWrapper {}

unsafe impl Sync for DefaultWrapper {}
