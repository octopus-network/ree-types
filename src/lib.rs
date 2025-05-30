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

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;

    #[test]
    fn test_ree_instruction_json() {
        let instruction_set_1 = IntentionSet {
            initiator_address: "bc1q8anrrgczju8zn02ww06slsfh9grm07de7r9e3k".to_string(),
            tx_fee_in_sats: 360,
            intentions: vec![Intention {
                exchange_id: "RICH_SWAP".to_string(),
                action: "add_liquidity".to_string(),
                action_params: String::new(),
                pool_address: "bc1pxtmh763568jd8pz9m8wekt2yrqyntqv2wk465mgpzlr9f2aq2vqs52l0hq"
                    .to_string(),
                nonce: 1,
                pool_utxo_spend: vec![],
                pool_utxo_receive: vec![
                    "4b004c33c5b7bce9a8f5a3a0dab48dd47e33486a8cea5f63ef558849f5604b88:1"
                        .to_string(),
                ],
                input_coins: vec![
                    InputCoin {
                        from: "bc1q8anrrgczju8zn02ww06slsfh9grm07de7r9e3k".to_string(),
                        coin: CoinBalance {
                            id: CoinId::btc(),
                            value: 23_000,
                        },
                    },
                    InputCoin {
                        from: "bc1q8anrrgczju8zn02ww06slsfh9grm07de7r9e3k".to_string(),
                        coin: CoinBalance {
                            id: CoinId::from_str("868703:142").unwrap(),
                            value: 959_000_000,
                        },
                    },
                ],
                output_coins: vec![],
            }],
        };
        println!(
            "Add liquidity sample instruction: {}\n",
            serde_json::to_string(&instruction_set_1).unwrap()
        );
        //
        //
        //
        let instruction_set_2 = IntentionSet {
            initiator_address: "bc1qvwvcttn5dtxleu73uuyh8w759gukjr22l7z503".to_string(),
            tx_fee_in_sats: 330,
            intentions: vec![Intention {
                exchange_id: "RICH_SWAP".to_string(),
                action: "withdraw_liquidity".to_string(),
                action_params: String::new(),
                pool_address: "bc1pu3pv54uxfps00a8ydle67fd3rktz090l07lyg7wadurq4h0lpjhqnet990"
                    .to_string(),
                nonce: 11,
                pool_utxo_spend: vec![
                    "71c9aa9a015e0fcd5cbd6354fbd61c290f9c0a77cecb920df1f0917e7ddc75b7:0"
                        .to_string(),
                ],
                pool_utxo_receive: vec![],
                input_coins: vec![],
                output_coins: vec![
                    OutputCoin {
                        to: "bc1qvwvcttn5dtxleu73uuyh8w759gukjr22l7z503".to_string(),
                        coin: CoinBalance {
                            id: CoinId::btc(),
                            value: 10_124,
                        },
                    },
                    OutputCoin {
                        to: "bc1qvwvcttn5dtxleu73uuyh8w759gukjr22l7z503".to_string(),
                        coin: CoinBalance {
                            id: CoinId::from_str("840106:129").unwrap(),
                            value: 7_072_563,
                        },
                    },
                ],
            }],
        };
        println!(
            "Withdraw liquidity sample instruction: {}\n",
            serde_json::to_string(&instruction_set_2).unwrap()
        );
        //
        //
        //
        let instruction_set_3 = IntentionSet {
            initiator_address: "bc1plvgrpk6mxwyppvqa5j275ujatn8qgs2dcm8m3r2w7sfkn395x6us9l5qdj"
                .to_string(),
            tx_fee_in_sats: 340,
            intentions: vec![Intention {
                exchange_id: "RICH_SWAP".to_string(),
                action: "swap".to_string(),
                action_params: String::new(),
                pool_address: "bc1ptnxf8aal3apeg8r4zysr6k2mhadg833se2dm4nssl7drjlqdh2jqa4tk3p"
                    .to_string(),
                nonce: 5,
                pool_utxo_spend: vec![
                    "17616a9d2258c41bea2175e64ecc2e5fc45ae18be5c9003e058cb0bb85301fd8:0"
                        .to_string(),
                ],
                pool_utxo_receive: vec![
                    "0cec5e1ac7688744dc7af59e8e3cd7be794b0f6dfec9357181759dc4c9c9e541:0"
                        .to_string(),
                ],
                input_coins: vec![InputCoin {
                    from: "bc1plvgrpk6mxwyppvqa5j275ujatn8qgs2dcm8m3r2w7sfkn395x6us9l5qdj"
                        .to_string(),
                    coin: CoinBalance {
                        id: CoinId::from_str("840000:846").unwrap(),
                        value: 10_000_000,
                    },
                }],
                output_coins: vec![OutputCoin {
                    to: "bc1plvgrpk6mxwyppvqa5j275ujatn8qgs2dcm8m3r2w7sfkn395x6us9l5qdj"
                        .to_string(),
                    coin: CoinBalance {
                        id: CoinId::btc(),
                        value: 25_523,
                    },
                }],
            }],
        };
        println!(
            "Runes swap btc sample instruction: {}\n",
            serde_json::to_string(&instruction_set_3).unwrap()
        );
        //
        //
        //
        let instruction_set_4 = IntentionSet {
            initiator_address: "bc1plvgrpk6mxwyppvqa5j275ujatn8qgs2dcm8m3r2w7sfkn395x6us9l5qdj"
                .to_string(),
            tx_fee_in_sats: 410,
            intentions: vec![
                Intention {
                    exchange_id: "RICH_SWAP".to_string(),
                    action: "swap".to_string(),
                    action_params: String::new(),
                    pool_address: "bc1ptnxf8aal3apeg8r4zysr6k2mhadg833se2dm4nssl7drjlqdh2jqa4tk3p"
                        .to_string(),
                    nonce: 5,
                    pool_utxo_spend: vec![
                        "17616a9d2258c41bea2175e64ecc2e5fc45ae18be5c9003e058cb0bb85301fd8:0"
                            .to_string(),
                    ],
                    pool_utxo_receive: vec![
                        "0cec5e1ac7688744dc7af59e8e3cd7be794b0f6dfec9357181759dc4c9c9e541:0"
                            .to_string(),
                    ],
                    input_coins: vec![InputCoin {
                        from: "bc1plvgrpk6mxwyppvqa5j275ujatn8qgs2dcm8m3r2w7sfkn395x6us9l5qdj"
                            .to_string(),
                        coin: CoinBalance {
                            id: CoinId::from_str("840000:846").unwrap(),
                            value: 10_000_000,
                        },
                    }],
                    output_coins: vec![OutputCoin {
                        to: "bc1pu3pv54uxfps00a8ydle67fd3rktz090l07lyg7wadurq4h0lpjhqnet990"
                            .to_string(),
                        coin: CoinBalance {
                            id: CoinId::btc(),
                            value: 25_523,
                        },
                    }],
                },
                Intention {
                    exchange_id: "RICH_SWAP".to_string(),
                    action: "swap".to_string(),
                    action_params: String::new(),
                    pool_address: "bc1pu3pv54uxfps00a8ydle67fd3rktz090l07lyg7wadurq4h0lpjhqnet990"
                        .to_string(),
                    nonce: 9,
                    pool_utxo_spend: vec![
                        "9c3590a30d7b5d27f264a295aec6ed15c83618c152c89b28b81a460fcbb66514:1"
                            .to_string(),
                    ],
                    pool_utxo_receive: vec![
                        "0cec5e1ac7688744dc7af59e8e3cd7be794b0f6dfec9357181759dc4c9c9e541:2"
                            .to_string(),
                    ],
                    input_coins: vec![InputCoin {
                        from: "bc1pu3pv54uxfps00a8ydle67fd3rktz090l07lyg7wadurq4h0lpjhqnet990"
                            .to_string(),
                        coin: CoinBalance {
                            id: CoinId::btc(),
                            value: 25_523,
                        },
                    }],
                    output_coins: vec![OutputCoin {
                        to: "bc1plvgrpk6mxwyppvqa5j275ujatn8qgs2dcm8m3r2w7sfkn395x6us9l5qdj"
                            .to_string(),
                        coin: CoinBalance {
                            id: CoinId::from_str("840106:129").unwrap(),
                            value: 672_563,
                        },
                    }],
                },
            ],
        };
        println!(
            "Runes swap runes sample instruction: {}\n",
            serde_json::to_string(&instruction_set_4).unwrap()
        );
    }
}
