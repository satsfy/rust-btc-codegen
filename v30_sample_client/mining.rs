// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - mining.
//! Types for methods found under the `== Mining ==` section.
//! Auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
///     https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
///     https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlocktemplateVerboseOne(pub String);

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
///     https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
///     https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlocktemplateVerboseTwo {
    /// compressed target of next block
    pub bits: String,
    pub capabilities: Vec<String>,
    /// data that should be included in the coinbase's scriptSig content
    #[serde(rename = "coinbaseaux")]
    pub coin_baseaux: std::collections::BTreeMap<String, String>,
    /// maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis)
    #[serde(rename = "coinbasevalue")]
    pub coin_basevalue: i64,
    /// current timestamp in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    pub curtime: i64,
    /// a valid witness commitment for the unmodified block template
    pub default_witness_commitment: Option<String>,
    /// The height of the next block
    pub height: i64,
    /// an id to include with a request to longpoll on an update to this template
    pub longpollid: String,
    /// The minimum timestamp appropriate for the next block time, expressed in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    #[serde(rename = "mintime")]
    pub min_time: i64,
    /// list of ways the block template may be changed
    pub mutable: Vec<String>,
    /// A range of valid nonces
    #[serde(rename = "noncerange")]
    pub nonce_range: String,
    /// The hash of current highest block
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: String,
    /// specific block rules that are to be enforced
    pub rules: Vec<String>,
    /// Only on signet
    pub signet_challenge: Option<String>,
    /// limit of sigops in blocks
    pub sigoplimit: i64,
    /// limit of block size
    #[serde(rename = "sizelimit")]
    pub size_limit: i64,
    /// The hash target
    pub target: String,
    /// contents of non-coinbase transactions that should be included in the next block
    pub transactions: Vec<GetBlocktemplateVerboseTwoTransactionsItem>,
    /// set of pending, supported versionbit (BIP 9) softfork deployments
    pub vbavailable: std::collections::BTreeMap<String, i64>,
    /// bit mask of versionbits the server requires set in submissions
    pub vbrequired: i64,
    /// The preferred block version
    pub version: i64,
    /// limit of block weight
    #[serde(rename = "weightlimit")]
    pub weight_limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlocktemplateVerboseTwoTransactionsItem {
    /// transaction data encoded in hexadecimal (byte-for-byte)
    pub data: String,
    /// array of numbers
    pub depends: Vec<i64>,
    /// difference in value between transaction inputs and outputs (in satoshis); for coinbase transactions, this is a negative Number of the total collected block fees (ie, not including the block subsidy); if key is not present, fee is unknown and clients MUST NOT assume there isn't one
    pub fee: i64,
    /// transaction hash including witness data, shown in byte-reversed hex
    pub hash: String,
    /// total SigOps cost, as counted for purposes of block limits; if key is not present, sigop cost is unknown and clients MUST NOT assume it is zero
    pub sigops: i64,
    /// transaction hash excluding witness data, shown in byte-reversed hex
    pub txid: String,
    /// total transaction weight, as counted for purposes of block limits
    pub weight: i64,
}

/// Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfo {
    /// The current nBits, compact representation of the block difficulty target
    pub bits: String,
    /// Minimum feerate of packages selected for block inclusion in BTC/kvB
    #[serde(rename = "blockmintxfee")]
    pub block_min_tx_fee: f64,
    /// The current block
    pub blocks: i64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// The number of block transactions (excluding coinbase) of the last assembled block (only present if a block was ever assembled)
    #[serde(rename = "currentblocktx")]
    pub current_block_tx: Option<i64>,
    /// The block weight (including reserved weight for block header, txs count and coinbase tx) of the last assembled block (only present if a block was ever assembled)
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<i64>,
    /// The current difficulty
    pub difficulty: i64,
    /// The network hashes per second
    #[serde(rename = "networkhashps")]
    pub network_hashps: i64,
    /// The next block
    pub next: GetMiningInfoNext,
    /// The size of the mempool
    #[serde(rename = "pooledtx")]
    pub pooled_tx: i64,
    /// The block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    pub signet_challenge: Option<String>,
    /// The current target
    pub target: String,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// The next block
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfoNext {
    /// The next target nBits
    pub bits: String,
    /// The next difficulty
    pub difficulty: i64,
    /// The next height
    pub height: i64,
    /// The next target
    pub target: String,
}

/// Result of the JSON-RPC method `getnetworkhashps`.
///
/// > getnetworkhashps
/// >
/// > Returns the estimated network hashes per second based on the last n blocks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkHashps(pub i64);

/// Result of the JSON-RPC method `getprioritisedtransactions`.
///
/// > getprioritisedtransactions
/// >
/// > Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrioritisedTransactions(
    /// prioritisation keyed by txid
    pub std::collections::BTreeMap<String, GetPrioritisedTransactionsEntry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrioritisedTransactionsEntry {
    /// transaction fee delta in satoshis
    pub fee_delta: i64,
    /// whether this transaction is currently in mempool
    pub in_mempool: bool,
    /// modified fee in satoshis. Only returned if in_mempool=true
    pub modified_fee: Option<i64>,
}

/// Result of the JSON-RPC method `prioritisetransaction`.
///
/// > prioritisetransaction
/// >
/// > Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PrioritiseTransaction(pub bool);

/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitBlockVerboseOne(pub String);

