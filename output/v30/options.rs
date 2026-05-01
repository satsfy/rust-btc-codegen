// SPDX-License-Identifier: CC0-1.0

//! Auto-generated options structs for Bitcoin Core `30`.
//!
//! This file is produced by `rust-btc-codegen` and committed to the tree. **Do not edit by hand.**
//! Re-run `just codegen` to regenerate.

#![allow(non_snake_case)]

/// Optional parameters for the [`addnode`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`addnode`]: ../methods/struct.Raw.html#method.add_node_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddNodeOptions {
    /// Attempt to connect using BIP324 v2 transport protocol (ignored for 'remove' command)
    /// Default in Bitcoin Core: `no default`.

    pub v2transport: Option<bool>,
}

/// Optional parameters for the [`addpeeraddress`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`addpeeraddress`]: ../methods/struct.Raw.html#method.add_peer_address_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPeerAddressOptions {
    /// If true, attempt to add the peer to the tried addresses table
    /// Default in Bitcoin Core: `False`.

    pub tried: Option<bool>,
}

/// Optional parameters for the [`bumpfee`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`bumpfee`]: ../methods/struct.Raw.html#method.bump_fee_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BumpFeeOptions {
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`converttopsbt`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`converttopsbt`]: ../methods/struct.Raw.html#method.convert_to_psbt_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToPsbtOptions {
    /// If true, any signatures in the input will be discarded and conversion
    ///                               will continue. If false, RPC will fail if any signatures are present.
    /// Default in Bitcoin Core: `False`.

    #[serde(rename = "permitsigdata")]
    pub permit_sigdata: Option<bool>,
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    /// Default in Bitcoin Core: `no default`.

    pub iswitness: Option<bool>,
}

/// Optional parameters for the [`createmultisig`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`createmultisig`]: ../methods/struct.Raw.html#method.create_multisig_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMultisigOptions {
    /// The address type to use. Options are "legacy", "p2sh-segwit", and "bech32".
    /// Default in Bitcoin Core: `'legacy'`.

    pub address_type: Option<String>,
}

/// Optional parameters for the [`createpsbt`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`createpsbt`]: ../methods/struct.Raw.html#method.create_psbt_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePsbtOptions {
    /// Raw locktime. Non-0 value also locktime-activates inputs
    /// Default in Bitcoin Core: `0`.

    pub locktime: Option<f64>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees. If provided, it is an error if explicit sequence numbers are incompatible.
    /// Default in Bitcoin Core: `True`.

    pub replaceable: Option<bool>,
    /// Transaction version
    /// Default in Bitcoin Core: `2`.

    pub version: Option<f64>,
}

/// Optional parameters for the [`createrawtransaction`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`createrawtransaction`]: ../methods/struct.Raw.html#method.create_raw_transaction_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRawTransactionOptions {
    /// Raw locktime. Non-0 value also locktime-activates inputs
    /// Default in Bitcoin Core: `0`.

    pub locktime: Option<f64>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees. If provided, it is an error if explicit sequence numbers are incompatible.
    /// Default in Bitcoin Core: `True`.

    pub replaceable: Option<bool>,
    /// Transaction version
    /// Default in Bitcoin Core: `2`.

    pub version: Option<f64>,
}

/// Optional parameters for the [`createwallet`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`createwallet`]: ../methods/struct.Raw.html#method.create_wallet_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletOptions {
    /// Disable the possibility of private keys (only watchonlys are possible in this mode).
    /// Default in Bitcoin Core: `False`.

    pub disable_private_keys: Option<bool>,
    /// Create a blank wallet. A blank wallet has no keys.
    /// Default in Bitcoin Core: `False`.

    pub blank: Option<bool>,
    /// Encrypt the wallet with this passphrase.
    /// Default in Bitcoin Core: `no default`.

    pub passphrase: Option<String>,
    /// Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
    /// Default in Bitcoin Core: `False`.

    pub avoid_reuse: Option<bool>,
    /// If set, must be "true"
    /// Default in Bitcoin Core: `True`.

    pub descriptors: Option<bool>,
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    /// Default in Bitcoin Core: `no default`.

    pub load_on_startup: Option<bool>,
    /// Use an external signer such as a hardware wallet. Requires -signer to be configured. Wallet creation will fail if keys cannot be fetched. Requires disable_private_keys and descriptors set to true.
    /// Default in Bitcoin Core: `False`.

    pub external_signer: Option<bool>,
}

/// Optional parameters for the [`createwalletdescriptor`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`createwalletdescriptor`]: ../methods/struct.Raw.html#method.create_wallet_descriptor_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletDescriptorOptions {
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`decoderawtransaction`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`decoderawtransaction`]: ../methods/struct.Raw.html#method.decode_raw_transaction_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeRawTransactionOptions {
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    /// Default in Bitcoin Core: `no default`.

    pub iswitness: Option<bool>,
}

/// Optional parameters for the [`deriveaddresses`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`deriveaddresses`]: ../methods/struct.Raw.html#method.derive_addresses_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeriveAddressesOptions {
    /// If a ranged descriptor is used, this specifies the end or the range (in [begin,end] notation) to derive.
    /// Default in Bitcoin Core: `no default`.

    pub range: Option<serde_json::Value>,
}

/// Optional parameters for the [`descriptorprocesspsbt`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`descriptorprocesspsbt`]: ../methods/struct.Raw.html#method.descriptor_process_psbt_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptorProcessPsbtOptions {
    /// The signature hash type to sign with if not specified by the PSBT. Must be one of
    ///        "DEFAULT"
    ///        "ALL"
    ///        "NONE"
    ///        "SINGLE"
    ///        "ALL|ANYONECANPAY"
    ///        "NONE|ANYONECANPAY"
    ///        "SINGLE|ANYONECANPAY"
    /// Default in Bitcoin Core: `'DEFAULT for Taproot, ALL otherwise'`.

    pub sighashtype: Option<String>,
    /// Include BIP 32 derivation paths for public keys if we know them
    /// Default in Bitcoin Core: `True`.

    pub bip32derivs: Option<bool>,
    /// Also finalize inputs if possible
    /// Default in Bitcoin Core: `True`.

    #[serde(rename = "finalize")]
    pub final_ize: Option<bool>,
}

/// Optional parameters for the [`disconnectnode`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`disconnectnode`]: ../methods/struct.Raw.html#method.disconnect_node_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectNodeOptions {
    /// The IP address/port of the node
    /// Default in Bitcoin Core: `no default`.

    pub address: Option<String>,
    /// The node ID (see getpeerinfo for node IDs)
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "nodeid")]
    pub node_id: Option<f64>,
}

/// Optional parameters for the [`dumptxoutset`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`dumptxoutset`]: ../methods/struct.Raw.html#method.dump_txout_set_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpTxoutSetOptions {
    /// The type of snapshot to create. Can be "latest" to create a snapshot of the current UTXO set or "rollback" to temporarily roll back the state of the node to a historical block before creating the snapshot of a historical UTXO set. This parameter can be omitted if a separate "rollback" named parameter is specified indicating the height or hash of a specific historical block. If "rollback" is specified and separate "rollback" named parameter is not specified, this will roll back to the latest valid snapshot block that can currently be loaded with loadtxoutset.
    /// Default in Bitcoin Core: `''`.

    #[serde(rename = "type")]
    pub type_: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`echo`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`echo`]: ../methods/struct.Raw.html#method.echo_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EchoOptions {
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg0: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg1: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg2: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg3: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg4: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg5: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg6: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg7: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg8: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg9: Option<String>,
}

/// Optional parameters for the [`echojson`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`echojson`]: ../methods/struct.Raw.html#method.echo_json_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EchoJsonOptions {
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg0: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg1: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg2: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg3: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg4: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg5: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg6: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg7: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg8: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub arg9: Option<String>,
}

/// Optional parameters for the [`estimaterawfee`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`estimaterawfee`]: ../methods/struct.Raw.html#method.estimate_raw_fee_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateRawFeeOptions {
    /// The proportion of transactions in a given feerate range that must have been
    /// confirmed within conf_target in order to consider those feerates as high enough and proceed to check
    /// lower buckets.
    /// Default in Bitcoin Core: `0.95`.

    pub threshold: Option<f64>,
}

/// Optional parameters for the [`estimatesmartfee`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`estimatesmartfee`]: ../methods/struct.Raw.html#method.estimate_smart_fee_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateSmartFeeOptions {
    /// The fee estimate mode.
    /// unset, economical, conservative
    /// unset means no mode set (default mode will be used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    /// Default in Bitcoin Core: `'economical'`.

    pub estimate_mode: Option<String>,
}

/// Optional parameters for the [`finalizepsbt`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`finalizepsbt`]: ../methods/struct.Raw.html#method.finalize_psbt_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalizePsbtOptions {
    /// If true and the transaction is complete,
    ///                              extract and return the complete transaction in normal network serialization instead of the PSBT.
    /// Default in Bitcoin Core: `True`.

    pub extract: Option<bool>,
}

/// Optional parameters for the [`fundrawtransaction`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`fundrawtransaction`]: ../methods/struct.Raw.html#method.fund_raw_transaction_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundRawTransactionOptions {
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    /// Default in Bitcoin Core: `no default`.

    pub iswitness: Option<bool>,
}

/// Optional parameters for the [`generateblock`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`generateblock`]: ../methods/struct.Raw.html#method.generate_block_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateBlockOptions {
    /// Whether to submit the block before the RPC call returns or to return it as hex.
    /// Default in Bitcoin Core: `True`.

    pub submit: Option<bool>,
}

/// Optional parameters for the [`generatetoaddress`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`generatetoaddress`]: ../methods/struct.Raw.html#method.generate_to_address_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateToAddressOptions {
    /// How many iterations to try.
    /// Default in Bitcoin Core: `1000000`.

    #[serde(rename = "maxtries")]
    pub max_tries: Option<f64>,
}

/// Optional parameters for the [`generatetodescriptor`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`generatetodescriptor`]: ../methods/struct.Raw.html#method.generate_to_descriptor_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateToDescriptorOptions {
    /// How many iterations to try.
    /// Default in Bitcoin Core: `1000000`.

    #[serde(rename = "maxtries")]
    pub max_tries: Option<f64>,
}

/// Optional parameters for the [`getaddednodeinfo`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getaddednodeinfo`]: ../methods/struct.Raw.html#method.get_added_node_info_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAddedNodeInfoOptions {
    /// If provided, return information about this specific node, otherwise all nodes are returned.
    /// Default in Bitcoin Core: `no default`.

    pub node: Option<String>,
}

/// Optional parameters for the [`getbalance`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getbalance`]: ../methods/struct.Raw.html#method.get_balance_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceOptions {
    /// Remains for backward compatibility. Must be excluded or set to "*".
    /// Default in Bitcoin Core: `no default`.

    pub dummy: Option<String>,
    /// Only include transactions confirmed at least this many times.
    /// Default in Bitcoin Core: `0`.

    #[serde(rename = "minconf")]
    pub min_conf: Option<f64>,
    /// No longer used
    /// Default in Bitcoin Core: `False`.

    pub include_watchonly: Option<bool>,
    /// (only available if avoid_reuse wallet flag is set) Do not include balance in dirty outputs; addresses are considered dirty if they have previously been used in a transaction.
    /// Default in Bitcoin Core: `True`.

    pub avoid_reuse: Option<bool>,
}

/// Optional parameters for the [`getblock`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getblock`]: ../methods/struct.Raw.html#method.get_block_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockOptions {
    /// 0 for hex-encoded data, 1 for a JSON object, 2 for JSON object with transaction data, and 3 for JSON object with transaction data including prevout information for inputs
    /// Default in Bitcoin Core: `1`.

    pub verbosity: Option<f64>,
}

/// Optional parameters for the [`getblockfilter`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getblockfilter`]: ../methods/struct.Raw.html#method.get_block_filter_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockFilterOptions {
    /// The type name of the filter
    /// Default in Bitcoin Core: `'basic'`.

    #[serde(rename = "filtertype")]
    pub filter_type: Option<String>,
}

/// Optional parameters for the [`getblockheader`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getblockheader`]: ../methods/struct.Raw.html#method.get_block_header_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockHeaderOptions {
    /// true for a json object, false for the hex-encoded data
    /// Default in Bitcoin Core: `True`.

    pub verbose: Option<bool>,
}

/// Optional parameters for the [`getblockstats`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getblockstats`]: ../methods/struct.Raw.html#method.get_blockstats_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockstatsOptions {
    /// Values to plot (see result below)
    /// Default in Bitcoin Core: `no default`.

    pub stats: Option<Vec<serde_json::Value>>,
}

/// Optional parameters for the [`getchaintxstats`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getchaintxstats`]: ../methods/struct.Raw.html#method.get_chain_txstats_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChainTxstatsOptions {
    /// Size of the window in number of blocks
    /// Default in Bitcoin Core: `no default`.

    pub nblocks: Option<f64>,
    /// The hash of the block that ends the window.
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
}

/// Optional parameters for the [`getdeploymentinfo`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getdeploymentinfo`]: ../methods/struct.Raw.html#method.get_deployment_info_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeploymentInfoOptions {
    /// The block hash at which to query deployment state
    /// Default in Bitcoin Core: `'hash of current chain tip'`.

    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
}

/// Optional parameters for the [`getdescriptoractivity`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getdescriptoractivity`]: ../methods/struct.Raw.html#method.get_descriptor_activity_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDescriptorActivityOptions {
    /// Whether to include unconfirmed activity
    /// Default in Bitcoin Core: `True`.

    pub include_mempool: Option<bool>,
}

/// Optional parameters for the [`gethdkeys`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`gethdkeys`]: ../methods/struct.Raw.html#method.get_hd_keys_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHdKeysOptions {
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`getindexinfo`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getindexinfo`]: ../methods/struct.Raw.html#method.get_index_info_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexInfoOptions {
    /// Filter results for an index with a specific name.
    /// Default in Bitcoin Core: `no default`.

    pub index_name: Option<String>,
}

/// Optional parameters for the [`getmemoryinfo`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getmemoryinfo`]: ../methods/struct.Raw.html#method.get_memory_info_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMemoryInfoOptions {
    /// determines what kind of information is returned.
    ///   - "stats" returns general statistics about memory usage in the daemon.
    ///   - "mallocinfo" returns an XML string describing low-level heap state (only available if compiled with glibc).
    /// Default in Bitcoin Core: `'stats'`.

    pub mode: Option<String>,
}

/// Optional parameters for the [`getmempoolancestors`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getmempoolancestors`]: ../methods/struct.Raw.html#method.get_mempool_ancestors_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMempoolAncestorsOptions {
    /// True for a json object, false for array of transaction ids
    /// Default in Bitcoin Core: `False`.

    pub verbose: Option<bool>,
}

/// Optional parameters for the [`getmempooldescendants`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getmempooldescendants`]: ../methods/struct.Raw.html#method.get_mempool_descendants_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMempoolDescendantsOptions {
    /// True for a json object, false for array of transaction ids
    /// Default in Bitcoin Core: `False`.

    pub verbose: Option<bool>,
}

/// Optional parameters for the [`getnetworkhashps`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getnetworkhashps`]: ../methods/struct.Raw.html#method.get_network_hashps_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkHashpsOptions {
    /// The number of previous blocks to calculate estimate from, or -1 for blocks since last difficulty change.
    /// Default in Bitcoin Core: `120`.

    pub nblocks: Option<f64>,
    /// To estimate at the time of the given height.
    /// Default in Bitcoin Core: `-1`.

    pub height: Option<f64>,
}

/// Optional parameters for the [`getnewaddress`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getnewaddress`]: ../methods/struct.Raw.html#method.get_new_address_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNewAddressOptions {
    /// The label name for the address to be linked to. It can also be set to the empty string "" to represent the default label. The label does not need to exist, it will be created if there is no label by the given name.
    /// Default in Bitcoin Core: `''`.

    pub label: Option<String>,
    /// The address type to use. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    /// Default in Bitcoin Core: `no default`.

    pub address_type: Option<String>,
}

/// Optional parameters for the [`getnodeaddresses`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getnodeaddresses`]: ../methods/struct.Raw.html#method.get_node_addresses_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeAddressesOptions {
    /// The maximum number of addresses to return. Specify 0 to return all known addresses.
    /// Default in Bitcoin Core: `1`.

    pub count: Option<f64>,
    /// Return only addresses of the specified network. Can be one of: ipv4, ipv6, onion, i2p, cjdns.
    /// Default in Bitcoin Core: `no default`.

    pub network: Option<String>,
}

/// Optional parameters for the [`getorphantxs`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getorphantxs`]: ../methods/struct.Raw.html#method.get_orphan_txs_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrphanTxsOptions {
    /// 0 for an array of txids (may contain duplicates), 1 for an array of objects with tx details, and 2 for details from (1) and tx hex
    /// Default in Bitcoin Core: `0`.

    pub verbosity: Option<f64>,
}

/// Optional parameters for the [`getrawchangeaddress`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getrawchangeaddress`]: ../methods/struct.Raw.html#method.get_raw_change_address_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRawChangeAddressOptions {
    /// The address type to use. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    /// Default in Bitcoin Core: `no default`.

    pub address_type: Option<String>,
}

/// Optional parameters for the [`getrawmempool`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getrawmempool`]: ../methods/struct.Raw.html#method.get_raw_mempool_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRawMempoolOptions {
    /// True for a json object, false for array of transaction ids
    /// Default in Bitcoin Core: `False`.

    pub verbose: Option<bool>,
    /// If verbose=false, returns a json object with transaction list and mempool sequence number attached.
    /// Default in Bitcoin Core: `False`.

    pub mempool_sequence: Option<bool>,
}

/// Optional parameters for the [`getrawtransaction`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getrawtransaction`]: ../methods/struct.Raw.html#method.get_raw_transaction_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRawTransactionOptions {
    /// 0 for hex-encoded data, 1 for a JSON object, and 2 for JSON object with fee and prevout
    /// Default in Bitcoin Core: `0`.

    pub verbosity: Option<f64>,
    /// The block in which to look for the transaction
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
}

/// Optional parameters for the [`getreceivedbyaddress`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getreceivedbyaddress`]: ../methods/struct.Raw.html#method.get_receivedby_address_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReceivedbyAddressOptions {
    /// Only include transactions confirmed at least this many times.
    /// Default in Bitcoin Core: `1`.

    #[serde(rename = "minconf")]
    pub min_conf: Option<f64>,
    /// Include immature coinbase transactions.
    /// Default in Bitcoin Core: `False`.

    pub include_immature_coinbase: Option<bool>,
}

/// Optional parameters for the [`getreceivedbylabel`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`getreceivedbylabel`]: ../methods/struct.Raw.html#method.get_receivedby_label_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReceivedbyLabelOptions {
    /// Only include transactions confirmed at least this many times.
    /// Default in Bitcoin Core: `1`.

    #[serde(rename = "minconf")]
    pub min_conf: Option<f64>,
    /// Include immature coinbase transactions.
    /// Default in Bitcoin Core: `False`.

    pub include_immature_coinbase: Option<bool>,
}

/// Optional parameters for the [`gettransaction`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`gettransaction`]: ../methods/struct.Raw.html#method.get_transaction_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionOptions {
    /// (DEPRECATED) No longer used
    /// Default in Bitcoin Core: `False`.

    pub include_watchonly: Option<bool>,
    /// Whether to include a `decoded` field containing the decoded transaction (equivalent to RPC decoderawtransaction)
    /// Default in Bitcoin Core: `False`.

    pub verbose: Option<bool>,
}

/// Optional parameters for the [`gettxout`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`gettxout`]: ../methods/struct.Raw.html#method.get_txout_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTxoutOptions {
    /// Whether to include the mempool. Note that an unspent output that is spent in the mempool won't appear.
    /// Default in Bitcoin Core: `True`.

    pub include_mempool: Option<bool>,
}

/// Optional parameters for the [`gettxoutproof`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`gettxoutproof`]: ../methods/struct.Raw.html#method.get_txout_proof_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTxoutProofOptions {
    /// If specified, looks for txid in the block with this hash
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
}

/// Optional parameters for the [`gettxoutsetinfo`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`gettxoutsetinfo`]: ../methods/struct.Raw.html#method.get_txout_set_info_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTxoutSetInfoOptions {
    /// Which UTXO set hash should be calculated. Options: 'hash_serialized_3' (the legacy algorithm), 'muhash', 'none'.
    /// Default in Bitcoin Core: `'hash_serialized_3'`.

    pub hash_type: Option<String>,
    /// The block hash or height of the target height (only available with coinstatsindex).
    /// Default in Bitcoin Core: `no default`.

    pub hash_or_height: Option<f64>,
    /// Use coinstatsindex, if available.
    /// Default in Bitcoin Core: `True`.

    pub use_index: Option<bool>,
}

/// Optional parameters for the [`help`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`help`]: ../methods/struct.Raw.html#method.help_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HelpOptions {
    /// The command to get help on
    /// Default in Bitcoin Core: `no default`.

    pub command: Option<String>,
}

/// Optional parameters for the [`importmempool`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`importmempool`]: ../methods/struct.Raw.html#method.import_mempool_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportMempoolOptions {
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`keypoolrefill`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`keypoolrefill`]: ../methods/struct.Raw.html#method.key_poolrefill_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPoolrefillOptions {
    /// The new keypool size
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "newsize")]
    pub new_size: Option<f64>,
}

/// Optional parameters for the [`listdescriptors`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`listdescriptors`]: ../methods/struct.Raw.html#method.list_descriptors_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDescriptorsOptions {
    /// Show private descriptors.
    /// Default in Bitcoin Core: `False`.

    pub private: Option<bool>,
}

/// Optional parameters for the [`listlabels`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`listlabels`]: ../methods/struct.Raw.html#method.list_labels_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLabelsOptions {
    /// Address purpose to list labels for ('send','receive'). An empty string is the same as not providing this argument.
    /// Default in Bitcoin Core: `no default`.

    pub purpose: Option<String>,
}

/// Optional parameters for the [`listreceivedbyaddress`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`listreceivedbyaddress`]: ../methods/struct.Raw.html#method.list_receivedby_address_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReceivedbyAddressOptions {
    /// The minimum number of confirmations before payments are included.
    /// Default in Bitcoin Core: `1`.

    #[serde(rename = "minconf")]
    pub min_conf: Option<f64>,
    /// Whether to include addresses that haven't received any payments.
    /// Default in Bitcoin Core: `False`.

    pub include_empty: Option<bool>,
    /// (DEPRECATED) No longer used
    /// Default in Bitcoin Core: `False`.

    pub include_watchonly: Option<bool>,
    /// If present and non-empty, only return information on this address.
    /// Default in Bitcoin Core: `no default`.

    pub address_filter: Option<String>,
    /// Include immature coinbase transactions.
    /// Default in Bitcoin Core: `False`.

    pub include_immature_coinbase: Option<bool>,
}

/// Optional parameters for the [`listreceivedbylabel`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`listreceivedbylabel`]: ../methods/struct.Raw.html#method.list_receivedby_label_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReceivedbyLabelOptions {
    /// The minimum number of confirmations before payments are included.
    /// Default in Bitcoin Core: `1`.

    #[serde(rename = "minconf")]
    pub min_conf: Option<f64>,
    /// Whether to include labels that haven't received any payments.
    /// Default in Bitcoin Core: `False`.

    pub include_empty: Option<bool>,
    /// (DEPRECATED) No longer used
    /// Default in Bitcoin Core: `False`.

    pub include_watchonly: Option<bool>,
    /// Include immature coinbase transactions.
    /// Default in Bitcoin Core: `False`.

    pub include_immature_coinbase: Option<bool>,
}

/// Optional parameters for the [`listsinceblock`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`listsinceblock`]: ../methods/struct.Raw.html#method.list_since_block_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSinceBlockOptions {
    /// If set, the block hash to list transactions since, otherwise list all transactions.
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// Return the nth block hash from the main chain. e.g. 1 would mean the best block hash. Note: this is not used as a filter, but only affects [lastblock] in the return value
    /// Default in Bitcoin Core: `1`.

    pub target_confirmations: Option<f64>,
    /// (DEPRECATED) No longer used
    /// Default in Bitcoin Core: `False`.

    pub include_watchonly: Option<bool>,
    /// Show transactions that were removed due to a reorg in the "removed" array
    /// (not guaranteed to work on pruned nodes)
    /// Default in Bitcoin Core: `True`.

    pub include_removed: Option<bool>,
    /// Also add entries for change outputs.
    /// Default in Bitcoin Core: `False`.

    pub include_change: Option<bool>,
    /// Return only incoming transactions paying to addresses with the specified label.
    /// Default in Bitcoin Core: `no default`.

    pub label: Option<String>,
}

/// Optional parameters for the [`listtransactions`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`listtransactions`]: ../methods/struct.Raw.html#method.list_transactions_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsOptions {
    /// If set, should be a valid label name to return only incoming transactions
    /// with the specified label, or "*" to disable filtering and return all transactions.
    /// Default in Bitcoin Core: `no default`.

    pub label: Option<String>,
    /// The number of transactions to return
    /// Default in Bitcoin Core: `10`.

    pub count: Option<f64>,
    /// The number of transactions to skip
    /// Default in Bitcoin Core: `0`.

    pub skip: Option<f64>,
    /// (DEPRECATED) No longer used
    /// Default in Bitcoin Core: `False`.

    pub include_watchonly: Option<bool>,
}

/// Optional parameters for the [`listunspent`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`listunspent`]: ../methods/struct.Raw.html#method.list_unspent_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUnspentOptions {
    /// The minimum confirmations to filter
    /// Default in Bitcoin Core: `1`.

    #[serde(rename = "minconf")]
    pub min_conf: Option<f64>,
    /// The maximum confirmations to filter
    /// Default in Bitcoin Core: `9999999`.

    #[serde(rename = "maxconf")]
    pub max_conf: Option<f64>,
    /// The bitcoin addresses to filter
    /// Default in Bitcoin Core: `[]`.

    pub addresses: Option<Vec<String>>,
    /// Include outputs that are not safe to spend
    /// See description of "safe" attribute below.
    /// Default in Bitcoin Core: `True`.

    pub include_unsafe: Option<bool>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub query_options: Option<serde_json::Value>,
}

/// Optional parameters for the [`loadwallet`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`loadwallet`]: ../methods/struct.Raw.html#method.load_wallet_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadWalletOptions {
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    /// Default in Bitcoin Core: `no default`.

    pub load_on_startup: Option<bool>,
}

/// Optional parameters for the [`lockunspent`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`lockunspent`]: ../methods/struct.Raw.html#method.lock_unspent_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LockUnspentOptions {
    /// The transaction outputs and within each, the txid (string) vout (numeric).
    /// Default in Bitcoin Core: `[]`.

    pub transactions: Option<Vec<serde_json::Value>>,
    /// Whether to write/erase this lock in the wallet database, or keep the change in memory only. Ignored for unlocking.
    /// Default in Bitcoin Core: `False`.

    pub persistent: Option<bool>,
}

/// Optional parameters for the [`logging`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`logging`]: ../methods/struct.Raw.html#method.logging_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingOptions {
    /// The categories to add to debug logging
    /// Default in Bitcoin Core: `no default`.

    pub include: Option<Vec<String>>,
    /// The categories to remove from debug logging
    /// Default in Bitcoin Core: `no default`.

    pub exclude: Option<Vec<String>>,
}

/// Optional parameters for the [`migratewallet`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`migratewallet`]: ../methods/struct.Raw.html#method.migrate_wallet_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrateWalletOptions {
    /// The name of the wallet to migrate. If provided both here and in the RPC endpoint, the two must be identical.
    /// Default in Bitcoin Core: `no default`.

    pub wallet_name: Option<String>,
    /// The wallet passphrase
    /// Default in Bitcoin Core: `no default`.

    pub passphrase: Option<String>,
}

/// Optional parameters for the [`prioritisetransaction`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`prioritisetransaction`]: ../methods/struct.Raw.html#method.prioritise_transaction_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrioritiseTransactionOptions {
    /// API-Compatibility for previous API. Must be zero or null.
    ///                   DEPRECATED. For forward compatibility use named arguments and omit this parameter.
    /// Default in Bitcoin Core: `no default`.

    pub dummy: Option<f64>,
}

/// Optional parameters for the [`psbtbumpfee`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`psbtbumpfee`]: ../methods/struct.Raw.html#method.psbt_bump_fee_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PsbtBumpFeeOptions {
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`rescanblockchain`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`rescanblockchain`]: ../methods/struct.Raw.html#method.rescan_blockchain_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RescanBlockchainOptions {
    /// block height where the rescan should start
    /// Default in Bitcoin Core: `0`.

    pub start_height: Option<f64>,
    /// the last block height that should be scanned. If none is provided it will rescan up to the tip at return time of this call.
    /// Default in Bitcoin Core: `no default`.

    pub stop_height: Option<f64>,
}

/// Optional parameters for the [`restorewallet`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`restorewallet`]: ../methods/struct.Raw.html#method.restore_wallet_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreWalletOptions {
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    /// Default in Bitcoin Core: `no default`.

    pub load_on_startup: Option<bool>,
}

/// Optional parameters for the [`scanblocks`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`scanblocks`]: ../methods/struct.Raw.html#method.scan_blocks_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanBlocksOptions {
    /// Array of scan objects. Required for "start" action
    /// Every scan object is either a string descriptor or an object:
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "scanobjects")]
    pub scan_objects: Option<Vec<serde_json::Value>>,
    /// Height to start to scan from
    /// Default in Bitcoin Core: `0`.

    pub start_height: Option<f64>,
    /// Height to stop to scan
    /// Default in Bitcoin Core: `no default`.

    pub stop_height: Option<f64>,
    /// The type name of the filter
    /// Default in Bitcoin Core: `'basic'`.

    #[serde(rename = "filtertype")]
    pub filter_type: Option<String>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`scantxoutset`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`scantxoutset`]: ../methods/struct.Raw.html#method.scan_txout_set_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanTxoutSetOptions {
    /// Array of scan objects. Required for "start" action
    /// Every scan object is either a string descriptor or an object:
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "scanobjects")]
    pub scan_objects: Option<Vec<serde_json::Value>>,
}

/// Optional parameters for the [`send`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`send`]: ../methods/struct.Raw.html#method.send_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendOptions {
    /// Confirmation target in blocks
    /// Default in Bitcoin Core: `no default`.

    pub conf_target: Option<f64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    /// Default in Bitcoin Core: `'unset'`.

    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    /// Default in Bitcoin Core: `no default`.

    pub fee_rate: Option<f64>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
    /// Transaction version
    /// Default in Bitcoin Core: `2`.

    pub version: Option<f64>,
}

/// Optional parameters for the [`sendall`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`sendall`]: ../methods/struct.Raw.html#method.send_all_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendAllOptions {
    /// Confirmation target in blocks
    /// Default in Bitcoin Core: `no default`.

    pub conf_target: Option<f64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    /// Default in Bitcoin Core: `'unset'`.

    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    /// Default in Bitcoin Core: `no default`.

    pub fee_rate: Option<f64>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`sendmany`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`sendmany`]: ../methods/struct.Raw.html#method.sendmany_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendmanyOptions {
    /// Must be set to "" for backwards compatibility.
    /// Default in Bitcoin Core: `'""'`.

    pub dummy: Option<String>,
    /// Ignored dummy value
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "minconf")]
    pub min_conf: Option<f64>,
    /// A comment
    /// Default in Bitcoin Core: `no default`.

    pub comment: Option<String>,
    /// The addresses.
    /// The fee will be equally deducted from the amount of each selected address.
    /// Those recipients will receive less bitcoins than you enter in their corresponding amount field.
    /// If no addresses are specified here, the sender pays the fee.
    /// Default in Bitcoin Core: `no default`.

    pub subtractfeefrom: Option<Vec<String>>,
    /// Signal that this transaction can be replaced by a transaction (BIP 125)
    /// Default in Bitcoin Core: `no default`.

    pub replaceable: Option<bool>,
    /// Confirmation target in blocks
    /// Default in Bitcoin Core: `no default`.

    pub conf_target: Option<f64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    /// Default in Bitcoin Core: `'unset'`.

    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    /// Default in Bitcoin Core: `no default`.

    pub fee_rate: Option<f64>,
    /// If true, return extra information about the transaction.
    /// Default in Bitcoin Core: `False`.

    pub verbose: Option<bool>,
}

/// Optional parameters for the [`sendrawtransaction`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`sendrawtransaction`]: ../methods/struct.Raw.html#method.send_raw_transaction_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendRawTransactionOptions {
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    /// Default in Bitcoin Core: `'0.10'`.

    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: Option<f64>,
    /// Reject transactions with provably unspendable outputs (e.g. 'datacarrier' outputs that use the OP_RETURN opcode) greater than the specified value, expressed in BTC.
    /// If burning funds through unspendable outputs is desired, increase this value.
    /// This check is based on heuristics and does not guarantee spendability of outputs.
    /// Default in Bitcoin Core: `'0.00'`.

    #[serde(rename = "maxburnamount")]
    pub max_burn_amount: Option<f64>,
}

/// Optional parameters for the [`sendtoaddress`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`sendtoaddress`]: ../methods/struct.Raw.html#method.send_to_address_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendToAddressOptions {
    /// A comment used to store what the transaction is for.
    /// This is not part of the transaction, just kept in your wallet.
    /// Default in Bitcoin Core: `no default`.

    pub comment: Option<String>,
    /// A comment to store the name of the person or organization
    /// to which you're sending the transaction. This is not part of the
    /// transaction, just kept in your wallet.
    /// Default in Bitcoin Core: `no default`.

    pub comment_to: Option<String>,
    /// The fee will be deducted from the amount being sent.
    /// The recipient will receive less bitcoins than you enter in the amount field.
    /// Default in Bitcoin Core: `False`.

    pub subtractfeefromamount: Option<bool>,
    /// Signal that this transaction can be replaced by a transaction (BIP 125)
    /// Default in Bitcoin Core: `no default`.

    pub replaceable: Option<bool>,
    /// Confirmation target in blocks
    /// Default in Bitcoin Core: `no default`.

    pub conf_target: Option<f64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    /// Default in Bitcoin Core: `'unset'`.

    pub estimate_mode: Option<String>,
    /// (only available if avoid_reuse wallet flag is set) Avoid spending from dirty addresses; addresses are considered
    /// dirty if they have previously been used in a transaction. If true, this also activates avoidpartialspends, grouping outputs by their addresses.
    /// Default in Bitcoin Core: `True`.

    pub avoid_reuse: Option<bool>,
    /// Specify a fee rate in sat/vB.
    /// Default in Bitcoin Core: `no default`.

    pub fee_rate: Option<f64>,
    /// If true, return extra information about the transaction.
    /// Default in Bitcoin Core: `False`.

    pub verbose: Option<bool>,
}

/// Optional parameters for the [`setban`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`setban`]: ../methods/struct.Raw.html#method.setban_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetbanOptions {
    /// time in seconds how long (or until when if [absolute] is set) the IP is banned (0 or empty means using the default time of 24h which can also be overwritten by the -bantime startup argument)
    /// Default in Bitcoin Core: `0`.

    pub bantime: Option<f64>,
    /// If set, the bantime must be an absolute timestamp expressed in UNIX epoch time
    /// Default in Bitcoin Core: `False`.

    pub absolute: Option<bool>,
}

/// Optional parameters for the [`setwalletflag`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`setwalletflag`]: ../methods/struct.Raw.html#method.set_walletflag_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWalletflagOptions {
    /// The new state.
    /// Default in Bitcoin Core: `True`.

    pub value: Option<bool>,
}

/// Optional parameters for the [`signrawtransactionwithkey`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`signrawtransactionwithkey`]: ../methods/struct.Raw.html#method.sign_raw_transactionwith_key_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRawTransactionwithKeyOptions {
    /// The previous dependent transaction outputs
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "prevtxs")]
    pub prev_txs: Option<Vec<serde_json::Value>>,
    /// The signature hash type. Must be one of:
    ///        "DEFAULT"
    ///        "ALL"
    ///        "NONE"
    ///        "SINGLE"
    ///        "ALL|ANYONECANPAY"
    ///        "NONE|ANYONECANPAY"
    ///        "SINGLE|ANYONECANPAY"
    /// Default in Bitcoin Core: `'DEFAULT for Taproot, ALL otherwise'`.

    pub sighashtype: Option<String>,
}

/// Optional parameters for the [`signrawtransactionwithwallet`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`signrawtransactionwithwallet`]: ../methods/struct.Raw.html#method.sign_raw_transactionwith_wallet_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRawTransactionwithWalletOptions {
    /// The previous dependent transaction outputs
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "prevtxs")]
    pub prev_txs: Option<Vec<serde_json::Value>>,
    /// The signature hash type. Must be one of
    ///        "DEFAULT"
    ///        "ALL"
    ///        "NONE"
    ///        "SINGLE"
    ///        "ALL|ANYONECANPAY"
    ///        "NONE|ANYONECANPAY"
    ///        "SINGLE|ANYONECANPAY"
    /// Default in Bitcoin Core: `'DEFAULT for Taproot, ALL otherwise'`.

    pub sighashtype: Option<String>,
}

/// Optional parameters for the [`simulaterawtransaction`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`simulaterawtransaction`]: ../methods/struct.Raw.html#method.simulate_raw_transaction_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateRawTransactionOptions {
    /// An array of hex strings of raw transactions.
    /// Default in Bitcoin Core: `no default`.

    #[serde(rename = "rawtxs")]
    pub raw_txs: Option<Vec<String>>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
}

/// Optional parameters for the [`stop`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`stop`]: ../methods/struct.Raw.html#method.stop_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOptions {
    /// how long to wait in ms
    /// Default in Bitcoin Core: `no default`.

    pub wait: Option<f64>,
}

/// Optional parameters for the [`submitblock`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`submitblock`]: ../methods/struct.Raw.html#method.submit_block_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBlockOptions {
    /// dummy value, for compatibility with BIP22. This value is ignored.
    /// Default in Bitcoin Core: `no default`.

    pub dummy: Option<String>,
}

/// Optional parameters for the [`submitpackage`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`submitpackage`]: ../methods/struct.Raw.html#method.submit_package_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitPackageOptions {
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    /// Default in Bitcoin Core: `'0.10'`.

    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: Option<f64>,
    /// Reject transactions with provably unspendable outputs (e.g. 'datacarrier' outputs that use the OP_RETURN opcode) greater than the specified value, expressed in BTC.
    /// If burning funds through unspendable outputs is desired, increase this value.
    /// This check is based on heuristics and does not guarantee spendability of outputs.
    /// Default in Bitcoin Core: `'0.00'`.

    #[serde(rename = "maxburnamount")]
    pub max_burn_amount: Option<f64>,
}

/// Optional parameters for the [`testmempoolaccept`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`testmempoolaccept`]: ../methods/struct.Raw.html#method.test_mempoolaccept_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestMempoolacceptOptions {
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    /// Default in Bitcoin Core: `'0.10'`.

    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: Option<f64>,
}

/// Optional parameters for the [`unloadwallet`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`unloadwallet`]: ../methods/struct.Raw.html#method.unload_wallet_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnloadWalletOptions {
    /// The name of the wallet to unload. If provided both here and in the RPC endpoint, the two must be identical.
    /// Default in Bitcoin Core: `no default`.

    pub wallet_name: Option<String>,
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    /// Default in Bitcoin Core: `no default`.

    pub load_on_startup: Option<bool>,
}

/// Optional parameters for the [`utxoupdatepsbt`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`utxoupdatepsbt`]: ../methods/struct.Raw.html#method.utxo_update_psbt_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoUpdatePsbtOptions {
    /// An array of either strings or objects
    /// Default in Bitcoin Core: `no default`.

    pub descriptors: Option<Vec<serde_json::Value>>,
}

/// Optional parameters for the [`verifychain`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`verifychain`]: ../methods/struct.Raw.html#method.verify_chain_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyChainOptions {
    /// How thorough the block verification is:
    /// - level 0 reads the blocks from disk
    /// - level 1 verifies block validity
    /// - level 2 verifies undo data
    /// - level 3 checks disconnection of tip blocks
    /// - level 4 tries to reconnect the blocks
    /// - each level includes the checks of the previous levels
    /// Default in Bitcoin Core: `no default`.

    pub checklevel: Option<f64>,
    /// The number of blocks to check.
    /// Default in Bitcoin Core: `no default`.

    pub nblocks: Option<f64>,
}

/// Optional parameters for the [`waitforblock`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`waitforblock`]: ../methods/struct.Raw.html#method.wait_for_block_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitForBlockOptions {
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    /// Default in Bitcoin Core: `0`.

    #[serde(rename = "timeout")]
    pub time_out: Option<f64>,
}

/// Optional parameters for the [`waitforblockheight`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`waitforblockheight`]: ../methods/struct.Raw.html#method.wait_for_block_height_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitForBlockHeightOptions {
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    /// Default in Bitcoin Core: `0`.

    #[serde(rename = "timeout")]
    pub time_out: Option<f64>,
}

/// Optional parameters for the [`waitfornewblock`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`waitfornewblock`]: ../methods/struct.Raw.html#method.wait_for_new_block_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitForNewBlockOptions {
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    /// Default in Bitcoin Core: `0`.

    #[serde(rename = "timeout")]
    pub time_out: Option<f64>,
    /// Method waits for the chain tip to differ from this.
    /// Default in Bitcoin Core: `no default`.

    pub current_tip: Option<String>,
}

/// Optional parameters for the [`walletcreatefundedpsbt`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`walletcreatefundedpsbt`]: ../methods/struct.Raw.html#method.wallet_create_funded_psbt_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletCreateFundedPsbtOptions {
    /// Leave empty to add inputs automatically. See add_inputs option.
    /// Default in Bitcoin Core: `no default`.

    pub inputs: Option<Vec<serde_json::Value>>,
    /// Raw locktime. Non-0 value also locktime-activates inputs
    /// Default in Bitcoin Core: `0`.

    pub locktime: Option<f64>,
    ///
    /// Default in Bitcoin Core: `no default`.

    pub options: Option<serde_json::Value>,
    /// Include BIP 32 derivation paths for public keys if we know them
    /// Default in Bitcoin Core: `True`.

    pub bip32derivs: Option<bool>,
    /// Transaction version
    /// Default in Bitcoin Core: `2`.

    pub version: Option<f64>,
}

/// Optional parameters for the [`walletprocesspsbt`] JSON-RPC method.
///
/// Every field is `None` by default; setting a field to `Some(_)` causes the value to be sent as the corresponding positional argument. Unset fields are serialised as JSON `null`, which Bitcoin Core treats as 'use the documented default'.
///
/// [`walletprocesspsbt`]: ../methods/struct.Raw.html#method.wallet_process_psbt_with

#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletProcessPsbtOptions {
    /// Also sign the transaction when updating (requires wallet to be unlocked)
    /// Default in Bitcoin Core: `True`.

    pub sign: Option<bool>,
    /// The signature hash type to sign with if not specified by the PSBT. Must be one of
    ///        "DEFAULT"
    ///        "ALL"
    ///        "NONE"
    ///        "SINGLE"
    ///        "ALL|ANYONECANPAY"
    ///        "NONE|ANYONECANPAY"
    ///        "SINGLE|ANYONECANPAY"
    /// Default in Bitcoin Core: `'DEFAULT for Taproot, ALL otherwise'`.

    pub sighashtype: Option<String>,
    /// Include BIP 32 derivation paths for public keys if we know them
    /// Default in Bitcoin Core: `True`.

    pub bip32derivs: Option<bool>,
    /// Also finalize inputs if possible
    /// Default in Bitcoin Core: `True`.

    #[serde(rename = "finalize")]
    pub final_ize: Option<bool>,
}
