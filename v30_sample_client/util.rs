// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - util.
//! Types for methods found under the `== Util ==` section.
//! Auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateMultisig {
    /// The value of the new multisig address.
    pub address: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// The string value of the hex-encoded redemption script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
    /// Any warnings resulting from the creation of this multisig
    pub warnings: Option<Vec<String>>,
}

/// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
///     pkh(<pubkey>)                                     P2PKH outputs for the given pubkey
///     wpkh(<pubkey>)                                    Native segwit P2PKH outputs for the given pubkey
///     sh(multi(<n>,<pubkey>,<pubkey>,...))              P2SH-multisig outputs for the given threshold and pubkeys
///     raw(<hex script>)                                 Outputs whose output script equals the specified hex-encoded bytes
///     tr(<pubkey>,multi_a(<n>,<pubkey>,<pubkey>,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// 
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DeriveAddressesVerboseOne(pub Vec<Vec<String>>);

/// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
///     pkh(<pubkey>)                                     P2PKH outputs for the given pubkey
///     wpkh(<pubkey>)                                    Native segwit P2PKH outputs for the given pubkey
///     sh(multi(<n>,<pubkey>,<pubkey>,...))              P2SH-multisig outputs for the given threshold and pubkeys
///     raw(<hex script>)                                 Outputs whose output script equals the specified hex-encoded bytes
///     tr(<pubkey>,multi_a(<n>,<pubkey>,<pubkey>,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// 
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DeriveAddressesVerboseZero(pub Vec<String>);

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateSmartFee {
    /// block number where estimate was found
/// The request target will be clamped between 2 and the highest target
/// fee estimation is able to return based on how long it has been running.
/// An error is returned if not enough transactions and blocks
/// have been observed to make an estimate for any number of blocks.
    pub blocks: i64,
    /// Errors encountered during processing (if there are any)
    pub errors: Option<Vec<String>>,
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    pub feerate: Option<i64>,
}

/// Analyses a descriptor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorInfo {
    /// The checksum for the input descriptor
    pub checksum: String,
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// Whether the input descriptor contained at least one private key
    pub hasprivatekeys: bool,
    /// Whether the descriptor is ranged
    pub isrange: bool,
    /// Whether the descriptor is solvable
    pub issolvable: bool,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    pub multipath_expansion: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `getindexinfo`.
///
/// > getindexinfo
/// >
/// > Returns the status of one or all available indices currently running in the node.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfo(
    /// Map entries
    pub std::collections::BTreeMap<String, GetIndexInfoEntry>,
);

/// The name of the index
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfoEntry {
    /// The block height to which the index is synced
    pub best_block_height: i64,
    /// Whether the index is synced or not
    pub synced: bool,
}

/// Result of the JSON-RPC method `signmessagewithprivkey`.
///
/// > signmessagewithprivkey
/// >
/// > Sign a message with the private key of an address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignMessagewithPrivKey(pub String);

/// Return information about the given bitcoin address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ValidateAddress {
    /// The bitcoin address validated
    pub address: Option<String>,
    /// Error message, if any
    pub error: Option<String>,
    /// Indices of likely error locations in address, if known (e.g. Bech32 errors)
    pub error_locations: Option<Vec<i64>>,
    /// If the key is a script
    pub isscript: Option<bool>,
    /// If the address is valid or not
    pub isvalid: bool,
    /// If the address is a witness address
    pub iswitness: Option<bool>,
    /// The hex-encoded output script generated by the address
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: Option<String>,
    /// The hex value of the witness program
    pub witness_program: Option<String>,
    /// The version number of the witness program
    pub witness_version: Option<i64>,
}

/// Result of the JSON-RPC method `verifymessage`.
///
/// > verifymessage
/// >
/// > Verify a signed message.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyMessage(pub bool);

