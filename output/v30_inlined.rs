pub struct AbortRescan(pub bool);
/// Result of the JSON-RPC method `addmultisigaddress`.
///
/// > addmultisigaddress nrequired ["key",...] ( "label" "address_type" )
/// >
/// > Add a nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// > Each key is a Bitcoin address or hex-encoded public key.
/// > This functionality is only intended for use with non-watchonly addresses.
/// > See `importaddress` for watchonly p2sh address support.
/// > If 'label' is specified, assign address to that label.
///
/// > Arguments:
/// > 1. nrequired                      (numeric, required) The number of required signatures out of the n keys or addresses.
/// > 2. "keys"                         (string, required) A json array of bitcoin addresses or hex-encoded public keys
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct ActiveCommand {
/// The name of the RPC command.
pub method: String,
/// The running time in microseconds.
pub duration: u64,
}

pub struct AddConnection {
/// The address of the newly added connection.
pub address: String,
/// Type of connection.
pub connection_type: String,
}

pub struct AddMultisigAddress {
/// The value of the new multisig address.
pub address: String,
/// The string value of the hex-encoded redemption script.
#[serde(rename = "redeemScript")]
pub redeem_script: String,
/// The descriptor for this multisig.
pub descriptor: String,
/// Any warnings resulting from the creation of this multisig.
pub warnings: Option<Vec<String>>,
}

pub struct AddPeerAddress {
/// Whether the peer address was successfully added to the address manager.
pub success: bool,
}

pub struct AddedNode {
/// The node IP address or name (as provided to addnode).
#[serde(rename = "addednode")]
pub added_node: String,
/// If connected.
pub connected: bool,
/// Only when connected = true.
pub addresses: Vec<AddedNodeAddress>,
}

pub struct AddedNodeAddress {
/// The bitcoin server IP and port we're connected to.
pub address: String,
/// Connection, inbound or outbound.
pub connected: String,
}

pub struct AddrManInfoNetwork {
/// Number of addresses in the new table, which represent potential peers the node has discovered but hasn't yet successfully connected to.
pub new: u64,
/// Number of addresses in the tried table, which represent peers the node has successfully connected to in the past.
pub tried: u64,
/// Total number of addresses in both new/tried tables.
pub total: u64,
}

pub struct AddressInformation {
/// Purpose of address.
pub purpose: AddressPurpose,
}

pub struct AnalyzePsbt {
/// Array of input objects.
pub inputs: Vec<AnalyzePsbtInput>,
/// Estimated vsize of the final signed transaction.
pub estimated_vsize: Option<u32>,
/// Estimated feerate of the final signed transaction in BTC/kB.
///
/// Shown only if all UTXO slots in the PSBT have been filled.
#[serde(rename = "estimated_feerate")]
pub estimated_fee_rate: Option<f64>,
/// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled.
pub fee: Option<f64>,
/// Role of the next person that this psbt needs to go to.
pub next: String,
}

pub struct AnalyzePsbtInput {
/// Whether a UTXO is provided.
pub has_utxo: bool,
/// Whether the input is finalized.
pub is_final: bool,
/// Things that are missing that are required to complete this input.
pub missing: Option<AnalyzePsbtInputMissing>,
/// Role of the next person that this input needs to go to.
pub next: Option<String>,
}

pub struct AnalyzePsbtInputMissing {
/// Public key ID, hash160 of the public key, of a public key whose BIP 32 derivation path is missing.
pub pubkeys: Option<Vec<String>>,
/// Public key ID, hash160 of the public key, of a public key whose signature is missing.
pub signatures: Option<Vec<String>>,
/// Hash160 of the redeemScript that is missing.
#[serde(rename = "redeemscript")]
pub redeem_script: Option<String>,
/// SHA256 of the witnessScript that is missing.
#[serde(rename = "witnessscript")]
pub witness_script: Option<String>,
}

pub struct Banned {
/// The IP/Subnet of the banned node.
pub address: String,
/// The UNIX epoch time the ban was created.
pub ban_created: u32,
/// The UNIX epoch time the ban was expires.
pub banned_until: u32,
/// The ban duration, in seconds.
pub ban_duration: u32,
/// The time remaining until the ban expires, in seconds.
pub time_remaining: u32,
}

pub struct Bip9Info {
/// The bit (0-28) in the block version field used to signal this softfork (only for "started" and "locked_in" status).
pub bit: Option<u8>,
/// The minimum median time past of a block at which the bit gains its meaning.
pub start_time: i64,
/// The median time past of a block at which the deployment is considered failed if not yet locked in.
pub timeout: i64,
/// Minimum height of blocks for which the rules may be enforced.
pub min_activation_height: u32,
/// Status of deployment at specified block (one of "defined", "started", "locked_in", "active", "failed").
pub status: String,
/// Height of the first block to which the status applies.
pub since: u32,
/// Status of deployment at the next block.
pub status_next: String,
/// Numeric statistics about signalling for a softfork (only for "started" and "locked_in" status).
pub statistics: Option<Bip9Statistics>,
/// Indicates blocks that signalled with a # and blocks that did not with a -.
pub signalling: Option<String>,
}

pub struct Bip9Softfork {
/// One of "defined", "started", "locked_in", "active", "failed".
pub status: Bip9SoftforkStatus,
/// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
pub bit: Option<u8>,
/// The minimum median time past of a block at which the bit gains its meaning.
#[serde(rename = "startTime")]
pub start_time: i64,
/// The median time past of a block at which the deployment is considered failed if not yet locked in.
pub timeout: i64,
/// Height of the first block to which the status applies.
pub since: i64,
}

pub struct Bip9SoftforkInfo {
/// One of "defined", "started", "locked_in", "active", "failed".
pub status: Bip9SoftforkStatus,
/// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
pub bit: Option<u8>,
/// The minimum median time past of a block at which the bit gains its meaning.
pub start_time: i64,
/// The median time past of a block at which the deployment is considered failed if not yet locked in.
pub timeout: i64,
/// Height of the first block to which the status applies.
pub since: i64,
/// Minimum height of blocks for which the rules may be enforced.
pub min_activation_height: i64,
/// Numeric statistics about BIP-9 signalling for a softfork (only for "started" status).
pub statistics: Option<Bip9SoftforkStatistics>,
}

pub struct Bip9SoftforkStatistics {
/// The length in blocks of the BIP9 signalling period.
pub period: i64,
/// The number of blocks with the version bit set required to activate the feature.
pub threshold: Option<i64>,
/// The number of blocks elapsed since the beginning of the current period.
pub elapsed: i64,
/// The number of blocks with the version bit set in the current period.
pub count: i64,
/// `false` if there are not enough blocks left in this period to pass activation threshold.
pub possible: Option<bool>,
}

pub struct Bip9Statistics {
/// The length in blocks of the signalling period.
pub period: u32,
/// The number of blocks with the version bit set required to activate the feature (only for "started" status).
pub threshold: Option<u32>,
/// The number of blocks elapsed since the beginning of the current period.
pub elapsed: u32,
/// The number of blocks with the version bit set in the current period.
pub count: u32,
/// Returns false if there are not enough blocks left in this period to pass activation threshold (only for "started" status).
pub possible: Option<bool>,
}

pub struct BlockTemplateTransaction {
/// Transaction data encoded in hexadecimal (byte-for-byte).
pub data: String,
/// Transaction id encoded in little-endian hexadecimal.
pub txid: String,
/// Hash encoded in little-endian hexadecimal (including witness data).
pub hash: String,
/// Array of numbers.
///
/// Transactions before this one (by 1-based index in 'transactions' list) that must be present in the final block if this one is.
pub depends: Vec<i64>,
/// Difference in value between transaction inputs and outputs (in satoshis); for coinbase
/// transactions, this is a negative Number of the total collected block fees (ie, not including
/// the block subsidy); if key is not present, fee is unknown and clients MUST NOT assume there
/// isn't one.
pub fee: i64,
/// Total SigOps cost, as counted for purposes of block limits; if key is not present, sigop
/// cost is unknown and clients MUST NOT assume it is zero.
pub sigops: i64,
/// Total transaction weight, as counted for purposes of block limits.
pub weight: u64,
}

pub struct BumpFee {
/// The id of the new transaction.
pub txid: String,
/// Fee of the replaced transaction.
#[serde(rename = "origfee")]
pub original_fee: f64,
/// Fee of the new transaction.
pub fee: f64,
/// Errors encountered during processing (may be empty).
pub errors: Vec<String>,
}

pub struct ChainState {
/// Number of blocks in this chainstate.
pub blocks: i64,
/// Blockhash of the tip.
#[serde(rename = "bestblockhash")]
pub best_block_hash: String,
/// nBits: compact representation of the block difficulty target.
pub bits: String,
/// The difficulty target.
pub target: String,
/// Difficulty of the tip.
pub difficulty: f64,
/// Progress towards the network tip.
#[serde(rename = "verificationprogress")]
pub verification_progress: f64,
/// The base block of the snapshot this chainstate is based on, if any.
#[serde(rename = "snapshot_blockhash")]
pub snapshot_block_hash: Option<String>,
/// Size of the coinsdb cache.
pub coins_db_cache_bytes: u64,
/// Size of the coinstip cache.
pub coins_tip_cache_bytes: u64,
/// Whether the chainstate is fully validated.
pub validated: bool,
}

pub struct ChainTips {
/// Height of the chain tip.
pub height: i64,
/// Block hash of the tip.
pub hash: String,
/// Zero for main chain.
#[serde(rename = "branchlen")]
pub branch_length: i64,
/// "active" for the main chain.
pub status: ChainTipsStatus,
}

pub struct CombinePsbt(
/// The base64-encoded partially signed transaction.
pub String,
);

pub struct CombineRawTransaction(
/// The hex-encoded raw transaction with signature(s).
pub String,
);

pub struct ConvertToPsbt(
/// The resulting raw transaction (base64-encoded string).
pub String,
);

pub struct CreateMultisig {
/// The value of the new multisig address.
pub address: String,
/// The string value of the hex-encoded redemption script.
#[serde(rename = "redeemScript")]
pub redeem_script: String,
/// The descriptor for this multisig.
pub descriptor: String,
/// Any warnings resulting from the creation of this multisig.
pub warnings: Option<Vec<String>>,
}

pub struct CreatePsbt(
/// The resulting raw transaction (base64-encoded string).
pub String,
);

pub struct CreateRawTransaction(
/// hex string of the transaction.
pub String,
);

pub struct CreateWallet {
/// The wallet name if created successfully.
///
/// If the wallet was created using a full path, the wallet_name will be the full path.
pub name: String,
/// Warning messages, if any, related to creating and loading the wallet.
pub warnings: Option<Vec<String>>,
}

pub struct CreateWalletDescriptor {
/// The public descriptors that were added to the wallet.
#[serde(rename = "descs")]
pub descriptors: Vec<String>,
}

pub struct DecodePsbt {
/// The decoded network-serialized unsigned transaction.
pub tx: RawTransaction,
/// The global xpubs.
pub global_xpubs: Vec<GlobalXpub>,
/// The PSBT version number. Not to be confused with the unsigned transaction version.
pub psbt_version: u32,
/// The global proprietary map.
pub proprietary: Option<Vec<Proprietary>>,
/// The unknown global fields.
pub unknown: Option<HashMap<String, String>>,
/// Array of transaction inputs.
pub inputs: Vec<PsbtInput>,
/// Array of transaction outputs.
pub outputs: Vec<PsbtOutput>,
/// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
pub fee: Option<f64>,
}

pub struct DecodeRawTransaction(pub RawTransaction);
/// Result of JSON-RPC method `decodescript`.
///
/// > decodescript "hexstring"
/// >
/// > Decode a hex-encoded script.
/// >
/// > Arguments:
/// > 1. "hexstring"     (string) the hex encoded script
// The docs on Core v0.17 appear to be way off what is actually returned.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct DecodeScript {
/// Script public key.
pub asm: String,
/// Inferred descriptor for the script.
#[serde(rename = "desc")]
pub descriptor: Option<String>,
/// The output type.
#[serde(rename = "type")]
pub type_: String,
/// Bitcoin address (only if a well-defined address exists).
pub address: Option<String>,
/// The required signatures.
#[serde(rename = "reqSigs")]
pub required_signatures: Option<u64>,
/// List of bitcoin addresses.
pub addresses: Option<Vec<String>>,
/// Address of P2SH script wrapping this redeem script (not returned if the script is already a P2SH).
pub p2sh: Option<String>,
/// Segwit data (see `DecodeScriptSegwit` for explanation).
pub segwit: Option<DecodeScriptSegwit>,
/// Address of the P2SH script wrapping this witness redeem script
#[serde(rename = "p2sh-segwit")]
pub p2sh_segwit: Option<String>,
}

pub struct DecodeScriptSegwit {
/// Script public key.
pub asm: String,
/// Hex encoded public key.
pub hex: String,
/// The output type.
#[serde(rename = "type")]
pub type_: String,
/// Bitcoin address (only if a well-defined address exists).
pub address: Option<String>,
/// The required signatures.
#[serde(rename = "reqSigs")]
pub required_signatures: Option<u64>,
/// List of bitcoin addresses.
pub addresses: Option<Vec<String>>,
/// Inferred descriptor for the script.
#[serde(rename = "desc")]
pub descriptor: Option<String>,
/// Address of P2SH script wrapping this redeem script (not returned if the script is already a P2SH).
#[serde(rename = "p2sh-segwit")]
pub p2sh_segwit: Option<String>,
}

pub struct DeploymentInfo {
/// One of "buried", "bip9".
#[serde(rename = "type")]
pub deployment_type: String,
/// Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
pub height: Option<u32>,
/// True if the rules are enforced for the mempool and the next block.
pub active: bool,
/// Status of bip9 softforks (only for "bip9" type).
pub bip9: Option<Bip9Info>,
}

pub struct DeriveAddresses(pub Vec<String>);
/// Result of JSON-RPC method `getdescriptorinfo`.
///
/// > getdescriptorinfo "descriptor"
/// >
/// > Analyses a descriptor.
/// > Returns information about the descriptor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct DeriveAddressesMultipath(pub Vec<DeriveAddresses>);
/// Result of JSON-RPC method `getdescriptorinfo`.
///
/// > getdescriptorinfo "descriptor"
/// >
/// > Analyses a descriptor.
/// > Returns information about the descriptor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct DescriptorInfo {
/// Descriptor string representation.
#[serde(rename = "desc")]
pub descriptor: String,
/// The creation time of the descriptor.
pub timestamp: u64,
/// Activeness flag.
pub active: bool,
/// Whether this is an internal or external descriptor; defined only for active descriptors.
pub internal: Option<bool>,
/// Defined only for ranged descriptors.
pub range: Option<[u64; 2]>,
/// Same as `next_index` field. Kept for compatibility reason.
pub next: Option<u64>,
/// The next index to generate addresses from; defined only for ranged descriptors.
pub next_index: Option<u64>,
}

pub struct DescriptorProcessPsbt {
/// The base64-encoded partially signed transaction.
pub psbt: String,
/// If the transaction has a complete set of signatures.
pub complete: bool,
/// The hex-encoded network transaction if complete.
pub hex: Option<String>,
}

pub struct DumpPrivKey(pub String); // The private key.
/// Result of the JSON-RPC method `dumpwallet`.
///
/// > dumpwallet "filename"
/// >
/// > Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// > Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// > Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// > only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// >
/// > Arguments:
/// > 1. "filename"    (string, required) The filename with path (either absolute or relative to bitcoind)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct DumpTxOutSet {
/// The number of coins written in the snapshot.
pub coins_written: f64,
/// The hash of the base of the snapshot.
pub base_hash: String,
/// The height of the base of the snapshot.
pub base_height: i64,
/// The absolute path that the snapshot was written to.
pub path: String,
/// The hash of the UTXO set contents.
#[serde(rename = "txoutset_hash")]
pub tx_out_set_hash: String,
/// The number of transactions in the chain up to and including the base block.
#[serde(rename = "nchaintx")]
pub n_chain_tx: i64,
}

pub struct DumpWallet {
/// The filename with full absolute path.
#[serde(rename = "filename")]
pub file_name: String,
}

pub struct EncryptWallet(pub String);
/// Result of the JSON-RPC method `getaddressesbylabel`.
///
/// > getaddressesbylabel "label"
/// >
/// > Returns the list of addresses assigned the specified label.
/// >
/// > Arguments:
/// > 1. "label"  (string, required) The label.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct EnumerateSigners {
/// List of external signers.
pub signers: Vec<Signers>,
}

pub struct EstimateRawFee {
/// Estimate for short time horizon.
pub short: Option<RawFeeDetail>,
/// Estimate for medium time horizon.
pub medium: Option<RawFeeDetail>,
/// Estimate for long time horizon.
pub long: RawFeeDetail,
}

pub struct EstimateSmartFee {
/// Estimate fee rate in BTC/kB.
#[serde(rename = "feerate")]
pub fee_rate: Option<f64>,
/// Errors encountered during processing.
pub errors: Option<Vec<String>>,
/// Block number where estimate was found.
pub blocks: u32,
}

pub struct FinalizePsbt {
/// The base64-encoded partially signed transaction if not extracted.
pub psbt: Option<String>,
/// The hex-encoded network transaction if extracted.
pub hex: Option<String>,
/// If the transaction has a complete set of signatures.
pub complete: bool,
}

pub struct FundRawTransaction {
/// The resulting raw transaction (hex-encoded string).
pub hex: String,
/// Fee in BTC the resulting transaction pays.
pub fee: f64,
/// The position of the added change output, or -1.
#[serde(rename = "changepos")]
pub change_position: i64,
}

pub struct Generate(
/// Hashes of blocks generated.
pub Vec<String>,
);

pub struct GenerateBlock {
/// Hash of generated block
pub hash: String,
/// Hex of generated block, only present when submit=false.
pub hex: Option<String>,
}

pub struct GenerateToAddress(
/// Hashes of blocks generated.
pub Vec<String>,
);

pub struct GenerateToDescriptor(
/// Hashes of blocks generated.
pub Vec<String>,
);

pub struct GetAddedNodeInfo(pub Vec<AddedNode>);
/// An added node item. Part of `getaddednodeinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetAddrManInfo(pub BTreeMap<String, AddrManInfoNetwork>);
/// Address manager information. Part of `getaddrmaninfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetAddressInfo {
/// The bitcoin address validated.
pub address: String,
/// The hex encoded scriptPubKey generated by the address.
#[serde(rename = "scriptPubKey")]
pub script_pubkey: String,
/// If the address is yours or not.
#[serde(rename = "ismine")]
pub is_mine: bool,
/// If the address is watchonly.
#[serde(rename = "iswatchonly")]
pub is_watch_only: bool,
/// Whether we know how to spend coins sent to this address, ignoring the possible lack of private keys.
pub solvable: bool,
/// A descriptor for spending coins sent to this address (only when solvable).
#[serde(rename = "desc")]
pub descriptor: Option<String>,
/// The descriptor used to derive this address if this is a descriptor wallet
#[serde(rename = "parent_desc")]
pub parent_descriptor: Option<String>,
/// If the key is a script.
#[serde(rename = "isscript")]
pub is_script: Option<bool>,
/// If the address was used for change output.
#[serde(rename = "ischange")]
pub is_change: bool,
/// If the address is a witness address.
#[serde(rename = "iswitness")]
pub is_witness: bool,
/// The version number of the witness program.
pub witness_version: Option<i64>,
/// The hex value of the witness program.
pub witness_program: Option<String>,
/// The output script type.
///
/// Only if "isscript" is true and the redeemscript is known.
pub script: Option<ScriptType>,
/// The redeemscript for the p2sh address.
pub hex: Option<String>,
/// Array of pubkeys associated with the known redeemscript (only if "script" is "multisig").
pub pubkeys: Option<Vec<String>>,
/// Number of signatures required to spend multisig output (only if "script" is "multisig").
#[serde(rename = "sigsrequired")]
pub sigs_required: Option<i64>,
/// The hex value of the raw public key, for single-key addresses (possibly embedded in P2SH or P2WSH).
pub pubkey: Option<String>,
/// Information about the address embedded in P2SH or P2WSH, if relevant and known.
pub embedded: Option<GetAddressInfoEmbedded>,
/// If the pubkey is compressed.
#[serde(rename = "iscompressed")]
pub is_compressed: Option<bool>,
/// The creation time of the key if available in seconds since epoch (Jan 1 1970 GMT).
pub timestamp: Option<u32>,
/// The HD keypath if the key is HD and available.
#[serde(rename = "hdkeypath")]
pub hd_key_path: Option<String>,
/// The Hash160 of the HD seed.
#[serde(rename = "hdseedid")]
pub hd_seed_id: Option<String>,
/// The fingerprint of the master key.
#[serde(rename = "hdmasterfingerprint")]
pub hd_master_fingerprint: Option<String>,
/// Array of labels associated with the address.
pub labels: Vec<String>,
}

pub struct GetAddressInfoEmbedded {
/// The bitcoin address validated.
pub address: String,
/// The hex encoded scriptPubKey generated by the address.
#[serde(rename = "scriptPubKey")]
pub script_pubkey: String,
/// Whether we know how to spend coins sent to this address, ignoring the possible lack of private keys.
pub solvable: Option<bool>,
/// A descriptor for spending coins sent to this address (only when solvable).
#[serde(rename = "desc")]
pub descriptor: Option<String>,
/// The descriptor used to derive this address if this is a descriptor wallet
#[serde(rename = "parent_desc")]
pub parent_descriptor: Option<String>,
/// If the key is a script.
#[serde(rename = "isscript")]
pub is_script: Option<bool>,
/// If the address was used for change output.
#[serde(rename = "ischange")]
pub is_change: Option<bool>,
/// If the address is a witness address.
#[serde(rename = "iswitness")]
pub is_witness: bool,
/// The version number of the witness program.
pub witness_version: Option<i64>,
/// The hex value of the witness program.
pub witness_program: Option<String>,
/// The output script type.
///
/// Only if "isscript" is true and the redeemscript is known.
pub script: Option<ScriptType>,
/// The redeemscript for the p2sh address.
pub hex: Option<String>,
/// Array of pubkeys associated with the known redeemscript (only if "script" is "multisig").
pub pubkeys: Option<Vec<String>>,
/// Number of signatures required to spend multisig output (only if "script" is "multisig").
#[serde(rename = "sigsrequired")]
pub sigs_required: Option<i64>,
/// The hex value of the raw public key, for single-key addresses (possibly embedded in P2SH or P2WSH).
pub pubkey: Option<String>,
/// If the pubkey is compressed.
#[serde(rename = "iscompressed")]
pub is_compressed: Option<bool>,
/// Array of labels associated with the address.
pub labels: Option<Vec<String>>,
}

pub struct GetAddressInfoLabel {
/// The label.
pub name: String,
/// Purpose of address ("send" for sending address, "receive" for receiving address).
pub purpose: AddressPurpose,
}

pub struct GetAddressesByLabel(pub BTreeMap<String, AddressInformation>);
/// Address information. Part of `getaddressesbylabel`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetBalance(pub f64);
/// Result of the JSON-RPC method `getnewaddress`.
///
/// > getnewaddress ( "label" "address_type" )
/// >
/// > Returns a new Bitcoin address for receiving payments.
/// > If 'label' is specified, it is added to the address book
/// > so payments received with the address will be associated with 'label'.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetBalances {
/// Balances from outputs that the wallet can sign.
pub mine: GetBalancesMine,
#[serde(rename = "watchonly")]
pub watch_only: Option<GetBalancesWatchOnly>,
/// Hash and height of the block this information was generated on.
#[serde(rename = "lastprocessedblock")]
pub last_processed_block: Option<LastProcessedBlock>,
}

pub struct GetBalancesMine {
/// Trusted balance (outputs created by the wallet or confirmed outputs).
pub trusted: f64,
/// Untrusted pending balance (outputs created by others that are in the mempool).
pub untrusted_pending: f64,
/// Balance from immature coinbase outputs.
pub immature: f64,
/// Balance from coins sent to addresses that were previously spent from (potentially privacy violating).
///
/// Only present if `avoid_reuse` is set.
pub used: Option<f64>,
}

pub struct GetBalancesWatchOnly {
/// Trusted balance (outputs created by the wallet or confirmed outputs).
pub trusted: f64,
/// Untrusted pending balance (outputs created by others that are in the mempool).
pub untrusted_pending: f64,
/// Balance from immature coinbase outputs.
pub immature: f64,
}

pub struct GetBestBlockHash(pub String);
/// Result of JSON-RPC method `getblock` with verbosity set to 0.
///
/// > getblock "blockhash" ( verbosity )
/// >
/// > If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// > If verbosity is 1, returns an Object with information about block `<hash>`.
/// > If verbosity is 2, returns an Object with information about block `<hash>` and information about each transaction.
/// >
/// > Arguments:
/// > 1. "blockhash"          (string, required) The block hash
/// > 2. verbosity              (numeric, optional, default=1) 0 for hex encoded data, 1 for a json object, and 2 for json object with transaction data
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetBlockCount(pub u64);
/// Result of JSON-RPC method `getblockhash`.
///
/// > Returns hash of block in best-block-chain at height provided.
/// >
/// > Arguments:
/// > 1. height         (numeric, required) The height index
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetBlockFilter {
/// The hex-encoded filter data.
pub filter: String,
/// The hex-encoded filter header.
pub header: String,
}

pub struct GetBlockHash(pub String);
/// Result of JSON-RPC method `getblockheader` with verbosity set to `false`.
///
/// > If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// > If verbose is true, returns an Object with information about blockheader 'hash'.
/// >
/// > Arguments:
/// > 1. "hash"          (string, required) The block hash
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetBlockHeader(pub String);
/// Result of JSON-RPC method `getblockheader` with verbosity set to `true`.
///
/// > If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// > If verbose is true, returns an Object with information about blockheader `<hash>`.
/// >
/// > Arguments:
/// > 1. "hash"          (string, required) The block hash
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetBlockHeaderVerbose {
/// The block hash.
pub hash: String,
/// The number of confirmations, or -1 if the block is not on the main chain.
pub confirmations: i64,
/// The block height or index.
pub height: i64,
/// The block version.
pub version: i32,
/// The block version formatted in hexadecimal.
#[serde(rename = "versionHex")]
pub version_hex: String,
/// The merkle root.
#[serde(rename = "merkleroot")]
pub merkle_root: String,
/// The block time in seconds since epoch (Jan 1 1970 GMT).
pub time: i64,
/// The median block time in seconds since epoch (Jan 1 1970 GMT).
#[serde(rename = "mediantime")]
pub median_time: i64,
/// The nonce.
pub nonce: i64,
/// The bits.
pub bits: String,
/// The difficulty target (hex-encoded).
pub target: String,
/// The difficulty.
pub difficulty: f64,
/// Expected number of hashes required to produce the current chain (in hex).
#[serde(rename = "chainwork")]
pub chain_work: String,
/// The number of transactions in the block.
#[serde(rename = "nTx")]
pub n_tx: u32,
/// The hash of the previous block (if available).
#[serde(rename = "previousblockhash")]
pub previous_block_hash: Option<String>,
/// The hash of the next block (if available).
#[serde(rename = "nextblockhash")]
pub next_block_hash: Option<String>,
}

pub struct GetBlockStats {
/// Average fee in the block.
#[serde(rename = "avgfee")]
pub average_fee: Option<u64>,
// FIXME: Remember these docs will become silently stale when unit changes in a later version of Core.
/// Average feerate (in satoshis per virtual byte).
#[serde(rename = "avgfeerate")]
pub average_fee_rate: Option<u64>,
/// Average transaction size.
#[serde(rename = "avgtxsize")]
pub average_tx_size: Option<i64>,
/// The block hash (to check for potential reorgs).
#[serde(rename = "blockhash")]
pub block_hash: Option<String>,
/// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per
/// virtual byte).
#[serde(rename = "feerate_percentiles")]
pub fee_rate_percentiles: Option<[u64; 5]>,
/// The height of the block.
pub height: Option<i64>,
/// The number of inputs (excluding coinbase).
#[serde(rename = "ins")]
pub inputs: Option<i64>,
/// Maximum fee in the block.
#[serde(rename = "maxfee")]
pub max_fee: Option<u64>,
/// Maximum feerate (in satoshis per virtual byte).
#[serde(rename = "maxfeerate")]
pub max_fee_rate: Option<u64>,
/// Maximum transaction size.
#[serde(rename = "maxtxsize")]
pub max_tx_size: Option<i64>,
/// Truncated median fee in the block.
#[serde(rename = "medianfee")]
pub median_fee: Option<u64>,
/// The block median time past.
#[serde(rename = "mediantime")]
pub median_time: Option<i64>,
/// Truncated median transaction size
#[serde(rename = "mediantxsize")]
pub median_tx_size: Option<i64>,
/// Minimum fee in the block.
#[serde(rename = "minfee")]
pub minimum_fee: Option<u64>,
/// Minimum feerate (in satoshis per virtual byte).
#[serde(rename = "minfeerate")]
pub minimum_fee_rate: Option<u64>,
/// Minimum transaction size.
#[serde(rename = "mintxsize")]
pub minimum_tx_size: Option<i64>,
/// The number of outputs.
#[serde(rename = "outs")]
pub outputs: Option<i64>,
/// The block subsidy.
pub subsidy: Option<u64>,
/// Total size of all segwit transactions.
#[serde(rename = "swtotal_size")]
pub segwit_total_size: Option<i64>,
/// Total weight of all segwit transactions divided by segwit scale factor (4).
#[serde(rename = "swtotal_weight")]
pub segwit_total_weight: Option<u64>,
/// The number of segwit transactions.
#[serde(rename = "swtxs")]
pub segwit_txs: Option<i64>,
/// The block time.
pub time: Option<i64>,
/// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee]).
pub total_out: Option<u64>,
/// Total size of all non-coinbase transactions.
pub total_size: Option<i64>,
/// Total weight of all non-coinbase transactions divided by segwit scale factor (4).
pub total_weight: Option<u64>,
/// The fee total.
#[serde(rename = "totalfee")]
pub total_fee: Option<u64>,
/// The number of transactions (excluding coinbase).
pub txs: Option<i64>,
/// The increase/decrease in the number of unspent outputs.
pub utxo_increase: Option<i32>,
/// The increase/decrease in size for the utxo index (not discounting op_return and similar).
#[serde(rename = "utxo_size_inc")]
pub utxo_size_increase: Option<i32>,
/// The increase/decrease in the number of unspent outputs, not counting unspendables.
pub utxo_increase_actual: Option<i32>,
/// The increase/decrease in size for the utxo index, not counting unspendables.
#[serde(rename = "utxo_size_inc_actual")]
pub utxo_size_increase_actual: Option<i32>,
}

pub struct GetBlockTemplate {
/// The preferred block version.
pub version: i32,
/// Specific block rules that are to be enforced.
pub rules: Vec<String>,
/// Set of pending, supported versionbit (BIP 9) softfork deployments.
///
/// Map of rules name to bit number - identifies the bit number as indicating acceptance and
/// readiness for the named softfork rule.
#[serde(rename = "vbavailable")]
pub version_bits_available: BTreeMap<String, u32>,
/// Client side supported features.
pub capabilities: Vec<String>,
/// Bit mask of versionbits the server requires set in submissions.
#[serde(rename = "vbrequired")]
pub version_bits_required: i64,
/// The hash of current highest block.
#[serde(rename = "previousblockhash")]
pub previous_block_hash: String,
/// Contents of non-coinbase transactions that should be included in the next block.
pub transactions: Vec<BlockTemplateTransaction>,
/// Data that should be included in the coinbase's scriptSig content.
///
/// Key name is to be ignored, and value included in scriptSig.
#[serde(rename = "coinbaseaux")]
pub coinbase_aux: BTreeMap<String, String>,
/// Maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis).
#[serde(rename = "coinbasevalue")]
pub coinbase_value: i64,
/// An id to include with a request to longpoll on an update to this template.
#[serde(rename = "longpollid")]
pub long_poll_id: Option<String>,
// This is in the docs but not actually returned (for v0.17 and v0.18 at least).
// coinbase_txn: ???, // Also I don't know what the JSON object represents: `{ ... }`
/// The hash target.
pub target: String,
/// The minimum timestamp appropriate for next block time in seconds since epoch (Jan 1 1970 GMT).
#[serde(rename = "mintime")]
pub min_time: u32,
/// List of ways the block template may be changed.
///
/// A way the block template may be changed, e.g. 'time', 'transactions', 'prevblock'.
pub mutable: Vec<String>,
/// A range of valid nonces.
#[serde(rename = "noncerange")]
pub nonce_range: String,
/// Limit of sigops in blocks.
#[serde(rename = "sigoplimit")]
pub sigop_limit: i64,
/// Limit of block size.
#[serde(rename = "sizelimit")]
pub size_limit: i64,
/// Limit of block weight.
#[serde(rename = "weightlimit")]
pub weight_limit: i64,
/// Current timestamp in seconds since epoch (Jan 1 1970 GMT).
#[serde(rename = "curtime")]
pub current_time: u64,
/// Compressed target of next block.
pub bits: String,
/// The height of the next block.
pub height: i64,
/// Optional signet challenge.
pub signet_challenge: Option<String>,
/// A valid witness commitment for the unmodified block template.
pub default_witness_commitment: Option<String>,
}

pub struct GetBlockVerboseOne {
/// The block hash (same as provided) in RPC call.
pub hash: String,
/// The number of confirmations, or -1 if the block is not on the main chain.
pub confirmations: i64,
/// The block size.
pub size: i64,
/// The block size excluding witness data.
#[serde(rename = "strippedsize")]
pub stripped_size: Option<i64>,
/// The block weight as defined in BIP-141.
pub weight: u64,
/// The block height or index.
pub height: i64,
/// The block version.
pub version: i32,
/// The block version formatted in hexadecimal.
#[serde(rename = "versionHex")]
pub version_hex: String,
/// The merkle root.
#[serde(rename = "merkleroot")]
pub merkle_root: String,
/// The transaction ids.
pub tx: Vec<String>,
/// The block time expressed in UNIX epoch time.
pub time: i64,
/// The median block time expressed in UNIX epoch time.
#[serde(rename = "mediantime")]
pub median_time: Option<i64>,
/// The nonce (this should be only 4 bytes).
pub nonce: i64,
/// nBits: compact representation of the block difficulty target.
pub bits: String,
/// The difficulty target.
pub target: String,
/// The difficulty.
pub difficulty: f64,
/// Expected number of hashes required to produce the chain up to this block (in hex).
#[serde(rename = "chainwork")]
pub chain_work: String,
/// The number of transactions in the block.
#[serde(rename = "nTx")]
pub n_tx: i64,
/// The hash of the previous block (if available).
#[serde(rename = "previousblockhash")]
pub previous_block_hash: Option<String>,
/// The hash of the next block (if available).
#[serde(rename = "nextblockhash")]
pub next_block_hash: Option<String>,
}

pub struct GetBlockVerboseThree {
/// The block hash (same as provided) in RPC call.
pub hash: String,
/// The number of confirmations, or -1 if the block is not on the main chain.
pub confirmations: i64,
/// The block size.
pub size: i64,
/// The block size excluding witness data.
#[serde(rename = "strippedsize")]
pub stripped_size: Option<i64>,
/// The block weight as defined in BIP-141.
pub weight: u64,
/// The block height or index.
pub height: i64,
/// The block version.
pub version: i32,
/// The block version formatted in hexadecimal.
#[serde(rename = "versionHex")]
pub version_hex: String,
/// The merkle root.
#[serde(rename = "merkleroot")]
pub merkle_root: String,
/// The transactions.
pub tx: Vec<GetBlockVerboseThreeTransaction>,
/// The block time expressed in UNIX epoch time.
pub time: i64,
/// The median block time expressed in UNIX epoch time.
#[serde(rename = "mediantime")]
pub median_time: Option<i64>,
/// The nonce (this should be only 4 bytes).
pub nonce: i64,
/// nBits: compact representation of the block difficulty target.
pub bits: String,
/// The difficulty target.
pub target: String,
/// The difficulty.
pub difficulty: f64,
/// Expected number of hashes required to produce the chain up to this block (in hex).
#[serde(rename = "chainwork")]
pub chain_work: String,
/// The number of transactions in the block.
#[serde(rename = "nTx")]
pub n_tx: i64,
/// The hash of the previous block (if available).
#[serde(rename = "previousblockhash")]
pub previous_block_hash: Option<String>,
/// The hash of the next block (if available).
#[serde(rename = "nextblockhash")]
pub next_block_hash: Option<String>,
}

pub struct GetBlockVerboseThreePrevout {
/// Coinbase or not.
pub generated: bool,
/// The height of the prevout.
pub height: i64,
/// The value in BTC.
pub value: f64,
/// The script pubkey.
#[serde(rename = "scriptPubKey")]
pub script_pubkey: ScriptPubKey,
}

pub struct GetBlockVerboseThreeTransaction {
/// The transaction data (same as `getrawtransaction` verbose output, plus prevout info).
#[serde(flatten)]
pub transaction: GetRawTransactionVerboseWithPrevout,
/// The transaction fee in BTC (omitted if block undo data is not available).
pub fee: Option<f64>,
}

pub struct GetBlockVerboseTwo {
/// The block hash (same as provided).
pub hash: String,
/// The number of confirmations, or -1 if the block is not on the main chain.
pub confirmations: i64,
/// The block size.
pub size: i64,
/// The block size excluding witness data.
#[serde(rename = "strippedsize")]
pub stripped_size: Option<i64>,
/// The block weight as defined in BIP 141.
pub weight: u64,
/// The block height or index.
pub height: i64,
/// The block version.
pub version: i32,
/// The block version formatted in hexadecimal.
#[serde(rename = "versionHex")]
pub version_hex: String,
/// The merkle root.
#[serde(rename = "merkleroot")]
pub merkle_root: String,
/// The transactions.
pub tx: Vec<GetBlockVerboseTwoTransaction>,
/// The block time expressed in UNIX epoch time.
pub time: i64,
/// The median block time expressed in UNIX epoch time.
#[serde(rename = "mediantime")]
pub median_time: Option<i64>,
/// The nonce.
pub nonce: i64,
/// nBits: compact representation of the block difficulty target.
pub bits: String,
/// The difficulty target.
pub target: String,
/// The difficulty.
pub difficulty: f64,
/// Expected number of hashes required to produce the chain up to this block (in hex).
#[serde(rename = "chainwork")]
pub chain_work: String,
/// The number of transactions in the block.
#[serde(rename = "nTx")]
pub n_tx: i64,
/// The hash of the previous block (if available).
#[serde(rename = "previousblockhash")]
pub previous_block_hash: Option<String>,
/// The hash of the next block (if available).
#[serde(rename = "nextblockhash")]
pub next_block_hash: Option<String>,
}

pub struct GetBlockVerboseTwoTransaction {
/// The transaction data (same as `getrawtransaction` verbose output).
#[serde(flatten)]
pub transaction: GetRawTransactionVerbose,
/// The transaction fee in BTC (omitted if block undo data is not available).
pub fee: Option<f64>,
}

pub struct GetBlockVerboseZero(
/// A string that is serialized, hex-encoded data for block 'hash'.
pub String,
);

pub struct GetBlockchainInfo {
/// Current network name as defined in BIP70 (main, test, signet, regtest).
pub chain: String,
/// The current number of blocks processed in the server.
pub blocks: i64,
/// The current number of headers we have validated.
pub headers: i64,
/// The hash of the currently best block.
#[serde(rename = "bestblockhash")]
pub best_block_hash: String,
/// nBits: compact representation of the block difficulty target.
pub bits: String,
/// The difficulty target.
pub target: String,
/// The current difficulty.
pub difficulty: f64,
/// The block time expressed in UNIX epoch time.
pub time: i64,
/// The median block time expressed in UNIX epoch time.
#[serde(rename = "mediantime")]
pub median_time: i64,
/// Estimate of verification progress (between 0 and 1).
#[serde(rename = "verificationprogress")]
pub verification_progress: f64,
/// Estimate of whether this node is in Initial Block Download (IBD) mode.
#[serde(rename = "initialblockdownload")]
pub initial_block_download: bool,
/// Total amount of work in active chain, in hexadecimal.
#[serde(rename = "chainwork")]
pub chain_work: String,
/// The estimated size of the block and undo files on disk.
pub size_on_disk: u64,
/// If the blocks are subject to pruning.
pub pruned: bool,
/// Lowest-height complete block stored (only present if pruning is enabled).
#[serde(rename = "pruneheight")]
pub prune_height: Option<i64>,
/// Whether automatic pruning is enabled (only present if pruning is enabled).
pub automatic_pruning: Option<bool>,
/// The target size used by pruning (only present if automatic pruning is enabled).
pub prune_target_size: Option<i64>,
/// The block challenge (aka. block script).
pub signet_challenge: Option<String>,
/// Any network and blockchain warnings.
pub warnings: Vec<String>,
}

pub struct GetChainStates {
/// The number of headers seen so far.
pub headers: i64,
/// List of the chainstates ordered by work, with the most-work (active) chainstate last.
#[serde(rename = "chainstates")]
pub chain_states: Vec<ChainState>,
}

pub struct GetChainTips(pub Vec<ChainTips>);
/// Chain tip item. Part of `getchaintips`.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetChainTxStats {
/// The timestamp for the final block in the window in UNIX format.
pub time: i64,
/// The total number of transactions in the chain up to that point.
#[serde(rename = "txcount")]
pub tx_count: i64,
/// The hash of the final block in the window.
pub window_final_block_hash: String,
/// The height of the final block in the window.
pub window_final_block_height: i64,
/// Size of the window in number of blocks.
pub window_block_count: i64,
/// The number of transactions in the window. Only returned if "window_block_count" is > 0.
pub window_tx_count: Option<i64>,
/// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0.
pub window_interval: Option<i64>,
/// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0.
#[serde(rename = "txrate")]
pub tx_rate: Option<f64>,
}

pub struct GetConnectionCount(pub u64);
/// Result of JSON-RPC method `getnettotals`.
///
/// > getnettotals
/// >
/// > Returns information about network traffic, including bytes in, bytes out,
/// > and current time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetDeploymentInfo {
/// Requested block hash (or tip).
pub hash: String,
/// Requested block height (or tip).
pub height: u32,
/// Deployments info, keyed by deployment name.
pub deployments: std::collections::BTreeMap<String, DeploymentInfo>,
}

pub struct GetDescriptorActivity {
pub activity: Vec<ActivityEntry>,
}

pub struct GetDescriptorInfo {
/// The descriptor in canonical form, without private keys. For a multipath descriptor, only the
/// first will be returned.
pub descriptor: String,
/// All descriptors produced by expanding multipath derivation elements. Only if the provided
/// descriptor specifies multipath derivation elements.
pub multipath_expansion: Option<Vec<String>>,
/// The checksum for the input descriptor.
pub checksum: String,
/// Whether the descriptor is ranged.
#[serde(rename = "isrange")]
pub is_range: bool,
/// Whether the descriptor is solvable.
#[serde(rename = "issolvable")]
pub is_solvable: bool,
/// Whether the input descriptor contained at least one private key.
#[serde(rename = "hasprivatekeys")]
pub has_private_keys: bool,
}

pub struct GetDifficulty(pub f64);
/// Result of JSON-RPC method `getmempoolancestors` with verbose set to `false`.
///
/// > getmempoolancestors txid (verbose)
/// >
/// > If txid is in the mempool, returns all in-mempool ancestors.
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetHdKeys(pub Vec<HdKey>);
/// HD key entry. Part of `gethdkeys`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetIndexInfo(pub BTreeMap<String, GetIndexInfoName>);
/// Index info details. Part of `getindexinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetIndexInfoName {
/// Whether the index is synced or not.
pub synced: bool,
/// The block height to which the index is synced.
pub best_block_height: u32,
}

pub struct GetMemoryInfoStats(pub BTreeMap<String, Locked>);
/// Information about locked memory manager. Part of `getmemoryinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetMempoolAncestors(pub Vec<String>);
/// Result of JSON-RPC method `getmempoolancestors` with verbose set to true.
///
/// Map of txid to `MempoolEntry` i.e., an ancestor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetMempoolAncestorsVerbose(pub BTreeMap<String, MempoolEntry>);
/// Result of JSON-RPC method `getmempooldescendants` with verbose set to `false`.
///
/// > getmempooldescendants txid (verbose)
/// >
/// > If txid is in the mempool, returns all in-mempool descendants.
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetMempoolDescendants(pub Vec<String>);
/// Result of JSON-RPC method `getmempooldescendants` with verbose set to true.
///
/// Map of txid to [`MempoolEntry`] i.e., a descendant.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetMempoolDescendantsVerbose(pub BTreeMap<String, MempoolEntry>);
/// Result of JSON-RPC method `getmempoolentry`.
///
/// > getmempoolentry txid
/// >
/// > Returns mempool data for given transaction
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetMempoolEntry(pub MempoolEntry);
/// Result of JSON-RPC method `getrawmempool` with verbose set to `true`.
///
/// Map of txid to [`MempoolEntry`].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetMempoolInfo {
/// True if the initial load attempt of the persisted mempool finished.
pub loaded: bool,
/// Current tx count.
pub size: i64,
/// Sum of all virtual transaction sizes as defined in BIP 141.
///
/// Differs from actual serialized size because witness data is discounted.
pub bytes: i64,
/// Total memory usage for the mempool.
pub usage: i64,
/// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction.
pub total_fee: f64,
/// Maximum memory usage for the mempool.
#[serde(rename = "maxmempool")]
pub max_mempool: i64,
/// Minimum fee rate in BTC/kB for a transaction to be accepted.
///
/// This is the maximum of `minrelaytxfee` and the minimum mempool fee.
#[serde(rename = "mempoolminfee")]
pub mempool_min_fee: f64,
/// Current minimum relay fee for transactions.
#[serde(rename = "minrelaytxfee")]
pub min_relay_tx_fee: f64,
/// Minimum fee rate increment for mempool limiting or replacement in BTC/kvB.
#[serde(rename = "incrementalrelayfee")]
pub incremental_relay_fee: f64,
/// Current number of transactions that haven't passed initial broadcast yet.
#[serde(rename = "unbroadcastcount")]
pub unbroadcast_count: i64,
/// True if the mempool accepts RBF without replaceability signaling inspection.
#[serde(rename = "fullrbf")]
pub full_rbf: bool,
/// True if the mempool accepts transactions with bare multisig outputs.
#[serde(rename = "permitbaremultisig")]
pub permit_bare_multisig: bool,
/// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool.
#[serde(rename = "maxdatacarriersize")]
pub max_data_carrier_size: u64,
}

pub struct GetMiningInfo {
/// The current block.
pub blocks: u64,
/// The block weight (including reserved weight for block header, txs count and coinbase tx) of
/// the last assembled block (only present if a block was ever assembled).
#[serde(rename = "currentblockweight")]
pub current_block_weight: Option<u64>,
/// The number of block transactions (excluding coinbase) of the last assembled block (only
/// present if a block was ever assembled).
#[serde(rename = "currentblocktx")]
pub current_block_tx: Option<i64>,
/// The current nBits, compact representation of the block difficulty target.
pub bits: String,
/// The current difficulty.
pub difficulty: f64,
/// The current target.
pub target: String,
/// The network hashes per second.
#[serde(rename = "networkhashps")]
pub network_hash_ps: f64,
/// The size of the mempool.
#[serde(rename = "pooledtx")]
pub pooled_tx: i64,
/// Minimum feerate of packages selected for block inclusion in BTC/kvB.
#[serde(rename = "blockmintxfee")]
pub block_min_tx_fee: f64,
/// Current network name as defined in BIP70 (main, test, regtest).
pub chain: String,
/// The block challenge (aka. block script), in hexadecimal (only present if the current network
/// is a signet).
pub signet_challenge: Option<String>,
/// The next block.
pub next: NextBlockInfo,
/// Any network and blockchain warnings.
pub warnings: Vec<String>,
}

pub struct GetNetTotals {
/// Total bytes received.
#[serde(rename = "totalbytesrecv")]
pub total_bytes_received: u64,
/// Total bytes sent.
#[serde(rename = "totalbytessent")]
pub total_bytes_sent: u64,
/// Current UNIX time in milliseconds.
#[serde(rename = "timemillis")]
pub time_millis: u64,
/// Upload target totals.
#[serde(rename = "uploadtarget")]
pub upload_target: UploadTarget,
}

pub struct GetNetworkInfo {
/// The server version.
pub version: usize,
/// The server subversion string.
pub subversion: String,
/// The protocol version.
#[serde(rename = "protocolversion")]
pub protocol_version: usize,
/// The services we offer to the network (hex string).
#[serde(rename = "localservices")]
pub local_services: String,
/// The services we offer to the network.
#[serde(rename = "localservicesnames")]
pub local_services_names: Vec<String>,
/// `true` if transaction relay is requested from peers.
#[serde(rename = "localrelay")]
pub local_relay: bool,
/// The time offset.
#[serde(rename = "timeoffset")]
pub time_offset: isize,
/// The total number of connections.
pub connections: usize,
/// The number of inbound connections.
pub connections_in: usize,
/// The number of outbound connections.
pub connections_out: usize,
/// Whether p2p networking is enabled.
#[serde(rename = "networkactive")]
pub network_active: bool,
/// Information per network.
pub networks: Vec<GetNetworkInfoNetwork>,
/// Minimum relay fee rate for transactions in BTC/kB.
#[serde(rename = "relayfee")]
pub relay_fee: f64,
/// Minimum fee rate increment for mempool limiting or replacement in BTC/kB.
#[serde(rename = "incrementalfee")]
pub incremental_fee: f64,
/// List of local addresses.
#[serde(rename = "localaddresses")]
pub local_addresses: Vec<GetNetworkInfoAddress>,
/// Any network and blockchain warnings.
pub warnings: Vec<String>,
}

pub struct GetNetworkInfoAddress {
/// Network address.
pub address: String,
/// Network port.
pub port: u16,
/// Relative score.
pub score: u32,
}

pub struct GetNetworkInfoNetwork {
/// Network (ipv4, ipv6, onion, i2p, cjdns).
pub name: String,
/// Is the network limited using -onlynet?.
pub limited: bool,
/// Is the network reachable?
pub reachable: bool,
/// ("host:port"): The proxy that is used for this network, or empty if none.
pub proxy: String,
/// Whether randomized credentials are used.
pub proxy_randomize_credentials: bool,
}

pub struct GetNewAddress(pub String);
/// Result of the JSON-RPC method `getrawchangeaddress`.
///
/// > getrawchangeaddress ( "address_type" )
/// >
/// > Returns a new Bitcoin address, for receiving change.
/// > This is for use with raw transactions, NOT normal use.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetNodeAddresses(pub Vec<NodeAddress>);
/// An node address item. Part of `getnodeaddresses`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetOrphanTxs(pub Vec<String>);
/// Result of JSON-RPC method `getorphantxs` verbosity 1.
///
/// > getorphantxs ( verbosity )
/// >
/// > Shows transactions in the tx orphanage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetOrphanTxsVerboseOne(pub Vec<GetOrphanTxsVerboseOneEntry>);
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetOrphanTxsVerboseOneEntry {
/// The transaction hash in hex
pub txid: String,
/// The transaction witness hash in hex
pub wtxid: String,
/// The serialized transaction size in bytes
pub bytes: u64,
/// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
pub vsize: u64,
/// The transaction weight as defined in BIP 141.
pub weight: u64,
/// The entry time into the orphanage expressed in UNIX epoch time.
#[serde(rename = "entry")]
pub entry_time: Option<u32>,
/// The orphan expiration time expressed in UNIX epoch time.
#[serde(rename = "expiration")]
pub expiration_time: Option<u32>,
/// List of peer ids that we store this transaction for.
pub from: Vec<u64>,
}

pub struct GetOrphanTxsVerboseTwo(pub Vec<GetOrphanTxsVerboseTwoEntry>);
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetOrphanTxsVerboseTwoEntry {
/// The transaction hash in hex
pub txid: String,
/// The transaction witness hash in hex
pub wtxid: String,
/// The serialized transaction size in bytes
pub bytes: u64,
/// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
pub vsize: u64,
/// The transaction weight as defined in BIP 141.
pub weight: u64,
/// List of peer ids that we store this transaction for.
pub from: Vec<u64>,
/// The entry time into the orphanage expressed in UNIX epoch time.
pub entry_time: Option<u32>,
/// The orphan expiration time expressed in UNIX epoch time.
pub expiration_time: Option<u32>,
/// The serialized, hex-encoded transaction data.
pub hex: String,
}

pub struct GetPeerInfo(pub Vec<PeerInfo>);
/// A peer info item. Part of `getpeerinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetPrioritisedTransactions(
/// prioritisation keyed by txid.
pub BTreeMap<String, PrioritisedTransaction>,
);

pub struct GetRawAddrMan {
/// Addresses in the "new" table (potential peers discovered but not yet connected to).
pub new: BTreeMap<String, RawAddrManEntry>,
/// Addresses in the "tried" table (peers successfully connected to in the past).
pub tried: BTreeMap<String, RawAddrManEntry>,
}

pub struct GetRawChangeAddress(pub String);
/// Result of the JSON-RPC method `getreceivedbyaddress`.
///
/// > getreceivedbyaddress "address" ( minconf )
/// >
/// > Returns the total amount received by the given address in transactions with at least minconf confirmations.
/// >
/// > Arguments:
/// > 1. "address"         (string, required) The bitcoin address for transactions.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetRawMempool(pub Vec<String>);
/// Result of JSON-RPC method `getrawmempool` with verbose set to `true`.
///
/// Map of txid to [`MempoolEntry`].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetRawMempoolSequence {
/// List of transaction ids in the mempool.
pub txids: Vec<String>,
/// The mempool sequence value.
pub mempool_sequence: u64,
}

pub struct GetRawMempoolVerbose(pub BTreeMap<String, MempoolEntry>);
/// Mempool data. Part of `getmempoolentry`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetRawTransaction(
/// The serialized, hex-encoded data for 'txid'.
pub String,
);

pub struct GetRawTransactionVerbose {
/// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument).
pub in_active_chain: Option<bool>,
/// The serialized, hex-encoded data for 'txid'.
pub hex: String,
/// The transaction id (same as provided).
pub txid: String,
/// The transaction hash (differs from txid for witness transactions).
pub hash: String,
/// The serialized transaction size.
pub size: u64,
/// The virtual transaction size (differs from size for witness transactions).
pub vsize: u64,
/// The transaction's weight (between vsize*4-3 and vsize*4).
pub weight: u64,
/// The version.
pub version: i32,
/// The lock time.
#[serde(rename = "locktime")]
pub lock_time: u32,
/// Array of transaction inputs.
#[serde(rename = "vin")]
pub inputs: Vec<RawTransactionInput>,
/// Array of transaction outputs.
#[serde(rename = "vout")]
pub outputs: Vec<RawTransactionOutput>,
// The following fields are all `None` if the transaction is in the mempool.
/// The block hash.
#[serde(rename = "blockhash")]
pub block_hash: Option<String>,
/// The confirmations.
pub confirmations: Option<u64>,
/// The transaction time in seconds since epoch (Jan 1 1970 GMT).
#[serde(rename = "time")]
pub transaction_time: Option<u64>,
/// The block time in seconds since epoch (Jan 1 1970 GMT).
#[serde(rename = "blocktime")]
pub block_time: Option<u64>,
}

pub struct GetRawTransactionVerboseWithPrevout {
/// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument).
pub in_active_chain: Option<bool>,
/// The serialized, hex-encoded data for 'txid'.
pub hex: String,
/// The transaction id (same as provided).
pub txid: String,
/// The transaction hash (differs from txid for witness transactions).
pub hash: String,
/// The serialized transaction size.
pub size: u64,
/// The virtual transaction size (differs from size for witness transactions).
pub vsize: u64,
/// The transaction's weight (between vsize*4-3 and vsize*4).
pub weight: u64,
/// The version.
pub version: i32,
/// The lock time.
#[serde(rename = "locktime")]
pub lock_time: u32,
/// Array of transaction inputs.
#[serde(rename = "vin")]
pub inputs: Vec<RawTransactionInputWithPrevout>,
/// Array of transaction outputs.
#[serde(rename = "vout")]
pub outputs: Vec<RawTransactionOutput>,
/// The block hash.
#[serde(rename = "blockhash")]
pub block_hash: Option<String>,
/// The confirmations.
pub confirmations: Option<u64>,
/// The transaction time in seconds since epoch (Jan 1 1970 GMT).
#[serde(rename = "time")]
pub transaction_time: Option<u64>,
/// The block time in seconds since epoch (Jan 1 1970 GMT).
#[serde(rename = "blocktime")]
pub block_time: Option<u64>,
}

pub struct GetReceivedByAddress(pub f64); // Amount in BTC.
/// Result of the JSON-RPC method `gettransaction`.
///
/// > gettransaction "txid" ( include_watchonly )
/// >
/// > Get detailed information about in-wallet transaction `<txid>`
/// >
/// > Arguments:
/// > 1. txid                 (string, required) The transaction id
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetReceivedByLabel(pub f64);
/// Result of the JSON-RPC method `getwalletinfo`.
///
/// > getwalletinfo
/// > Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetRpcInfo {
/// All active commands
pub active_commands: Vec<ActiveCommand>,
/// The complete file path to the debug log
#[serde(rename = "logpath")]
pub log_path: String,
}

pub struct GetTransaction {
/// The transaction amount in BTC.
pub amount: f64,
/// The amount of the fee in BTC.
///
/// This is negative and only available for the 'send' category of transactions.
pub fee: Option<f64>,
/// The number of confirmations.
pub confirmations: i64,
/// Only present if the transaction's only input is a coinbase one.
pub generated: Option<bool>,
/// Whether we consider the outputs of this unconfirmed transaction safe to spend.
pub trusted: Option<bool>,
/// The block hash.
#[serde(rename = "blockhash")]
pub block_hash: Option<String>,
/// The block height containing the transaction.
#[serde(rename = "blockheight")]
pub block_height: Option<i64>,
/// The index of the transaction in the block that includes it.
#[serde(rename = "blockindex")]
pub block_index: Option<i64>,
/// The time in seconds since epoch (1 Jan 1970 GMT).
#[serde(rename = "blocktime")]
pub block_time: Option<u32>,
/// The transaction id.
pub txid: String,
/// The hash of serialized transaction, including witness data.
pub wtxid: Option<String>,
/// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
#[serde(rename = "walletconflicts")]
pub wallet_conflicts: Vec<String>,
/// Only if 'category' is 'send'. The txid if this tx was replaced.
pub replaced_by_txid: Option<String>,
/// Only if 'category' is 'send'. The txid if this tx replaces another.
pub replaces_txid: Option<String>,
/// Transactions in the mempool that directly conflict with either this transaction or an ancestor
/// transaction.
#[serde(rename = "mempoolconflicts")]
pub mempool_conflicts: Option<Vec<String>>,
/// If a comment to is associated with the transaction.
pub to: Option<String>,
/// The transaction time in seconds since epoch (1 Jan 1970 GMT).
pub time: u32,
/// The time received in seconds since epoch (1 Jan 1970 GMT).
#[serde(rename = "timereceived")]
pub time_received: u32,
/// If a comment is associated with the transaction, only present if not empty.
pub comment: Option<String>,
/// Whether this transaction could be replaced due to BIP125 (replace-by-fee);

pub struct GetTransactionDetail {
/// Only returns true if imported addresses were involved in transaction.
#[serde(rename = "involvesWatchonly")]
pub involves_watch_only: Option<bool>,
/// DEPRECATED. The account name involved in the transaction, can be "" for the default account.
pub account: Option<String>, // Docs are wrong, this is not documented as optional.
/// The bitcoin address involved in the transaction.
pub address: String,
/// The category, either 'send' or 'receive'.
pub category: TransactionCategory,
///  The amount in BTC.
pub amount: f64,
/// A comment for the address/transaction, if any.
pub label: Option<String>,
/// the vout value.
pub vout: u32,
/// The amount of the fee.
///
/// This is negative and only available for the 'send' category of transactions.
pub fee: Option<f64>,
/// If the transaction has been abandoned (inputs are respendable).
///
/// Only available for the 'send' category of transactions.
pub abandoned: Option<bool>,
/// Only if 'category' is 'received'. List of parent descriptors for the output script of this
/// coin.
#[serde(rename = "parent_descs")]
pub parent_descriptors: Option<Vec<String>>,
}

pub struct GetTxOut {
/// The hash of the block at the tip of the chain.
#[serde(rename = "bestblock")]
pub best_block: String,
/// The number of confirmations.
pub confirmations: u32, // TODO: Change this to an i64.
/// The transaction value in BTC.
pub value: f64,
/// The script pubkey.
#[serde(rename = "scriptPubKey")]
pub script_pubkey: ScriptPubKey,
/// Coinbase or not.
pub coinbase: bool,
}

pub struct GetTxOutSetInfo {
/// The current block height (index).
pub height: i64,
/// The hash of the block at the tip of the chain.
#[serde(rename = "bestblock")]
pub best_block: String,
/// The number of transactions with unspent outputs (not available when coinstatsindex is used).
pub transactions: Option<i64>,
/// The number of unspent transaction outputs.
#[serde(rename = "txouts")]
pub tx_outs: i64,
/// A meaningless metric for UTXO set size.
#[serde(rename = "bogosize")]
pub bogo_size: i64,
/// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen).
pub hash_serialized_3: Option<String>,
/// The estimated size of the chainstate on disk (not available when coinstatsindex is used).
pub disk_size: Option<i64>,
/// The total amount.
pub total_amount: f64,
/// The serialized hash (only present if 'muhash' hash_type is chosen).
pub muhash: Option<String>,
/// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used).
pub total_unspendable_amount: Option<f64>,
/// Info on amounts in the block at this block height (only available if coinstatsindex is used).
pub block_info: Option<GetTxOutSetInfoBlockInfo>,
}

pub struct GetTxOutSetInfoBlockInfo {
/// Total amount of all prevouts spent in this block.
#[serde(rename = "prevout_spent")]
pub prevout_spent: f64,
/// Coinbase subsidy amount of this block.
pub coinbase: f64,
/// Total amount of new outputs created by this block.
#[serde(rename = "new_outputs_ex_coinbase")]
pub new_outputs_ex_coinbase: f64,
/// Total amount of unspendable outputs created in this block.
pub unspendable: f64,
/// Detailed view of unspendable categories.
pub unspendables: GetTxOutSetInfoUnspendables,
}

pub struct GetTxOutSetInfoUnspendables {
/// The unspendable amount of the Genesis block subsidy.
pub genesis_block: f64,
/// Transactions overridden by duplicates (no longer possible with BIP30).
pub bip30: f64,
/// Amounts sent to scripts that are unspendable (for example OP_RETURN outputs).
pub scripts: f64,
/// Fee rewards that miners did not claim in their coinbase transaction.
pub unclaimed_rewards: f64,
}

pub struct GetTxSpendingPrevout(pub Vec<GetTxSpendingPrevoutItem>);
/// A transaction item. Part of `gettxspendingprevout`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetTxSpendingPrevoutItem {
/// The transaction id of the checked output
pub txid: String,
/// The vout value of the checked output
pub vout: u32,
/// The transaction id of the mempool transaction spending this output (omitted if unspent)
#[serde(rename = "spendingtxid")]
pub spending_txid: Option<String>,
}

pub struct GetUnconfirmedBalance(pub f64); // Core docs are missing so this is just a guess.
/// Result of the JSON-RPC method `getwalletinfo`.
///
/// > getwalletinfo
/// > Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct GetWalletInfo {
/// the wallet name
#[serde(rename = "walletname")]
pub wallet_name: String,
/// the wallet version
#[serde(rename = "walletversion")]
pub wallet_version: i64,
/// the database format (bdb or sqlite)
pub format: String,
/// the total number of transactions in the wallet
#[serde(rename = "txcount")]
pub tx_count: i64,
/// how many new keys are pre-generated (only counts external keys)
#[serde(rename = "keypoolsize")]
pub keypool_size: i64,
/// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
#[serde(rename = "keypoolsize_hd_internal")]
pub keypool_size_hd_internal: Option<i64>,
/// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
pub unlocked_until: Option<u32>,
/// the transaction fee configuration, set in BTC/kvB
#[serde(rename = "paytxfee")]
pub pay_tx_fee: f64,
/// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
pub private_keys_enabled: bool,
/// whether this wallet tracks clean/dirty coins in terms of reuse
pub avoid_reuse: bool,
/// current scanning details, or false if no scan is in progress
pub scanning: GetWalletInfoScanning,
/// whether this wallet uses descriptors for scriptPubKey management
pub descriptors: bool,
/// whether this wallet is configured to use an external signer such as a hardware wallet
pub external_signer: bool,
/// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
pub blank: bool,
/// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
pub birthtime: Option<u32>,
/// The flags currently set on the wallet.
pub flags: Vec<String>,
/// hash and height of the block this information was generated on
#[serde(rename = "lastprocessedblock")]
pub last_processed_block: Option<LastProcessedBlock>,
}

pub struct GetZmqNotifications {
/// Type of notification.
#[serde(rename = "type")]
pub type_: String,
/// Address of the publisher.
pub address: String,
/// Outbound message high water mark.
pub hwm: u64,
}

pub struct GlobalXpub {
/// The extended public key this path corresponds to.
pub xpub: String,
/// The fingerprint of the master key.
pub master_fingerprint: String,
/// The path.
pub path: String,
}

pub struct HdKey {
/// The extended public key.
pub xpub: String,
/// Whether the wallet has the private key for this xpub.
pub has_private: bool,
/// The extended private key if "private" is true.
#[serde(rename = "xprv")]
pub xpriv: Option<String>,
/// Array of descriptor objects that use this HD key.
pub descriptors: Vec<HdKeyDescriptor>,
}

pub struct HdKeyDescriptor {
/// Descriptor string representation.
#[serde(rename = "desc")]
pub descriptor: String,
/// Whether this descriptor is currently used to generate new addresses.
pub active: bool,
}

pub struct ImportDescriptors(
/// Response is an array with the same size as the input that has the execution result.
pub Vec<ImportDescriptorsResult>,
);

pub struct ImportDescriptorsResult {
/// Whether the import was successful.
pub success: bool,
/// Warnings, if any.
pub warnings: Option<Vec<String>>,
/// Error object, if any.
pub error: Option<serde_json::Value>,
}

pub struct ImportMulti(pub Vec<ImportMultiEntry>);
/// A single import multi entry. Part of `importmulti`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]

pub struct ImportMultiEntry {
/// The success.
pub success: bool,
/// The warnings.
pub warnings: Option<Vec<String>>,
/// The error.
pub error: Option<JsonRpcError>,
}

pub struct JoinPsbts(
/// The base64-encoded partially signed transaction.
pub String,
);

pub struct JsonRpcError {
/// The error code.
pub code: i32,
/// The error message.
pub message: String,
/// The error data.
pub data: Option<serde_json::Value>, // Can hold arbitrary extra information
}

pub struct LastProcessedBlock {
/// Hash of the block this information was generated on.
pub hash: String,
/// Height of the block this information was generated on.
pub height: i64,
}

pub struct ListAddressGroupings(pub Vec<Vec<ListAddressGroupingsItem>>);
/// List item type. Part of `listaddressgroupings`.
///
/// Core encodes items as a JSON array with either 2 elements `[address, amount]` when there is no
/// label or 3 elements `[address, amount, label]` when a label is present. Represent this as an
/// untagged enum of tuple variants so Serde can match either length without custom code.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ListAddressGroupingsItem {
/// Entry without label.
Two(String, f64),
/// Entry with label.
Three(String, f64, String),
}

pub struct ListBanned(pub Vec<Banned>);
/// An banned item. Part of `listbanned`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct ListDescriptors {
/// Name of wallet this operation was performed on.
pub wallet_name: String,
/// Array of descriptor objects.
pub descriptors: Vec<DescriptorInfo>,
}

pub struct ListLabels(pub Vec<String>);
/// Result of the JSON-RPC method `listlockunspent`.
///
/// > listlockunspent
/// >
/// > Returns list of temporarily unspendable outputs.
/// > See the lockunspent call to lock and unlock transactions for spending.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct ListLockUnspent(pub Vec<ListLockUnspentItem>);
/// List lock unspent item. Part of of `listlockunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct ListLockUnspentItem {
/// The transaction id locked.
pub txid: String,
/// The vout value.
pub vout: i64,
}

pub struct ListReceivedByAddress(pub Vec<ListReceivedByAddressItem>);
/// List received by address item. Part of of `listreceivedbyaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct ListReceivedByAddressItem {
/// Only returned if imported addresses were involved in transaction.
#[serde(rename = "involvesWatchonly")]
pub involves_watch_only: Option<bool>,
/// The receiving address.
pub address: String,
/// The total amount in BTC received by the address.
pub amount: f64,
/// The number of confirmations of the most recent transaction included.
pub confirmations: i64,
/// The label of the receiving address. The default label is "".
pub label: String,
/// The ids of transactions received with the address.
pub txids: Vec<String>,
}

pub struct ListReceivedByLabel(pub Vec<ListReceivedByLabelItem>);
/// List received by label item. Part of of `listreceivedbyaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct ListReceivedByLabelItem {
/// Only returned if imported addresses were involved in transaction.
#[serde(rename = "involvesWatchonly")]
pub involves_watch_only: Option<bool>,
/// The total amount received by addresses with this label.
pub amount: f64,
/// The number of confirmations of the most recent transaction included.
pub confirmations: i64,
/// The label of the receiving address. The default label is "".
pub label: String,
}

pub struct ListSinceBlock {
/// All the transactions.
pub transactions: Vec<TransactionItem>,
/// Only present if `include_removed=true`.
///
/// Note: transactions that were re-added in the active chain will appear as-is in this array,
/// and may thus have a positive confirmation count.
pub removed: Vec<TransactionItem>,
/// The hash of the block (target_confirmations-1) from the best block on the main chain.
///
/// This is typically used to feed back into listsinceblock the next time you call it. So you
/// would generally use a target_confirmations of say 6, so you will be continually
/// re-notified of transactions until they've reached 6 confirmations plus any new ones.
#[serde(rename = "lastblock")]
pub last_block: String,
}

pub struct ListTransactions(pub Vec<TransactionItem>);
}

pub struct ListUnspent(pub Vec<ListUnspentItem>);
/// Unspent transaction output, part of `listunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct ListUnspentItem {
/// The transaction id.
pub txid: String,
/// The vout value.
pub vout: i64,
/// The bitcoin address of the transaction.
pub address: String,
/// The associated label, or "" for the default label.
pub label: Option<String>,
/// The script key.
#[serde(rename = "scriptPubKey")]
pub script_pubkey: String,
/// The transaction amount in BTC.
pub amount: f64,
/// The number of confirmations.
pub confirmations: i64,
/// The redeemScript if scriptPubKey is P2SH.
#[serde(rename = "redeemScript")]
pub redeem_script: Option<String>,
/// Whether we have the private keys to spend this output.
pub spendable: bool,
/// Whether we know how to spend this output, ignoring the lack of keys.
pub solvable: bool,
/// A descriptor for spending this output (only when solvable)
#[serde(rename = "desc")]
pub descriptor: Option<String>,
/// Whether this output is considered safe to spend. Unconfirmed transactions from outside keys
/// and unconfirmed replacement transactions are considered unsafe and are not eligible for
/// spending by fundrawtransaction and sendtoaddress.
pub safe: bool,
/// List of parent descriptors for the scriptPubKey of this coin.
#[serde(rename = "parent_descs")]
pub parent_descriptors: Option<Vec<String>>,
}

pub struct ListWalletDir {
/// The list of wallets in the wallet directory.
pub wallets: Vec<ListWalletDirWallet>,
}

pub struct ListWalletDirWallet {
/// The wallet name.
pub name: String,
/// Warning messages, if any, related to loading the wallet.
pub warnings: Option<Vec<String>>,
}

pub struct ListWallets(pub Vec<String>);
/// Result of the JSON-RPC method `loadwallet`.
///
/// > loadwallet "filename"
/// >
/// > Loads a wallet from a wallet file or directory.
/// > Note that all wallet command-line options used when starting bitcoind will be
/// > applied to the new wallet (eg -zapwallettxes, upgradewallet, rescan, etc).
/// >
/// > Arguments:
/// > 1. "filename"    (string, required) The wallet directory or .dat file.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct LoadTxOutSet {
/// The number of coins loaded from the snapshot.
pub coins_loaded: f64,
/// The hash of the base of the snapshot.
pub tip_hash: String,
/// The height of the base of the snapshot.
pub base_height: i64,
/// The absolute path that the snapshot was loaded from.
pub path: String,
}

pub struct LoadWallet {
/// The wallet name if loaded successfully.
pub name: String,
/// Warning messages, if any, related to loading the wallet.
pub warnings: Option<Vec<String>>,
}

pub struct LockUnspent(pub bool);
/// Result of the JSON-RPC method `rescanblockchain`.
///
/// > rescanblockchain ("start_height") ("stop_height")
/// >
/// > Rescan the local blockchain for wallet related transactions.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct Locked {
/// Number of bytes used.
pub used: u64,
/// Number of bytes available in current arenas.
pub free: u64,
/// Total number of bytes managed.
pub total: u64,
/// Amount of bytes that succeeded locking.
///
/// If this number is smaller than total, locking pages failed at some point and key data could
/// be swapped to disk.
pub locked: u64,
/// Number allocated chunks.
pub chunks_used: u64,
/// Number unused chunks.
pub chunks_free: u64,
}

pub struct Logging {
pub addrman: bool,
pub bench: bool,
pub blockstorage: bool,
pub cmpctblock: bool,
pub coindb: bool,
pub estimatefee: bool,
pub http: bool,
pub i2p: bool,
pub ipc: bool,
pub leveldb: bool,
pub libevent: bool,
pub mempool: bool,
pub mempoolrej: bool,
pub net: bool,
pub prune: bool,
pub proxy: bool,
pub qt: bool,
pub rand: bool,
pub reindex: bool,
pub rpc: bool,
pub scan: bool,
pub selectcoins: bool,
pub tor: bool,
pub txpackages: bool,
pub txreconciliation: bool,
pub validation: bool,
pub walletdb: bool,
pub zmq: bool,
}

pub struct MempoolAcceptance {
/// The transaction hash in hex.
pub txid: String,
/// The transaction witness hash in hex.
pub wtxid: String,
/// If the mempool allows this tx to be inserted.
pub allowed: bool,
/// Virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted (only present when 'allowed' is true).
pub vsize: Option<i64>,
/// Transaction fees (only present if 'allowed' is true).
pub fees: Option<MempoolAcceptanceFees>,
/// Rejection string (only present when 'allowed' is false).
#[serde(rename = "reject-reason")]
pub reject_reason: Option<String>,
/// Rejection details (only present when 'allowed' is false and rejection details exist)
#[serde(rename = "reject-details")]
pub reject_details: Option<String>,
}

pub struct MempoolAcceptanceFees {
/// Transaction fee in BTC.
pub base: f64,
/// The effective feerate in BTC per KvB. May differ from the base feerate if, for example, there
/// are modified fees from `prioritisetransaction` or a package feerate was used.
#[serde(rename = "effective-feerate", default)]
pub effective_fee_rate: Option<f64>,
/// Transactions whose fees and vsizes are included in `effective_feerate`.
#[serde(rename = "effective-includes", default)]
pub effective_includes: Vec<String>,
}

pub struct MempoolEntry {
/// Virtual transaction size as defined in BIP 141.
///
/// This is different from actual serialized size for witness transactions as witness data is
/// discounted.
pub vsize: i64,
/// Transaction weight as defined in BIP 141.
pub weight: i64,
/// Local time transaction entered pool in seconds since 1 Jan 1970 GMT.
pub time: i64,
/// Block height when transaction entered pool.
pub height: i64,
/// Number of in-mempool descendant transactions (including this one).
#[serde(rename = "descendantcount")]
pub descendant_count: i64,
/// Virtual transaction size of in-mempool descendants (including this one).
#[serde(rename = "descendantsize")]
pub descendant_size: i64,
/// Number of in-mempool ancestor transactions (including this one).
#[serde(rename = "ancestorcount")]
pub ancestor_count: i64,
/// Virtual transaction size of in-mempool ancestors (including this one).
#[serde(rename = "ancestorsize")]
pub ancestor_size: i64,
/// Hash of serialized transaction, including witness data.
pub wtxid: String,
/// Fee object which contains the base fee, modified fee (with fee deltas), and ancestor/descendant fee totals all in BTC.
pub fees: MempoolEntryFees,
/// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
pub depends: Vec<String>,
/// Unconfirmed transactions spending outputs from this transaction (child transaction id).
#[serde(rename = "spentby")]
pub spent_by: Vec<String>,
/// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor
/// signaling BIP125 replaceability.
#[serde(rename = "bip125-replaceable")]
pub bip125_replaceable: bool,
/// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by
/// any peers)
pub unbroadcast: bool,
}

pub struct MempoolEntryFees {
/// Transaction fee in BTC.
pub base: f64,
/// Transaction fee with fee deltas used for mining priority in BTC.
pub modified: f64,
/// Modified fees (see above) of in-mempool ancestors (including this one) in BTC
pub ancestor: f64,
/// Modified fees (see above) of in-mempool descendants (including this one) in BTC.
pub descendant: f64,
}

pub struct MigrateWallet {
/// The name of the primary migrated wallet
pub wallet_name: String,
/// The name of the migrated wallet containing the watchonly scripts
pub watchonly_name: Option<String>,
/// The name of the migrated wallet containing solvable but not watched scripts
pub solvables_name: Option<String>,
/// The location of the backup of the original wallet
pub backup_path: String,
}

pub struct Musig2PartialSig {
/// The compressed public key of the participant that created this partial signature.
pub participant_pubkey: String,
/// The compressed aggregate public key for which this partial signature is for.
pub aggregate_pubkey: String,
/// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
pub leaf_hash: Option<String>,
/// The partial signature itself.
pub partial_sig: String,
}

pub struct Musig2ParticipantPubKeys {
/// The compressed aggregate public key for which the participants create.
pub aggregate_pubkey: String,
/// The compressed public keys that are aggregated for aggregate_pubkey.
pub participant_pubkeys: Vec<String>,
}

pub struct Musig2Pubnonce {
/// The compressed public key of the participant that created this pubnonce.
pub participant_pubkey: String,
/// The compressed aggregate public key for which this pubnonce is for.
pub aggregate_pubkey: String,
/// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
pub leaf_hash: Option<String>,
/// The public nonce itself.
pub pubnonce: String,
}

pub struct NextBlockInfo {
/// The next height.
pub height: u64,
/// The next nBits.
pub bits: String,
/// The next difficulty.
pub difficulty: f64,
/// The next target.
pub target: String,
}

pub struct NodeAddress {
/// Timestamp in seconds since epoch (Jan 1 1970 GMT) when the node was last seen.
pub time: u64,
/// The services offered.
pub services: u64,
/// The address of the node.
pub address: String,
/// The port of the node.
pub port: u16,
/// The network (ipv4, ipv6, onion, i2p, cjdns) the node connected through.
pub network: String,
}

pub struct PeerInfo {
/// Peer index.
pub id: u32,
/// The IP address and port of the peer ("host:port").
#[serde(rename = "addr")]
pub address: String,
/// Bind address of the connection to the peer ("ip:port").
#[serde(rename = "addrbind")]
pub address_bind: Option<String>,
/// Local address as reported by the peer.
#[serde(rename = "addrlocal")]
pub address_local: Option<String>,
/// Network (ipv4, ipv6, onion, i2p, cjdns, not_publicly_routable) the peer connected through.
pub network: String,
/// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying peer selection (only displayed if the -asmap config option is set).
pub mapped_as: Option<u32>,
/// The services offered.
pub services: String,
/// The services offered, in human-readable form.
#[serde(rename = "servicesnames")]
pub services_names: Vec<String>,
/// Whether peer has asked us to relay transactions to it.
#[serde(rename = "relaytxes")]
pub relay_transactions: bool,
/// The time in seconds since epoch (Jan 1 1970 GMT) of the last send.
#[serde(rename = "lastsend")]
pub last_send: i64,
/// The time in seconds since epoch (Jan 1 1970 GMT) of the last receive.
#[serde(rename = "lastrecv")]
pub last_received: i64,
/// The UNIX epoch time of the last valid transaction received from this peer.
pub last_transaction: i64,
/// The UNIX epoch time of the last block received from this peer.
pub last_block: i64,
/// The total bytes sent.
#[serde(rename = "bytessent")]
pub bytes_sent: u64,
/// The total bytes received.
#[serde(rename = "bytesrecv")]
pub bytes_received: u64,
/// The connection time in seconds since epoch (Jan 1 1970 GMT).
#[serde(rename = "conntime")]
pub connection_time: i64,
/// The time offset in seconds.
#[serde(rename = "timeoffset")]
pub time_offset: i64,
/// Ping time (if available).
#[serde(rename = "pingtime")]
pub ping_time: Option<f64>,
/// Minimum observed ping time (if any at all).
#[serde(rename = "minping")]
pub minimum_ping: Option<f64>,
/// Ping wait (if non-zero).
#[serde(rename = "pingwait")]
pub ping_wait: Option<f64>,
/// The peer version, such as 70001.
pub version: u32,
/// The string version (e.g. "/Satoshi:0.8.5/").
#[serde(rename = "subver")]
pub subversion: String,
/// Inbound (true) or Outbound (false).
pub inbound: bool,
/// Whether we selected peer as (compact blocks) high-bandwidth peer.
pub bip152_hb_to: bool,
/// Whether peer selected us as (compact blocks) high-bandwidth peer.
pub bip152_hb_from: bool,
/// Whether connection was due to addnode/-connect or if it was an automatic/inbound connection.
#[serde(rename = "addnode")]
pub add_node: Option<bool>,
/// The starting height (block) of the peer.
#[serde(rename = "startingheight")]
pub starting_height: Option<i64>,
/// The current height of header pre-synchronization with this peer, or -1 if no low-work sync is
/// in progress.
pub presynced_headers: Option<i64>,
/// The ban score.
#[serde(rename = "banscore")]
pub ban_score: Option<i64>,
/// The last header we have in common with this peer.
pub synced_headers: Option<i64>,
/// The last block we have in common with this peer.
pub synced_blocks: Option<i64>,
/// The heights of blocks we're currently asking from this peer.
pub inflight: Option<Vec<u64>>,
/// Whether we participate in address relay with this peer.
#[serde(rename = "addr_relay_enabled")]
pub addresses_relay_enabled: Option<bool>,
/// The total number of addresses processed, excluding those dropped due to rate limiting.
#[serde(rename = "addr_processed")]
pub addresses_processed: Option<usize>,
/// The total number of addresses dropped due to rate limiting.
#[serde(rename = "addr_rate_limited")]
pub addresses_rate_limited: Option<usize>,
/// Any special permissions that have been granted to this peer.
pub permissions: Vec<String>,
/// The minimum fee rate for transactions this peer accepts.
#[serde(rename = "minfeefilter")]
pub minimum_fee_filter: f64,
/// The total bytes sent aggregated by message type.
#[serde(rename = "bytessent_per_msg")]
pub bytes_sent_per_message: BTreeMap<String, u64>,
/// The total bytes received aggregated by message type.
#[serde(rename = "bytesrecv_per_msg")]
pub bytes_received_per_message: BTreeMap<String, u64>,
/// Type of connection.
pub connection_type: Option<String>,
/// Type of transport protocol:
/// detecting (peer could be v1 or v2),
/// v1 (plaintext transport protocol),
/// v2 (BIP324 encrypted transport protocol).
pub transport_protocol_type: String,
/// The session ID for this connection, or "" if there is none ("v2" transport protocol only).
pub session_id: String,
}

pub struct PrioritisedTransaction {
/// Transaction fee delta in satoshis.
pub fee_delta: i64,
/// Whether this transaction is currently in mempool.
pub in_mempool: bool,
/// Modified fee in satoshis. Only returned if in_mempool=true.
pub modified_fee: Option<u64>,
}

pub struct Proprietary {
/// The hex string for the proprietary identifier.
identifier: String,
/// The number for the subtype.
subtype: i64,
/// The hex for the key.
key: String,
/// The hex for the value.
value: String,
}

pub struct PruneBlockchain(
/// The height of the last block pruned.
pub i64,
);

pub struct PsbtBumpFee {
/// The base64-encoded unsigned PSBT of the new transaction.
pub psbt: String,
/// The fee of the replaced transaction.
#[serde(rename = "origfee")]
pub original_fee: f64,
/// The fee of the new transaction.
pub fee: f64,
/// Errors encountered during processing (may be empty).
pub errors: Vec<String>,
}

pub struct PsbtInput {
/// Decoded network transaction for non-witness UTXOs.
pub non_witness_utxo: Option<RawTransaction>,
/// Transaction output for witness UTXOs.
pub witness_utxo: Option<WitnessUtxo>,
/// The public key and signature that corresponds to it.
pub partial_signatures: Option<HashMap<String, String>>,
/// The sighash type to be used.
pub sighash: Option<String>,
/// The redeem script.
pub redeem_script: Option<PsbtScript>,
/// The witness script.
pub witness_script: Option<PsbtScript>,
/// The public key with the derivation path as the value.
pub bip32_derivs: Option<Vec<Bip32Deriv>>,
/// The final scriptsig.
#[serde(rename = "final_scriptsig")]
pub final_script_sig: Option<ScriptSig>,
/// Hex-encoded witness data (if any).
#[serde(rename = "final_scriptwitness")]
pub final_script_witness: Option<Vec<String>>,
/// The hash and preimage that corresponds to it.
pub ripemd160_preimages: Option<HashMap<String, String>>,
/// The hash and preimage that corresponds to it.
pub sha256_preimages: Option<HashMap<String, String>>,
/// The hash and preimage that corresponds to it.
pub hash160_preimages: Option<HashMap<String, String>>,
/// The hash and preimage that corresponds to it.
pub hash256_preimages: Option<HashMap<String, String>>,
/// Hex-encoded signature for the Taproot key path spend.
pub taproot_key_path_sig: Option<String>,
/// The signature for the pubkey and leaf hash combination.
pub taproot_script_path_sigs: Option<Vec<TaprootScriptPathSig>>,
/// Scripts and control blocks for script path spends.
pub taproot_scripts: Option<Vec<TaprootScript>>,
/// BIP-32 derivation paths for keys.
pub taproot_bip32_derivs: Option<Vec<TaprootBip32Deriv>>,
/// The hex-encoded Taproot x-only internal key.
pub taproot_internal_key: Option<String>,
/// The hex-encoded Taproot merkle root.
pub taproot_merkle_root: Option<String>,
/// MuSig2 participant public keys.
pub musig2_participant_pubkeys: Option<Vec<Musig2ParticipantPubKeys>>,
/// MuSig2 public nonces.
pub musig2_pubnonces: Option<Vec<Musig2Pubnonce>>,
/// MuSig2 partial signatures.
pub musig2_partial_sigs: Option<Vec<Musig2PartialSig>>,
/// The input proprietary map.
pub proprietary: Option<Vec<Proprietary>>,
/// The unknown input fields.
pub unknown: Option<HashMap<String, String>>,
}

pub struct PsbtOutput {
/// The redeem script.
pub redeem_script: Option<PsbtScript>,
/// The witness script.
pub witness_script: Option<PsbtScript>,
/// The public key with the derivation path as the value.
pub bip32_derivs: Option<Vec<Bip32Deriv>>,
/// The hex-encoded Taproot x-only internal key.
pub taproot_internal_key: Option<String>,
/// The tuples that make up the Taproot tree, in depth first search order.
pub taproot_tree: Option<Vec<TaprootLeaf>>,
/// BIP32 derivation paths for keys.
pub taproot_bip32_derivs: Option<Vec<TaprootBip32Deriv>>,
/// MuSig2 participant public keys.
pub musig2_participant_pubkeys: Option<Vec<Musig2ParticipantPubKeys>>,
/// The output proprietary map.
pub proprietary: Option<Vec<Proprietary>>,
/// The unknown global fields.
pub unknown: Option<HashMap<String, String>>,
}

pub struct RawAddrManEntry {
/// The address of the node.
pub address: String,
/// Mapped AS (Autonomous System) number at the end of the BGP route to the peer.
/// Only present if the -asmap config option is set.
pub mapped_as: Option<u32>,
/// The port number of the node.
pub port: u16,
/// The network (ipv4, ipv6, onion, i2p, cjdns) of the address.
pub network: String,
/// The services offered by the node.
pub services: u64,
/// The UNIX epoch time when the node was last seen.
pub time: i64,
/// The address that relayed the address to us.
pub source: String,
/// The network (ipv4, ipv6, onion, i2p, cjdns) of the source address.
pub source_network: String,
/// Mapped AS (Autonomous System) number at the end of the BGP route to the source.
/// Only present if the -asmap config option is set.
pub source_mapped_as: Option<u32>,
}

pub struct RawFeeDetail {
/// Estimate fee rate in BTC/kB.
#[serde(rename = "feerate")]
pub fee_rate: Option<f64>,
/// Exponential decay (per block) for historical moving average of confirmation data.
pub decay: f64,
/// The resolution of confirmation targets at this time horizon.
pub scale: u32,
/// Information about the lowest range of feerates to succeed in meeting the threshold.
pub pass: Option<RawFeeRange>,
/// Information about the highest range of feerates to fail to meet the threshold.
pub fail: Option<RawFeeRange>,
/// Errors encountered during processing.
pub errors: Option<Vec<String>>,
}

pub struct RawFeeRange {
/// Start of feerate range.
#[serde(rename = "startrange")]
pub start_range: f64,
/// End of feerate range.
#[serde(rename = "endrange")]
pub end_range: f64,
/// Number of txs over history horizon in the feerate range that were confirmed within target.
#[serde(rename = "withintarget")]
pub within_target: f64,
/// Number of txs over history horizon in the feerate range that were confirmed at any point.
#[serde(rename = "totalconfirmed")]
pub total_confirmed: f64,
/// Current number of txs in mempool in the feerate range unconfirmed for at least target blocks.
#[serde(rename = "inmempool")]
pub in_mempool: f64,
/// Number of txs over history horizon in the feerate range that left mempool unconfirmed after target.
#[serde(rename = "leftmempool")]
pub left_mempool: f64,
}

pub struct RawTransactionInputWithPrevout {
/// The input data.
#[serde(flatten)]
pub input: RawTransactionInput,
/// (only if undo data is available).
pub prevout: Option<GetBlockVerboseThreePrevout>,
}

pub struct ReceiveActivity {
// Note: 'type' field is used for deserialization tag, not included here explicitly.
/// The total amount in BTC of the new output.
pub amount: f64,
/// The block that this receive is in.
#[serde(default, skip_serializing_if = "Option::is_none")]
#[serde(rename = "blockhash")]
pub block_hash: Option<String>,
/// The height of the receive.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub height: Option<i64>,
/// The txid of the receiving transaction.
pub txid: String,
/// The vout of the receiving output.
pub vout: u32,
/// The ScriptPubKey.
pub output_spk: ScriptPubKey,
}

pub struct RescanBlockchain {
/// The block height where the rescan has started.
pub start_height: i64,
/// The height of the last rescanned block.
pub stop_height: i64,
}

pub struct RestoreWallet {
/// The wallet name if restored successfully.
pub name: String,
/// Warning message if wallet was not loaded cleanly.
pub warning: Option<String>,
}

pub struct SaveMempool {
/// The directory and file where the mempool was saved.
pub filename: String,
}

pub struct ScanBlocksAbort(pub bool);
/// Result of JSON-RPC method `scanblocks` with action "start".
///
/// > scanblocks "start" [scanobjects,...] ( start_height stop_height "filtertype" "options" )
/// >
/// > Arguments:
/// > 1. scanobjects                            (json array, required) Array of scan objects
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]

pub struct ScanBlocksStart {
/// The height we started the scan from
pub from_height: i64,
/// The height we ended the scan at
pub to_height: i64,
/// Blocks that may have matched a scanobject
pub relevant_blocks: Vec<String>,
/// True if the scan process was not aborted
pub completed: bool,
}

pub struct ScanBlocksStatus {
/// Approximate percent complete
pub progress: f64,
/// Height of the block currently being scanned
pub current_height: i64,
}

pub struct ScanTxOutSetAbort(pub bool);
/// Result of JSON-RPC method `scantxoutset` when action is `status`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct ScanTxOutSetStart {
/// Whether the scan was completed.
pub success: bool,
/// The number of unspent transaction outputs scanned.
#[serde(rename = "txouts")]
pub tx_outs: u64,
/// The block height at which the scan was done.
pub height: u64,
/// The hash of the block at the tip of the chain.
#[serde(rename = "bestblock")]
pub best_block: String,
/// The unspents.
pub unspents: Vec<ScanTxOutSetUnspent>,
/// The total amount of all found unspent outputs in BTC.
pub total_amount: f64,
}

pub struct ScanTxOutSetStatus {
/// Approximate percent complete.
pub progress: f64,
}

pub struct ScanTxOutSetUnspent {
/// The transaction id.
pub txid: String,
/// The vout value.
pub vout: u32,
/// The output script.
#[serde(rename = "scriptPubKey")]
pub script_pubkey: String,
/// A specialized descriptor for the matched output script.
#[serde(rename = "desc")]
pub descriptor: String,
/// The total amount in BTC of the unspent output.
pub amount: f64,
/// Whether this is a coinbase output.
pub coinbase: bool,
/// Height of the unspent transaction output.
pub height: u64,
/// Blockhash of the unspent transaction output.
#[serde(rename = "blockhash")]
pub block_hash: String,
/// Number of confirmations of the unspent transaction output when the scan was done.
pub confirmations: u64,
}

pub struct Send {
/// If the transaction has a complete set of signatures.
pub complete: bool,
/// The transaction id for the send.
pub txid: Option<String>,
/// If add_to_wallet is false, the hex-encoded raw transaction with signature(s).
pub hex: Option<String>,
/// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction.
pub psbt: Option<String>,
}

pub struct SendAll {
/// If the transaction has a complete set of signatures.
pub complete: bool,
/// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
pub txid: Option<String>,
/// If add_to_wallet is false, the hex-encoded raw transaction with signature(s).
pub hex: Option<String>,
/// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction.
pub psbt: Option<String>,
}

pub struct SendMany(
/// The transaction id for the send.
pub String,
);

pub struct SendManyVerbose {
/// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
pub txid: String,
/// The transaction fee reason.
pub fee_reason: String,
}

pub struct SendRawTransaction(
/// The transaction hash in hex.
pub String,
);

pub struct SendToAddress(pub String);
/// Result of JSON-RPC method `settxfee`.
///
/// > settxfee
/// >
/// > Arguments:
/// > 1. amount         (numeric or string, required) The transaction fee in BTC/kB
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]

pub struct SetNetworkActive(pub bool);
}

pub struct SetTxFee(pub bool);
/// Result of the JSON-RPC method `signmessage`.
///
/// > signmessage "address" "message"
/// >
/// > Sign a message with the private key of an address
/// >
/// > Arguments:
/// > 1. "address"         (string, required) The bitcoin address to use for the private key.
/// > 2. "message"         (string, required) The message to create a signature of.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct SetWalletFlag {
/// The name of the flag that was modified.
pub flag_name: String,
/// The new state of the flag.
pub flag_state: bool,
/// Any warnings associated with the change. (Always optional, but docs only state this from v24).
pub warnings: Option<String>,
}

pub struct SignFail {
/// The hash of the referenced, previous transaction.
pub txid: String,
/// The index of the output to spent and used as input.
pub vout: u64,
/// The hex-encoded signature script.
#[serde(rename = "scriptSig")]
pub script_sig: String,
/// Script sequence number.
pub sequence: u32,
/// Verification or signing error related to the input.
pub error: String,
}

pub struct SignMessage(
/// The signature of the message encoded in base 64.
pub String,
);

pub struct SignMessageWithPrivKey(pub String);
/// Result of JSON-RPC method `validateaddress`.
///
/// > validateaddress "address"
/// >
/// > Return information about the given bitcoin address.
/// > DEPRECATION WARNING: Parts of this command have been deprecated and moved to getaddressinfo. Clients must
/// > transition to using getaddressinfo to access this information before upgrading to v0.18. The following deprecated
/// > fields have moved to getaddressinfo and will only be shown here with -deprecatedrpc=validateaddress: ismine, iswatchonly,
/// > script, hex, pubkeys, sigsrequired, pubkey, addresses, embedded, iscompressed, account, timestamp, hdkeypath, kdmasterkeyid.
/// >
/// > Arguments:
/// > 1. "address"                    (string, required) The bitcoin address to validate
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct SignRawTransaction {
/// The hex-encoded raw transaction with signature(s).
pub hex: String,
/// If the transaction has a complete set of signatures.
pub complete: bool,
/// Script verification errors (if there are any).
pub errors: Option<Vec<SignFail>>,
}

pub struct Signers {
/// Master key fingerprint.
pub fingerprint: String,
/// Device name.
pub name: String,
}

pub struct SimulateRawTransaction {
/// The wallet balance change (negative means decrease).
pub balance_change: f64,
}

pub struct Softfork {
/// The [`SoftforkType`]: one of "buried", "bip9".
#[serde(rename = "type")]
pub type_: SoftforkType,
/// The status of bip9 softforks (only for "bip9" type).
pub bip9: Option<Bip9SoftforkInfo>,
///  Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
pub height: Option<i64>,
/// `true` if the rules are enforced for the mempool and the next block.
pub active: bool,
}

pub struct SoftforkReject {
/// `true` if threshold reached.
pub status: bool,
}

pub struct SpendActivity {
// Note: 'type' field is used for deserialization tag, not included here explicitly.
/// The total amount in BTC of the spent output.
pub amount: f64,
/// The block hash.
#[serde(default, skip_serializing_if = "Option::is_none")]
#[serde(rename = "blockhash")]
pub block_hash: Option<String>,
/// Height of the spend.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub height: Option<i64>,
/// The txid of the spending transaction.
pub spend_txid: String,
/// The vout of the spend.
pub spend_vout: u32,
/// The txid of the prevout.
pub prevout_txid: String,
/// The vout of the spend.
pub prevout_vout: u32,
/// The prev scriptPubKey.
pub prevout_spk: ScriptPubKey,
}

pub struct SubmitPackage {
/// The transaction package result message.
///
/// "success" indicates all transactions were accepted into or are already in the mempool.
pub package_msg: String,
/// Transaction results keyed by wtxid.
#[serde(rename = "tx-results")]
pub tx_results: BTreeMap<String, SubmitPackageTxResult>,
/// List of txids of replaced transactions.
#[serde(rename = "replaced-transactions")]
pub replaced_transactions: Vec<String>,
}

pub struct SubmitPackageTxResult {
/// The transaction id.
pub txid: String,
/// The wtxid of a different transaction with the same txid but different witness found in the mempool.
///
/// If set, this means the submitted transaction was ignored.
#[serde(rename = "other-wtxid")]
pub other_wtxid: Option<String>,
/// Sigops-adjusted virtual transaction size.
pub vsize: Option<i64>,
/// Transaction fees.
pub fees: Option<SubmitPackageTxResultFees>,
/// The transaction error string, if it was rejected by the mempool.
pub error: Option<String>,
}

pub struct SubmitPackageTxResultFees {
/// Transaction fee.
#[serde(rename = "base")]
pub base_fee: f64,
/// The effective feerate.
///
/// Will be `None` if the transaction was already in the mempool. For example, the package
/// feerate and/or feerate with modified fees from the `prioritisetransaction` JSON-RPC method.
#[serde(rename = "effective-feerate")]
pub effective_fee_rate: Option<f64>,
/// If [`Self::effective_fee_rate`] is provided, this holds the wtxid's of the transactions
/// whose fees and vsizes are included in effective-feerate.
#[serde(rename = "effective-includes")]
pub effective_includes: Option<Vec<String>>,
}

pub struct TaprootBip32Deriv {
/// The x-only public key this path corresponds to.
pub pubkey: String,
/// The fingerprint of the master key.
pub master_fingerprint: String,
/// The path.
pub path: String,
/// The hashes of the leaves this pubkey appears in.
pub leaf_hashes: Vec<String>,
}

pub struct TaprootLeaf {
/// The depth of this element in the tree.
pub depth: u32,
/// The version of this leaf.
#[serde(rename = "leaf_ver")]
pub leaf_version: u32,
/// The hex-encoded script itself.
pub script: String,
}

pub struct TaprootScript {
/// A leaf script.
pub script: String,
/// The version number for the leaf script.
#[serde(rename = "leaf_ver")]
pub leaf_version: u32,
/// The control blocks for this script.
pub control_blocks: Vec<String>,
}

pub struct TaprootScriptPathSig {
/// The x-only pubkey for this signature.
pub pubkey: String,
/// The leaf hash for this signature.
pub leaf_hash: String,
/// The signature itself.
pub sig: String,
}

pub struct TestMempoolAccept(pub Vec<MempoolAcceptance>);
/// A single mempool acceptance test result. Part of `testmempoolaccept`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct TransactionItem {
/// Only returns true if imported addresses were involved in transaction.
#[serde(rename = "involvesWatchonly")]
pub involves_watch_only: Option<bool>,
/// The bitcoin address of the transaction.
pub address: Option<String>,
/// The transaction category.
pub category: super::TransactionCategory,
/// The amount in BTC.
///
/// This is negative for the 'send' category, and is positive for all other categories.
pub amount: f64,
/// The vout value.
pub vout: i64,
/// The amount of the fee in BTC.
///
/// This is negative and only available for the 'send' category of transactions.
pub fee: Option<f64>,
/// The number of confirmations for the transaction. Negative confirmations means the
/// transaction conflicted that many blocks ago.
pub confirmations: i64,
/// Only present if transaction only input is a coinbase one.
pub generated: Option<bool>,
/// Only present if we consider transaction to be trusted and so safe to spend from.
pub trusted: Option<bool>,
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
pub block_time: Option<u32>,
/// The transaction id.
pub txid: String,
/// The hash of serialized transaction, including witness data.
pub wtxid: String,
/// Conflicting transaction ids.
#[serde(rename = "walletconflicts")]
pub wallet_conflicts: Vec<String>,
/// The txid if this tx was replaced.
pub replaced_by_txid: Option<String>,
/// The txid if this tx replaces one.
pub replaces_txid: Option<String>,
/// If a comment is associated with the transaction, only present if not empty.
pub comment: Option<String>,
/// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction.
#[serde(rename = "mempoolconflicts")]
pub mempool_conflicts: Option<Vec<String>>,
/// If a comment to is associated with the transaction.
pub to: Option<String>,
/// The transaction time expressed in UNIX epoch time.
pub time: u32,
/// The time received expressed in UNIX epoch time.
#[serde(rename = "timereceived")]
pub time_received: u32,
/// ("yes|no|unknown") Whether this transaction could be replaced due to BIP125 (replace-by-fee);

pub struct UnloadWallet {
/// Warning messages, if any, related to loading the wallet.
pub warnings: Option<Vec<String>>,
}

pub struct UpgradeWallet {
/// Name of wallet this operation was performed on
pub wallet_name: String,
/// Version of wallet before this operation
pub previous_version: u32,
/// Version of wallet after this operation
pub current_version: u32,
/// Description of result, if no error
pub result: Option<String>,
/// Error message (if there is one)
pub error: Option<String>,
}

pub struct UploadTarget {
/// Length of the measuring timeframe in seconds.
pub timeframe: u64,
/// Target in bytes.
pub target: u64,
/// True if target is reached.
pub target_reached: bool,
/// True if serving historical blocks.
pub serve_historical_blocks: bool,
/// Bytes left in current time cycle.
pub bytes_left_in_cycle: u64,
/// Seconds left in current time cycle.
pub time_left_in_cycle: u64,
}

pub struct UtxoUpdatePsbt(
/// The base64-encoded partially signed transaction with inputs updated.
pub String,
);

pub struct ValidateAddress {
/// If the address is valid or not. If not, this is the only property returned.
#[serde(rename = "isvalid")]
pub is_valid: bool,
/// The bitcoin address validated.
pub address: String,
/// The hex encoded scriptPubKey generated by the address.
#[serde(rename = "scriptPubKey")]
pub script_pubkey: String,
/// If the key is a script.
#[serde(rename = "isscript")]
pub is_script: bool,
/// If the address is a witness address.
#[serde(rename = "iswitness")]
pub is_witness: bool,
/// The version number of the witness program.
pub witness_version: Option<i64>,
/// The hex value of the witness program.
pub witness_program: Option<String>,
}

pub struct VerifyChain(pub bool);
/// Result of JSON-RPC method `verifytxoutproof`.
///
/// > verifytxoutproof "proof"
/// >
/// > Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// > and throwing an RPC error if the block is not in our best chain
/// >
/// > Arguments:
/// > 1. "proof"    (string, required) The hex-encoded proof generated by gettxoutproof
///
/// Inner field is the txid(s) which the proof commits to, or empty array if the proof can not be validated.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]

pub struct VerifyMessage(pub bool);
}

pub struct VerifyTxOutProof(pub Vec<String>);
}

pub struct WaitForBlock {
/// The blockhash.
pub hash: String,
/// Block height.
pub height: i64,
}

pub struct WaitForBlockHeight {
/// The blockhash.
pub hash: String,
/// Block height.
pub height: i64,
}

pub struct WaitForNewBlock {
/// The blockhash.
pub hash: String,
/// Block height.
pub height: i64,
}

pub struct WalletCreateFundedPsbt {
/// The resulting raw transaction (base64-encoded string).
pub psbt: String,
/// Fee in BTC the resulting transaction pays.
pub fee: f64,
/// The position of the added change output, or -1.
#[serde(rename = "changepos")]
pub change_position: i64,
}

pub struct WalletDisplayAddress {
/// The address as confirmed by the signer
pub address: String,
}

pub struct WalletProcessPsbt {
/// The base64-encoded partially signed transaction.
pub psbt: String,
/// If the transaction has a complete set of signatures.
pub complete: bool,
/// The hex-encoded network transaction if complete.
pub hex: Option<String>,
}

