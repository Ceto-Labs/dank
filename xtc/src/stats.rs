use crate::history::HistoryBuffer;
use ic_kit::candid::{CandidType, Nat};
use ic_kit::{ic, macros::*};
use serde::Deserialize;

#[derive(Deserialize, CandidType, Clone, Default)]
pub struct StatsDataV0 {
    supply: Nat,
    history_events: u64,
    balance: u64,
    // Usage statistics
    transfers_count: u64,
    mints_count: u64,
    burns_count: u64,
    proxy_calls_count: u64,
    canisters_created_count: u64,
}

impl From<StatsDataV0> for StatsData {
    fn from(s: StatsDataV0) -> Self {
        StatsData {
            supply: s.supply,
            fee: Nat::default(),
            history_events: s.history_events,
            balance: s.balance,
            transfers_count: s.transfers_count,
            mints_count: s.mints_count,
            burns_count: s.burns_count,
            proxy_calls_count: s.proxy_calls_count,
            canisters_created_count: s.canisters_created_count,
        }
    }
}

#[derive(Deserialize, CandidType, Clone, Default)]
pub struct StatsData {
    pub supply: Nat,
    pub fee: Nat,
    pub history_events: u64,
    pub balance: u64,
    // Usage statistics
    pub transfers_count: u64,
    pub mints_count: u64,
    pub burns_count: u64,
    pub proxy_calls_count: u64,
    pub canisters_created_count: u64,
}

pub enum CountTarget {
    Transfer,
    Mint,
    Burn,
    ProxyCall,
    CanisterCreated,
}

impl StatsData {
    #[inline]
    pub fn load(data: StatsData) {
        let stats = ic::get_mut::<StatsData>();
        *stats = data;
    }

    #[inline]
    pub fn get() -> StatsData {
        let stats = ic::get_mut::<StatsData>();
        stats.history_events = ic::get::<HistoryBuffer>().len() as u64;
        stats.balance = ic::balance();
        stats.clone()
    }

    #[inline]
    pub fn increment(target: CountTarget) {
        let stats = ic::get_mut::<StatsData>();
        match target {
            CountTarget::Transfer => stats.transfers_count += 1,
            CountTarget::Mint => stats.mints_count += 1,
            CountTarget::Burn => stats.burns_count += 1,
            CountTarget::ProxyCall => stats.proxy_calls_count += 1,
            CountTarget::CanisterCreated => stats.canisters_created_count += 1,
        }
    }

    #[inline]
    pub fn deposit(amount: u64) {
        let stats = ic::get_mut::<StatsData>();
        stats.supply += amount;
    }

    #[inline]
    pub fn withdraw(amount: u64) {
        let stats = ic::get_mut::<StatsData>();
        stats.supply -= amount;
    }

    #[inline]
    pub fn capture_fee(amount: u64) {
        let stats = ic::get_mut::<StatsData>();
        stats.fee += amount;
    }
}

#[query]
fn stats() -> StatsData {
    StatsData::get()
}
