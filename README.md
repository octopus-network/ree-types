# REE Types

This repository contains the essential data type definitions for REE (Runes Exchange Environment).

## Exchange Interfaces

In REE, every exchange must implement the following six functions:

| Function Name      | Parameters               | Return Type           | Description |
|-------------------|------------------------|----------------------|-------------|
| `get_pool_list`   | -       | `Vec<PoolInfo>`  | See [Get Pool List](#get-pool-list). |
| `get_pool_info`   | `GetPoolInfoArgs`       | `Option<PoolInfo>`   | See [Get Pool Info](#get-pool-info). |
| `get_minimal_tx_value` | `GetMinimalTxValueArgs` | `u64` | See [Get Minimal Tx Value](#get-minimal-tx-value). |
| `execute_tx`      | `ExecuteTxArgs`         | `Result<String, String>` | See [Execute Tx](#execute-tx). |
| `unconfirm_txs`     | `UnconfirmTxsArgs`        | `Result<(), String>`  | See [Unconfirm Txs](#unconfirm-txs). |
| `rollback_tx`     | `RollbackTxArgs`        | `Result<(), String>`  | See [Rollback Tx](#rollback-tx). |
| `new_block`     | `NewBlockArgs`        | `Result<(), String>`  | See [New Block](#new-block). |

Implementation Notes:

- The REE Orchestrator calls these functions to interact with exchanges **WITHOUT** attaching any cycles.
- Every exchange **MUST** implement these functions **exactly as defined** in this repository. Failure to do so will prevent the exchange from being registered in the REE Orchestrator, or may cause a registered exchange to be halted.
- These functions may be implemented as `async` or synchronous.
- The `get_pool_list`, `get_pool_info` and `get_minimal_tx_value` may be declared with `#[ic_cdk::query]` or `#[ic_cdk::update]` in the exchange canister. The other functions **MUST** be declared with `#[ic_cdk::update]`.
- All parameters and return types are defined in the `ree_types::exchange_interfaces` module.

### Get Pool List

Returns all of pools' basic information maintained by the exchange.

Return Type: `Vec<PoolBasic>`, where `PoolBasic` is defined as:

```rust
pub struct PoolBasic {
    pub name: String,
    pub address: String,
}
```

### Get Pool Info

Returns detailed information about a specified pool.

Parameters:

```rust
pub struct GetPoolInfoArgs {
    pub pool_address: String,
}
```

Return Type: `Option<PoolInfo>`, where `PoolInfo` is defined as:

```rust
pub struct PoolInfo {
    pub key: Pubkey,
    pub key_derivation_path: Vec<Vec<u8>>,
    pub name: String,
    pub address: String,
    pub nonce: u64,
    pub coin_reserved: Vec<CoinBalance>,
    pub btc_reserved: u64,
    pub utxos: Vec<Utxo>,
    pub attributes: String,
}
```

### Get Minimal Tx Value

Returns the minimum transaction value that can be accepted by the exchange, considering the zero-confirmation transaction queue length for a specific pool.

Parameters:

```rust
pub struct GetMinimalTxValueArgs {
    pub pool_address: String,
    pub zero_confirmed_tx_queue_length: u32,
}
```

Return Type: `u64`, the minimal transaction value in `sats`.

### Execute Tx

Executes a transaction in the exchange.

Parameters:

```rust
pub struct ExecuteTxArgs {
    pub psbt_hex: String,
    pub txid: Txid,
    pub intention_set: IntentionSet,
    pub intention_index: u32,
    pub zero_confirmed_tx_queue_length: u32,
}
```

Return Type:

- `Ok(String)`: The signed PSBT data in hex format. The exchange can add corresponding signature(s) to the PSBT data or not, but a valid PSBT data with the same `txid` with the given `psbt_hex` **MUST** be returned.
- `Err(String)`: An error message if execution fails.

### Unconfirm Txs

Unconfirm previously confirmed transaction(s) in the exchange. (This may caused by a reorg of the Bitcoin blockchain.)

Parameters:

```rust
pub struct UnconfirmTxsArgs {
    pub txids: Vec<Txid>,
}
```

Return Type:

- `Ok(())`: On success.
- `Err(String)`: If an error occurs.

### Rollback Tx

Rolls back a transaction in the exchange. **All transactions following the given transaction should also be considered canceled.**

Parameters:

```rust
pub struct RollbackTxArgs {
    pub txid: Txid,
}
```

Return Type:

- `Ok(())`: On success.
- `Err(String)`: If an error occurs.

### New Block

Notifies the exchange of a new block. The `confirmed_txids` are an array of txid which are executed by the exchange previously, these txids are included in the given block. The exchange can use this information to update its internal state.

Parameters:

```rust
pub struct NewBlockArgs {
    pub block_height: u32,
    pub block_hash: String,
    pub block_timestamp: u64,
    pub confirmed_txids: Vec<Txid>,
}
```

Return Type:

- `Ok(())`: On success.
- `Err(String)`: If an error occurs.

## REE Invoke

The `invoke` function in the REE Orchestrator serves as the main entry point for the REE protocol. This function takes `InvokeArgs` as a parameter, which includes the following fields:

```rust
pub struct InvokeArgs {
    pub psbt_hex: String,
    pub intention_set: IntentionSet,
}
```

Where `IntentionSet` is defined as:

```rust
pub struct IntentionSet {
    pub initiator_address: String,
    pub tx_fee_in_sats: u64,
    pub intentions: Vec<Intention>,
}

pub struct Intention {
    pub exchange_id: String,
    pub action: String,
    pub action_params: String,
    pub pool_address: String,
    pub nonce: u64,
    pub pool_utxo_spend: Vec<String>,
    pub pool_utxo_receive: Vec<String>,
    pub input_coins: Vec<InputCoin>,
    pub output_coins: Vec<OutputCoin>,
}

pub struct InputCoin {
    // The address of the owner of the coins
    pub from: String,
    pub coin: CoinBalance,
}

pub struct OutputCoin {
    // The address of the receiver of the coins
    pub to: String,
    pub coin: CoinBalance,
}
```

The `invoke` function returns a `Result<String, String>`, where:

- The `Ok` value is the `txid` of the final Bitcoin transaction, which will be formed and broadcasted.
- The `Err` value is an error message if the execution of `invoke` fails.

The `invoke` function will call the `execute_tx` function of the exchange canister(s) based on the provided `IntentionSet`. If all intentions are successfully executed, the function broadcasts the final Bitcoin transaction and returns the `txid`.

Before invoking the exchange canisters, the Orchestrator performs necessary validations on the `IntentionSet` to ensure it aligns with the provided PSBT data.

### Intention Details

Each `IntentionSet` can contain multiple `Intention` objects, reflecting the user's intentions. The `Intention` struct consists of the following fields:

- `exchange_id`: The identifier of a registered exchange responsible for executing the intention. The Orchestrator will validate this field.
- `action`: The specific action to be executed by the exchange. The Orchestrator will **NOT** validate this field.
- `action_params`: Parameters for the action, specific to the exchange. The Orchestrator will **NOT** validate this field.
- `pool_address`: The address of the exchange pool where the intention will be executed. The Orchestrator will validate this field.
- `nonce`: A nonce representing the pool state in the exchange. The Orchestrator will **NOT** validate this field.
- `pool_utxo_spend`: The UTXO(s) owned by the pool that will be spent in the intention.
- `pool_utxo_receive`: The UTXO(s) that the pool will receive as part of the intention. These UTXOs should correspond to the outputs of the final Bitcoin transaction.
- `input_coins`: The coins that will be spent in the intention. These should either come from the inputs of the final Bitcoin transaction or from previously generated `output_coins`.
- `output_coins`: The coins that will be received in the intention. These should appear as outputs in the final Bitcoin transaction.
