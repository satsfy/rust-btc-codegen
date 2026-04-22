// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - hidden.
//! Types for methods found under the `== Hidden ==` section.
//! Auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Open an outbound connection to a specified node. This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddConnection {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,
}

/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddPeerAddress {
    /// error description, if the address could not be added
    pub error: Option<String>,
    /// whether the peer address was successfully added to the address manager table
    pub success: bool,
}

/// Simply echo back the input arguments. This command is for testing.
/// 
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// 
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Echo {

}

/// Result of the JSON-RPC method `echoipc`.
///
/// > echoipc
/// >
/// > Echo back the input argument, passing it through a spawned process in a multiprocess build.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EchoIpc(pub String);

/// Simply echo back the input arguments. This command is for testing.
/// 
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// 
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EchoJson {

}

/// WARNING: This interface is unstable and may disappear or change!
/// 
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
/// 
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFee {
    /// estimate for long time horizon
    pub long: Option<EstimateRawFeeLong>,
    /// estimate for medium time horizon
    pub medium: Option<EstimateRawFeeMedium>,
    /// estimate for short time horizon
    pub short: Option<EstimateRawFeeShort>,
}

/// estimate for long time horizon
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeLong {

}

/// estimate for medium time horizon
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeMedium {

}

/// estimate for short time horizon
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeShort {
    /// exponential decay (per block) for historical moving average of confirmation data
    pub decay: i64,
    /// Errors encountered during processing (if there are any)
    pub errors: Option<Vec<String>>,
    /// information about the highest range of feerates to fail to meet the threshold
    pub fail: Option<EstimateRawFeeShortFail>,
    /// estimate fee rate in BTC/kvB
    pub feerate: Option<i64>,
    /// information about the lowest range of feerates to succeed in meeting the threshold
    pub pass: Option<EstimateRawFeeShortPass>,
    /// The resolution of confirmation targets at this time horizon
    pub scale: i64,
}

/// information about the highest range of feerates to fail to meet the threshold
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeShortFail {

}

/// information about the lowest range of feerates to succeed in meeting the threshold
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeShortPass {
    /// end of feerate range
    pub endrange: i64,
    /// current number of txs in mempool in the feerate range unconfirmed for at least target blocks
    pub inmempool: i64,
    /// number of txs over history horizon in the feerate range that left mempool unconfirmed after target
    pub leftmempool: i64,
    /// start of feerate range
    #[serde(rename = "startrange")]
    pub start_range: i64,
    /// number of txs over history horizon in the feerate range that were confirmed at any point
    #[serde(rename = "totalconfirmed")]
    pub total_conf_irmed: i64,
    /// number of txs over history horizon in the feerate range that were confirmed within target
    pub withintarget: i64,
}

/// has been replaced by the -generate cli option. Refer to -help for more information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Generate {

}

/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateBlock {
    /// hash of generated block
    pub hash: String,
    /// hex of generated block, only present when submit=false
    pub hex: Option<String>,
}

/// Result of the JSON-RPC method `generatetoaddress`.
///
/// > generatetoaddress
/// >
/// > Mine to a specified address and return the block hashes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateToAddress(pub Vec<String>);

/// Result of the JSON-RPC method `generatetodescriptor`.
///
/// > generatetodescriptor
/// >
/// > Mine to a specified descriptor and return the block hashes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateToDescriptor(pub Vec<String>);

/// Result of the JSON-RPC method `getmempoolfeeratediagram`.
///
/// > getmempoolfeeratediagram
/// >
/// > Returns the feerate diagram for the whole mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolFeeratediagram(pub Vec<GetMempoolFeeratediagramItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolFeeratediagramItem {
    /// cumulative fee
    pub fee: i64,
    /// cumulative sigops-adjusted weight
    pub weight: i64,
}

/// Shows transactions in the tx orphanage.
/// 
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseOne(pub Vec<GetOrphanTxsVerboseOneItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseOneItem {
    /// The serialized transaction size in bytes
    pub bytes: i64,
    pub from: Vec<i64>,
    /// The transaction hash in hex
    pub txid: String,
    /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// The transaction weight as defined in BIP 141.
    pub weight: i64,
    /// The transaction witness hash in hex
    pub wtxid: String,
}

/// Shows transactions in the tx orphanage.
/// 
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseTwo(pub Vec<GetOrphanTxsVerboseTwoItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseTwoItem {
    /// The serialized transaction size in bytes
    pub bytes: i64,
    pub from: Vec<i64>,
    /// The serialized, hex-encoded transaction data
    pub hex: String,
    /// The transaction hash in hex
    pub txid: String,
    /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// The transaction weight as defined in BIP 141.
    pub weight: i64,
    /// The transaction witness hash in hex
    pub wtxid: String,
}

/// Shows transactions in the tx orphanage.
/// 
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxsVerboseZero(pub Vec<String>);

/// Result of the JSON-RPC method `getrawaddrman`.
///
/// > getrawaddrman
/// >
/// > EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawAddrman(
    /// Map entries
    pub std::collections::BTreeMap<String, std::collections::BTreeMap<String, GetRawAddrmanEntryEntry>>,
);

/// the location in the address manager table (<bucket>/<position>)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawAddrmanEntryEntry {
    /// The address of the node
    pub address: String,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying peer selection (only displayed if the -asmap config option is set)
    pub mapped_as: Option<i64>,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) of the address
    pub network: String,
    /// The port number of the node
    pub port: i64,
    /// The services offered by the node
    pub services: i64,
    /// The address that relayed the address to us
    pub source: String,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the source, used for diversifying peer selection (only displayed if the -asmap config option is set)
    pub source_mapped_as: Option<i64>,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) of the source address
    pub source_network: String,
    /// The UNIX epoch time when the node was last seen
    pub time: i64,
}

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendmsgToPeer {

}

