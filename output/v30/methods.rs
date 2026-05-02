// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method implementations for Bitcoin Core `30`.
//!
//! Produced by `rust-btc-codegen`. **Do not edit by hand** — re-run
//! `just codegen` to regenerate. Hand-written model wrappers live in
//! `client_async/model/`; this module is the raw, version-specific surface.

#![allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde_json::json;

use super::options::*;
use super::types::*;
use crate::client_async::error::Result;
use crate::client_async::raw::Raw;

impl<'a> Raw<'a> {

    // ---------- blockchain ----------
    /// `dumptxoutset` — required arguments only.
    ///
    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    ///
    /// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
    /// 
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn dump_tx_out_set(&self, path: String) -> Result<DumpTxOutSet> {
        self.client.call_raw("dumptxoutset", &[json!(path)]).await
    }

    /// `dumptxoutset` — with all optional arguments via [`DumpTxOutSetOptions`].
    ///
    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    pub async fn dump_tx_out_set_with(&self, path: String, opts: DumpTxOutSetOptions) -> Result<DumpTxOutSet> {
        self.client.call_raw("dumptxoutset", &[json!(path), json!(opts.type_), json!(opts.options)]).await
    }

    /// `getbestblockhash` — required arguments only.
    ///
    /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    pub async fn get_best_block_hash(&self) -> Result<GetBestBlockHash> {
        self.client.call_raw("getbestblockhash", &[(); 0] as &[()]).await
    }

    /// `getblock` — required arguments only.
    ///
    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    ///
    /// If verbosity is 1, returns an Object with information about block \<hash\>.
    /// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    pub async fn get_block(&self, block_hash: String) -> Result<serde_json::Value> {
        self.client.call_raw("getblock", &[json!(block_hash)]).await
    }

    /// `getblock` — with all optional arguments via [`GetBlockOptions`].
    ///
    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    pub async fn get_block_with(&self, block_hash: String, opts: GetBlockOptions) -> Result<serde_json::Value> {
        self.client.call_raw("getblock", &[json!(block_hash), json!(opts.verbosity)]).await
    }

    /// `getblockchaininfo` — required arguments only.
    ///
    /// Returns an object containing various state info regarding blockchain processing.
    pub async fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
        self.client.call_raw("getblockchaininfo", &[(); 0] as &[()]).await
    }

    /// `getblockcount` — required arguments only.
    ///
    /// Returns the height of the most-work fully-validated chain.
    ///
    /// The genesis block has height 0.
    pub async fn get_block_count(&self) -> Result<GetBlockCount> {
        self.client.call_raw("getblockcount", &[(); 0] as &[()]).await
    }

    /// `getblockfilter` — required arguments only.
    ///
    /// Retrieve a BIP 157 content filter for a particular block.
    pub async fn get_block_filter(&self, block_hash: String) -> Result<GetBlockFilter> {
        self.client.call_raw("getblockfilter", &[json!(block_hash)]).await
    }

    /// `getblockfilter` — with all optional arguments via [`GetBlockFilterOptions`].
    ///
    /// Retrieve a BIP 157 content filter for a particular block.
    pub async fn get_block_filter_with(&self, block_hash: String, opts: GetBlockFilterOptions) -> Result<GetBlockFilter> {
        self.client.call_raw("getblockfilter", &[json!(block_hash), json!(opts.filter_type)]).await
    }

    /// `getblockfrompeer` — required arguments only.
    ///
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
    pub async fn get_block_from_peer(&self, block_hash: String, peer_id: f64) -> Result<GetBlockFromPeer> {
        self.client.call_raw("getblockfrompeer", &[json!(block_hash), json!(peer_id)]).await
    }

    /// `getblockhash` — required arguments only.
    ///
    /// Returns hash of block in best-block-chain at height provided.
    pub async fn get_block_hash(&self, height: i64) -> Result<GetBlockHash> {
        self.client.call_raw("getblockhash", &[json!(height)]).await
    }

    /// `getblockheader` — required arguments only.
    ///
    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
    ///
    /// If verbose is true, returns an Object with information about blockheader \<hash\>.
    pub async fn get_block_header(&self, block_hash: String) -> Result<serde_json::Value> {
        self.client.call_raw("getblockheader", &[json!(block_hash)]).await
    }

    /// `getblockheader` — with all optional arguments via [`GetBlockHeaderOptions`].
    ///
    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
    pub async fn get_block_header_with(&self, block_hash: String, opts: GetBlockHeaderOptions) -> Result<serde_json::Value> {
        self.client.call_raw("getblockheader", &[json!(block_hash), json!(opts.verbose)]).await
    }

    /// `getblockstats` — required arguments only.
    ///
    /// Compute per block statistics for a given window. All amounts are in satoshis.
    ///
    /// It won't work for some heights with pruning.
    pub async fn get_block_stats(&self, hash_or_height: f64) -> Result<GetBlockStats> {
        self.client.call_raw("getblockstats", &[json!(hash_or_height)]).await
    }

    /// `getblockstats` — with all optional arguments via [`GetBlockStatsOptions`].
    ///
    /// Compute per block statistics for a given window. All amounts are in satoshis.
    pub async fn get_block_stats_with(&self, hash_or_height: f64, opts: GetBlockStatsOptions) -> Result<GetBlockStats> {
        self.client.call_raw("getblockstats", &[json!(hash_or_height), json!(opts.stats)]).await
    }

    /// `getchainstates` — required arguments only.
    ///
    /// Return information about chainstates.
    pub async fn get_chain_states(&self) -> Result<GetChainStates> {
        self.client.call_raw("getchainstates", &[(); 0] as &[()]).await
    }

    /// `getchaintips` — required arguments only.
    ///
    /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    pub async fn get_chain_tips(&self) -> Result<GetChainTips> {
        self.client.call_raw("getchaintips", &[(); 0] as &[()]).await
    }

    /// `getchaintxstats` — required arguments only.
    ///
    /// Compute statistics about the total number and rate of transactions in the chain.
    pub async fn get_chain_tx_stats(&self) -> Result<GetChainTxStats> {
        self.client.call_raw("getchaintxstats", &[(); 0] as &[()]).await
    }

    /// `getchaintxstats` — with all optional arguments via [`GetChainTxStatsOptions`].
    ///
    /// Compute statistics about the total number and rate of transactions in the chain.
    pub async fn get_chain_tx_stats_with(&self, opts: GetChainTxStatsOptions) -> Result<GetChainTxStats> {
        self.client.call_raw("getchaintxstats", &[json!(opts.n_block_s), json!(opts.block_hash)]).await
    }

    /// `getdeploymentinfo` — required arguments only.
    ///
    /// Returns an object containing various state info regarding deployments of consensus changes.
    pub async fn get_deployment_info(&self) -> Result<GetDeploymentInfo> {
        self.client.call_raw("getdeploymentinfo", &[(); 0] as &[()]).await
    }

    /// `getdeploymentinfo` — with all optional arguments via [`GetDeploymentInfoOptions`].
    ///
    /// Returns an object containing various state info regarding deployments of consensus changes.
    pub async fn get_deployment_info_with(&self, opts: GetDeploymentInfoOptions) -> Result<GetDeploymentInfo> {
        self.client.call_raw("getdeploymentinfo", &[json!(opts.block_hash)]).await
    }

    /// `getdescriptoractivity` — required arguments only.
    ///
    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
    ///
    /// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn get_descriptor_activity(&self, block_hash_es: Vec<String>, scan_objects: Vec<String>) -> Result<GetDescriptorActivity> {
        self.client.call_raw("getdescriptoractivity", &[json!(block_hash_es), json!(scan_objects)]).await
    }

    /// `getdescriptoractivity` — with all optional arguments via [`GetDescriptorActivityOptions`].
    ///
    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
    pub async fn get_descriptor_activity_with(&self, block_hash_es: Vec<String>, scan_objects: Vec<String>, opts: GetDescriptorActivityOptions) -> Result<GetDescriptorActivity> {
        self.client.call_raw("getdescriptoractivity", &[json!(block_hash_es), json!(scan_objects), json!(opts.include_mempool)]).await
    }

    /// `getdifficulty` — required arguments only.
    ///
    /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    pub async fn get_difficulty(&self) -> Result<GetDifficulty> {
        self.client.call_raw("getdifficulty", &[(); 0] as &[()]).await
    }

    /// `getmempoolancestors` — required arguments only.
    ///
    /// If txid is in the mempool, returns all in-mempool ancestors.
    pub async fn get_mempool_ancestors(&self, txid: String) -> Result<serde_json::Value> {
        self.client.call_raw("getmempoolancestors", &[json!(txid)]).await
    }

    /// `getmempoolancestors` — with all optional arguments via [`GetMempoolAncestorsOptions`].
    ///
    /// If txid is in the mempool, returns all in-mempool ancestors.
    pub async fn get_mempool_ancestors_with(&self, txid: String, opts: GetMempoolAncestorsOptions) -> Result<serde_json::Value> {
        self.client.call_raw("getmempoolancestors", &[json!(txid), json!(opts.verbose)]).await
    }

    /// `getmempoolcluster` — required arguments only.
    ///
    /// Returns mempool data for given cluster
    pub async fn get_mempool_cluster(&self, txid: String) -> Result<GetMempoolCluster> {
        self.client.call_raw("getmempoolcluster", &[json!(txid)]).await
    }

    /// `getmempooldescendants` — required arguments only.
    ///
    /// If txid is in the mempool, returns all in-mempool descendants.
    pub async fn get_mempool_descendants(&self, txid: String) -> Result<serde_json::Value> {
        self.client.call_raw("getmempooldescendants", &[json!(txid)]).await
    }

    /// `getmempooldescendants` — with all optional arguments via [`GetMempoolDescendantsOptions`].
    ///
    /// If txid is in the mempool, returns all in-mempool descendants.
    pub async fn get_mempool_descendants_with(&self, txid: String, opts: GetMempoolDescendantsOptions) -> Result<serde_json::Value> {
        self.client.call_raw("getmempooldescendants", &[json!(txid), json!(opts.verbose)]).await
    }

    /// `getmempoolentry` — required arguments only.
    ///
    /// Returns mempool data for given transaction
    pub async fn get_mempool_entry(&self, txid: String) -> Result<GetMempoolEntry> {
        self.client.call_raw("getmempoolentry", &[json!(txid)]).await
    }

    /// `getmempoolinfo` — required arguments only.
    ///
    /// Returns details on the active state of the TX memory pool.
    pub async fn get_mempool_info(&self) -> Result<GetMempoolInfo> {
        self.client.call_raw("getmempoolinfo", &[(); 0] as &[()]).await
    }

    /// `getrawmempool` — required arguments only.
    ///
    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    ///
    /// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    pub async fn get_raw_mempool(&self) -> Result<serde_json::Value> {
        self.client.call_raw("getrawmempool", &[(); 0] as &[()]).await
    }

    /// `getrawmempool` — with all optional arguments via [`GetRawMempoolOptions`].
    ///
    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    pub async fn get_raw_mempool_with(&self, opts: GetRawMempoolOptions) -> Result<serde_json::Value> {
        self.client.call_raw("getrawmempool", &[json!(opts.verbose), json!(opts.mempool_sequence)]).await
    }

    /// `gettxout` — required arguments only.
    ///
    /// Returns details about an unspent transaction output.
    pub async fn get_tx_out(&self, txid: String, n: i64) -> Result<serde_json::Value> {
        self.client.call_raw("gettxout", &[json!(txid), json!(n)]).await
    }

    /// `gettxout` — with all optional arguments via [`GetTxOutOptions`].
    ///
    /// Returns details about an unspent transaction output.
    pub async fn get_tx_out_with(&self, txid: String, n: i64, opts: GetTxOutOptions) -> Result<serde_json::Value> {
        self.client.call_raw("gettxout", &[json!(txid), json!(n), json!(opts.include_mempool)]).await
    }

    /// `gettxoutproof` — required arguments only.
    ///
    /// Returns a hex-encoded proof that "txid" was included in a block.
    ///
    /// NOTE: By default this function only works sometimes. This is when there is an
    /// unspent output in the utxo for this transaction. To make it always work,
    /// you need to maintain a transaction index, using the -txindex command line option or
    /// specify the block in which the transaction is included manually (by blockhash).
    pub async fn get_tx_out_proof(&self, txids: Vec<String>) -> Result<GetTxOutProof> {
        self.client.call_raw("gettxoutproof", &[json!(txids)]).await
    }

    /// `gettxoutproof` — with all optional arguments via [`GetTxOutProofOptions`].
    ///
    /// Returns a hex-encoded proof that "txid" was included in a block.
    pub async fn get_tx_out_proof_with(&self, txids: Vec<String>, opts: GetTxOutProofOptions) -> Result<GetTxOutProof> {
        self.client.call_raw("gettxoutproof", &[json!(txids), json!(opts.block_hash)]).await
    }

    /// `gettxoutsetinfo` — required arguments only.
    ///
    /// Returns statistics about the unspent transaction output set.
    ///
    /// Note this call may take some time if you are not using coinstatsindex.
    pub async fn get_tx_out_set_info(&self) -> Result<GetTxOutSetInfo> {
        self.client.call_raw("gettxoutsetinfo", &[(); 0] as &[()]).await
    }

    /// `gettxoutsetinfo` — with all optional arguments via [`GetTxOutSetInfoOptions`].
    ///
    /// Returns statistics about the unspent transaction output set.
    pub async fn get_tx_out_set_info_with(&self, opts: GetTxOutSetInfoOptions) -> Result<GetTxOutSetInfo> {
        self.client.call_raw("gettxoutsetinfo", &[json!(opts.hash_type), json!(opts.hash_or_height), json!(opts.use_index)]).await
    }

    /// `gettxspendingprevout` — required arguments only.
    ///
    /// Scans the mempool to find transactions spending any of the given outputs
    pub async fn get_tx_spending_prevout(&self, outputs: Vec<serde_json::Value>) -> Result<GetTxSpendingPrevout> {
        self.client.call_raw("gettxspendingprevout", &[json!(outputs)]).await
    }

    /// `importmempool` — required arguments only.
    ///
    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    ///
    /// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    pub async fn import_mempool(&self, file_path: String) -> Result<ImportMempool> {
        self.client.call_raw("importmempool", &[json!(file_path)]).await
    }

    /// `importmempool` — with all optional arguments via [`ImportMempoolOptions`].
    ///
    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    pub async fn import_mempool_with(&self, file_path: String, opts: ImportMempoolOptions) -> Result<ImportMempool> {
        self.client.call_raw("importmempool", &[json!(file_path), json!(opts.options)]).await
    }

    /// `loadtxoutset` — required arguments only.
    ///
    /// Load the serialized UTXO set from a file.
    ///
    /// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
    /// 
    /// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
    /// 
    /// You can find more information on this process in the `assumeutxo` design document (\<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md\>).
    pub async fn load_tx_out_set(&self, path: String) -> Result<LoadTxOutSet> {
        self.client.call_raw("loadtxoutset", &[json!(path)]).await
    }

    /// `preciousblock` — required arguments only.
    ///
    /// Treats a block as if it were received before others with the same work.
    ///
    /// A later preciousblock call can override the effect of an earlier one.
    /// 
    /// The effects of preciousblock are not retained across restarts.
    pub async fn precious_block(&self, block_hash: String) -> Result<()> {
        self.client.call_raw("preciousblock", &[json!(block_hash)]).await
    }

    /// `pruneblockchain` — required arguments only.
    ///
    /// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
    ///
    /// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
    pub async fn prune_blockchain(&self, height: i64) -> Result<PruneBlockchain> {
        self.client.call_raw("pruneblockchain", &[json!(height)]).await
    }

    /// `savemempool` — required arguments only.
    ///
    /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    pub async fn save_mempool(&self) -> Result<SaveMempool> {
        self.client.call_raw("savemempool", &[(); 0] as &[()]).await
    }

    /// `scanblocks` — required arguments only.
    ///
    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    ///
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn scan_block_s(&self, action: String) -> Result<serde_json::Value> {
        self.client.call_raw("scanblocks", &[json!(action)]).await
    }

    /// `scanblocks` — with all optional arguments via [`ScanBlockSOptions`].
    ///
    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    pub async fn scan_block_s_with(&self, action: String, opts: ScanBlockSOptions) -> Result<serde_json::Value> {
        self.client.call_raw("scanblocks", &[json!(action), json!(opts.scan_objects), json!(opts.start_height), json!(opts.stop_height), json!(opts.filter_type), json!(opts.options)]).await
    }

    /// `scantxoutset` — required arguments only.
    ///
    /// Scans the unspent transaction output set for entries that match certain output descriptors.
    ///
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
    pub async fn scan_tx_out_set(&self, action: String) -> Result<serde_json::Value> {
        self.client.call_raw("scantxoutset", &[json!(action)]).await
    }

    /// `scantxoutset` — with all optional arguments via [`ScanTxOutSetOptions`].
    ///
    /// Scans the unspent transaction output set for entries that match certain output descriptors.
    pub async fn scan_tx_out_set_with(&self, action: String, opts: ScanTxOutSetOptions) -> Result<serde_json::Value> {
        self.client.call_raw("scantxoutset", &[json!(action), json!(opts.scan_objects)]).await
    }

    /// `verifychain` — required arguments only.
    ///
    /// Verifies blockchain database.
    pub async fn verify_chain(&self) -> Result<VerifyChain> {
        self.client.call_raw("verifychain", &[(); 0] as &[()]).await
    }

    /// `verifychain` — with all optional arguments via [`VerifyChainOptions`].
    ///
    /// Verifies blockchain database.
    pub async fn verify_chain_with(&self, opts: VerifyChainOptions) -> Result<VerifyChain> {
        self.client.call_raw("verifychain", &[json!(opts.checklevel), json!(opts.n_block_s)]).await
    }

    /// `verifytxoutproof` — required arguments only.
    ///
    /// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
    ///
    /// and throwing an RPC error if the block is not in our best chain
    pub async fn verify_tx_out_proof(&self, proof: String) -> Result<VerifyTxOutProof> {
        self.client.call_raw("verifytxoutproof", &[json!(proof)]).await
    }

    /// `waitforblock` — required arguments only.
    ///
    /// Waits for a specific new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    /// 
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_block(&self, block_hash: String) -> Result<WaitForBlock> {
        self.client.call_raw("waitforblock", &[json!(block_hash)]).await
    }

    /// `waitforblock` — with all optional arguments via [`WaitForBlockOptions`].
    ///
    /// Waits for a specific new block and returns useful info about it.
    pub async fn wait_for_block_with(&self, block_hash: String, opts: WaitForBlockOptions) -> Result<WaitForBlock> {
        self.client.call_raw("waitforblock", &[json!(block_hash), json!(opts.time_out)]).await
    }

    /// `waitforblockheight` — required arguments only.
    ///
    /// Waits for (at least) block height and returns the height and hash
    ///
    /// of the current tip.
    /// 
    /// Returns the current block on timeout or exit.
    /// 
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_block_height(&self, height: i64) -> Result<WaitForBlockHeight> {
        self.client.call_raw("waitforblockheight", &[json!(height)]).await
    }

    /// `waitforblockheight` — with all optional arguments via [`WaitForBlockHeightOptions`].
    ///
    /// Waits for (at least) block height and returns the height and hash
    pub async fn wait_for_block_height_with(&self, height: i64, opts: WaitForBlockHeightOptions) -> Result<WaitForBlockHeight> {
        self.client.call_raw("waitforblockheight", &[json!(height), json!(opts.time_out)]).await
    }

    /// `waitfornewblock` — required arguments only.
    ///
    /// Waits for any new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    /// 
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_new_block(&self) -> Result<WaitForNewBlock> {
        self.client.call_raw("waitfornewblock", &[(); 0] as &[()]).await
    }

    /// `waitfornewblock` — with all optional arguments via [`WaitForNewBlockOptions`].
    ///
    /// Waits for any new block and returns useful info about it.
    pub async fn wait_for_new_block_with(&self, opts: WaitForNewBlockOptions) -> Result<WaitForNewBlock> {
        self.client.call_raw("waitfornewblock", &[json!(opts.time_out), json!(opts.current_tip)]).await
    }


    // ---------- control ----------
    /// `getmemoryinfo` — required arguments only.
    ///
    /// Returns an object containing information about memory usage.
    pub async fn get_memory_info(&self) -> Result<serde_json::Value> {
        self.client.call_raw("getmemoryinfo", &[(); 0] as &[()]).await
    }

    /// `getmemoryinfo` — with all optional arguments via [`GetMemoryInfoOptions`].
    ///
    /// Returns an object containing information about memory usage.
    pub async fn get_memory_info_with(&self, opts: GetMemoryInfoOptions) -> Result<serde_json::Value> {
        self.client.call_raw("getmemoryinfo", &[json!(opts.mode)]).await
    }

    /// `getopenrpc` — required arguments only.
    ///
    /// Return an OpenRPC document describing the RPC API.
    pub async fn get_open_rpc(&self) -> Result<GetOpenRpc> {
        self.client.call_raw("getopenrpc", &[(); 0] as &[()]).await
    }

    /// `getrpcinfo` — required arguments only.
    ///
    /// Returns details of the RPC server.
    pub async fn get_rpc_info(&self) -> Result<GetRpcInfo> {
        self.client.call_raw("getrpcinfo", &[(); 0] as &[()]).await
    }

    /// `help` — required arguments only.
    ///
    /// List all commands, or get help for a specified command.
    pub async fn help(&self) -> Result<Help> {
        self.client.call_raw("help", &[(); 0] as &[()]).await
    }

    /// `help` — with all optional arguments via [`HelpOptions`].
    ///
    /// List all commands, or get help for a specified command.
    pub async fn help_with(&self, opts: HelpOptions) -> Result<Help> {
        self.client.call_raw("help", &[json!(opts.command)]).await
    }

    /// `logging` — required arguments only.
    ///
    /// Gets and sets the logging configuration.
    ///
    /// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
    /// When called with arguments, adds or removes categories from debug logging and return the lists above.
    /// The arguments are evaluated in order "include", "exclude".
    /// If an item is both included and excluded, it will thus end up being excluded.
    /// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, kernel, leveldb, libevent, mempool, mempoolrej, net, privatebroadcast, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
    /// In addition, the following are available as category names with special meanings:
    ///   - "all",  "1" : represent all logging categories.
    pub async fn logging(&self) -> Result<Logging> {
        self.client.call_raw("logging", &[(); 0] as &[()]).await
    }

    /// `logging` — with all optional arguments via [`LoggingOptions`].
    ///
    /// Gets and sets the logging configuration.
    pub async fn logging_with(&self, opts: LoggingOptions) -> Result<Logging> {
        self.client.call_raw("logging", &[json!(opts.include), json!(opts.exclude)]).await
    }

    /// `stop` — required arguments only.
    ///
    /// Request a graceful shutdown of Bitcoin Core.
    pub async fn stop(&self) -> Result<Stop> {
        self.client.call_raw("stop", &[(); 0] as &[()]).await
    }

    /// `stop` — with all optional arguments via [`StopOptions`].
    ///
    /// Request a graceful shutdown of Bitcoin Core.
    pub async fn stop_with(&self, opts: StopOptions) -> Result<Stop> {
        self.client.call_raw("stop", &[json!(opts.wait)]).await
    }

    /// `uptime` — required arguments only.
    ///
    /// Returns the total uptime of the server.
    pub async fn uptime(&self) -> Result<Uptime> {
        self.client.call_raw("uptime", &[(); 0] as &[()]).await
    }


    // ---------- hidden ----------
    /// `addconnection` — required arguments only.
    ///
    /// Open an outbound connection to a specified node. This RPC is for testing only.
    pub async fn add_connection(&self, address: String, connection_type: String, v2transport: bool) -> Result<AddConnection> {
        self.client.call_raw("addconnection", &[json!(address), json!(connection_type), json!(v2transport)]).await
    }

    /// `addpeeraddress` — required arguments only.
    ///
    /// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    pub async fn add_peer_address(&self, address: String, port: i64) -> Result<AddPeerAddress> {
        self.client.call_raw("addpeeraddress", &[json!(address), json!(port)]).await
    }

    /// `addpeeraddress` — with all optional arguments via [`AddPeerAddressOptions`].
    ///
    /// Add the address of a potential peer to an address manager table. This RPC is for testing only.
    pub async fn add_peer_address_with(&self, address: String, port: i64, opts: AddPeerAddressOptions) -> Result<AddPeerAddress> {
        self.client.call_raw("addpeeraddress", &[json!(address), json!(port), json!(opts.tried)]).await
    }

    /// `echo` — required arguments only.
    ///
    /// Simply echo back the input arguments. This command is for testing.
    ///
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    /// 
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    pub async fn echo(&self) -> Result<Echo> {
        self.client.call_raw("echo", &[(); 0] as &[()]).await
    }

    /// `echo` — with all optional arguments via [`EchoOptions`].
    ///
    /// Simply echo back the input arguments. This command is for testing.
    pub async fn echo_with(&self, opts: EchoOptions) -> Result<Echo> {
        self.client.call_raw("echo", &[json!(opts.arg0), json!(opts.arg1), json!(opts.arg2), json!(opts.arg3), json!(opts.arg4), json!(opts.arg5), json!(opts.arg6), json!(opts.arg7), json!(opts.arg8), json!(opts.arg9)]).await
    }

    /// `echoipc` — required arguments only.
    ///
    /// Echo back the input argument, passing it through a spawned process in a multiprocess build.
    ///
    /// This command is for testing.
    pub async fn echo_ipc(&self, arg: String) -> Result<EchoIpc> {
        self.client.call_raw("echoipc", &[json!(arg)]).await
    }

    /// `echojson` — required arguments only.
    ///
    /// Simply echo back the input arguments. This command is for testing.
    ///
    /// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
    /// 
    /// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
    pub async fn echo_json(&self) -> Result<EchoJson> {
        self.client.call_raw("echojson", &[(); 0] as &[()]).await
    }

    /// `echojson` — with all optional arguments via [`EchoJsonOptions`].
    ///
    /// Simply echo back the input arguments. This command is for testing.
    pub async fn echo_json_with(&self, opts: EchoJsonOptions) -> Result<EchoJson> {
        self.client.call_raw("echojson", &[json!(opts.arg0), json!(opts.arg1), json!(opts.arg2), json!(opts.arg3), json!(opts.arg4), json!(opts.arg5), json!(opts.arg6), json!(opts.arg7), json!(opts.arg8), json!(opts.arg9)]).await
    }

    /// `estimaterawfee` — required arguments only.
    ///
    /// WARNING: This interface is unstable and may disappear or change!
    ///
    /// WARNING: This is an advanced API call that is tightly coupled to the specific
    /// implementation of fee estimation. The parameters it can be called with
    /// and the results it returns will change if the internal implementation changes.
    /// 
    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    /// confirmation within conf_target blocks if possible. Uses virtual transaction size as
    /// defined in BIP 141 (witness data is discounted).
    pub async fn estimate_raw_fee(&self, conf_target: i64) -> Result<EstimateRawFee> {
        self.client.call_raw("estimaterawfee", &[json!(conf_target)]).await
    }

    /// `estimaterawfee` — with all optional arguments via [`EstimateRawFeeOptions`].
    ///
    /// WARNING: This interface is unstable and may disappear or change!
    pub async fn estimate_raw_fee_with(&self, conf_target: i64, opts: EstimateRawFeeOptions) -> Result<EstimateRawFee> {
        self.client.call_raw("estimaterawfee", &[json!(conf_target), json!(opts.threshold)]).await
    }

    /// `generate` — required arguments only.
    ///
    /// has been replaced by the -generate cli option. Refer to -help for more information.
    pub async fn generate(&self) -> Result<Generate> {
        self.client.call_raw("generate", &[(); 0] as &[()]).await
    }

    /// `generateblock` — required arguments only.
    ///
    /// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    pub async fn generate_block(&self, output: String, transactions: Vec<String>) -> Result<GenerateBlock> {
        self.client.call_raw("generateblock", &[json!(output), json!(transactions)]).await
    }

    /// `generateblock` — with all optional arguments via [`GenerateBlockOptions`].
    ///
    /// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
    pub async fn generate_block_with(&self, output: String, transactions: Vec<String>, opts: GenerateBlockOptions) -> Result<GenerateBlock> {
        self.client.call_raw("generateblock", &[json!(output), json!(transactions), json!(opts.submit)]).await
    }

    /// `generatetoaddress` — required arguments only.
    ///
    /// Mine to a specified address and return the block hashes.
    pub async fn generate_to_address(&self, n_block_s: i64, address: String) -> Result<GenerateToAddress> {
        self.client.call_raw("generatetoaddress", &[json!(n_block_s), json!(address)]).await
    }

    /// `generatetoaddress` — with all optional arguments via [`GenerateToAddressOptions`].
    ///
    /// Mine to a specified address and return the block hashes.
    pub async fn generate_to_address_with(&self, n_block_s: i64, address: String, opts: GenerateToAddressOptions) -> Result<GenerateToAddress> {
        self.client.call_raw("generatetoaddress", &[json!(n_block_s), json!(address), json!(opts.max_tries)]).await
    }

    /// `generatetodescriptor` — required arguments only.
    ///
    /// Mine to a specified descriptor and return the block hashes.
    pub async fn generate_to_descriptor(&self, num_blocks: i64, descriptor: String) -> Result<GenerateToDescriptor> {
        self.client.call_raw("generatetodescriptor", &[json!(num_blocks), json!(descriptor)]).await
    }

    /// `generatetodescriptor` — with all optional arguments via [`GenerateToDescriptorOptions`].
    ///
    /// Mine to a specified descriptor and return the block hashes.
    pub async fn generate_to_descriptor_with(&self, num_blocks: i64, descriptor: String, opts: GenerateToDescriptorOptions) -> Result<GenerateToDescriptor> {
        self.client.call_raw("generatetodescriptor", &[json!(num_blocks), json!(descriptor), json!(opts.max_tries)]).await
    }

    /// `getmempoolfeeratediagram` — required arguments only.
    ///
    /// Returns the feerate diagram for the whole mempool.
    pub async fn get_mempool_fee_rate_diagram(&self) -> Result<GetMempoolFeeRateDiagram> {
        self.client.call_raw("getmempoolfeeratediagram", &[(); 0] as &[()]).await
    }

    /// `getorphantxs` — required arguments only.
    ///
    /// Shows transactions in the tx orphanage.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    pub async fn get_orphan_tx_s(&self) -> Result<serde_json::Value> {
        self.client.call_raw("getorphantxs", &[(); 0] as &[()]).await
    }

    /// `getorphantxs` — with all optional arguments via [`GetOrphanTxSOptions`].
    ///
    /// Shows transactions in the tx orphanage.
    pub async fn get_orphan_tx_s_with(&self, opts: GetOrphanTxSOptions) -> Result<serde_json::Value> {
        self.client.call_raw("getorphantxs", &[json!(opts.verbosity)]).await
    }

    /// `getrawaddrman` — required arguments only.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Returns information on all address manager entries for the new and tried tables.
    pub async fn get_raw_addrman(&self) -> Result<GetRawAddrman> {
        self.client.call_raw("getrawaddrman", &[(); 0] as &[()]).await
    }

    /// `invalidateblock` — required arguments only.
    ///
    /// Permanently marks a block as invalid, as if it violated a consensus rule.
    pub async fn invalidate_block(&self, block_hash: String) -> Result<()> {
        self.client.call_raw("invalidateblock", &[json!(block_hash)]).await
    }

    /// `mockscheduler` — required arguments only.
    ///
    /// Bump the scheduler into the future (-regtest only)
    pub async fn mock_scheduler(&self, delta_time: f64) -> Result<()> {
        self.client.call_raw("mockscheduler", &[json!(delta_time)]).await
    }

    /// `reconsiderblock` — required arguments only.
    ///
    /// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
    ///
    /// This can be used to undo the effects of invalidateblock.
    pub async fn reconsider_block(&self, block_hash: String) -> Result<()> {
        self.client.call_raw("reconsiderblock", &[json!(block_hash)]).await
    }

    /// `sendmsgtopeer` — required arguments only.
    ///
    /// Send a p2p message to a peer specified by id.
    ///
    /// The message type and body must be provided, the message header will be generated.
    /// This RPC is for testing only.
    pub async fn send_msg_to_peer(&self, peer_id: f64, msg_type: String, msg: String) -> Result<SendMsgToPeer> {
        self.client.call_raw("sendmsgtopeer", &[json!(peer_id), json!(msg_type), json!(msg)]).await
    }

    /// `setmocktime` — required arguments only.
    ///
    /// Set the local time to given timestamp (-regtest only)
    pub async fn set_mock_time(&self, timestamp: f64) -> Result<()> {
        self.client.call_raw("setmocktime", &[json!(timestamp)]).await
    }

    /// `syncwithvalidationinterfacequeue` — required arguments only.
    ///
    /// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
    pub async fn sync_with_validation_interface_queue(&self) -> Result<()> {
        self.client.call_raw("syncwithvalidationinterfacequeue", &[(); 0] as &[()]).await
    }


    // ---------- mining ----------
    /// `getblocktemplate` — required arguments only.
    ///
    /// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
    ///
    /// It returns data needed to construct a block to work on.
    /// For full specification, see BIPs 22, 23, 9, and 145:
    ///     https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
    ///     https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
    ///     https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
    ///     https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
    pub async fn get_block_template(&self, template_request: serde_json::Value) -> Result<serde_json::Value> {
        self.client.call_raw("getblocktemplate", &[json!(template_request)]).await
    }

    /// `getmininginfo` — required arguments only.
    ///
    /// Returns a json object containing mining-related information.
    pub async fn get_mining_info(&self) -> Result<GetMiningInfo> {
        self.client.call_raw("getmininginfo", &[(); 0] as &[()]).await
    }

    /// `getnetworkhashps` — required arguments only.
    ///
    /// Returns the estimated network hashes per second based on the last n blocks.
    ///
    /// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
    /// Pass in [height] to estimate the network speed at the time when a certain block was found.
    pub async fn get_network_hashps(&self) -> Result<GetNetworkHashps> {
        self.client.call_raw("getnetworkhashps", &[(); 0] as &[()]).await
    }

    /// `getnetworkhashps` — with all optional arguments via [`GetNetworkHashpsOptions`].
    ///
    /// Returns the estimated network hashes per second based on the last n blocks.
    pub async fn get_network_hashps_with(&self, opts: GetNetworkHashpsOptions) -> Result<GetNetworkHashps> {
        self.client.call_raw("getnetworkhashps", &[json!(opts.n_block_s), json!(opts.height)]).await
    }

    /// `getprioritisedtransactions` — required arguments only.
    ///
    /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
    pub async fn get_prioritised_transactions(&self) -> Result<GetPrioritisedTransactions> {
        self.client.call_raw("getprioritisedtransactions", &[(); 0] as &[()]).await
    }

    /// `prioritisetransaction` — required arguments only.
    ///
    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    pub async fn prioritise_transaction(&self, txid: String, fee_delta: f64) -> Result<PrioritiseTransaction> {
        self.client.call_raw("prioritisetransaction", &[json!(txid), json!(fee_delta)]).await
    }

    /// `prioritisetransaction` — with all optional arguments via [`PrioritiseTransactionOptions`].
    ///
    /// Accepts the transaction into mined blocks at a higher (or lower) priority
    pub async fn prioritise_transaction_with(&self, txid: String, fee_delta: f64, opts: PrioritiseTransactionOptions) -> Result<PrioritiseTransaction> {
        self.client.call_raw("prioritisetransaction", &[json!(txid), json!(fee_delta), json!(opts.dummy)]).await
    }

    /// `submitblock` — required arguments only.
    ///
    /// Attempts to submit new block to network.
    ///
    /// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
    pub async fn submit_block(&self, hex_data: String) -> Result<serde_json::Value> {
        self.client.call_raw("submitblock", &[json!(hex_data)]).await
    }

    /// `submitblock` — with all optional arguments via [`SubmitBlockOptions`].
    ///
    /// Attempts to submit new block to network.
    pub async fn submit_block_with(&self, hex_data: String, opts: SubmitBlockOptions) -> Result<serde_json::Value> {
        self.client.call_raw("submitblock", &[json!(hex_data), json!(opts.dummy)]).await
    }

    /// `submitheader` — required arguments only.
    ///
    /// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
    ///
    /// Throws when the header is invalid.
    pub async fn submit_header(&self, hex_data: String) -> Result<()> {
        self.client.call_raw("submitheader", &[json!(hex_data)]).await
    }


    // ---------- network ----------
    /// `addnode` — required arguments only.
    ///
    /// Attempts to add or remove a node from the addnode list.
    ///
    /// Or try a connection to a node once.
    /// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
    /// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
    /// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    pub async fn add_node(&self, node: String, command: String) -> Result<()> {
        self.client.call_raw("addnode", &[json!(node), json!(command)]).await
    }

    /// `addnode` — with all optional arguments via [`AddNodeOptions`].
    ///
    /// Attempts to add or remove a node from the addnode list.
    pub async fn add_node_with(&self, node: String, command: String, opts: AddNodeOptions) -> Result<()> {
        self.client.call_raw("addnode", &[json!(node), json!(command), json!(opts.v2transport)]).await
    }

    /// `clearbanned` — required arguments only.
    ///
    /// Clear all banned IPs.
    pub async fn clear_banned(&self) -> Result<()> {
        self.client.call_raw("clearbanned", &[(); 0] as &[()]).await
    }

    /// `disconnectnode` — required arguments only.
    ///
    /// Immediately disconnects from the specified peer node.
    ///
    /// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
    /// 
    /// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
    pub async fn disconnect_node(&self) -> Result<()> {
        self.client.call_raw("disconnectnode", &[(); 0] as &[()]).await
    }

    /// `disconnectnode` — with all optional arguments via [`DisconnectNodeOptions`].
    ///
    /// Immediately disconnects from the specified peer node.
    pub async fn disconnect_node_with(&self, opts: DisconnectNodeOptions) -> Result<()> {
        self.client.call_raw("disconnectnode", &[json!(opts.address), json!(opts.node_id)]).await
    }

    /// `getaddednodeinfo` — required arguments only.
    ///
    /// Returns information about the given added node, or all added nodes
    ///
    /// (note that onetry addnodes are not listed here)
    pub async fn get_add_ed_node_info(&self) -> Result<GetAddEdNodeInfo> {
        self.client.call_raw("getaddednodeinfo", &[(); 0] as &[()]).await
    }

    /// `getaddednodeinfo` — with all optional arguments via [`GetAddEdNodeInfoOptions`].
    ///
    /// Returns information about the given added node, or all added nodes
    pub async fn get_add_ed_node_info_with(&self, opts: GetAddEdNodeInfoOptions) -> Result<GetAddEdNodeInfo> {
        self.client.call_raw("getaddednodeinfo", &[json!(opts.node)]).await
    }

    /// `getaddrmaninfo` — required arguments only.
    ///
    /// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
    pub async fn get_addrman_info(&self) -> Result<GetAddrmanInfo> {
        self.client.call_raw("getaddrmaninfo", &[(); 0] as &[()]).await
    }

    /// `getconnectioncount` — required arguments only.
    ///
    /// Returns the number of connections to other nodes.
    pub async fn get_connection_count(&self) -> Result<GetConnectionCount> {
        self.client.call_raw("getconnectioncount", &[(); 0] as &[()]).await
    }

    /// `getnettotals` — required arguments only.
    ///
    /// Returns information about network traffic, including bytes in, bytes out,
    ///
    /// and current system time.
    pub async fn get_net_totals(&self) -> Result<GetNetTotals> {
        self.client.call_raw("getnettotals", &[(); 0] as &[()]).await
    }

    /// `getnetworkinfo` — required arguments only.
    ///
    /// Returns an object containing various state info regarding P2P networking.
    pub async fn get_network_info(&self) -> Result<GetNetworkInfo> {
        self.client.call_raw("getnetworkinfo", &[(); 0] as &[()]).await
    }

    /// `getnodeaddresses` — required arguments only.
    ///
    /// Return known addresses, after filtering for quality and recency.
    ///
    /// These can potentially be used to find new peers in the network.
    /// The total number of addresses known to the node may be higher.
    pub async fn get_node_addresses(&self) -> Result<GetNodeAddresses> {
        self.client.call_raw("getnodeaddresses", &[(); 0] as &[()]).await
    }

    /// `getnodeaddresses` — with all optional arguments via [`GetNodeAddressesOptions`].
    ///
    /// Return known addresses, after filtering for quality and recency.
    pub async fn get_node_addresses_with(&self, opts: GetNodeAddressesOptions) -> Result<GetNodeAddresses> {
        self.client.call_raw("getnodeaddresses", &[json!(opts.count), json!(opts.network)]).await
    }

    /// `getpeerinfo` — required arguments only.
    ///
    /// Returns data about each connected network peer as a json array of objects.
    pub async fn get_peer_info(&self) -> Result<GetPeerInfo> {
        self.client.call_raw("getpeerinfo", &[(); 0] as &[()]).await
    }

    /// `listbanned` — required arguments only.
    ///
    /// List all manually banned IPs/Subnets.
    pub async fn list_banned(&self) -> Result<ListBanned> {
        self.client.call_raw("listbanned", &[(); 0] as &[()]).await
    }

    /// `ping` — required arguments only.
    ///
    /// Requests that a ping be sent to all other nodes, to measure ping time.
    ///
    /// Results are provided in getpeerinfo.
    /// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    pub async fn ping(&self) -> Result<()> {
        self.client.call_raw("ping", &[(); 0] as &[()]).await
    }

    /// `setban` — required arguments only.
    ///
    /// Attempts to add or remove an IP/Subnet from the banned list.
    pub async fn set_ban(&self, subnet: String, command: String) -> Result<()> {
        self.client.call_raw("setban", &[json!(subnet), json!(command)]).await
    }

    /// `setban` — with all optional arguments via [`SetBanOptions`].
    ///
    /// Attempts to add or remove an IP/Subnet from the banned list.
    pub async fn set_ban_with(&self, subnet: String, command: String, opts: SetBanOptions) -> Result<()> {
        self.client.call_raw("setban", &[json!(subnet), json!(command), json!(opts.ban_time), json!(opts.absolute)]).await
    }

    /// `setnetworkactive` — required arguments only.
    ///
    /// Disable/enable all p2p network activity.
    pub async fn set_network_active(&self, state: bool) -> Result<SetNetworkActive> {
        self.client.call_raw("setnetworkactive", &[json!(state)]).await
    }


    // ---------- rawtransactions ----------
    /// `analyzepsbt` — required arguments only.
    ///
    /// Analyzes and provides information about the current status of a PSBT and its inputs
    pub async fn analyze_psbt(&self, psbt: String) -> Result<AnalyzePsbt> {
        self.client.call_raw("analyzepsbt", &[json!(psbt)]).await
    }

    /// `combinepsbt` — required arguments only.
    ///
    /// Combine multiple partially signed Bitcoin transactions into one transaction.
    ///
    /// Implements the Combiner role.
    pub async fn combine_psbt(&self, tx_s: Vec<String>) -> Result<CombinePsbt> {
        self.client.call_raw("combinepsbt", &[json!(tx_s)]).await
    }

    /// `combinerawtransaction` — required arguments only.
    ///
    /// Combine multiple partially signed transactions into one transaction.
    ///
    /// The combined transaction may be another partially signed transaction or a 
    /// fully signed transaction.
    pub async fn combine_raw_transaction(&self, tx_s: Vec<String>) -> Result<CombineRawTransaction> {
        self.client.call_raw("combinerawtransaction", &[json!(tx_s)]).await
    }

    /// `converttopsbt` — required arguments only.
    ///
    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    ///
    /// createpsbt and walletcreatefundedpsbt should be used for new applications.
    pub async fn convert_to_psbt(&self, hex_string: String) -> Result<ConvertToPsbt> {
        self.client.call_raw("converttopsbt", &[json!(hex_string)]).await
    }

    /// `converttopsbt` — with all optional arguments via [`ConvertToPsbtOptions`].
    ///
    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    pub async fn convert_to_psbt_with(&self, hex_string: String, opts: ConvertToPsbtOptions) -> Result<ConvertToPsbt> {
        self.client.call_raw("converttopsbt", &[json!(hex_string), json!(opts.permit_sig_data), json!(opts.is_witness)]).await
    }

    /// `createpsbt` — required arguments only.
    ///
    /// Creates a transaction in the Partially Signed Transaction format.
    ///
    /// Implements the Creator role.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    pub async fn create_psbt(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>) -> Result<CreatePsbt> {
        self.client.call_raw("createpsbt", &[json!(inputs), json!(outputs)]).await
    }

    /// `createpsbt` — with all optional arguments via [`CreatePsbtOptions`].
    ///
    /// Creates a transaction in the Partially Signed Transaction format.
    pub async fn create_psbt_with(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, opts: CreatePsbtOptions) -> Result<CreatePsbt> {
        self.client.call_raw("createpsbt", &[json!(inputs), json!(outputs), json!(opts.locktime), json!(opts.replaceable), json!(opts.version)]).await
    }

    /// `createrawtransaction` — required arguments only.
    ///
    /// Create a transaction spending the given inputs and creating new outputs.
    ///
    /// Outputs can be addresses or data.
    /// Returns hex-encoded raw transaction.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    pub async fn create_raw_transaction(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>) -> Result<CreateRawTransaction> {
        self.client.call_raw("createrawtransaction", &[json!(inputs), json!(outputs)]).await
    }

    /// `createrawtransaction` — with all optional arguments via [`CreateRawTransactionOptions`].
    ///
    /// Create a transaction spending the given inputs and creating new outputs.
    pub async fn create_raw_transaction_with(&self, inputs: Vec<serde_json::Value>, outputs: Vec<serde_json::Value>, opts: CreateRawTransactionOptions) -> Result<CreateRawTransaction> {
        self.client.call_raw("createrawtransaction", &[json!(inputs), json!(outputs), json!(opts.locktime), json!(opts.replaceable), json!(opts.version)]).await
    }

    /// `decodepsbt` — required arguments only.
    ///
    /// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    pub async fn decode_psbt(&self, psbt: String) -> Result<DecodePsbt> {
        self.client.call_raw("decodepsbt", &[json!(psbt)]).await
    }

    /// `decoderawtransaction` — required arguments only.
    ///
    /// Return a JSON object representing the serialized, hex-encoded transaction.
    pub async fn decode_raw_transaction(&self, hex_string: String) -> Result<DecodeRawTransaction> {
        self.client.call_raw("decoderawtransaction", &[json!(hex_string)]).await
    }

    /// `decoderawtransaction` — with all optional arguments via [`DecodeRawTransactionOptions`].
    ///
    /// Return a JSON object representing the serialized, hex-encoded transaction.
    pub async fn decode_raw_transaction_with(&self, hex_string: String, opts: DecodeRawTransactionOptions) -> Result<DecodeRawTransaction> {
        self.client.call_raw("decoderawtransaction", &[json!(hex_string), json!(opts.is_witness)]).await
    }

    /// `decodescript` — required arguments only.
    ///
    /// Decode a hex-encoded script.
    pub async fn decode_script(&self, hex_string: String) -> Result<DecodeScript> {
        self.client.call_raw("decodescript", &[json!(hex_string)]).await
    }

    /// `descriptorprocesspsbt` — required arguments only.
    ///
    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
    ///
    ///  
    /// Then, sign the inputs we are able to with information from the output descriptors.
    pub async fn descriptor_process_psbt(&self, psbt: String, descriptors: Vec<String>) -> Result<DescriptorProcessPsbt> {
        self.client.call_raw("descriptorprocesspsbt", &[json!(psbt), json!(descriptors)]).await
    }

    /// `descriptorprocesspsbt` — with all optional arguments via [`DescriptorProcessPsbtOptions`].
    ///
    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
    pub async fn descriptor_process_psbt_with(&self, psbt: String, descriptors: Vec<String>, opts: DescriptorProcessPsbtOptions) -> Result<DescriptorProcessPsbt> {
        self.client.call_raw("descriptorprocesspsbt", &[json!(psbt), json!(descriptors), json!(opts.sig_hash_type), json!(opts.bip32derivs), json!(opts.finalize)]).await
    }

    /// `finalizepsbt` — required arguments only.
    ///
    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    ///
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
    pub async fn finalize_psbt(&self, psbt: String) -> Result<FinalizePsbt> {
        self.client.call_raw("finalizepsbt", &[json!(psbt)]).await
    }

    /// `finalizepsbt` — with all optional arguments via [`FinalizePsbtOptions`].
    ///
    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    pub async fn finalize_psbt_with(&self, psbt: String, opts: FinalizePsbtOptions) -> Result<FinalizePsbt> {
        self.client.call_raw("finalizepsbt", &[json!(psbt), json!(opts.extract)]).await
    }

    /// `fundrawtransaction` — required arguments only.
    ///
    /// If the transaction has no inputs, they will be automatically selected to meet its out value.
    ///
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
    pub async fn fund_raw_transaction(&self, hex_string: String) -> Result<FundRawTransaction> {
        self.client.call_raw("fundrawtransaction", &[json!(hex_string)]).await
    }

    /// `fundrawtransaction` — with all optional arguments via [`FundRawTransactionOptions`].
    ///
    /// If the transaction has no inputs, they will be automatically selected to meet its out value.
    pub async fn fund_raw_transaction_with(&self, hex_string: String, opts: FundRawTransactionOptions) -> Result<FundRawTransaction> {
        self.client.call_raw("fundrawtransaction", &[json!(hex_string), json!(opts.options), json!(opts.is_witness)]).await
    }

    /// `getrawtransaction` — required arguments only.
    ///
    /// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
    ///
    /// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
    /// If a blockhash argument is passed, it will return the transaction if
    /// the specified block is available and the transaction is in that block.
    /// 
    /// Hint: Use gettransaction for wallet transactions.
    /// 
    /// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
    /// If verbosity is 1, returns a JSON Object with information about the transaction.
    /// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
    pub async fn get_raw_transaction(&self, txid: String) -> Result<serde_json::Value> {
        self.client.call_raw("getrawtransaction", &[json!(txid)]).await
    }

    /// `getrawtransaction` — with all optional arguments via [`GetRawTransactionOptions`].
    ///
    /// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
    pub async fn get_raw_transaction_with(&self, txid: String, opts: GetRawTransactionOptions) -> Result<serde_json::Value> {
        self.client.call_raw("getrawtransaction", &[json!(txid), json!(opts.verbosity), json!(opts.block_hash)]).await
    }

    /// `joinpsbts` — required arguments only.
    ///
    /// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
    ///
    /// No input in any of the PSBTs can be in more than one of the PSBTs.
    pub async fn join_psbts(&self, tx_s: Vec<String>) -> Result<JoinPsbts> {
        self.client.call_raw("joinpsbts", &[json!(tx_s)]).await
    }

    /// `sendrawtransaction` — required arguments only.
    ///
    /// Submit a raw transaction (serialized, hex-encoded) to the network.
    ///
    /// If -privatebroadcast is disabled, then the transaction will be put into the
    /// local mempool of the node and will be sent unconditionally to all currently
    /// connected peers, so using sendrawtransaction for manual rebroadcast will degrade
    /// privacy by leaking the transaction's origin, as nodes will normally not
    /// rebroadcast non-wallet transactions already in their mempool.
    /// 
    /// If -privatebroadcast is enabled, then the transaction will be sent only via
    /// dedicated, short-lived connections to Tor or I2P peers or IPv4/IPv6 peers
    /// via the Tor network. This conceals the transaction's origin. The transaction
    /// will only enter the local mempool when it is received back from the network.
    /// 
    /// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
    /// 
    /// Related RPCs: createrawtransaction, signrawtransactionwithkey
    pub async fn send_raw_transaction(&self, hex_string: String) -> Result<SendRawTransaction> {
        self.client.call_raw("sendrawtransaction", &[json!(hex_string)]).await
    }

    /// `sendrawtransaction` — with all optional arguments via [`SendRawTransactionOptions`].
    ///
    /// Submit a raw transaction (serialized, hex-encoded) to the network.
    pub async fn send_raw_transaction_with(&self, hex_string: String, opts: SendRawTransactionOptions) -> Result<SendRawTransaction> {
        self.client.call_raw("sendrawtransaction", &[json!(hex_string), json!(opts.max_fee_rate), json!(opts.max_burn_amount)]).await
    }

    /// `signrawtransactionwithkey` — required arguments only.
    ///
    /// Sign inputs for raw transaction (serialized, hex-encoded).
    ///
    /// The second argument is an array of base58-encoded private
    /// keys that will be the only keys used to sign the transaction.
    /// The third optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    pub async fn sign_raw_transaction_with_key(&self, hex_string: String, priv_keys: Vec<String>) -> Result<SignRawTransactionWithKey> {
        self.client.call_raw("signrawtransactionwithkey", &[json!(hex_string), json!(priv_keys)]).await
    }

    /// `signrawtransactionwithkey` — with all optional arguments via [`SignRawTransactionWithKeyOptions`].
    ///
    /// Sign inputs for raw transaction (serialized, hex-encoded).
    pub async fn sign_raw_transaction_with_key_with(&self, hex_string: String, priv_keys: Vec<String>, opts: SignRawTransactionWithKeyOptions) -> Result<SignRawTransactionWithKey> {
        self.client.call_raw("signrawtransactionwithkey", &[json!(hex_string), json!(priv_keys), json!(opts.prev_tx_s), json!(opts.sig_hash_type)]).await
    }

    /// `submitpackage` — required arguments only.
    ///
    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    ///
    /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
    /// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
    /// Warning: successful submission does not mean the transactions will propagate throughout the network.
    pub async fn submit_package(&self, package: Vec<String>) -> Result<SubmitPackage> {
        self.client.call_raw("submitpackage", &[json!(package)]).await
    }

    /// `submitpackage` — with all optional arguments via [`SubmitPackageOptions`].
    ///
    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    pub async fn submit_package_with(&self, package: Vec<String>, opts: SubmitPackageOptions) -> Result<SubmitPackage> {
        self.client.call_raw("submitpackage", &[json!(package), json!(opts.max_fee_rate), json!(opts.max_burn_amount)]).await
    }

    /// `testmempoolaccept` — required arguments only.
    ///
    /// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
    ///
    /// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
    /// 
    /// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
    /// 
    /// The maximum number of transactions allowed is 25.
    /// 
    /// This checks if transactions violate the consensus or policy rules.
    /// 
    /// See sendrawtransaction call.
    pub async fn test_mempool_accept(&self, raw_tx_s: Vec<String>) -> Result<TestMempoolAccept> {
        self.client.call_raw("testmempoolaccept", &[json!(raw_tx_s)]).await
    }

    /// `testmempoolaccept` — with all optional arguments via [`TestMempoolAcceptOptions`].
    ///
    /// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
    pub async fn test_mempool_accept_with(&self, raw_tx_s: Vec<String>, opts: TestMempoolAcceptOptions) -> Result<TestMempoolAccept> {
        self.client.call_raw("testmempoolaccept", &[json!(raw_tx_s), json!(opts.max_fee_rate)]).await
    }

    /// `utxoupdatepsbt` — required arguments only.
    ///
    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    pub async fn utxo_update_psbt(&self, psbt: String) -> Result<UtxoUpdatePsbt> {
        self.client.call_raw("utxoupdatepsbt", &[json!(psbt)]).await
    }

    /// `utxoupdatepsbt` — with all optional arguments via [`UtxoUpdatePsbtOptions`].
    ///
    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    pub async fn utxo_update_psbt_with(&self, psbt: String, opts: UtxoUpdatePsbtOptions) -> Result<UtxoUpdatePsbt> {
        self.client.call_raw("utxoupdatepsbt", &[json!(psbt), json!(opts.descriptors)]).await
    }


    // ---------- signer ----------
    /// `enumeratesigners` — required arguments only.
    ///
    /// Returns a list of external signers from -signer.
    pub async fn enumerate_signers(&self) -> Result<EnumerateSigners> {
        self.client.call_raw("enumeratesigners", &[(); 0] as &[()]).await
    }


    // ---------- util ----------
    /// `createmultisig` — required arguments only.
    ///
    /// Creates a multi-signature address with n signatures of m keys required.
    ///
    /// It returns a json object with the address and redeemScript.
    pub async fn create_multisig(&self, n_required: f64, keys: Vec<String>) -> Result<CreateMultisig> {
        self.client.call_raw("createmultisig", &[json!(n_required), json!(keys)]).await
    }

    /// `createmultisig` — with all optional arguments via [`CreateMultisigOptions`].
    ///
    /// Creates a multi-signature address with n signatures of m keys required.
    pub async fn create_multisig_with(&self, n_required: f64, keys: Vec<String>, opts: CreateMultisigOptions) -> Result<CreateMultisig> {
        self.client.call_raw("createmultisig", &[json!(n_required), json!(keys), json!(opts.address_type)]).await
    }

    /// `deriveaddresses` — required arguments only.
    ///
    /// Derives one or more addresses corresponding to an output descriptor.
    ///
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
    pub async fn derive_addresses(&self, descriptor: String) -> Result<serde_json::Value> {
        self.client.call_raw("deriveaddresses", &[json!(descriptor)]).await
    }

    /// `deriveaddresses` — with all optional arguments via [`DeriveAddressesOptions`].
    ///
    /// Derives one or more addresses corresponding to an output descriptor.
    pub async fn derive_addresses_with(&self, descriptor: String, opts: DeriveAddressesOptions) -> Result<serde_json::Value> {
        self.client.call_raw("deriveaddresses", &[json!(descriptor), json!(opts.range)]).await
    }

    /// `estimatesmartfee` — required arguments only.
    ///
    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    ///
    /// confirmation within conf_target blocks if possible and return the number of blocks
    /// for which the estimate is valid. Uses virtual transaction size as defined
    /// in BIP 141 (witness data is discounted).
    pub async fn estimate_smart_fee(&self, conf_target: i64) -> Result<EstimateSmartFee> {
        self.client.call_raw("estimatesmartfee", &[json!(conf_target)]).await
    }

    /// `estimatesmartfee` — with all optional arguments via [`EstimateSmartFeeOptions`].
    ///
    /// Estimates the approximate fee per kilobyte needed for a transaction to begin
    pub async fn estimate_smart_fee_with(&self, conf_target: i64, opts: EstimateSmartFeeOptions) -> Result<EstimateSmartFee> {
        self.client.call_raw("estimatesmartfee", &[json!(conf_target), json!(opts.estimate_mode)]).await
    }

    /// `getdescriptorinfo` — required arguments only.
    ///
    /// Analyses a descriptor.
    pub async fn get_descriptor_info(&self, descriptor: String) -> Result<GetDescriptorInfo> {
        self.client.call_raw("getdescriptorinfo", &[json!(descriptor)]).await
    }

    /// `getindexinfo` — required arguments only.
    ///
    /// Returns the status of one or all available indices currently running in the node.
    pub async fn get_index_info(&self) -> Result<GetIndexInfo> {
        self.client.call_raw("getindexinfo", &[(); 0] as &[()]).await
    }

    /// `getindexinfo` — with all optional arguments via [`GetIndexInfoOptions`].
    ///
    /// Returns the status of one or all available indices currently running in the node.
    pub async fn get_index_info_with(&self, opts: GetIndexInfoOptions) -> Result<GetIndexInfo> {
        self.client.call_raw("getindexinfo", &[json!(opts.index_name)]).await
    }

    /// `signmessagewithprivkey` — required arguments only.
    ///
    /// Sign a message with the private key of an address
    pub async fn sign_message_with_priv_key(&self, priv_key: String, message: String) -> Result<SignMessageWithPrivKey> {
        self.client.call_raw("signmessagewithprivkey", &[json!(priv_key), json!(message)]).await
    }

    /// `validateaddress` — required arguments only.
    ///
    /// Return information about the given bitcoin address.
    pub async fn validate_address(&self, address: String) -> Result<ValidateAddress> {
        self.client.call_raw("validateaddress", &[json!(address)]).await
    }

    /// `verifymessage` — required arguments only.
    ///
    /// Verify a signed message.
    pub async fn verify_message(&self, address: String, signature: String, message: String) -> Result<VerifyMessage> {
        self.client.call_raw("verifymessage", &[json!(address), json!(signature), json!(message)]).await
    }


    // ---------- wallet ----------
    /// `abandontransaction` — required arguments only.
    ///
    /// Mark in-wallet transaction \<txid\> as abandoned
    ///
    /// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
    /// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
    /// It only works on transactions which are not included in a block and are not currently in the mempool.
    /// It has no effect on transactions which are already abandoned.
    pub async fn abandon_transaction(&self, txid: String) -> Result<()> {
        self.client.call_raw("abandontransaction", &[json!(txid)]).await
    }

    /// `abortrescan` — required arguments only.
    ///
    /// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
    ///
    /// Note: Use "getwalletinfo" to query the scanning progress.
    pub async fn abort_rescan(&self) -> Result<AbortRescan> {
        self.client.call_raw("abortrescan", &[(); 0] as &[()]).await
    }

    /// `backupwallet` — required arguments only.
    ///
    /// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    pub async fn backup_wallet(&self, destination: String) -> Result<()> {
        self.client.call_raw("backupwallet", &[json!(destination)]).await
    }

    /// `bumpfee` — required arguments only.
    ///
    /// Bumps the fee of a transaction T, replacing it with a new transaction B.
    ///
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
    pub async fn bump_fee(&self, txid: String) -> Result<BumpFee> {
        self.client.call_raw("bumpfee", &[json!(txid)]).await
    }

    /// `bumpfee` — with all optional arguments via [`BumpFeeOptions`].
    ///
    /// Bumps the fee of a transaction T, replacing it with a new transaction B.
    pub async fn bump_fee_with(&self, txid: String, opts: BumpFeeOptions) -> Result<BumpFee> {
        self.client.call_raw("bumpfee", &[json!(txid), json!(opts.options)]).await
    }

    /// `createwallet` — required arguments only.
    ///
    /// Creates and loads a new wallet.
    pub async fn create_wallet(&self, wallet_name: String) -> Result<CreateWallet> {
        self.client.call_raw("createwallet", &[json!(wallet_name)]).await
    }

    /// `createwallet` — with all optional arguments via [`CreateWalletOptions`].
    ///
    /// Creates and loads a new wallet.
    pub async fn create_wallet_with(&self, wallet_name: String, opts: CreateWalletOptions) -> Result<CreateWallet> {
        self.client.call_raw("createwallet", &[json!(wallet_name), json!(opts.disable_private_keys), json!(opts.blank), json!(opts.passphrase), json!(opts.avoid_reuse), json!(opts.descriptors), json!(opts.load_on_startup), json!(opts.external_signer)]).await
    }

    /// `createwalletdescriptor` — required arguments only.
    ///
    /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn create_wallet_descriptor(&self, type_: String) -> Result<CreateWalletDescriptor> {
        self.client.call_raw("createwalletdescriptor", &[json!(type_)]).await
    }

    /// `createwalletdescriptor` — with all optional arguments via [`CreateWalletDescriptorOptions`].
    ///
    /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    pub async fn create_wallet_descriptor_with(&self, type_: String, opts: CreateWalletDescriptorOptions) -> Result<CreateWalletDescriptor> {
        self.client.call_raw("createwalletdescriptor", &[json!(type_), json!(opts.options)]).await
    }

    /// `encryptwallet` — required arguments only.
    ///
    /// Encrypts the wallet with 'passphrase'. This is for first time encryption.
    ///
    /// After this, any calls that interact with private keys such as sending or signing 
    /// will require the passphrase to be set prior to making these calls.
    /// Use the walletpassphrase call for this, and then walletlock call.
    /// If the wallet is already encrypted, use the walletpassphrasechange call.
    /// ** IMPORTANT **
    /// For security reasons, the encryption process will generate a new HD seed, resulting
    /// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
    /// securely back up the newly generated wallet file using the backupwallet RPC.
    pub async fn encrypt_wallet(&self, passphrase: String) -> Result<EncryptWallet> {
        self.client.call_raw("encryptwallet", &[json!(passphrase)]).await
    }

    /// `getaddressesbylabel` — required arguments only.
    ///
    /// Returns the list of addresses assigned the specified label.
    pub async fn get_addresses_by_label(&self, label: String) -> Result<GetAddressesByLabel> {
        self.client.call_raw("getaddressesbylabel", &[json!(label)]).await
    }

    /// `getaddressinfo` — required arguments only.
    ///
    /// Return information about the given bitcoin address.
    ///
    /// Some of the information will only be present if the address is in the active wallet.
    pub async fn get_address_info(&self, address: String) -> Result<GetAddressInfo> {
        self.client.call_raw("getaddressinfo", &[json!(address)]).await
    }

    /// `getbalance` — required arguments only.
    ///
    /// Returns the total available balance.
    ///
    /// The available balance is what the wallet considers currently spendable, and is
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    pub async fn get_balance(&self) -> Result<GetBalance> {
        self.client.call_raw("getbalance", &[(); 0] as &[()]).await
    }

    /// `getbalance` — with all optional arguments via [`GetBalanceOptions`].
    ///
    /// Returns the total available balance.
    pub async fn get_balance_with(&self, opts: GetBalanceOptions) -> Result<GetBalance> {
        self.client.call_raw("getbalance", &[json!(opts.dummy), json!(opts.min_conf), json!(opts.include_watchonly), json!(opts.avoid_reuse)]).await
    }

    /// `getbalances` — required arguments only.
    ///
    /// Returns an object with all balances in BTC.
    pub async fn get_balances(&self) -> Result<GetBalances> {
        self.client.call_raw("getbalances", &[(); 0] as &[()]).await
    }

    /// `gethdkeys` — required arguments only.
    ///
    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    pub async fn get_hd_keys(&self) -> Result<GetHdKeys> {
        self.client.call_raw("gethdkeys", &[(); 0] as &[()]).await
    }

    /// `gethdkeys` — with all optional arguments via [`GetHdKeysOptions`].
    ///
    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    pub async fn get_hd_keys_with(&self, opts: GetHdKeysOptions) -> Result<GetHdKeys> {
        self.client.call_raw("gethdkeys", &[json!(opts.options)]).await
    }

    /// `getnewaddress` — required arguments only.
    ///
    /// Returns a new Bitcoin address for receiving payments.
    ///
    /// If 'label' is specified, it is added to the address book 
    /// so payments received with the address will be associated with 'label'.
    pub async fn get_new_address(&self) -> Result<GetNewAddress> {
        self.client.call_raw("getnewaddress", &[(); 0] as &[()]).await
    }

    /// `getnewaddress` — with all optional arguments via [`GetNewAddressOptions`].
    ///
    /// Returns a new Bitcoin address for receiving payments.
    pub async fn get_new_address_with(&self, opts: GetNewAddressOptions) -> Result<GetNewAddress> {
        self.client.call_raw("getnewaddress", &[json!(opts.label), json!(opts.address_type)]).await
    }

    /// `getrawchangeaddress` — required arguments only.
    ///
    /// Returns a new Bitcoin address, for receiving change.
    ///
    /// This is for use with raw transactions, NOT normal use.
    pub async fn get_raw_change_address(&self) -> Result<GetRawChangeAddress> {
        self.client.call_raw("getrawchangeaddress", &[(); 0] as &[()]).await
    }

    /// `getrawchangeaddress` — with all optional arguments via [`GetRawChangeAddressOptions`].
    ///
    /// Returns a new Bitcoin address, for receiving change.
    pub async fn get_raw_change_address_with(&self, opts: GetRawChangeAddressOptions) -> Result<GetRawChangeAddress> {
        self.client.call_raw("getrawchangeaddress", &[json!(opts.address_type)]).await
    }

    /// `getreceivedbyaddress` — required arguments only.
    ///
    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    pub async fn get_received_by_address(&self, address: String) -> Result<GetReceivedByAddress> {
        self.client.call_raw("getreceivedbyaddress", &[json!(address)]).await
    }

    /// `getreceivedbyaddress` — with all optional arguments via [`GetReceivedByAddressOptions`].
    ///
    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    pub async fn get_received_by_address_with(&self, address: String, opts: GetReceivedByAddressOptions) -> Result<GetReceivedByAddress> {
        self.client.call_raw("getreceivedbyaddress", &[json!(address), json!(opts.min_conf), json!(opts.include_immature_coinbase)]).await
    }

    /// `getreceivedbylabel` — required arguments only.
    ///
    /// Returns the total amount received by addresses with \<label\> in transactions with at least [minconf] confirmations.
    pub async fn get_received_by_label(&self, label: String) -> Result<GetReceivedByLabel> {
        self.client.call_raw("getreceivedbylabel", &[json!(label)]).await
    }

    /// `getreceivedbylabel` — with all optional arguments via [`GetReceivedByLabelOptions`].
    ///
    /// Returns the total amount received by addresses with \<label\> in transactions with at least [minconf] confirmations.
    pub async fn get_received_by_label_with(&self, label: String, opts: GetReceivedByLabelOptions) -> Result<GetReceivedByLabel> {
        self.client.call_raw("getreceivedbylabel", &[json!(label), json!(opts.min_conf), json!(opts.include_immature_coinbase)]).await
    }

    /// `gettransaction` — required arguments only.
    ///
    /// Get detailed information about in-wallet transaction \<txid\>
    pub async fn get_transaction(&self, txid: String) -> Result<GetTransaction> {
        self.client.call_raw("gettransaction", &[json!(txid)]).await
    }

    /// `gettransaction` — with all optional arguments via [`GetTransactionOptions`].
    ///
    /// Get detailed information about in-wallet transaction \<txid\>
    pub async fn get_transaction_with(&self, txid: String, opts: GetTransactionOptions) -> Result<GetTransaction> {
        self.client.call_raw("gettransaction", &[json!(txid), json!(opts.include_watchonly), json!(opts.verbose)]).await
    }

    /// `getwalletinfo` — required arguments only.
    ///
    /// Returns an object containing various wallet state info.
    pub async fn get_wallet_info(&self) -> Result<GetWalletInfo> {
        self.client.call_raw("getwalletinfo", &[(); 0] as &[()]).await
    }

    /// `importdescriptors` — required arguments only.
    ///
    /// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
    ///
    /// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
    /// 
    /// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
    /// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
    /// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    pub async fn import_descriptors(&self, requests: Vec<serde_json::Value>) -> Result<ImportDescriptors> {
        self.client.call_raw("importdescriptors", &[json!(requests)]).await
    }

    /// `importprunedfunds` — required arguments only.
    ///
    /// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    pub async fn import_prune_d_fund_s(&self, raw_transaction: String, tx_out_proof: String) -> Result<()> {
        self.client.call_raw("importprunedfunds", &[json!(raw_transaction), json!(tx_out_proof)]).await
    }

    /// `keypoolrefill` — required arguments only.
    ///
    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    ///
    /// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
    /// 
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn key_pool_refill(&self) -> Result<()> {
        self.client.call_raw("keypoolrefill", &[(); 0] as &[()]).await
    }

    /// `keypoolrefill` — with all optional arguments via [`KeyPoolRefillOptions`].
    ///
    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    pub async fn key_pool_refill_with(&self, opts: KeyPoolRefillOptions) -> Result<()> {
        self.client.call_raw("keypoolrefill", &[json!(opts.new_size)]).await
    }

    /// `listaddressgroupings` — required arguments only.
    ///
    /// Lists groups of addresses which have had their common ownership
    ///
    /// made public by common use as inputs or as the resulting change
    /// in past transactions
    pub async fn list_address_groupings(&self) -> Result<ListAddressGroupings> {
        self.client.call_raw("listaddressgroupings", &[(); 0] as &[()]).await
    }

    /// `listdescriptors` — required arguments only.
    ///
    /// List all descriptors present in a wallet.
    pub async fn list_descriptors(&self) -> Result<ListDescriptors> {
        self.client.call_raw("listdescriptors", &[(); 0] as &[()]).await
    }

    /// `listdescriptors` — with all optional arguments via [`ListDescriptorsOptions`].
    ///
    /// List all descriptors present in a wallet.
    pub async fn list_descriptors_with(&self, opts: ListDescriptorsOptions) -> Result<ListDescriptors> {
        self.client.call_raw("listdescriptors", &[json!(opts.private)]).await
    }

    /// `listlabels` — required arguments only.
    ///
    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    pub async fn list_labels(&self) -> Result<ListLabels> {
        self.client.call_raw("listlabels", &[(); 0] as &[()]).await
    }

    /// `listlabels` — with all optional arguments via [`ListLabelsOptions`].
    ///
    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    pub async fn list_labels_with(&self, opts: ListLabelsOptions) -> Result<ListLabels> {
        self.client.call_raw("listlabels", &[json!(opts.purpose)]).await
    }

    /// `listlockunspent` — required arguments only.
    ///
    /// Returns list of temporarily unspendable outputs.
    ///
    /// See the lockunspent call to lock and unlock transactions for spending.
    pub async fn list_lock_unspent(&self) -> Result<ListLockUnspent> {
        self.client.call_raw("listlockunspent", &[(); 0] as &[()]).await
    }

    /// `listreceivedbyaddress` — required arguments only.
    ///
    /// List balances by receiving address.
    pub async fn list_received_by_address(&self) -> Result<ListReceivedByAddress> {
        self.client.call_raw("listreceivedbyaddress", &[(); 0] as &[()]).await
    }

    /// `listreceivedbyaddress` — with all optional arguments via [`ListReceivedByAddressOptions`].
    ///
    /// List balances by receiving address.
    pub async fn list_received_by_address_with(&self, opts: ListReceivedByAddressOptions) -> Result<ListReceivedByAddress> {
        self.client.call_raw("listreceivedbyaddress", &[json!(opts.min_conf), json!(opts.include_empty), json!(opts.include_watchonly), json!(opts.address_filter), json!(opts.include_immature_coinbase)]).await
    }

    /// `listreceivedbylabel` — required arguments only.
    ///
    /// List received transactions by label.
    pub async fn list_received_by_label(&self) -> Result<ListReceivedByLabel> {
        self.client.call_raw("listreceivedbylabel", &[(); 0] as &[()]).await
    }

    /// `listreceivedbylabel` — with all optional arguments via [`ListReceivedByLabelOptions`].
    ///
    /// List received transactions by label.
    pub async fn list_received_by_label_with(&self, opts: ListReceivedByLabelOptions) -> Result<ListReceivedByLabel> {
        self.client.call_raw("listreceivedbylabel", &[json!(opts.min_conf), json!(opts.include_empty), json!(opts.include_watchonly), json!(opts.include_immature_coinbase)]).await
    }

    /// `listsinceblock` — required arguments only.
    ///
    /// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
    ///
    /// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
    /// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    pub async fn list_since_block(&self) -> Result<ListSinceBlock> {
        self.client.call_raw("listsinceblock", &[(); 0] as &[()]).await
    }

    /// `listsinceblock` — with all optional arguments via [`ListSinceBlockOptions`].
    ///
    /// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
    pub async fn list_since_block_with(&self, opts: ListSinceBlockOptions) -> Result<ListSinceBlock> {
        self.client.call_raw("listsinceblock", &[json!(opts.block_hash), json!(opts.target_confirmations), json!(opts.include_watchonly), json!(opts.include_removed), json!(opts.include_change), json!(opts.label)]).await
    }

    /// `listtransactions` — required arguments only.
    ///
    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    ///
    /// Returns up to 'count' most recent transactions ordered from oldest to newest while skipping the first number of 
    /// transactions specified in the 'skip' argument. A transaction can have multiple entries in this RPC response. 
    /// For instance, a wallet transaction that pays three addresses — one wallet-owned and two external — will produce 
    /// four entries. The payment to the wallet-owned address appears both as a send entry and as a receive entry. 
    /// As a result, the RPC response will contain one entry in the receive category and three entries in the send category.
    pub async fn list_transactions(&self) -> Result<ListTransactions> {
        self.client.call_raw("listtransactions", &[(); 0] as &[()]).await
    }

    /// `listtransactions` — with all optional arguments via [`ListTransactionsOptions`].
    ///
    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    pub async fn list_transactions_with(&self, opts: ListTransactionsOptions) -> Result<ListTransactions> {
        self.client.call_raw("listtransactions", &[json!(opts.label), json!(opts.count), json!(opts.skip), json!(opts.include_watchonly)]).await
    }

    /// `listunspent` — required arguments only.
    ///
    /// Returns array of unspent transaction outputs
    ///
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
    pub async fn list_unspent(&self) -> Result<ListUnspent> {
        self.client.call_raw("listunspent", &[(); 0] as &[()]).await
    }

    /// `listunspent` — with all optional arguments via [`ListUnspentOptions`].
    ///
    /// Returns array of unspent transaction outputs
    pub async fn list_unspent_with(&self, opts: ListUnspentOptions) -> Result<ListUnspent> {
        self.client.call_raw("listunspent", &[json!(opts.min_conf), json!(opts.max_conf), json!(opts.addresses), json!(opts.include_unsafe), json!(opts.query_options)]).await
    }

    /// `listwalletdir` — required arguments only.
    ///
    /// Returns a list of wallets in the wallet directory.
    pub async fn list_wallet_dir(&self) -> Result<ListWalletDir> {
        self.client.call_raw("listwalletdir", &[(); 0] as &[()]).await
    }

    /// `listwallets` — required arguments only.
    ///
    /// Returns a list of currently loaded wallets.
    ///
    /// For full information on the wallet, use "getwalletinfo"
    pub async fn list_wallets(&self) -> Result<ListWallets> {
        self.client.call_raw("listwallets", &[(); 0] as &[()]).await
    }

    /// `loadwallet` — required arguments only.
    ///
    /// Loads a wallet from a wallet file or directory.
    ///
    /// Note that all wallet command-line options used when starting bitcoind will be
    /// applied to the new wallet.
    pub async fn load_wallet(&self, file_name: String) -> Result<LoadWallet> {
        self.client.call_raw("loadwallet", &[json!(file_name)]).await
    }

    /// `loadwallet` — with all optional arguments via [`LoadWalletOptions`].
    ///
    /// Loads a wallet from a wallet file or directory.
    pub async fn load_wallet_with(&self, file_name: String, opts: LoadWalletOptions) -> Result<LoadWallet> {
        self.client.call_raw("loadwallet", &[json!(file_name), json!(opts.load_on_startup)]).await
    }

    /// `lockunspent` — required arguments only.
    ///
    /// Updates list of temporarily unspendable outputs.
    ///
    /// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
    /// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
    /// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
    /// Manually selected coins are automatically unlocked.
    /// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
    /// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
    /// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
    /// Also see the listunspent call
    pub async fn lock_unspent(&self, unlock: bool) -> Result<LockUnspent> {
        self.client.call_raw("lockunspent", &[json!(unlock)]).await
    }

    /// `lockunspent` — with all optional arguments via [`LockUnspentOptions`].
    ///
    /// Updates list of temporarily unspendable outputs.
    pub async fn lock_unspent_with(&self, unlock: bool, opts: LockUnspentOptions) -> Result<LockUnspent> {
        self.client.call_raw("lockunspent", &[json!(unlock), json!(opts.transactions), json!(opts.persistent)]).await
    }

    /// `migratewallet` — required arguments only.
    ///
    /// Migrate the wallet to a descriptor wallet.
    ///
    /// A new wallet backup will need to be made.
    /// 
    /// The migration process will create a backup of the wallet before migrating. This backup
    /// file will be named \<wallet name\>-\<timestamp\>.legacy.bak and can be found in the directory
    /// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
    /// Encrypted wallets must have the passphrase provided as an argument to this call.
    /// 
    /// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
    pub async fn mig_rate_wallet(&self) -> Result<MigRateWallet> {
        self.client.call_raw("migratewallet", &[(); 0] as &[()]).await
    }

    /// `migratewallet` — with all optional arguments via [`MigRateWalletOptions`].
    ///
    /// Migrate the wallet to a descriptor wallet.
    pub async fn mig_rate_wallet_with(&self, opts: MigRateWalletOptions) -> Result<MigRateWallet> {
        self.client.call_raw("migratewallet", &[json!(opts.wallet_name), json!(opts.passphrase)]).await
    }

    /// `psbtbumpfee` — required arguments only.
    ///
    /// Bumps the fee of a transaction T, replacing it with a new transaction B.
    ///
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
    pub async fn psbt_bump_fee(&self, txid: String) -> Result<PsbtBumpFee> {
        self.client.call_raw("psbtbumpfee", &[json!(txid)]).await
    }

    /// `psbtbumpfee` — with all optional arguments via [`PsbtBumpFeeOptions`].
    ///
    /// Bumps the fee of a transaction T, replacing it with a new transaction B.
    pub async fn psbt_bump_fee_with(&self, txid: String, opts: PsbtBumpFeeOptions) -> Result<PsbtBumpFee> {
        self.client.call_raw("psbtbumpfee", &[json!(txid), json!(opts.options)]).await
    }

    /// `removeprunedfunds` — required arguments only.
    ///
    /// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    pub async fn remove_prune_d_fund_s(&self, txid: String) -> Result<()> {
        self.client.call_raw("removeprunedfunds", &[json!(txid)]).await
    }

    /// `rescanblockchain` — required arguments only.
    ///
    /// Rescan the local blockchain for wallet related transactions.
    ///
    /// Note: Use "getwalletinfo" to query the scanning progress.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    pub async fn rescan_blockchain(&self) -> Result<RescanBlockchain> {
        self.client.call_raw("rescanblockchain", &[(); 0] as &[()]).await
    }

    /// `rescanblockchain` — with all optional arguments via [`RescanBlockchainOptions`].
    ///
    /// Rescan the local blockchain for wallet related transactions.
    pub async fn rescan_blockchain_with(&self, opts: RescanBlockchainOptions) -> Result<RescanBlockchain> {
        self.client.call_raw("rescanblockchain", &[json!(opts.start_height), json!(opts.stop_height)]).await
    }

    /// `restorewallet` — required arguments only.
    ///
    /// Restores and loads a wallet from backup.
    ///
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    pub async fn restore_wallet(&self, wallet_name: String, backup_file: String) -> Result<RestoreWallet> {
        self.client.call_raw("restorewallet", &[json!(wallet_name), json!(backup_file)]).await
    }

    /// `restorewallet` — with all optional arguments via [`RestoreWalletOptions`].
    ///
    /// Restores and loads a wallet from backup.
    pub async fn restore_wallet_with(&self, wallet_name: String, backup_file: String, opts: RestoreWalletOptions) -> Result<RestoreWallet> {
        self.client.call_raw("restorewallet", &[json!(wallet_name), json!(backup_file), json!(opts.load_on_startup)]).await
    }

    /// `send` — required arguments only.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Send a transaction.
    pub async fn send(&self, outputs: Vec<serde_json::Value>) -> Result<SendResult> {
        self.client.call_raw("send", &[json!(outputs)]).await
    }

    /// `send` — with all optional arguments via [`SendResultOptions`].
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    pub async fn send_with(&self, outputs: Vec<serde_json::Value>, opts: SendResultOptions) -> Result<SendResult> {
        self.client.call_raw("send", &[json!(outputs), json!(opts.conf_target), json!(opts.estimate_mode), json!(opts.fee_rate), json!(opts.options), json!(opts.version)]).await
    }

    /// `sendall` — required arguments only.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
    pub async fn send_all(&self, recipients: Vec<String>) -> Result<SendAll> {
        self.client.call_raw("sendall", &[json!(recipients)]).await
    }

    /// `sendall` — with all optional arguments via [`SendAllOptions`].
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    pub async fn send_all_with(&self, recipients: Vec<String>, opts: SendAllOptions) -> Result<SendAll> {
        self.client.call_raw("sendall", &[json!(recipients), json!(opts.conf_target), json!(opts.estimate_mode), json!(opts.fee_rate), json!(opts.options)]).await
    }

    /// `sendmany` — required arguments only.
    ///
    /// Send multiple times. Amounts are double-precision floating point numbers.
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_many(&self, amounts: serde_json::Value) -> Result<serde_json::Value> {
        self.client.call_raw("sendmany", &[json!(amounts)]).await
    }

    /// `sendmany` — with all optional arguments via [`SendManyOptions`].
    ///
    /// Send multiple times. Amounts are double-precision floating point numbers.
    pub async fn send_many_with(&self, amounts: serde_json::Value, opts: SendManyOptions) -> Result<serde_json::Value> {
        self.client.call_raw("sendmany", &[json!(amounts), json!(opts.dummy), json!(opts.min_conf), json!(opts.comment), json!(opts.subtract_fee_from), json!(opts.replaceable), json!(opts.conf_target), json!(opts.estimate_mode), json!(opts.fee_rate), json!(opts.verbose)]).await
    }

    /// `sendtoaddress` — required arguments only.
    ///
    /// Send an amount to a given address.
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_to_address(&self, address: String, amount: f64) -> Result<serde_json::Value> {
        self.client.call_raw("sendtoaddress", &[json!(address), json!(amount)]).await
    }

    /// `sendtoaddress` — with all optional arguments via [`SendToAddressOptions`].
    ///
    /// Send an amount to a given address.
    pub async fn send_to_address_with(&self, address: String, amount: f64, opts: SendToAddressOptions) -> Result<serde_json::Value> {
        self.client.call_raw("sendtoaddress", &[json!(address), json!(amount), json!(opts.comment), json!(opts.comment_to), json!(opts.subtract_fee_from_amount), json!(opts.replaceable), json!(opts.conf_target), json!(opts.estimate_mode), json!(opts.avoid_reuse), json!(opts.fee_rate), json!(opts.verbose)]).await
    }

    /// `setlabel` — required arguments only.
    ///
    /// Sets the label associated with the given address.
    pub async fn set_label(&self, address: String, label: String) -> Result<()> {
        self.client.call_raw("setlabel", &[json!(address), json!(label)]).await
    }

    /// `settxfee` — required arguments only.
    ///
    /// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
    ///
    /// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
    pub async fn set_tx_fee(&self, amount: f64) -> Result<SetTxFee> {
        self.client.call_raw("settxfee", &[json!(amount)]).await
    }

    /// `setwalletflag` — required arguments only.
    ///
    /// Change the state of the given wallet flag for a wallet.
    pub async fn set_wallet_flag(&self, flag: String) -> Result<SetWalletFlag> {
        self.client.call_raw("setwalletflag", &[json!(flag)]).await
    }

    /// `setwalletflag` — with all optional arguments via [`SetWalletFlagOptions`].
    ///
    /// Change the state of the given wallet flag for a wallet.
    pub async fn set_wallet_flag_with(&self, flag: String, opts: SetWalletFlagOptions) -> Result<SetWalletFlag> {
        self.client.call_raw("setwalletflag", &[json!(flag), json!(opts.value)]).await
    }

    /// `signmessage` — required arguments only.
    ///
    /// Sign a message with the private key of an address
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sign_message(&self, address: String, message: String) -> Result<SignMessage> {
        self.client.call_raw("signmessage", &[json!(address), json!(message)]).await
    }

    /// `signrawtransactionwithwallet` — required arguments only.
    ///
    /// Sign inputs for raw transaction (serialized, hex-encoded).
    ///
    /// The second optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sign_raw_transaction_with_wallet(&self, hex_string: String) -> Result<SignRawTransactionWithWallet> {
        self.client.call_raw("signrawtransactionwithwallet", &[json!(hex_string)]).await
    }

    /// `signrawtransactionwithwallet` — with all optional arguments via [`SignRawTransactionWithWalletOptions`].
    ///
    /// Sign inputs for raw transaction (serialized, hex-encoded).
    pub async fn sign_raw_transaction_with_wallet_with(&self, hex_string: String, opts: SignRawTransactionWithWalletOptions) -> Result<SignRawTransactionWithWallet> {
        self.client.call_raw("signrawtransactionwithwallet", &[json!(hex_string), json!(opts.prev_tx_s), json!(opts.sig_hash_type)]).await
    }

    /// `simulaterawtransaction` — required arguments only.
    ///
    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    pub async fn simulate_raw_transaction(&self) -> Result<SimulateRawTransaction> {
        self.client.call_raw("simulaterawtransaction", &[(); 0] as &[()]).await
    }

    /// `simulaterawtransaction` — with all optional arguments via [`SimulateRawTransactionOptions`].
    ///
    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    pub async fn simulate_raw_transaction_with(&self, opts: SimulateRawTransactionOptions) -> Result<SimulateRawTransaction> {
        self.client.call_raw("simulaterawtransaction", &[json!(opts.raw_tx_s), json!(opts.options)]).await
    }

    /// `unloadwallet` — required arguments only.
    ///
    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    ///
    /// If both are specified, they must be identical.
    pub async fn unload_wallet(&self) -> Result<UnloadWallet> {
        self.client.call_raw("unloadwallet", &[(); 0] as &[()]).await
    }

    /// `unloadwallet` — with all optional arguments via [`UnloadWalletOptions`].
    ///
    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    pub async fn unload_wallet_with(&self, opts: UnloadWalletOptions) -> Result<UnloadWallet> {
        self.client.call_raw("unloadwallet", &[json!(opts.wallet_name), json!(opts.load_on_startup)]).await
    }

    /// `walletcreatefundedpsbt` — required arguments only.
    ///
    /// Creates and funds a transaction in the Partially Signed Transaction format.
    ///
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    pub async fn wallet_create_funded_psbt(&self, outputs: Vec<serde_json::Value>) -> Result<WalletCreateFundedPsbt> {
        self.client.call_raw("walletcreatefundedpsbt", &[json!(outputs)]).await
    }

    /// `walletcreatefundedpsbt` — with all optional arguments via [`WalletCreateFundedPsbtOptions`].
    ///
    /// Creates and funds a transaction in the Partially Signed Transaction format.
    pub async fn wallet_create_funded_psbt_with(&self, outputs: Vec<serde_json::Value>, opts: WalletCreateFundedPsbtOptions) -> Result<WalletCreateFundedPsbt> {
        self.client.call_raw("walletcreatefundedpsbt", &[json!(outputs), json!(opts.inputs), json!(opts.locktime), json!(opts.options), json!(opts.bip32derivs), json!(opts.version)]).await
    }

    /// `walletdisplayaddress` — required arguments only.
    ///
    /// Display address on an external signer for verification.
    pub async fn wallet_display_address(&self, address: String) -> Result<WalletDisplayAddress> {
        self.client.call_raw("walletdisplayaddress", &[json!(address)]).await
    }

    /// `walletlock` — required arguments only.
    ///
    /// Removes the wallet encryption key from memory, locking the wallet.
    ///
    /// After calling this method, you will need to call walletpassphrase again
    /// before being able to call any methods which require the wallet to be unlocked.
    pub async fn wallet_lock(&self) -> Result<()> {
        self.client.call_raw("walletlock", &[(); 0] as &[()]).await
    }

    /// `walletpassphrase` — required arguments only.
    ///
    /// Stores the wallet decryption key in memory for 'timeout' seconds.
    ///
    /// This is needed prior to performing transactions related to private keys such as sending bitcoins
    /// 
    /// Note:
    /// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
    /// time that overrides the old one.
    pub async fn wallet_passphrase(&self, passphrase: String, time_out: i64) -> Result<()> {
        self.client.call_raw("walletpassphrase", &[json!(passphrase), json!(time_out)]).await
    }

    /// `walletpassphrasechange` — required arguments only.
    ///
    /// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
    pub async fn wallet_passphrase_change(&self, oldpassphrase: String, new_passphrase: String) -> Result<()> {
        self.client.call_raw("walletpassphrasechange", &[json!(oldpassphrase), json!(new_passphrase)]).await
    }

    /// `walletprocesspsbt` — required arguments only.
    ///
    /// Update a PSBT with input information from our wallet and then sign inputs
    ///
    /// that we can sign for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn wallet_process_psbt(&self, psbt: String) -> Result<WalletProcessPsbt> {
        self.client.call_raw("walletprocesspsbt", &[json!(psbt)]).await
    }

    /// `walletprocesspsbt` — with all optional arguments via [`WalletProcessPsbtOptions`].
    ///
    /// Update a PSBT with input information from our wallet and then sign inputs
    pub async fn wallet_process_psbt_with(&self, psbt: String, opts: WalletProcessPsbtOptions) -> Result<WalletProcessPsbt> {
        self.client.call_raw("walletprocesspsbt", &[json!(psbt), json!(opts.sign), json!(opts.sig_hash_type), json!(opts.bip32derivs), json!(opts.finalize)]).await
    }

}
