extern crate alloc;

use alloc::str::FromStr;
use candid::CandidType;
use serde::{Deserialize, Serialize};

mod coin_id;
pub mod exchange_interfaces;
mod intention;
pub mod orchestrator_interfaces;
pub mod psbt;
mod pubkey;
pub mod schnorr;
mod txid;

pub use bitcoin;
pub use coin_id::CoinId;
pub use intention::*;
pub use pubkey::Pubkey;
pub use txid::{TxRecord, Txid};

#[derive(
    CandidType, Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct CoinBalance {
    pub id: CoinId,
    pub value: u128,
}

#[derive(CandidType, Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Utxo {
    pub txid: Txid,
    pub vout: u32,
    pub maybe_rune: Option<CoinBalance>,
    pub sats: u64,
}

impl Utxo {
    pub fn try_from(
        outpoint: impl AsRef<str>,
        maybe_rune: Option<CoinBalance>,
        sats: u64,
    ) -> Result<Self, String> {
        let parts = outpoint.as_ref().split(':').collect::<Vec<_>>();
        let txid = parts
            .get(0)
            .map(|s| Txid::from_str(s).map_err(|_| "Invalid txid in outpoint."))
            .transpose()?
            .ok_or("Invalid txid in outpoint.")?;
        let vout = parts
            .get(1)
            .map(|s| s.parse::<u32>().map_err(|_| "Invalid vout in outpoint."))
            .transpose()?
            .ok_or("Invalid vout in outpoint")?;
        Ok(Utxo {
            txid,
            vout,
            maybe_rune,
            sats,
        })
    }

    pub fn outpoint(&self) -> String {
        format!("{}:{}", self.txid, self.vout)
    }

    pub fn rune_amount(&self) -> u128 {
        self.maybe_rune.map(|r| r.value).unwrap_or_default()
    }
}
