// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - wallet.
//! Types for methods found under the `== Wallet ==` section.
//! Auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `abortrescan`.
///
/// > abortrescan
/// >
/// > Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AbortRescan(pub bool);

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
    pub origfee: f64,
    /// The id of the new transaction.
    pub txid: String,
}

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

/// Result of the JSON-RPC method `encryptwallet`.
///
/// > encryptwallet
/// >
/// > Encrypts the wallet with 'passphrase'. This is for first time encryption.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EncryptWallet(pub String);

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
    pub ischange: bool,
    /// If the pubkey is compressed.
    pub iscompressed: Option<bool>,
    /// If the address is yours.
    pub ismine: bool,
    /// If the key is a script.
    pub isscript: Option<bool>,
    /// (DEPRECATED) Always false.
    pub iswatchonly: bool,
    /// If the address is a witness address.
    pub iswitness: bool,
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
    pub sigsrequired: Option<i64>,
    /// If we know how to spend coins sent to this address, ignoring the possible lack of private keys.
    pub solvable: bool,
    /// The creation time of the key, if available, expressed in UNIX epoch time.
    #[serde(rename = "timestamp")]
    pub times_tamp: Option<i64>,
    /// The hex value of the witness program.
    pub witness_program: Option<String>,
    /// The version number of the witness program.
    pub witness_version: Option<i64>,
}

/// Information about the address embedded in P2SH or P2WSH, if relevant and known.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressInfoEmbedded {

}

/// Result of the JSON-RPC method `getaddressesbylabel`.
///
/// > getaddressesbylabel
/// >
/// > Returns the list of addresses assigned the specified label.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressesbyLabel(
    /// json object with addresses as keys
    pub std::collections::BTreeMap<String, GetAddressesbyLabelEntry>,
);

/// json object with information about address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressesbyLabelEntry {
    /// Purpose of address ("send" for sending address, "receive" for receiving address)
    pub purpose: String,
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
    pub last_processed_block: GetBalancesLastprocessedblock,
    /// balances from outputs that the wallet can sign
    #[serde(rename = "mine")]
    pub min_e: GetBalancesMine,
}

/// hash and height of the block this information was generated on
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalancesLastprocessedblock {
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

/// Result of the JSON-RPC method `getnewaddress`.
///
/// > getnewaddress
/// >
/// > Returns a new Bitcoin address for receiving payments.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNewAddress(pub String);

/// Result of the JSON-RPC method `getrawchangeaddress`.
///
/// > getrawchangeaddress
/// >
/// > Returns a new Bitcoin address, for receiving change.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawChangeAddress(pub String);

/// Result of the JSON-RPC method `getreceivedbyaddress`.
///
/// > getreceivedbyaddress
/// >
/// > Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetReceivedbyAddress(pub String);

/// Result of the JSON-RPC method `getreceivedbylabel`.
///
/// > getreceivedbylabel
/// >
/// > Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetReceivedbyLabel(pub String);

/// Get detailed information about in-wallet transaction <txid>
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
    pub decoded: Option<GetTransactionDecoded>,
    pub details: Vec<GetTransactionDetailsItem>,
    /// The amount of the fee in BTC. This is negative and only available for the
/// 'send' category of transactions.
    pub fee: Option<f64>,
    /// Only present if the transaction's only input is a coinbase one.
    pub generated: Option<bool>,
    /// Raw data for transaction
    pub hex: String,
    /// hash and height of the block this information was generated on
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetTransactionLastprocessedblock,
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
    pub wtxid: String,
}

/// The decoded transaction (only present when `verbose` is passed)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecoded {
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
    pub vin: Vec<GetTransactionDecodedVinItem>,
    pub vout: Vec<GetTransactionDecodedVoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodedVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(rename = "coinbase")]
    pub coin_base: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<GetTransactionDecodedVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    pub txid: Option<String>,
    #[serde(rename = "txinwitness")]
    pub tx_inwitness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    pub vout: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodedVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodedVoutItem {
    /// Output script is change (only present if true)
    pub ischange: Option<bool>,
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetTransactionDecodedVoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDecodedVoutItemScriptPubKey {
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
    pub vout: i64,
}

/// hash and height of the block this information was generated on
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionLastprocessedblock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: i64,
}

/// Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfo {
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    pub birthtime: Option<i64>,
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
    pub key_poolsize: i64,
    /// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
    pub keypoolsize_hd_internal: Option<i64>,
    /// hash and height of the block this information was generated on
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: GetWalletInfoLastprocessedblock,
    /// the transaction fee configuration, set in BTC/kvB
    pub paytxfee: f64,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: GetWalletInfoScanning,
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
pub struct GetWalletInfoLastprocessedblock {
    /// hash of the block this information was generated on
    pub hash: String,
    /// height of the block this information was generated on
    pub height: i64,
}

/// current scanning details, or false if no scan is in progress
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfoScanning {
    /// elapsed seconds since scan start
    pub duration: i64,
    /// scanning progress percentage [0.0, 1.0]
    pub progress: i64,
}

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
pub struct ImportDescriptorsItemError {

}

/// Result of the JSON-RPC method `listaddressgroupings`.
///
/// > listaddressgroupings
/// >
/// > Lists groups of addresses which have had their common ownership
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListAddressGroupings(pub Vec<Vec<Vec<serde_json::Value>>>);

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
    pub range: Option<Vec<serde_json::Value>>,
    /// The creation time of the descriptor
    #[serde(rename = "timestamp")]
    pub times_tamp: i64,
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
    pub vout: i64,
}

/// Result of the JSON-RPC method `listreceivedbyaddress`.
///
/// > listreceivedbyaddress
/// >
/// > List balances by receiving address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListReceivedbyAddress(pub Vec<ListReceivedbyAddressItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListReceivedbyAddressItem {
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
pub struct ListReceivedbyLabel(pub Vec<ListReceivedbyLabelItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListReceivedbyLabelItem {
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
    pub generated: Option<bool>,
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
    pub vout: i64,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conf_licts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    pub wtxid: String,
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
    pub generated: Option<bool>,
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
    pub vout: i64,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conf_licts: Vec<String>,
    /// The hash of serialized transaction, including witness data.
    pub wtxid: String,
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
    pub reused: Option<bool>,
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
    pub vout: i64,
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

/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// 
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named <wallet name>-<timestamp>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// 
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MigrateWallet {
    /// The location of the backup of the original wallet
    pub backup_path: String,
    /// The name of the migrated wallet containing solvable but not watched scripts
    pub solvables_name: Option<String>,
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    pub watchonly_name: Option<String>,
}

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
    pub origfee: f64,
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

/// EXPERIMENTAL warning: this call may be changed in future releases.
/// 
/// Send a transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Send {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Option<String>,
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

/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendmanyVerboseOne {
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
pub struct SendmanyVerboseZero(pub String);

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
pub struct SetWalletflag {
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

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionwithWallet {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Option<Vec<SignRawTransactionwithWalletErrorsItem>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionwithWalletErrorsItem {
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
    pub vout: i64,
    pub witness: Vec<String>,
}

/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SimulateRawTransaction {
    /// The wallet balance change (negative means decrease).
    pub balance_change: f64,
}

/// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
/// If both are specified, they must be identical.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UnloadWallet {
    /// Warning messages, if any, related to unloading the wallet.
    pub warnings: Option<Vec<String>>,
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

