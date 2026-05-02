// SPDX-License-Identifier: CC0-1.0

//! Auto-generated return types for Bitcoin Core `30`.
//!
//! Produced by `rust-btc-codegen`. **Do not edit by hand** — re-run
//! `just codegen` to regenerate.

#![allow(non_camel_case_types, clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `abortrescan`.
///
/// > abortrescan
/// >
/// > Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AbortRescan(pub bool);

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

/// Analyzes and provides information about the current status of a PSBT and its inputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbt {
    /// Error message (if there is one)
    pub error: Option<String>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    pub estimated_feerate: Option<f64>,
    /// Estimated vsize of the final signed transaction
    pub estimated_vsize: Option<i64>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    pub fee: Option<f64>,
    pub inputs: Option<Vec<AnalyzePsbtInputsItem>>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtInputsItem {
    /// Whether a UTXO is provided
    pub has_utxo: bool,
    /// Whether the input is finalized
    pub is_final: bool,
    /// Things that are missing that are required to complete this input
    pub missing: Option<AnalyzePsbtInputsItemMissing>,
    /// Role of the next person that this input needs to go to
    pub next: Option<String>,
}

/// Things that are missing that are required to complete this input
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtInputsItemMissing {
    pub pubkeys: Option<Vec<String>>,
    /// Hash160 of the redeem script that is missing
    #[serde(rename = "redeemscript")]
    pub redeem_script: Option<String>,
    pub signatures: Option<Vec<String>>,
    /// SHA256 of the witness script that is missing
    #[serde(rename = "witnessscript")]
    pub witness_script: Option<String>,
}

/// Bumps the fee of a transaction T, replacing it with a new transaction B.
/// A transaction with the given txid must be in the wallet.
/// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
/// It may add a new change output if one does not already exist.
/// All inputs in the original transaction will be included in the replacement transaction.
/// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
/// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
/// The user can specify a confirmation target for estimatesmartfee.
/// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
/// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// returned by getnetworkinfo) to enter the node's mempool.
/// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BumpFee {
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
    /// The fee of the new transaction.
    pub fee: f64,
    /// The fee of the replaced transaction.
    #[serde(rename = "origfee")]
    pub orig_fee: f64,
    /// The id of the new transaction.
    pub txid: String,
}

/// Result of the JSON-RPC method `combinepsbt`.
///
/// > combinepsbt
/// >
/// > Combine multiple partially signed Bitcoin transactions into one transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CombinePsbt(pub String);

/// Result of the JSON-RPC method `combinerawtransaction`.
///
/// > combinerawtransaction
/// >
/// > Combine multiple partially signed transactions into one transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CombineRawTransaction(pub String);

/// Result of the JSON-RPC method `converttopsbt`.
///
/// > converttopsbt
/// >
/// > Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ConvertToPsbt(pub String);

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

/// Result of the JSON-RPC method `createpsbt`.
///
/// > createpsbt
/// >
/// > Creates a transaction in the Partially Signed Transaction format.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreatePsbt(pub String);

/// Result of the JSON-RPC method `createrawtransaction`.
///
/// > createrawtransaction
/// >
/// > Create a transaction spending the given inputs and creating new outputs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateRawTransaction(pub String);

/// Creates and loads a new wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWallet {
    /// The wallet name if created successfully. If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWalletDescriptor {
    /// The public descriptors that were added to the wallet
    #[serde(rename = "descs")]
    pub desc_s: Vec<String>,
}

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbt {
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    pub fee: Option<f64>,
    pub global_xpubs: Vec<DecodePsbtGlobalXpubsItem>,
    pub inputs: Vec<DecodePsbtInputsItem>,
    pub outputs: Vec<DecodePsbtOutPutsItem>,
    /// The global proprietary map
    pub proprietary: Vec<DecodePsbtProprietaryItem>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    pub psbt_version: i64,
    /// The decoded network-serialized unsigned transaction.
    pub tx: DecodePsbtTx,
    /// The unknown global fields
    pub unknown: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtGlobalXpubsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The extended public key this path corresponds to
    pub xpub: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItem {
    pub bip32_derivs: Option<Vec<DecodePsbtInputsItemBip32DerivsItem>>,
    #[serde(rename = "final_scriptSig")]
    pub final_scriptsig: Option<DecodePsbtInputsItemFinalScriptSig>,
    pub final_scriptwitness: Option<Vec<String>>,
    pub hash160_preimages: Option<std::collections::BTreeMap<String, String>>,
    pub hash256_preimages: Option<std::collections::BTreeMap<String, String>>,
    pub musig2_partial_sigs: Option<Vec<DecodePsbtInputsItemMusig2PartialSigsItem>>,
    pub musig2_participant_pubkeys: Option<Vec<DecodePsbtInputsItemMusig2ParticipantPubkeysItem>>,
    pub musig2_pubnonces: Option<Vec<DecodePsbtInputsItemMusig2PubnoncesItem>>,
    /// Decoded network transaction for non-witness UTXOs
    pub non_witness_utxo: Option<DecodePsbtInputsItemNonWitnessUtxo>,
    pub partial_signatures: Option<std::collections::BTreeMap<String, String>>,
    /// The input proprietary map
    pub proprietary: Option<Vec<DecodePsbtInputsItemProprietaryItem>>,
    pub redeem_script: Option<DecodePsbtInputsItemRedeemScript>,
    pub ripemd160_preimages: Option<std::collections::BTreeMap<String, String>>,
    pub sha256_preimages: Option<std::collections::BTreeMap<String, String>>,
    /// The sighash type to be used
    #[serde(rename = "sighash")]
    pub sig_hash: Option<String>,
    pub taproot_bip32_derivs: Option<Vec<DecodePsbtInputsItemTaprootBip32DerivsItem>>,
    /// The hex-encoded Taproot x-only internal key
    pub taproot_internal_key: Option<String>,
    /// hex-encoded signature for the Taproot key path spend
    pub taproot_key_path_sig: Option<String>,
    /// The hex-encoded Taproot merkle root
    pub taproot_merkle_root: Option<String>,
    pub taproot_script_path_sigs: Option<Vec<DecodePsbtInputsItemTaprootScriptPathSigsItem>>,
    pub taproot_scripts: Option<Vec<DecodePsbtInputsItemTaprootScriptsItem>>,
    /// The unknown input fields
    pub unknown: Option<std::collections::BTreeMap<String, String>>,
    pub witness_script: Option<DecodePsbtInputsItemWitnessScript>,
    /// Transaction output for witness UTXOs
    pub witness_utxo: Option<DecodePsbtInputsItemWitnessUtxo>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemBip32DerivsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The public key with the derivation path as the value.
    pub pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemFinalScriptSig {
    /// Disassembly of the final signature script
    pub asm: String,
    /// The raw final signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2PartialSigsItem {
    /// The compressed aggregate public key for which this partial signature is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    pub leaf_hash: Option<String>,
    /// The partial signature itself.
    pub partial_sig: String,
    /// The compressed public key of the participant that created this partial signature.
    pub participant_pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2ParticipantPubkeysItem {
    /// The compressed aggregate public key for which the participants create.
    pub aggregate_pubkey: String,
    pub participant_pubkeys: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2PubnoncesItem {
    /// The compressed aggregate public key for which this pubnonce is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    pub leaf_hash: Option<String>,
    /// The compressed public key of the participant that created this pubnonce.
    pub participant_pubkey: String,
    /// The public nonce itself.
    #[serde(rename = "pubnonce")]
    pub pub_nonce: String,
}

/// Decoded network transaction for non-witness UTXOs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemNonWitnessUtxo {}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    #[serde(rename = "subtype")]
    pub sub_type: i64,
    /// The hex for the value
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemRedeemScript {
    /// Disassembly of the redeem script
    pub asm: String,
    /// The raw redeem script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootBip32DerivsItem {
    /// The hashes of the leaves this pubkey appears in
    pub leaf_hashes: Vec<String>,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The x-only public key this path corresponds to
    pub pubkey: String,
}

/// The signature for the pubkey and leaf hash combination
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootScriptPathSigsItem {
    /// The leaf hash for this signature
    pub leaf_hash: String,
    /// The x-only pubkey for this signature
    pub pubkey: String,
    /// The signature itself
    pub sig: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootScriptsItem {
    /// The control blocks for this script
    pub control_blocks: Vec<String>,
    /// The version number for the leaf script
    pub leaf_ver: i64,
    /// A leaf script
    pub script: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessScript {
    /// Disassembly of the witness script
    pub asm: String,
    /// The raw witness script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

/// Transaction output for witness UTXOs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessUtxo {
    /// The value in BTC
    pub amount: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: DecodePsbtInputsItemWitnessUtxoScriptPubKey,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessUtxoScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutPutsItem {
    pub bip32_derivs: Option<Vec<DecodePsbtOutPutsItemBip32DerivsItem>>,
    pub musig2_participant_pubkeys: Option<Vec<DecodePsbtOutPutsItemMusig2ParticipantPubkeysItem>>,
    /// The output proprietary map
    pub proprietary: Option<Vec<DecodePsbtOutPutsItemProprietaryItem>>,
    pub redeem_script: Option<DecodePsbtOutPutsItemRedeemScript>,
    pub taproot_bip32_derivs: Option<Vec<DecodePsbtOutPutsItemTaprootBip32DerivsItem>>,
    /// The hex-encoded Taproot x-only internal key
    pub taproot_internal_key: Option<String>,
    /// The tuples that make up the Taproot tree, in depth first search order
    pub taproot_tree: Option<Vec<DecodePsbtOutPutsItemTaprootTreeItem>>,
    /// The unknown output fields
    pub unknown: Option<std::collections::BTreeMap<String, String>>,
    pub witness_script: Option<DecodePsbtOutPutsItemWitnessScript>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutPutsItemBip32DerivsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The public key this path corresponds to
    pub pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutPutsItemMusig2ParticipantPubkeysItem {
    /// The compressed aggregate public key for which the participants create.
    pub aggregate_pubkey: String,
    pub participant_pubkeys: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutPutsItemProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    #[serde(rename = "subtype")]
    pub sub_type: i64,
    /// The hex for the value
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutPutsItemRedeemScript {
    /// Disassembly of the redeem script
    pub asm: String,
    /// The raw redeem script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutPutsItemTaprootBip32DerivsItem {
    /// The hashes of the leaves this pubkey appears in
    pub leaf_hashes: Vec<String>,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The x-only public key this path corresponds to
    pub pubkey: String,
}

/// A single leaf script in the taproot tree
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutPutsItemTaprootTreeItem {
    /// The depth of this element in the tree
    pub depth: i64,
    /// The version of this leaf
    pub leaf_ver: i64,
    /// The hex-encoded script itself
    pub script: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutPutsItemWitnessScript {
    /// Disassembly of the witness script
    pub asm: String,
    /// The raw witness script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    #[serde(rename = "subtype")]
    pub sub_type: i64,
    /// The hex for the value
    pub value: String,
}

/// The decoded network-serialized unsigned transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtTx {}

/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransaction {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<DecodeRawTransactionVinItem>,
    #[serde(rename = "vout")]
    pub v_out: Vec<DecodeRawTransactionVOutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVOutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: DecodeRawTransactionVOutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVOutItemScriptPubKey {
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(rename = "coinbase")]
    pub coin_base: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<DecodeRawTransactionVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    pub txid: Option<String>,
    #[serde(rename = "txinwitness")]
    pub tx_in_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(rename = "vout")]
    pub v_out: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

/// Decode a hex-encoded script.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScript {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    pub segwit: Option<DecodeScriptSegwit>,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScriptSegwit {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// address of the P2SH script wrapping this witness redeem script
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: String,
    /// The type of the output script (e.g. witness_v0_keyhash or witness_v0_scripthash)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
///     pkh(\<pubkey\>)                                     P2PKH outputs for the given pubkey
///     wpkh(\<pubkey\>)                                    Native segwit P2PKH outputs for the given pubkey
///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...))              P2SH-multisig outputs for the given threshold and pubkeys
///     raw(\<hex script\>)                                 Outputs whose output script equals the specified hex-encoded bytes
///     tr(\<pubkey\>,multi_a(\<n\>,\<pubkey\>,\<pubkey\>,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// 
/// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DeriveAddressesVerboseOne(pub Vec<Vec<String>>);

/// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
///     pkh(\<pubkey\>)                                     P2PKH outputs for the given pubkey
///     wpkh(\<pubkey\>)                                    Native segwit P2PKH outputs for the given pubkey
///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...))              P2SH-multisig outputs for the given threshold and pubkeys
///     raw(\<hex script\>)                                 Outputs whose output script equals the specified hex-encoded bytes
///     tr(\<pubkey\>,multi_a(\<n\>,\<pubkey\>,\<pubkey\>,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// 
/// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DeriveAddressesVerboseZero(pub Vec<String>);

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool. 
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DescriptorProcessPsbt {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
}

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// 
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// 
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxOutSet {
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

/// Simply echo back the input arguments. This command is for testing.
/// 
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// 
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Echo {}

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
pub struct EchoJson {}

/// Result of the JSON-RPC method `encryptwallet`.
///
/// > encryptwallet
/// >
/// > Encrypts the wallet with 'passphrase'. This is for first time encryption.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EncryptWallet(pub String);

/// Returns a list of external signers from -signer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSigners {
    pub signers: Vec<EnumerateSignersSignersItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSignersSignersItem {
    /// Master key fingerprint
    pub fingerprint: String,
    /// Device name
    pub name: String,
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
pub struct EstimateRawFeeLong {}

/// estimate for medium time horizon
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeMedium {}

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
    #[serde(rename = "feerate")]
    pub fee_rate: Option<i64>,
    /// information about the lowest range of feerates to succeed in meeting the threshold
    pub pass: Option<EstimateRawFeeShortPass>,
    /// The resolution of confirmation targets at this time horizon
    pub scale: i64,
}

/// information about the highest range of feerates to fail to meet the threshold
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeShortFail {}

/// information about the lowest range of feerates to succeed in meeting the threshold
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeShortPass {
    /// end of feerate range
    pub endrange: i64,
    /// current number of txs in mempool in the feerate range unconfirmed for at least target blocks
    #[serde(rename = "inmempool")]
    pub in_mempool: i64,
    /// number of txs over history horizon in the feerate range that left mempool unconfirmed after target
    #[serde(rename = "leftmempool")]
    pub left_mempool: i64,
    /// start of feerate range
    #[serde(rename = "startrange")]
    pub start_range: i64,
    /// number of txs over history horizon in the feerate range that were confirmed at any point
    #[serde(rename = "totalconfirmed")]
    pub total_conf_irmed: i64,
    /// number of txs over history horizon in the feerate range that were confirmed within target
    #[serde(rename = "withintarget")]
    pub with_in_target: i64,
}

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
    #[serde(rename = "blocks")]
    pub block_s: i64,
    /// Errors encountered during processing (if there are any)
    pub errors: Option<Vec<String>>,
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    #[serde(rename = "feerate")]
    pub fee_rate: Option<i64>,
}

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FinalizePsbt {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if extracted
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction if not extracted
    pub psbt: Option<String>,
}

/// If the transaction has no inputs, they will be automatically selected to meet its out value.
/// It will add at most one change output to the outputs.
/// No existing outputs will be modified unless "subtractFeeFromOutputs" is specified.
/// Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// The inputs added will not be signed, use signrawtransactionwithkey
/// or signrawtransactionwithwallet for that.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
/// Note that all inputs selected must be of standard form and P2SH scripts must be
/// in the wallet using importdescriptors (to calculate fees).
/// You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// Note that if specifying an exact fee rate, the resulting transaction may have a higher fee rate
/// if the transaction has unconfirmed inputs. This is because the wallet will attempt to make the
/// entire package have the given fee rate, not the resulting transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundRawTransaction {
    /// The position of the added change output, or -1
    #[serde(rename = "changepos")]
    pub change_pos: i64,
    /// Fee in BTC the resulting transaction pays
    pub fee: f64,
    /// The resulting raw transaction (hex-encoded string)
    pub hex: String,
}

/// has been replaced by the -generate cli option. Refer to -help for more information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Generate {}

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

/// Result of the JSON-RPC method `getaddednodeinfo`.
///
/// > getaddednodeinfo
/// >
/// > Returns information about the given added node, or all added nodes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddEdNodeInfo(pub Vec<GetAddEdNodeInfoItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddEdNodeInfoItem {
    /// The node IP address or name (as provided to addnode)
    #[serde(rename = "addednode")]
    pub added_node: String,
    /// Only when connected = true
    pub addresses: Vec<GetAddEdNodeInfoItemAddressesItem>,
    /// If connected
    pub connected: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddEdNodeInfoItemAddressesItem {
    /// The bitcoin server IP and port we're connected to
    pub address: String,
    /// connection, inbound or outbound
    pub connected: String,
}

/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressInfo {
    /// The bitcoin address validated.
    pub address: String,
    /// A descriptor for spending coins sent to this address (only when solvable).
    pub desc: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    pub embedded: Option<GetAddressInfoEmbedded>,
    /// The HD keypath, if the key is HD and available.
    #[serde(rename = "hdkeypath")]
    pub hd_key_path: Option<String>,
    /// The fingerprint of the master key.
    #[serde(rename = "hdmasterfingerprint")]
    pub hd_masterfingerprint: Option<String>,
    /// The Hash160 of the HD seed.
    #[serde(rename = "hdseedid")]
    pub hd_seedid: Option<String>,
    /// The redeemscript for the p2sh address.
    pub hex: Option<String>,
    /// If the address was used for change output.
    #[serde(rename = "ischange")]
    pub is_change: bool,
    /// If the pubkey is compressed.
    pub iscompressed: Option<bool>,
    /// If the address is yours.
    #[serde(rename = "ismine")]
    pub is_min_e: bool,
    /// If the key is a script.
    #[serde(rename = "isscript")]
    pub is_script: Option<bool>,
    /// (DEPRECATED) Always false.
    #[serde(rename = "iswatchonly")]
    pub is_watch_only: bool,
    /// If the address is a witness address.
    #[serde(rename = "iswitness")]
    pub is_witness: bool,
    /// Array of labels associated with the address. Currently limited to one label but returned
    /// as an array to keep the API stable if multiple labels are enabled in the future.
    pub labels: Vec<String>,
    /// The descriptor used to derive this address if this is a descriptor wallet
    pub parent_desc: Option<String>,
    /// The hex value of the raw public key for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if script is multisig).
    pub pubkeys: Option<Vec<String>>,
    /// The output script type. Only if isscript is true and the redeemscript is known. Possible
    /// types: nonstandard, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_keyhash,
    /// witness_v0_scripthash, witness_unknown.
    pub script: Option<String>,
    /// The hex-encoded output script generated by the address.
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    /// The number of signatures required to spend multisig output (only if script is multisig).
    #[serde(rename = "sigsrequired")]
    pub sigs_required: Option<i64>,
    /// If we know how to spend coins sent to this address, ignoring the possible lack of private keys.
    pub solvable: bool,
    /// The creation time of the key, if available, expressed in UNIX epoch time.
    pub timestamp: Option<i64>,
    /// The hex value of the witness program.
    pub witness_program: Option<String>,
    /// The version number of the witness program.
    pub witness_version: Option<i64>,
}

/// Information about the address embedded in P2SH or P2WSH, if relevant and known.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressInfoEmbedded {}

/// Result of the JSON-RPC method `getaddressesbylabel`.
///
/// > getaddressesbylabel
/// >
/// > Returns the list of addresses assigned the specified label.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressesByLabel(
    /// json object with addresses as keys
    pub std::collections::BTreeMap<String, GetAddressesByLabelEntry>,
);

/// json object with information about address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressesByLabelEntry {
    /// Purpose of address ("send" for sending address, "receive" for receiving address)
    pub purpose: String,
}

/// Result of the JSON-RPC method `getaddrmaninfo`.
///
/// > getaddrmaninfo
/// >
/// > Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddrmanInfo(
    /// json object with network type as keys
    pub std::collections::BTreeMap<String, GetAddrmanInfoEntry>,
);

/// the network (ipv4, ipv6, onion, i2p, cjdns, all_networks)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddrmanInfoEntry {
    /// number of addresses in the new table, which represent potential peers the node has discovered but hasn't yet successfully connected to.
    pub new: i64,
    /// total number of addresses in both new/tried tables
    pub total: i64,
    /// number of addresses in the tried table, which represent peers the node has successfully connected to in the past.
    pub tried: i64,
}

/// Result of the JSON-RPC method `getbalance`.
///
/// > getbalance
/// >
/// > Returns the total available balance.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalance(pub String);

/// Returns an object with all balances in BTC.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalances {
    /// hash and height of the block this information was generated on
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetBalancesLastProcessEdBlock,
    /// balances from outputs that the wallet can sign
    #[serde(rename = "mine")]
    pub min_e: GetBalancesMine,
}

/// hash and height of the block this information was generated on
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalancesLastProcessEdBlock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: i64,
}

/// balances from outputs that the wallet can sign
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalancesMine {
    /// balance from immature coinbase outputs
    pub immature: f64,
    /// trusted balance (outputs created by the wallet or confirmed outputs)
    pub trusted: f64,
    /// untrusted pending balance (outputs created by others that are in the mempool)
    pub untrusted_pending: f64,
    /// (only present if avoid_reuse is set) balance from coins sent to addresses that were previously spent from (potentially privacy violating)
    pub used: Option<f64>,
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
pub struct GetBlockFromPeer {}

/// Result of the JSON-RPC method `getblockhash`.
///
/// > getblockhash
/// >
/// > Returns hash of block in best-block-chain at height provided.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHash(pub String);

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader \<hash\>.
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
/// If verbose is true, returns an Object with information about blockheader \<hash\>.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderVerboseZero(pub String);

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockStats {
    /// Average fee in the block
    #[serde(rename = "avgfee")]
    pub avg_fee: Option<i64>,
    /// Average feerate (in satoshis per virtual byte)
    #[serde(rename = "avgfeerate")]
    pub avg_fee_rate: Option<i64>,
    /// Average transaction size
    #[serde(rename = "avgtxsize")]
    pub avg_tx_size: Option<i64>,
    /// The block hash (to check for potential reorgs)
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    pub feerate_percentiles: Option<Vec<i64>>,
    /// The height of the block
    pub height: Option<i64>,
    /// The number of inputs (excluding coinbase)
    pub ins: Option<i64>,
    /// Maximum fee in the block
    #[serde(rename = "maxfee")]
    pub max_fee: Option<i64>,
    /// Maximum feerate (in satoshis per virtual byte)
    #[serde(rename = "maxfeerate")]
    pub max_fee_rate: Option<i64>,
    /// Maximum transaction size
    #[serde(rename = "maxtxsize")]
    pub max_tx_size: Option<i64>,
    /// Truncated median fee in the block
    #[serde(rename = "medianfee")]
    pub median_fee: Option<i64>,
    /// The block median time past
    pub mediantime: Option<i64>,
    /// Truncated median transaction size
    #[serde(rename = "mediantxsize")]
    pub median_tx_size: Option<i64>,
    /// Minimum fee in the block
    #[serde(rename = "minfee")]
    pub min_fee: Option<i64>,
    /// Minimum feerate (in satoshis per virtual byte)
    #[serde(rename = "minfeerate")]
    pub min_fee_rate: Option<i64>,
    /// Minimum transaction size
    #[serde(rename = "mintxsize")]
    pub min_tx_size: Option<i64>,
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
    #[serde(rename = "swtxs")]
    pub sw_tx_s: Option<i64>,
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
    #[serde(rename = "txs")]
    pub tx_s: Option<i64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    pub utxo_increase: Option<i64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    pub utxo_increase_actual: Option<i64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    pub utxo_size_inc: Option<i64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    pub utxo_size_inc_actual: Option<i64>,
}

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
///     https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
///     https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockTemplateVerboseOne(pub String);

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
///     https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
///     https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockTemplateVerboseTwo {
    /// compressed target of next block
    pub bits: String,
    pub capabilities: Vec<String>,
    /// data that should be included in the coinbase's scriptSig content
    #[serde(rename = "coinbaseaux")]
    pub coin_baseaux: std::collections::BTreeMap<String, String>,
    /// maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis)
    #[serde(rename = "coinbasevalue")]
    pub coin_base_value: i64,
    /// current timestamp in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    #[serde(rename = "curtime")]
    pub cur_time: i64,
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
    #[serde(rename = "sigoplimit")]
    pub sigop_limit: i64,
    /// limit of block size
    #[serde(rename = "sizelimit")]
    pub size_limit: i64,
    /// The hash target
    pub target: String,
    /// contents of non-coinbase transactions that should be included in the next block
    pub transactions: Vec<GetBlockTemplateVerboseTwoTransactionsItem>,
    /// set of pending, supported versionbit (BIP 9) softfork deployments
    pub vbavailable: std::collections::BTreeMap<String, i64>,
    /// bit mask of versionbits the server requires set in submissions
    #[serde(rename = "vbrequired")]
    pub vb_required: i64,
    /// The preferred block version
    pub version: i64,
    /// limit of block weight
    #[serde(rename = "weightlimit")]
    pub weight_limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockTemplateVerboseTwoTransactionsItem {
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

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block \<hash\>.
/// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
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
/// If verbosity is 1, returns an Object with information about block \<hash\>.
/// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
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
    pub prevout: GetBlockVerboseThreeTxItemVinItemPrevout,
}

/// (Only if undo information is available)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItemVinItemPrevout {
    /// Coinbase or not
    #[serde(rename = "generated")]
    pub gene_rate_d: bool,
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
/// If verbosity is 1, returns an Object with information about block \<hash\>.
/// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
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
/// If verbosity is 1, returns an Object with information about block \<hash\>.
/// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
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
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    #[serde(rename = "blocks")]
    pub block_s: i64,
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

/// Return information about chainstates.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStates {
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: Vec<GetChainStatesChainStatesItem>,
    /// the number of headers seen so far
    pub headers: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStatesChainStatesItem {
    /// blockhash of the tip
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// number of blocks in this chainstate
    #[serde(rename = "blocks")]
    pub block_s: i64,
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
    #[serde(rename = "validated")]
    pub validate_d: bool,
    /// progress towards the network tip
    #[serde(rename = "verificationprogress")]
    pub verification_progress: i64,
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
pub struct GetChainTxStats {
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

/// Result of the JSON-RPC method `getconnectioncount`.
///
/// > getconnectioncount
/// >
/// > Returns the number of connections to other nodes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetConnectionCount(pub i64);

/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfo {
    pub deployments: std::collections::BTreeMap<String, GetDeploymentInfoDeploymentS>,
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
pub struct GetDeploymentInfoDeploymentS {
    /// true if the rules are enforced for the mempool and the next block
    pub active: bool,
    /// status of bip9 softforks (only for "bip9" type)
    pub bip9: Option<GetDeploymentInfoDeploymentSBip9>,
    /// height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
    pub height: Option<i64>,
    /// one of "buried", "bip9"
    #[serde(rename = "type")]
    pub type_: String,
}

/// status of bip9 softforks (only for "bip9" type)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeploymentSBip9 {
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
    pub statistics: Option<GetDeploymentInfoDeploymentSBip9Statistics>,
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
pub struct GetDeploymentInfoDeploymentSBip9Statistics {
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
    pub activity: Vec<GetDescriptorActivityActivityItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityActivityItem {
    /// The total amount in BTC of the spent output
    pub amount: f64,
    /// The blockhash this spend appears in (omitted if unconfirmed)
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// Height of the spend (omitted if unconfirmed)
    pub height: Option<i64>,
    pub prevout_spk: GetDescriptorActivityActivityItemPrevoutSpk,
    /// The txid of the prevout
    pub prevout_txid: String,
    /// The vout of the prevout
    pub prevout_vout: i64,
    /// The txid of the spending transaction
    pub spend_txid: String,
    /// The input index of the spend
    pub spend_vin: i64,
    /// always 'spend'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityActivityItemPrevoutSpk {
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

/// Analyses a descriptor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorInfo {
    /// The checksum for the input descriptor
    pub checksum: String,
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// Whether the input descriptor contained at least one private key
    #[serde(rename = "hasprivatekeys")]
    pub hasprivate_keys: bool,
    /// Whether the descriptor is ranged
    pub isrange: bool,
    /// Whether the descriptor is solvable
    pub issolvable: bool,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    pub multipath_expansion: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `getdifficulty`.
///
/// > getdifficulty
/// >
/// > Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDifficulty(pub i64);

/// Result of the JSON-RPC method `gethdkeys`.
///
/// > gethdkeys
/// >
/// > List all BIP 32 HD keys in the wallet and which descriptors use them.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetHdKeys(pub Vec<GetHdKeysItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetHdKeysItem {
    /// Array of descriptor objects that use this HD key
    pub descriptors: Vec<GetHdKeysItemDescriptorsItem>,
    /// Whether the wallet has the private key for this xpub
    pub has_private: bool,
    /// The extended private key if "private" is true
    pub xprv: Option<String>,
    /// The extended public key
    pub xpub: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetHdKeysItemDescriptorsItem {
    /// Whether this descriptor is currently used to generate new addresses
    pub active: bool,
    /// Descriptor string representation
    pub desc: String,
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
    #[serde(rename = "synced")]
    pub sync_ed: bool,
}

/// Returns an object containing information about memory usage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoVerboseOne(pub String);

/// Returns an object containing information about memory usage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoVerboseZero {
    /// Information about locked memory manager
    #[serde(rename = "locked")]
    pub lock_ed: GetMemoryInfoVerboseZeroLockEd,
}

/// Information about locked memory manager
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoVerboseZeroLockEd {
    /// Number unused chunks
    pub chunks_free: i64,
    /// Number allocated chunks
    pub chunks_used: i64,
    /// Number of bytes available in current arenas
    pub free: i64,
    /// Amount of bytes that succeeded locking. If this number is smaller than total, locking pages failed at some point and key data could be swapped to disk.
    #[serde(rename = "locked")]
    pub lock_ed: i64,
    /// Total number of bytes managed
    pub total: i64,
    /// Number of bytes used
    pub used: i64,
}

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
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetMempoolAncestorsVerboseOneEntryFeeS,
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
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerboseOneEntryFeeS {
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

/// Returns mempool data for given cluster
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolCluster {
    /// chunks in this cluster (in mining order)
    pub chunks: Vec<GetMempoolClusterChunksItem>,
    /// total sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop')
    #[serde(rename = "clusterweight")]
    pub cluster_weight: i64,
    /// number of transactions
    #[serde(rename = "txcount")]
    pub tx_count: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolClusterChunksItem {
    /// fees of the transactions in this chunk
    #[serde(rename = "chunkfee")]
    pub chunk_fee: i64,
    /// sigops-adjusted weight of all transactions in this chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// transactions in this chunk in mining order
    #[serde(rename = "txs")]
    pub tx_s: Vec<String>,
}

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
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetMempoolDescendantsVerboseOneEntryFeeS,
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
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerboseOneEntryFeeS {
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
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetMempoolEntryFeeS,
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
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntryFeeS {
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

/// Result of the JSON-RPC method `getmempoolfeeratediagram`.
///
/// > getmempoolfeeratediagram
/// >
/// > Returns the feerate diagram for the whole mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolFeeRateDiagram(pub Vec<GetMempoolFeeRateDiagramItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolFeeRateDiagramItem {
    /// cumulative fee
    pub fee: i64,
    /// cumulative sigops-adjusted weight
    pub weight: i64,
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
    pub limit_cluster_count: i64,
    /// Maximum size of a cluster in virtual bytes (configured by -limitclustersize)
    #[serde(rename = "limitclustersize")]
    pub limit_cluster_size: i64,
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
    #[serde(rename = "blocks")]
    pub block_s: i64,
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

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotals {
    /// Current system UNIX epoch time in milliseconds
    #[serde(rename = "timemillis")]
    pub time_millis: i64,
    /// Total bytes received
    #[serde(rename = "totalbytesrecv")]
    pub total_bytes_recv: i64,
    /// Total bytes sent
    #[serde(rename = "totalbytessent")]
    pub total_bytes_sent: i64,
    #[serde(rename = "uploadtarget")]
    pub upload_target: GetNetTotalsUpLoadTarGet,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotalsUpLoadTarGet {
    /// Bytes left in current time cycle
    pub bytes_left_in_cycle: i64,
    /// True if serving historical blocks
    pub serve_historical_blocks: bool,
    /// Target in bytes
    pub target: i64,
    /// True if target is reached
    pub target_reached: bool,
    /// Seconds left in current time cycle
    pub time_left_in_cycle: i64,
    /// Length of the measuring timeframe in seconds
    #[serde(rename = "timeframe")]
    pub time_frame: i64,
}

/// Result of the JSON-RPC method `getnetworkhashps`.
///
/// > getnetworkhashps
/// >
/// > Returns the estimated network hashes per second based on the last n blocks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkHashps(pub i64);

/// Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfo {
    /// the total number of connections
    pub connections: i64,
    /// the number of inbound connections
    pub connections_in: i64,
    /// the number of outbound connections
    pub connections_out: i64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: i64,
    /// list of local addresses
    #[serde(rename = "localaddresses")]
    pub local_addresses: Vec<GetNetworkInfoLocalAddressesItem>,
    /// true if transaction relay is requested from peers
    #[serde(rename = "localrelay")]
    pub local_relay: bool,
    /// the services we offer to the network
    pub localservices: String,
    /// the services we offer to the network, in human-readable form
    #[serde(rename = "localservicesnames")]
    pub localservices_names: Vec<String>,
    /// whether p2p networking is enabled
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// information per network
    pub networks: Vec<GetNetworkInfoNetworkSItem>,
    /// the protocol version
    #[serde(rename = "protocolversion")]
    pub protocol_version: i64,
    /// minimum relay fee rate for transactions in BTC/kvB
    #[serde(rename = "relayfee")]
    pub relay_fee: i64,
    /// the server subversion string
    #[serde(rename = "subversion")]
    pub sub_version: String,
    /// the time offset
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    /// the server version
    pub version: i64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoLocalAddressesItem {
    /// network address
    pub address: String,
    /// network port
    pub port: i64,
    /// relative score
    pub score: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoNetworkSItem {
    /// is the network limited using -onlynet?
    #[serde(rename = "limited")]
    pub limit_ed: bool,
    /// network (ipv4, ipv6, onion, i2p, cjdns)
    pub name: String,
    /// ("host:port") the proxy that is used for this network, or empty if none
    pub proxy: String,
    /// Whether randomized credentials are used
    pub proxy_randomize_credentials: bool,
    /// is the network reachable?
    pub reachable: bool,
}

/// Result of the JSON-RPC method `getnewaddress`.
///
/// > getnewaddress
/// >
/// > Returns a new Bitcoin address for receiving payments.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNewAddress(pub String);

/// Result of the JSON-RPC method `getnodeaddresses`.
///
/// > getnodeaddresses
/// >
/// > Return known addresses, after filtering for quality and recency.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNodeAddresses(pub Vec<GetNodeAddressesItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNodeAddressesItem {
    /// The address of the node
    pub address: String,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) the node connected through
    pub network: String,
    /// The port number of the node
    pub port: i64,
    /// The services offered by the node
    pub services: i64,
    /// The UNIX epoch time when the node was last seen
    pub time: i64,
}

/// Return an OpenRPC document describing the RPC API.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOpenRpc {}

/// Shows transactions in the tx orphanage.
/// 
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxSVerboseOne(pub Vec<GetOrphanTxSVerboseOneItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxSVerboseOneItem {
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
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Shows transactions in the tx orphanage.
/// 
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxSVerboseTwo(pub Vec<GetOrphanTxSVerboseTwoItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxSVerboseTwoItem {
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
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Shows transactions in the tx orphanage.
/// 
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetOrphanTxSVerboseZero(pub Vec<String>);

/// Result of the JSON-RPC method `getpeerinfo`.
///
/// > getpeerinfo
/// >
/// > Returns data about each connected network peer as a json array of objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPeerInfo(pub Vec<GetPeerInfoItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPeerInfoItem {
    /// (host:port) The IP address/hostname optionally followed by :port of the peer
    pub addr: String,
    /// The total number of addresses processed, excluding those dropped due to rate limiting
    pub addr_processed: i64,
    /// The total number of addresses dropped due to rate limiting
    pub addr_rate_limited: i64,
    /// Whether we participate in address relay with this peer
    pub addr_relay_enabled: bool,
    /// (ip:port) Bind address of the connection to the peer
    pub addrbind: Option<String>,
    /// (ip:port) Local address as reported by the peer
    pub addrlocal: Option<String>,
    /// Whether peer selected us as (compact blocks) high-bandwidth peer
    pub bip152_hb_from: bool,
    /// Whether we selected peer as (compact blocks) high-bandwidth peer
    pub bip152_hb_to: bool,
    /// The total bytes received
    #[serde(rename = "bytesrecv")]
    pub bytes_recv: i64,
    pub bytesrecv_per_msg: std::collections::BTreeMap<String, i64>,
    /// The total bytes sent
    #[serde(rename = "bytessent")]
    pub bytes_sent: i64,
    pub bytessent_per_msg: std::collections::BTreeMap<String, i64>,
    /// Type of connection: 
    /// outbound-full-relay (default automatic connections),
    /// block-relay-only (does not relay transactions or addresses),
    /// inbound (initiated by the peer),
    /// manual (added via addnode RPC or -addnode/-connect configuration options),
    /// addr-fetch (short-lived automatic connection for soliciting addresses),
    /// feeler (short-lived automatic connection for testing addresses),
    /// private-broadcast (short-lived automatic connection for broadcasting privacy-sensitive transactions).
    /// Please note this output is unlikely to be stable in upcoming releases as we iterate to
    /// best capture connection behaviors.
    pub connection_type: String,
    /// The UNIX epoch time of the connection
    #[serde(rename = "conntime")]
    pub conn_time: i64,
    /// Peer index
    pub id: i64,
    /// Inbound (true) or Outbound (false)
    pub inbound: bool,
    pub inflight: Vec<i64>,
    /// How many txs we have queued to announce to this peer
    pub inv_to_send: i64,
    /// The UNIX epoch time of the last block received from this peer
    pub last_block: i64,
    /// Mempool sequence number of this peer's last INV
    pub last_inv_sequence: i64,
    /// The UNIX epoch time of the last valid transaction received from this peer
    pub last_transaction: i64,
    /// The UNIX epoch time of the last receive
    #[serde(rename = "lastrecv")]
    pub last_recv: i64,
    /// The UNIX epoch time of the last send
    #[serde(rename = "lastsend")]
    pub last_send: i64,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying
    /// peer selection (only displayed if the -asmap config option is set)
    pub mapped_as: Option<i64>,
    /// The minimum fee rate for transactions this peer accepts
    #[serde(rename = "minfeefilter")]
    pub min_fee_filter: i64,
    /// The minimum observed ping time in seconds, if any
    #[serde(rename = "minping")]
    pub min_ping: Option<i64>,
    /// Network (ipv4, ipv6, onion, i2p, cjdns, not_publicly_routable)
    pub network: String,
    /// Any special permissions that have been granted to this peer
    pub permissions: Vec<String>,
    /// The last ping time in seconds, if any
    #[serde(rename = "pingtime")]
    pub ping_time: Option<i64>,
    /// The duration in seconds of an outstanding ping (if non-zero)
    pub pingwait: Option<i64>,
    /// The current height of header pre-synchronization with this peer, or -1 if no low-work sync is in progress
    pub presynced_headers: i64,
    /// Whether we relay transactions to this peer
    #[serde(rename = "relaytxes")]
    pub relay_tx_es: bool,
    /// The services offered
    pub services: String,
    /// the services offered, in human-readable form
    #[serde(rename = "servicesnames")]
    pub services_names: Vec<String>,
    /// The session ID for this connection, or "" if there is none ("v2" transport protocol only).
    /// 
    pub session_id: String,
    /// (DEPRECATED, returned only if config option -deprecatedrpc=startingheight is passed) The starting height (block) of the peer
    #[serde(rename = "startingheight")]
    pub start_ing_height: Option<i64>,
    /// The string version
    pub subver: String,
    /// The last block we have in common with this peer
    pub synced_blocks: i64,
    /// The last header we have in common with this peer
    pub synced_headers: i64,
    /// The time offset in seconds
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    /// Type of transport protocol: 
    /// detecting (peer could be v1 or v2),
    /// v1 (plaintext transport protocol),
    /// v2 (BIP324 encrypted transport protocol).
    /// 
    pub transport_protocol_type: String,
    /// The peer version, such as 70001
    pub version: i64,
}

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

/// the location in the address manager table (\<bucket\>/\<position\>)
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

/// Result of the JSON-RPC method `getrawchangeaddress`.
///
/// > getrawchangeaddress
/// >
/// > Returns a new Bitcoin address, for receiving change.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawChangeAddress(pub String);

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
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetRawMempoolVerboseOneEntryFeeS,
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
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerboseOneEntryFeeS {
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

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// 
/// Hint: Use gettransaction for wallet transactions.
/// 
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOne {
    /// the block hash
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The block time expressed in UNIX epoch time
    #[serde(rename = "blocktime")]
    pub block_time: Option<i64>,
    /// The confirmations
    pub confirmations: Option<i64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized, hex-encoded data for 'txid'
    pub hex: String,
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
    pub in_active_chain: Option<bool>,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// Same as "blocktime"
    pub time: Option<i64>,
    /// The transaction id (same as provided)
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<GetRawTransactionVerboseOneVinItem>,
    #[serde(rename = "vout")]
    pub v_out: Vec<GetRawTransactionVerboseOneVOutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOneVOutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetRawTransactionVerboseOneVOutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOneVOutItemScriptPubKey {
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOneVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(rename = "coinbase")]
    pub coin_base: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<GetRawTransactionVerboseOneVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    pub txid: Option<String>,
    #[serde(rename = "txinwitness")]
    pub tx_in_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(rename = "vout")]
    pub v_out: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOneVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// 
/// Hint: Use gettransaction for wallet transactions.
/// 
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseTwo {
    /// transaction fee in BTC, omitted if block undo data is not available
    pub fee: Option<i64>,
    pub vin: Vec<GetRawTransactionVerboseTwoVinItem>,
}

/// utxo being spent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseTwoVinItem {
    /// The previous output, omitted if block undo data is not available
    pub prevout: Option<GetRawTransactionVerboseTwoVinItemPrevout>,
}

/// The previous output, omitted if block undo data is not available
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseTwoVinItemPrevout {
    /// Coinbase or not
    #[serde(rename = "generated")]
    pub gene_rate_d: bool,
    /// The height of the prevout
    pub height: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey {
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

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// 
/// Hint: Use gettransaction for wallet transactions.
/// 
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseZero(pub String);

/// Result of the JSON-RPC method `getreceivedbyaddress`.
///
/// > getreceivedbyaddress
/// >
/// > Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetReceivedByAddress(pub String);

/// Result of the JSON-RPC method `getreceivedbylabel`.
///
/// > getreceivedbylabel
/// >
/// > Returns the total amount received by addresses with \<label\> in transactions with at least [minconf] confirmations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetReceivedByLabel(pub String);

/// Returns details of the RPC server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfo {
    /// All active commands
    pub active_commands: Vec<GetRpcInfoActiveCommandsItem>,
    /// The complete file path to the debug log
    #[serde(rename = "logpath")]
    pub log_path: String,
}

/// Information about an active command
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfoActiveCommandsItem {
    /// The running time in microseconds
    pub duration: i64,
    /// The name of the RPC command
    pub method: String,
}

/// Get detailed information about in-wallet transaction \<txid\>
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransaction {
    /// The amount in BTC
    pub amount: f64,
    /// ("yes|no|unknown") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The block height containing the transaction.
    #[serde(rename = "blockheight")]
    pub block_height: Option<i64>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<i64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(rename = "blocktime")]
    pub block_time: Option<i64>,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The decoded transaction (only present when `verbose` is passed)
    pub decoded: Option<GetTransactionDecodeD>,
    pub details: Vec<GetTransactionDetailsItem>,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    pub fee: Option<f64>,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(rename = "generated")]
    pub gene_rate_d: Option<bool>,
    /// Raw data for transaction
    pub hex: String,
    /// hash and height of the block this information was generated on
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetTransactionLastProcessEdBlock,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conf_licts: Vec<String>,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<Vec<String>>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: i64,
    /// The time received expressed in UNIX epoch time.
    #[serde(rename = "timereceived")]
    pub time_received: i64,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: String,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conf_licts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// The decoded transaction (only present when `verbose` is passed)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodeD {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<GetTransactionDecodeDVinItem>,
    #[serde(rename = "vout")]
    pub v_out: Vec<GetTransactionDecodeDVOutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodeDVOutItem {
    /// Output script is change (only present if true)
    #[serde(rename = "ischange")]
    pub is_change: Option<bool>,
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetTransactionDecodeDVOutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodeDVOutItemScriptPubKey {
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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodeDVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(rename = "coinbase")]
    pub coin_base: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<GetTransactionDecodeDVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    pub txid: Option<String>,
    #[serde(rename = "txinwitness")]
    pub tx_in_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(rename = "vout")]
    pub v_out: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodeDVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDetailsItem {
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// The bitcoin address involved in the transaction.
    pub address: Option<String>,
    /// The amount in BTC
    pub amount: f64,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// The amount of the fee in BTC. This is negative and only available for the 
    /// 'send' category of transactions.
    pub fee: Option<f64>,
    /// A comment for the address/transaction, if any
    pub label: Option<String>,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<Vec<String>>,
    /// the vout value
    #[serde(rename = "vout")]
    pub v_out: i64,
}

/// hash and height of the block this information was generated on
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionLastProcessEdBlock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: i64,
}

/// Result of the JSON-RPC method `gettxoutproof`.
///
/// > gettxoutproof
/// >
/// > Returns a hex-encoded proof that "txid" was included in a block.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutProof(pub String);

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfo {
    /// The hash of the block at which these statistics are calculated
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    pub block_info: Option<GetTxOutSetInfoBlockInfo>,
    /// Database-independent, meaningless metric indicating the UTXO set size
    #[serde(rename = "bogosize")]
    pub bogo_size: i64,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    pub disk_size: Option<i64>,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    pub hash_serialized_3: Option<String>,
    /// The block height (index) of the returned statistics
    pub height: i64,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    #[serde(rename = "muhash")]
    pub mu_hash: Option<String>,
    /// The total amount of coins in the UTXO set
    pub total_amount: f64,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    pub total_unspendable_amount: Option<f64>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    pub transactions: Option<i64>,
    /// The number of unspent transaction outputs
    #[serde(rename = "txouts")]
    pub tx_out_s: i64,
}

/// Info on amounts in the block at this block height (only available if coinstatsindex is used)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoBlockInfo {
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
    pub unspendables: GetTxOutSetInfoBlockInfoUnspendables,
}

/// Detailed view of the unspendable categories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoBlockInfoUnspendables {
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
pub struct GetTxOutVerboseOne {
    /// The hash of the block at the tip of the chain
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// Coinbase or not
    #[serde(rename = "coinbase")]
    pub coin_base: bool,
    /// The number of confirmations
    pub confirmations: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetTxOutVerboseOneScriptPubKey,
    /// The transaction value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutVerboseOneScriptPubKey {
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
pub struct GetTxSpendingPrevout(pub Vec<GetTxSpendingPrevoutItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevoutItem {
    /// the transaction id of the mempool transaction spending this output (omitted if unspent)
    #[serde(rename = "spendingtxid")]
    pub spending_txid: Option<String>,
    /// the transaction id of the checked output
    pub txid: String,
    /// the vout value of the checked output
    #[serde(rename = "vout")]
    pub v_out: i64,
}

/// Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfo {
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    #[serde(rename = "birthtime")]
    pub birth_time: Option<i64>,
    /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
    pub blank: bool,
    /// whether this wallet uses descriptors for output script management
    pub descriptors: bool,
    /// whether this wallet is configured to use an external signer such as a hardware wallet
    pub external_signer: bool,
    /// The flags currently set on the wallet
    pub flags: Vec<String>,
    /// the database format (only sqlite)
    pub format: String,
    /// how many new keys are pre-generated (only counts external keys)
    #[serde(rename = "keypoolsize")]
    pub key_pool_size: i64,
    /// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
    pub keypoolsize_hd_internal: Option<i64>,
    /// hash and height of the block this information was generated on
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetWalletInfoLastProcessEdBlock,
    /// the transaction fee configuration, set in BTC/kvB
    #[serde(rename = "paytxfee")]
    pub pay_tx_fee: f64,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: GetWalletInfoScanNing,
    /// the total number of transactions in the wallet
    #[serde(rename = "txcount")]
    pub tx_count: i64,
    /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
    pub unlocked_until: Option<i64>,
    /// the wallet name
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    /// (DEPRECATED) only related to unsupported legacy wallet, returns the latest version 169900 for backwards compatibility
    #[serde(rename = "walletversion")]
    pub wallet_version: i64,
}

/// hash and height of the block this information was generated on
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfoLastProcessEdBlock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: i64,
}

/// current scanning details, or false if no scan is in progress
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfoScanNing {
    /// elapsed seconds since scan start
    pub duration: i64,
    /// scanning progress percentage [0.0, 1.0]
    pub progress: i64,
}

/// Result of the JSON-RPC method `help`.
///
/// > help
/// >
/// > List all commands, or get help for a specified command.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Help(pub String);

/// Result of the JSON-RPC method `importdescriptors`.
///
/// > importdescriptors
/// >
/// > Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportDescriptors(pub Vec<ImportDescriptorsItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportDescriptorsItem {
    pub error: Option<ImportDescriptorsItemError>,
    pub success: bool,
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportDescriptorsItemError {}

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportMempool {}

/// Result of the JSON-RPC method `joinpsbts`.
///
/// > joinpsbts
/// >
/// > Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct JoinPsbts(pub String);

/// Result of the JSON-RPC method `listaddressgroupings`.
///
/// > listaddressgroupings
/// >
/// > Lists groups of addresses which have had their common ownership
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListAddressGroupings(pub Vec<Vec<Vec<String>>>);

/// Result of the JSON-RPC method `listbanned`.
///
/// > listbanned
/// >
/// > List all manually banned IPs/Subnets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListBanned(pub Vec<ListBannedItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListBannedItem {
    /// The IP/Subnet of the banned node
    pub address: String,
    /// The UNIX epoch time the ban was created
    pub ban_created: i64,
    /// The ban duration, in seconds
    pub ban_duration: i64,
    /// The UNIX epoch time the ban expires
    pub banned_until: i64,
    /// The time remaining until the ban expires, in seconds
    pub time_remaining: i64,
}

/// List all descriptors present in a wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListDescriptors {
    /// Array of descriptor objects (sorted by descriptor string representation)
    pub descriptors: Vec<ListDescriptorsDescriptorsItem>,
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListDescriptorsDescriptorsItem {
    /// Whether this descriptor is currently used to generate new addresses
    pub active: bool,
    /// Descriptor string representation
    pub desc: String,
    /// True if this descriptor is used to generate change addresses. False if this descriptor is used to generate receiving addresses; defined only for active descriptors
    pub internal: Option<bool>,
    /// Same as next_index field. Kept for compatibility reason.
    pub next: Option<i64>,
    /// The next index to generate addresses from; defined only for ranged descriptors
    pub next_index: Option<i64>,
    /// Defined only for ranged descriptors
    pub range: Option<Vec<i64>>,
    /// The creation time of the descriptor
    pub timestamp: i64,
}

/// Result of the JSON-RPC method `listlabels`.
///
/// > listlabels
/// >
/// > Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListLabels(pub Vec<String>);

/// Result of the JSON-RPC method `listlockunspent`.
///
/// > listlockunspent
/// >
/// > Returns list of temporarily unspendable outputs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListLockUnspent(pub Vec<ListLockUnspentItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListLockUnspentItem {
    /// The transaction id locked
    pub txid: String,
    /// The vout value
    #[serde(rename = "vout")]
    pub v_out: i64,
}

/// Result of the JSON-RPC method `listreceivedbyaddress`.
///
/// > listreceivedbyaddress
/// >
/// > List balances by receiving address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListReceivedByAddress(pub Vec<ListReceivedByAddressItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListReceivedByAddressItem {
    /// The receiving address
    pub address: String,
    /// The total amount in BTC received by the address
    pub amount: f64,
    /// The number of confirmations of the most recent transaction included
    pub confirmations: i64,
    /// The label of the receiving address. The default label is ""
    pub label: String,
    pub txids: Vec<String>,
}

/// Result of the JSON-RPC method `listreceivedbylabel`.
///
/// > listreceivedbylabel
/// >
/// > List received transactions by label.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListReceivedByLabel(pub Vec<ListReceivedByLabelItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListReceivedByLabelItem {
    /// The total amount received by addresses with this label
    pub amount: f64,
    /// The number of confirmations of the most recent transaction included
    pub confirmations: i64,
    /// The label of the receiving address. The default label is ""
    pub label: String,
}

/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListSinceBlock {
    /// The hash of the block (target_confirmations-1) from the best block on the main chain, or the genesis hash if the referenced block does not exist yet. This is typically used to feed back into listsinceblock the next time you call it. So you would generally use a target_confirmations of say 6, so you will be continually re-notified of transactions until they've reached 6 confirmations plus any new ones
    #[serde(rename = "lastblock")]
    pub last_block: String,
    /// <structure is the same as "transactions" above, only present if include_removed=true>
    /// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
    pub removed: Option<Vec<serde_json::Value>>,
    pub transactions: Vec<ListSinceBlockTransactionsItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListSinceBlockTransactionsItem {
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// The bitcoin address of the transaction (not returned if the output does not have an address, e.g. OP_RETURN null data).
    pub address: Option<String>,
    /// The amount in BTC. This is negative for the 'send' category, and is positive
    /// for all other categories
    pub amount: f64,
    /// ("yes|no|unknown") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The block height containing the transaction.
    #[serde(rename = "blockheight")]
    pub block_height: Option<i64>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<i64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(rename = "blocktime")]
    pub block_time: Option<i64>,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    pub fee: Option<f64>,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(rename = "generated")]
    pub gene_rate_d: Option<bool>,
    /// A comment for the address/transaction, if any
    pub label: Option<String>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conf_licts: Vec<String>,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<Vec<String>>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: i64,
    /// The time received expressed in UNIX epoch time.
    #[serde(rename = "timereceived")]
    pub time_received: i64,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: String,
    /// the vout value
    #[serde(rename = "vout")]
    pub v_out: i64,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conf_licts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Result of the JSON-RPC method `listtransactions`.
///
/// > listtransactions
/// >
/// > If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListTransactions(pub Vec<ListTransactionsItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListTransactionsItem {
    /// 'true' if the transaction has been abandoned (inputs are respendable).
    pub abandoned: bool,
    /// The bitcoin address of the transaction (not returned if the output does not have an address, e.g. OP_RETURN null data).
    pub address: Option<String>,
    /// The amount in BTC. This is negative for the 'send' category, and is positive
    /// for all other categories
    pub amount: f64,
    /// ("yes|no|unknown") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The block height containing the transaction.
    #[serde(rename = "blockheight")]
    pub block_height: Option<i64>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<i64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(rename = "blocktime")]
    pub block_time: Option<i64>,
    /// The transaction category.
    /// "send"                  Transactions sent.
    /// "receive"               Non-coinbase transactions received.
    /// "generate"              Coinbase transactions received with more than 100 confirmations.
    /// "immature"              Coinbase transactions received with 100 or fewer confirmations.
    /// "orphan"                Orphaned coinbase transactions received.
    pub category: String,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    pub fee: Option<f64>,
    /// Only present if the transaction's only input is a coinbase one.
    #[serde(rename = "generated")]
    pub gene_rate_d: Option<bool>,
    /// A comment for the address/transaction, if any
    pub label: Option<String>,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    #[serde(rename = "mempoolconflicts")]
    pub mempool_conf_licts: Vec<String>,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<Vec<String>>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: i64,
    /// The time received expressed in UNIX epoch time.
    #[serde(rename = "timereceived")]
    pub time_received: i64,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: String,
    /// the vout value
    #[serde(rename = "vout")]
    pub v_out: i64,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conf_licts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Result of the JSON-RPC method `listunspent`.
///
/// > listunspent
/// >
/// > Returns array of unspent transaction outputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListUnspent(pub Vec<ListUnspentItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListUnspentItem {
    /// the bitcoin address
    pub address: Option<String>,
    /// the transaction output amount in BTC
    pub amount: f64,
    /// The number of in-mempool ancestor transactions, including this one (if transaction is in the mempool)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: Option<i64>,
    /// The total fees of in-mempool ancestors (including this one) with fee deltas used for mining priority in sat (if transaction is in the mempool)
    #[serde(rename = "ancestorfees")]
    pub ancestor_fees: Option<f64>,
    /// The virtual transaction size of in-mempool ancestors, including this one (if transaction is in the mempool)
    #[serde(rename = "ancestorsize")]
    pub ancestors_ize: Option<i64>,
    /// The number of confirmations
    pub confirmations: i64,
    /// (only when solvable) A descriptor for spending this output
    pub desc: Option<String>,
    /// The associated label, or "" for the default label
    pub label: Option<String>,
    /// List of parent descriptors for the output script of this coin.
    pub parent_descs: Vec<String>,
    /// The redeem script if the output script is P2SH
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<String>,
    /// (only present if avoid_reuse is set) Whether this output is reused/dirty (sent to an address that was previously spent from)
    #[serde(rename = "reused")]
    pub re_used: Option<bool>,
    /// Whether this output is considered safe to spend. Unconfirmed transactions
    /// from outside keys and unconfirmed replacement transactions are considered unsafe
    /// and are not eligible for spending by fundrawtransaction and sendtoaddress.
    pub safe: bool,
    /// the output script
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    /// Whether we know how to spend this output, ignoring the lack of keys
    pub solvable: bool,
    /// (DEPRECATED) Always true
    pub spendable: bool,
    /// the transaction id
    pub txid: String,
    /// the vout value
    #[serde(rename = "vout")]
    pub v_out: i64,
    /// witness script if the output script is P2WSH or P2SH-P2WSH
    #[serde(rename = "witnessScript")]
    pub witness_script: Option<String>,
}

/// Returns a list of wallets in the wallet directory.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListWalletDir {
    pub wallets: Vec<ListWalletDirWalletsItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListWalletDirWalletsItem {
    /// The wallet name
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `listwallets`.
///
/// > listwallets
/// >
/// > Returns a list of currently loaded wallets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListWallets(pub Vec<String>);

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// 
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// 
/// You can find more information on this process in the `assumeutxo` design document (\<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md\>).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxOutSet {
    /// the height of the base of the snapshot
    pub base_height: i64,
    /// the number of coins loaded from the snapshot
    pub coins_loaded: i64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
}

/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `lockunspent`.
///
/// > lockunspent
/// >
/// > Updates list of temporarily unspendable outputs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LockUnspent(pub bool);

/// Result of the JSON-RPC method `logging`.
///
/// > logging
/// >
/// > Gets and sets the logging configuration.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Logging(
    /// keys are the logging categories, and values indicates its status
    pub std::collections::BTreeMap<String, bool>,
);

/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// 
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named \<wallet name\>-\<timestamp\>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// 
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MigRateWallet {
    /// The location of the backup of the original wallet
    pub backup_path: String,
    /// The name of the migrated wallet containing solvable but not watched scripts
    pub solvables_name: Option<String>,
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    pub watchonly_name: Option<String>,
}

/// Result of the JSON-RPC method `prioritisetransaction`.
///
/// > prioritisetransaction
/// >
/// > Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PrioritiseTransaction(pub bool);

/// Result of the JSON-RPC method `pruneblockchain`.
///
/// > pruneblockchain
/// >
/// > Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PruneBlockchain(pub i64);

/// Bumps the fee of a transaction T, replacing it with a new transaction B.
/// Returns a PSBT instead of creating and signing a new transaction.
/// A transaction with the given txid must be in the wallet.
/// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
/// It may add a new change output if one does not already exist.
/// All inputs in the original transaction will be included in the replacement transaction.
/// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
/// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
/// The user can specify a confirmation target for estimatesmartfee.
/// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
/// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// returned by getnetworkinfo) to enter the node's mempool.
/// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtBumpFee {
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
    /// The fee of the new transaction.
    pub fee: f64,
    /// The fee of the replaced transaction.
    #[serde(rename = "origfee")]
    pub orig_fee: f64,
    /// The base64-encoded unsigned PSBT of the new transaction.
    pub psbt: String,
}

/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RescanBlockchain {
    /// The block height where the rescan started (the requested height or 0)
    pub start_height: i64,
    /// The height of the last rescanned block. May be null in rare cases if there was a reorg and the call didn't scan any blocks because they were already scanned in the background.
    pub stop_height: i64,
}

/// Restores and loads a wallet from backup.
/// 
/// The rescan is significantly faster if block filters are available
/// (using startup option "-blockfilterindex=1").
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RestoreWallet {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning messages, if any, related to restoring and loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SaveMempool {
    /// the directory and file where the mempool was saved
    #[serde(rename = "filename")]
    pub file_name: String,
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlockSVerboseOne {
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
pub struct ScanBlockSVerboseThree(pub bool);

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlockSVerboseTwo {
    /// Height of the block currently being scanned
    pub current_height: i64,
    /// Approximate percent complete
    pub progress: i64,
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(\<address\>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(\<hex script\>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(\<pubkey\>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(\<pubkey\>)                        P2PKH outputs for the given pubkey
///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(\<pubkey\>)                         P2TR
///     tr(\<pubkey\>,{pk(\<pubkey\>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(\<pubkey\>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(\<pubkey\>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetVerboseOne(pub bool);

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(\<address\>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(\<hex script\>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(\<pubkey\>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(\<pubkey\>)                        P2PKH outputs for the given pubkey
///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(\<pubkey\>)                         P2TR
///     tr(\<pubkey\>,{pk(\<pubkey\>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(\<pubkey\>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(\<pubkey\>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetVerboseTwo {
    /// Approximate percent complete
    pub progress: i64,
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(\<address\>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(\<hex script\>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(\<pubkey\>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(\<pubkey\>)                        P2PKH outputs for the given pubkey
///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(\<pubkey\>)                         P2TR
///     tr(\<pubkey\>,{pk(\<pubkey\>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(\<pubkey\>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(\<pubkey\>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetVerboseZero {
    /// The hash of the block at the tip of the chain
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The block height at which the scan was done
    pub height: i64,
    /// Whether the scan was completed
    pub success: bool,
    /// The total amount of all found unspent outputs in BTC
    pub total_amount: f64,
    /// The number of unspent transaction outputs scanned
    #[serde(rename = "txouts")]
    pub tx_out_s: i64,
    #[serde(rename = "unspents")]
    pub unspent_s: Vec<ScanTxOutSetVerboseZeroUnspentSItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetVerboseZeroUnspentSItem {
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
    #[serde(rename = "vout")]
    pub v_out: i64,
}

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// 
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendAll {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Option<String>,
}

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendManyVerboseOne {
    /// The transaction fee reason.
    pub fee_reason: String,
    /// The transaction id for the send. Only 1 transaction is created regardless of
    /// the number of addresses.
    pub txid: String,
}

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendManyVerboseZero(pub String);

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendMsgToPeer {}

/// Result of the JSON-RPC method `sendrawtransaction`.
///
/// > sendrawtransaction
/// >
/// > Submit a raw transaction (serialized, hex-encoded) to the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendRawTransaction(pub String);

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// 
/// Send a transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendResult {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Option<String>,
}

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendToAddressVerboseOne {
    /// The transaction fee reason.
    pub fee_reason: String,
    /// The transaction id.
    pub txid: String,
}

/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendToAddressVerboseZero(pub String);

/// Result of the JSON-RPC method `setnetworkactive`.
///
/// > setnetworkactive
/// >
/// > Disable/enable all p2p network activity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetNetworkActive(pub bool);

/// Result of the JSON-RPC method `settxfee`.
///
/// > settxfee
/// >
/// > (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetTxFee(pub bool);

/// Change the state of the given wallet flag for a wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetWalletFlag {
    /// The name of the flag that was modified
    pub flag_name: String,
    /// The new state of the flag
    pub flag_state: bool,
    /// Any warnings associated with the change
    pub warnings: Option<String>,
}

/// Result of the JSON-RPC method `signmessage`.
///
/// > signmessage
/// >
/// > Sign a message with the private key of an address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignMessage(pub String);

/// Result of the JSON-RPC method `signmessagewithprivkey`.
///
/// > signmessagewithprivkey
/// >
/// > Sign a message with the private key of an address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignMessageWithPrivKey(pub String);

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithKey {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Option<Vec<SignRawTransactionWithKeyErrorsItem>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithKeyErrorsItem {
    /// Verification or signing error related to the input
    pub error: String,
    /// The hex-encoded signature script
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number
    pub sequence: i64,
    /// The hash of the referenced, previous transaction
    pub txid: String,
    /// The index of the output to spent and used as input
    #[serde(rename = "vout")]
    pub v_out: i64,
    pub witness: Vec<String>,
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithWallet {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Option<Vec<SignRawTransactionWithWalletErrorsItem>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithWalletErrorsItem {
    /// Verification or signing error related to the input
    pub error: String,
    /// The hex-encoded signature script
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number
    pub sequence: i64,
    /// The hash of the referenced, previous transaction
    pub txid: String,
    /// The index of the output to spent and used as input
    #[serde(rename = "vout")]
    pub v_out: i64,
    pub witness: Vec<String>,
}

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SimulateRawTransaction {
    /// The wallet balance change (negative means decrease).
    pub balance_change: f64,
}

/// Result of the JSON-RPC method `stop`.
///
/// > stop
/// >
/// > Request a graceful shutdown of Bitcoin Core.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Stop(pub String);

/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitBlockVerboseOne(pub String);

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackage {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// List of txids of replaced transactions
    #[serde(rename = "replaced-transactions")]
    pub replaced_transactions: Option<Vec<String>>,
    /// The transaction results keyed by wtxid. An entry is returned for every submitted wtxid.
    #[serde(rename = "tx-results")]
    pub tx_results: std::collections::BTreeMap<String, SubmitPackageTxResults>,
}

/// transaction wtxid
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageTxResults {
    /// Error string if rejected from mempool, or "package-not-validated" when the package aborts before any per-tx processing.
    pub error: Option<String>,
    /// Transaction fees
    pub fees: Option<SubmitPackageTxResultsFeeS>,
    /// The wtxid of a different transaction with the same txid but different witness found in the mempool. This means the submitted transaction was ignored.
    #[serde(rename = "other-wtxid")]
    pub other_wtxid: Option<String>,
    /// The transaction hash in hex
    pub txid: String,
    /// Sigops-adjusted virtual transaction size.
    pub vsize: Option<i64>,
}

/// Transaction fees
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageTxResultsFeeS {
    /// transaction fee in BTC
    pub base: f64,
    /// if the transaction was not already in the mempool, the effective feerate in BTC per KvB. For example, the package feerate and/or feerate with modified fees from prioritisetransaction.
    #[serde(rename = "effective-feerate")]
    pub effective_feerate: Option<f64>,
    /// if effective-feerate is provided, the wtxids of the transactions whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes")]
    pub effective_includes: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `testmempoolaccept`.
///
/// > testmempoolaccept
/// >
/// > Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAccept(pub Vec<TestMempoolAcceptItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAcceptItem {
    /// Whether this tx would be accepted to the mempool and pass client-specified maxfeerate. If not present, the tx was not fully validated due to a failure in another tx in the list.
    pub allowed: Option<bool>,
    /// Transaction fees (only present if 'allowed' is true)
    pub fees: Option<TestMempoolAcceptItemFeeS>,
    /// Package validation error, if any (only possible if rawtxs had more than 1 transaction).
    #[serde(rename = "package-error")]
    pub package_error: Option<String>,
    /// Rejection details (only present when 'allowed' is false and rejection details exist)
    #[serde(rename = "reject-details")]
    pub reject_details: Option<String>,
    /// Rejection reason (only present when 'allowed' is false)
    #[serde(rename = "reject-reason")]
    pub reject_reason: Option<String>,
    /// The transaction hash in hex
    pub txid: String,
    /// Virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted (only present when 'allowed' is true)
    pub vsize: Option<i64>,
    /// The transaction witness hash in hex
    #[serde(rename = "wtxid")]
    pub w_txid: String,
}

/// Transaction fees (only present if 'allowed' is true)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAcceptItemFeeS {
    /// transaction fee in BTC
    pub base: f64,
    /// the effective feerate in BTC per KvB. May differ from the base feerate if, for example, there are modified fees from prioritisetransaction or a package feerate was used.
    #[serde(rename = "effective-feerate")]
    pub effective_feerate: f64,
    /// transactions whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes")]
    pub effective_includes: Vec<String>,
}

/// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
/// If both are specified, they must be identical.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UnloadWallet {
    /// Warning messages, if any, related to unloading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `uptime`.
///
/// > uptime
/// >
/// > Returns the total uptime of the server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Uptime(pub i64);

/// Result of the JSON-RPC method `utxoupdatepsbt`.
///
/// > utxoupdatepsbt
/// >
/// > Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UtxoUpdatePsbt(pub String);

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
    #[serde(rename = "isscript")]
    pub is_script: Option<bool>,
    /// If the address is valid or not
    pub isvalid: bool,
    /// If the address is a witness address
    #[serde(rename = "iswitness")]
    pub is_witness: Option<bool>,
    /// The hex-encoded output script generated by the address
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: Option<String>,
    /// The hex value of the witness program
    pub witness_program: Option<String>,
    /// The version number of the witness program
    pub witness_version: Option<i64>,
}

/// Result of the JSON-RPC method `verifychain`.
///
/// > verifychain
/// >
/// > Verifies blockchain database.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyChain(pub bool);

/// Result of the JSON-RPC method `verifymessage`.
///
/// > verifymessage
/// >
/// > Verify a signed message.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyMessage(pub bool);

/// Result of the JSON-RPC method `verifytxoutproof`.
///
/// > verifytxoutproof
/// >
/// > Verifies that a proof points to a transaction in a block, returning the transaction it commits to
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyTxOutProof(pub Vec<String>);

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

/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletCreateFundedPsbt {
    /// The position of the added change output, or -1
    #[serde(rename = "changepos")]
    pub change_pos: i64,
    /// Fee in BTC the resulting transaction pays
    pub fee: f64,
    /// The resulting raw transaction (base64-encoded string)
    pub psbt: String,
}

/// Display address on an external signer for verification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletDisplayAddress {
    /// The address as confirmed by the signer
    pub address: String,
}

/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletProcessPsbt {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
}

