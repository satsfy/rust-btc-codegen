// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - blockchain.
//! Types for methods found under the `== Blockchain ==` section.
//! Auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// 
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// 
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxoutSet {
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: i64,
    /// the number of coins written in the snapshot
    pub coins_written: i64,
    /// the number of transactions in the chain up to and including the base block
    pub nchaintx: i64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: String,
}

/// Result of the JSON-RPC method `getbestblockhash`.
///
/// > getbestblockhash
/// >
/// > Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBestBlockHash(pub String);

/// Result of the JSON-RPC method `getblockcount`.
///
/// > getblockcount
/// >
/// > Returns the height of the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockCount(pub i64);

/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFilter {
    /// the hex-encoded filter data
    pub filter: String,
    /// the hex-encoded filter header
    pub header: String,
}

/// Attempt to fetch block from a given peer.
/// 
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
/// 
/// Returns an empty JSON object if the request was successfully scheduled.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFromPeer {

}

/// Result of the JSON-RPC method `getblockhash`.
///
/// > getblockhash
/// >
/// > Returns hash of block in best-block-chain at height provided.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHash(pub String);

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderVerboseOne {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the current chain
    pub chainwork: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: i64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: i64,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: i64,
    /// The merkle root
    pub merkleroot: String,
    /// The number of transactions in the block
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: i64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    /// The block version
    pub version: i64,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
}

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderVerboseZero(pub String);

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseOne {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    pub chainwork: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: i64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: i64,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: i64,
    /// The merkle root
    pub merkleroot: String,
    /// The number of transactions in the block
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: i64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The block size
    pub size: i64,
    /// The block size excluding witness data
    #[serde(rename = "strippedsize")]
    pub stripped_size: i64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    /// The transaction ids
    pub tx: Vec<String>,
    /// The block version
    pub version: i64,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The block weight as defined in BIP 141
    pub weight: i64,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThree {
    pub tx: Vec<GetBlockVerboseThreeTxItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItem {
    pub vin: Vec<GetBlockVerboseThreeTxItemVinItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItemVinItem {
    /// (Only if undo information is available)
    #[serde(rename = "prevout")]
    pub prev_out: GetBlockVerboseThreeTxItemVinItemPrevout,
}

/// (Only if undo information is available)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItemVinItemPrevout {
    /// Coinbase or not
    pub generated: bool,
    /// The height of the prevout
    pub height: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseTwo {
    pub tx: Vec<GetBlockVerboseTwoTxItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseTwoTxItem {
    /// The transaction fee in BTC, omitted if block undo data is not available
    pub fee: i64,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseZero(pub String);

/// Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockchainInfo {
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    pub automatic_pruning: Option<bool>,
    /// the hash of the currently best block
    #[serde(rename = "bestblockhash")]
    pub bestblock_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: i64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// total amount of work in active chain, in hexadecimal
    pub chainwork: String,
    /// the current difficulty
    pub difficulty: i64,
    /// the current number of headers we have validated
    pub headers: i64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: i64,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    pub prune_target_size: Option<i64>,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// height of the last block pruned, plus one (only present if pruning is enabled)
    pub pruneheight: Option<i64>,
    /// the block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    pub signet_challenge: Option<String>,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: i64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    /// estimate of verification progress [0..1]
    #[serde(rename = "verificationprogress")]
    pub verification_progress: i64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockstats {
    /// Average fee in the block
    pub avgfee: Option<i64>,
    /// Average feerate (in satoshis per virtual byte)
    pub avgfeerate: Option<i64>,
    /// Average transaction size
    pub avgtxsize: Option<i64>,
    /// The block hash (to check for potential reorgs)
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    pub feerate_percentiles: Option<Vec<serde_json::Value>>,
    /// The height of the block
    pub height: Option<i64>,
    /// The number of inputs (excluding coinbase)
    pub ins: Option<i64>,
    /// Maximum fee in the block
    #[serde(rename = "maxfee")]
    pub max_fee: Option<i64>,
    /// Maximum feerate (in satoshis per virtual byte)
    #[serde(rename = "maxfeerate")]
    pub max_feerate: Option<i64>,
    /// Maximum transaction size
    #[serde(rename = "maxtxsize")]
    pub max_txs_ize: Option<i64>,
    /// Truncated median fee in the block
    pub medianfee: Option<i64>,
    /// The block median time past
    pub mediantime: Option<i64>,
    /// Truncated median transaction size
    pub mediantxsize: Option<i64>,
    /// Minimum fee in the block
    #[serde(rename = "minfee")]
    pub min_fee: Option<i64>,
    /// Minimum feerate (in satoshis per virtual byte)
    #[serde(rename = "minfeerate")]
    pub min_feerate: Option<i64>,
    /// Minimum transaction size
    #[serde(rename = "mintxsize")]
    pub min_txs_ize: Option<i64>,
    /// The number of outputs
    #[serde(rename = "outs")]
    pub out_s: Option<i64>,
    /// The block subsidy
    pub subsidy: Option<i64>,
    /// Total size of all segwit transactions
    pub swtotal_size: Option<i64>,
    /// Total weight of all segwit transactions
    pub swtotal_weight: Option<i64>,
    /// The number of segwit transactions
    pub swtxs: Option<i64>,
    /// The block time
    pub time: Option<i64>,
    /// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee])
    pub total_out: Option<i64>,
    /// Total size of all non-coinbase transactions
    pub total_size: Option<i64>,
    /// Total weight of all non-coinbase transactions
    pub total_weight: Option<i64>,
    /// The fee total
    #[serde(rename = "totalfee")]
    pub total_fee: Option<i64>,
    /// The number of transactions (including coinbase)
    pub txs: Option<i64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    pub utxo_increase: Option<i64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    pub utxo_increase_actual: Option<i64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    pub utxo_size_inc: Option<i64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    pub utxo_size_inc_actual: Option<i64>,
}

/// Result of the JSON-RPC method `getchaintips`.
///
/// > getchaintips
/// >
/// > Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTips(pub Vec<GetChainTipsItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTipsItem {
    /// zero for main chain, otherwise length of branch connecting the tip to the main chain
    pub branchlen: i64,
    /// block hash of the tip
    pub hash: String,
    /// height of the chain tip
    pub height: i64,
    /// status of the chain, "active" for the main chain
/// Possible values for status:
/// 1.  "invalid"               This branch contains at least one invalid block
/// 2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
/// 3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
/// 4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
/// 5.  "active"                This is the tip of the active main chain, which is certainly valid
    pub status: String,
}

/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTxstats {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: i64,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    #[serde(rename = "txcount")]
    pub tx_count: Option<i64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0 and if window_tx_count exists.
    #[serde(rename = "txrate")]
    pub tx_rate: Option<i64>,
    /// Size of the window in number of blocks
    pub window_block_count: i64,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: i64,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0
    pub window_interval: Option<i64>,
    /// The number of transactions in the window. Only returned if "window_block_count" is > 0 and if txcount exists for the start and end of the window.
    pub window_tx_count: Option<i64>,
}

/// Return information about chainstates.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainstates {
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: Vec<GetChainstatesChainstatesItem>,
    /// the number of headers seen so far
    pub headers: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainstatesChainstatesItem {
    /// blockhash of the tip
    #[serde(rename = "bestblockhash")]
    pub bestblock_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// number of blocks in this chainstate
    pub blocks: i64,
    /// size of the coinsdb cache
    pub coins_db_cache_bytes: i64,
    /// size of the coinstip cache
    pub coins_tip_cache_bytes: i64,
    /// difficulty of the tip
    pub difficulty: i64,
    /// the base block of the snapshot this chainstate is based on, if any
    pub snapshot_blockhash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// whether the chainstate is fully validated. True if all blocks in the chainstate were validated, false if the chain is based on a snapshot and the snapshot has not yet been validated.
    pub validated: bool,
    /// progress towards the network tip
    #[serde(rename = "verificationprogress")]
    pub verification_progress: i64,
}

/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfo {
    pub deployments: std::collections::BTreeMap<String, GetDeploymentInfoDeployments>,
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: i64,
    /// script verify flags for the block
    pub script_flags: Vec<String>,
}

/// name of the deployment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeployments {
    /// true if the rules are enforced for the mempool and the next block
    pub active: bool,
    /// status of bip9 softforks (only for "bip9" type)
    pub bip9: Option<GetDeploymentInfoDeploymentsBip9>,
    /// height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
    pub height: Option<i64>,
    /// one of "buried", "bip9"
    #[serde(rename = "type")]
    pub type_: String,
}

/// status of bip9 softforks (only for "bip9" type)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeploymentsBip9 {
    /// the bit (0-28) in the block version field used to signal this softfork (only for "started" and "locked_in" status)
    pub bit: Option<i64>,
    /// minimum height of blocks for which the rules may be enforced
    pub min_activation_height: i64,
    /// indicates blocks that signalled with a # and blocks that did not with a -
    pub signalling: Option<String>,
    /// height of the first block to which the status applies
    pub since: i64,
    /// the minimum median time past of a block at which the bit gains its meaning
    pub start_time: i64,
    /// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
    pub statistics: Option<GetDeploymentInfoDeploymentsBip9Statistics>,
    /// status of deployment at specified block (one of "defined", "started", "locked_in", "active", "failed")
    pub status: String,
    /// status of deployment at the next block
    pub status_next: String,
    /// the median time past of a block at which the deployment is considered failed if not yet locked in
    #[serde(rename = "timeout")]
    pub time_out: i64,
}

/// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeploymentsBip9Statistics {
    /// the number of blocks with the version bit set in the current period
    pub count: i64,
    /// the number of blocks elapsed since the beginning of the current period
    pub elapsed: i64,
    /// the length in blocks of the signalling period
    pub period: i64,
    /// returns false if there are not enough blocks left in this period to pass activation threshold (only for "started" status)
    pub possible: Option<bool>,
    /// the number of blocks with the version bit set required to activate the feature (only for "started" status)
    pub threshold: Option<i64>,
}

/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivity {
    /// events
    pub activity: Vec<serde_json::Value>,
}

/// Result of the JSON-RPC method `getdifficulty`.
///
/// > getdifficulty
/// >
/// > Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDifficulty(pub i64);

/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerboseOne(
    /// Map entries
    pub std::collections::BTreeMap<String, GetMempoolAncestorsVerboseOneEntry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerboseOneEntry {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestors_ize: i64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
/// 
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    pub chunkweight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetMempoolAncestorsVerboseOneEntryFees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerboseOneEntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of chunk, denominated in BTC
    pub chunk: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerboseZero(pub Vec<String>);

/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerboseOne(
    /// Map entries
    pub std::collections::BTreeMap<String, GetMempoolDescendantsVerboseOneEntry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerboseOneEntry {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestors_ize: i64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
/// 
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    pub chunkweight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetMempoolDescendantsVerboseOneEntryFees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerboseOneEntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of chunk, denominated in BTC
    pub chunk: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerboseZero(pub Vec<String>);

/// Returns mempool data for given transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntry {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestors_ize: i64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
/// 
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    pub chunkweight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetMempoolEntryFees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of chunk, denominated in BTC
    pub chunk: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfo {
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: i64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
    #[serde(rename = "fullrbf")]
    pub full_rbf: bool,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: i64,
    /// Maximum number of transactions that can be in a cluster (configured by -limitclustercount)
    #[serde(rename = "limitclustercount")]
    pub limit_clustercount: i64,
    /// Maximum size of a cluster in virtual bytes (configured by -limitclustersize)
    #[serde(rename = "limitclustersize")]
    pub limit_clustersize: i64,
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool
    #[serde(rename = "maxdatacarriersize")]
    pub max_data_carrier_size: i64,
    /// Maximum memory usage for the mempool
    #[serde(rename = "maxmempool")]
    pub max_mempool: i64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// True if the mempool accepts transactions with bare multisig outputs
    #[serde(rename = "permitbaremultisig")]
    pub permit_bare_multisig: bool,
    /// Current tx count
    pub size: i64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: i64,
    /// Total memory usage for the mempool
    pub usage: i64,
}

/// Returns mempool data for given cluster
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolcluster {
    /// chunks in this cluster (in mining order)
    pub chunks: Vec<GetMempoolclusterChunksItem>,
    /// total sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop')
    pub clusterweight: i64,
    /// number of transactions
    #[serde(rename = "txcount")]
    pub tx_count: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolclusterChunksItem {
    /// fees of the transactions in this chunk
    pub chunkfee: i64,
    /// sigops-adjusted weight of all transactions in this chunk
    pub chunkweight: i64,
    /// transactions in this chunk in mining order
    pub txs: Vec<String>,
}

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// 
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerboseOne(
    /// Map entries
    pub std::collections::BTreeMap<String, GetRawMempoolVerboseOneEntry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerboseOneEntry {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestors_ize: i64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
/// 
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    pub chunkweight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetRawMempoolVerboseOneEntryFees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerboseOneEntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of chunk, denominated in BTC
    pub chunk: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// 
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerboseTwo {
    /// The mempool sequence value.
    pub mempool_sequence: i64,
    pub txids: Vec<String>,
}

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// 
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerboseZero(pub Vec<String>);

/// Result of the JSON-RPC method `gettxoutproof`.
///
/// > gettxoutproof
/// >
/// > Returns a hex-encoded proof that "txid" was included in a block.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutProof(pub String);

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutSetInfo {
    /// The hash of the block at which these statistics are calculated
    pub bestblock: String,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    pub block_info: Option<GetTxoutSetInfoBlockInfo>,
    /// Database-independent, meaningless metric indicating the UTXO set size
    pub bogosize: i64,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    pub disk_size: Option<i64>,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    pub hash_serialized_3: Option<String>,
    /// The block height (index) of the returned statistics
    pub height: i64,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    pub muhash: Option<String>,
    /// The total amount of coins in the UTXO set
    pub total_amount: f64,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    pub total_unspendable_amount: Option<f64>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    pub transactions: Option<i64>,
    /// The number of unspent transaction outputs
    #[serde(rename = "txouts")]
    pub txout_s: i64,
}

/// Info on amounts in the block at this block height (only available if coinstatsindex is used)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutSetInfoBlockInfo {
    /// Coinbase subsidy amount of this block
    #[serde(rename = "coinbase")]
    pub coin_base: f64,
    /// Total amount of new outputs created by this block
    pub new_outputs_ex_coinbase: f64,
    /// Total amount of all prevouts spent in this block
    pub prevout_spent: f64,
    /// Total amount of unspendable outputs created in this block
    pub unspendable: f64,
    /// Detailed view of the unspendable categories
    pub unspendables: GetTxoutSetInfoBlockInfoUnspendables,
}

/// Detailed view of the unspendable categories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutSetInfoBlockInfoUnspendables {
    /// Transactions overridden by duplicates (no longer possible with BIP30)
    pub bip30: f64,
    /// The unspendable amount of the Genesis block subsidy
    pub genesis_block: f64,
    /// Amounts sent to scripts that are unspendable (for example OP_RETURN outputs)
    pub scripts: f64,
    /// Fee rewards that miners did not claim in their coinbase transaction
    pub unclaimed_rewards: f64,
}

/// Returns details about an unspent transaction output.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutVerboseOne {
    /// The hash of the block at the tip of the chain
    pub bestblock: String,
    /// Coinbase or not
    #[serde(rename = "coinbase")]
    pub coin_base: bool,
    /// The number of confirmations
    pub confirmations: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetTxoutVerboseOneScriptPubKey,
    /// The transaction value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutVerboseOneScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type, eg pubkeyhash
    #[serde(rename = "type")]
    pub type_: String,
}

/// Result of the JSON-RPC method `gettxspendingprevout`.
///
/// > gettxspendingprevout
/// >
/// > Scans the mempool to find transactions spending any of the given outputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxspendingprevOut(pub Vec<GetTxspendingprevOutItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxspendingprevOutItem {
    /// the transaction id of the mempool transaction spending this output (omitted if unspent)
    #[serde(rename = "spendingtxid")]
    pub spending_txid: Option<String>,
    /// the transaction id of the checked output
    pub txid: String,
    /// the vout value of the checked output
    pub vout: i64,
}

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportMempool {

}

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// 
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// 
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxoutSet {
    /// the height of the base of the snapshot
    pub base_height: i64,
    /// the number of coins loaded from the snapshot
    pub coins_loaded: i64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
}

/// Result of the JSON-RPC method `pruneblockchain`.
///
/// > pruneblockchain
/// >
/// > Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PruneBlockchain(pub i64);

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SaveMempool {
    /// the directory and file where the mempool was saved
    pub filename: String,
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksVerboseOne {
    /// true if the scan process was not aborted
    pub completed: bool,
    /// The height we started the scan from
    pub from_height: i64,
    /// Blocks that may have matched a scanobject.
    pub relevant_blocks: Vec<String>,
    /// The height we ended the scan at
    pub to_height: i64,
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksVerboseThree(pub bool);

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksVerboseTwo {
    /// Height of the block currently being scanned
    pub current_height: i64,
    /// Approximate percent complete
    pub progress: i64,
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(<pubkey>)                        P2PKH outputs for the given pubkey
///     sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(<pubkey>)                         P2TR
///     tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxoutSetVerboseOne(pub bool);

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(<pubkey>)                        P2PKH outputs for the given pubkey
///     sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(<pubkey>)                         P2TR
///     tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxoutSetVerboseTwo {
    /// Approximate percent complete
    pub progress: i64,
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(<pubkey>)                        P2PKH outputs for the given pubkey
///     sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(<pubkey>)                         P2TR
///     tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxoutSetVerboseZero {
    /// The hash of the block at the tip of the chain
    pub bestblock: String,
    /// The block height at which the scan was done
    pub height: i64,
    /// Whether the scan was completed
    pub success: bool,
    /// The total amount of all found unspent outputs in BTC
    pub total_amount: f64,
    /// The number of unspent transaction outputs scanned
    #[serde(rename = "txouts")]
    pub txout_s: i64,
    #[serde(rename = "unspents")]
    pub unspent_s: Vec<ScanTxoutSetVerboseZeroUnspentsItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxoutSetVerboseZeroUnspentsItem {
    /// The total amount in BTC of the unspent output
    pub amount: f64,
    /// Blockhash of the unspent transaction output
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// Whether this is a coinbase output
    #[serde(rename = "coinbase")]
    pub coin_base: bool,
    /// Number of confirmations of the unspent transaction output when the scan was done
    pub confirmations: i64,
    /// A specialized descriptor for the matched output script
    pub desc: String,
    /// Height of the unspent transaction output
    pub height: i64,
    /// The output script
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    /// The transaction id
    pub txid: String,
    /// The vout value
    pub vout: i64,
}

/// Result of the JSON-RPC method `verifychain`.
///
/// > verifychain
/// >
/// > Verifies blockchain database.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyChain(pub bool);

/// Result of the JSON-RPC method `verifytxoutproof`.
///
/// > verifytxoutproof
/// >
/// > Verifies that a proof points to a transaction in a block, returning the transaction it commits to
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyTxoutProof(pub Vec<String>);

/// Waits for a specific new block and returns useful info about it.
/// 
/// Returns the current block on timeout or exit.
/// 
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlock {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
/// 
/// Returns the current block on timeout or exit.
/// 
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockHeight {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}

/// Waits for any new block and returns useful info about it.
/// 
/// Returns the current block on timeout or exit.
/// 
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForNewBlock {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}

