// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `30`
//!
//! Auto-generated from OpenRPC specification.

//! <details>
//! <summary> == Blockchain == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `dumptxoutset` | `DumpTxoutSet` |
//! | `getbestblockhash` | returns string |
//! | `getblock` | `GetBlock` |
//! | `getblockchaininfo` | `GetBlockchainInfo` |
//! | `getblockcount` | returns number |
//! | `getblockfilter` | `GetBlockFilter` |
//! | `getblockfrompeer` | `GetBlockFromPeer` |
//! | `getblockhash` | returns string |
//! | `getblockheader` | `GetBlockHeader` |
//! | `getblockstats` | `GetBlockstats` |
//! | `getchainstates` | `GetChainstates` |
//! | `getchaintips` | `GetChainTips` |
//! | `getchaintxstats` | `GetChainTxstats` |
//! | `getdeploymentinfo` | `GetDeploymentInfo` |
//! | `getdescriptoractivity` | `GetDescriptorActivity` |
//! | `getdifficulty` | returns number |
//! | `getmempoolancestors` | `GetMempoolAncestors` |
//! | `getmempoolcluster` | `GetMempoolcluster` |
//! | `getmempooldescendants` | `GetMempoolDescendants` |
//! | `getmempoolentry` | `GetMempoolEntry` |
//! | `getmempoolinfo` | `GetMempoolInfo` |
//! | `getrawmempool` | `GetRawMempool` |
//! | `gettxout` | `GetTxout` |
//! | `gettxoutproof` | returns string |
//! | `gettxoutsetinfo` | `GetTxoutSetInfo` |
//! | `gettxspendingprevout` | `GetTxspendingprevOut` |
//! | `importmempool` | `ImportMempool` |
//! | `loadtxoutset` | `LoadTxoutSet` |
//! | `preciousblock` | returns nothing |
//! | `pruneblockchain` | returns number |
//! | `savemempool` | `SaveMempool` |
//! | `scanblocks` | `ScanBlocks` |
//! | `scantxoutset` | `ScanTxoutSet` |
//! | `verifychain` | returns boolean |
//! | `verifytxoutproof` | `VerifyTxoutProof` |
//! | `waitforblock` | `WaitForBlock` |
//! | `waitforblockheight` | `WaitForBlockHeight` |
//! | `waitfornewblock` | `WaitForNewBlock` |
//!
//! </details>
//!
//! <details>
//! <summary> == Control == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `getmemoryinfo` | `GetMemoryInfo` |
//! | `getopenrpc` | `GetopenRpc` |
//! | `getrpcinfo` | `GetRpcInfo` |
//! | `help` | returns string |
//! | `logging` | `Logging` |
//! | `stop` | returns string |
//! | `uptime` | returns number |
//!
//! </details>
//!
//! <details>
//! <summary> == Hidden == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `addconnection` | `AddConnection` |
//! | `addpeeraddress` | `AddPeerAddress` |
//! | `echo` | `Echo` |
//! | `echoipc` | returns string |
//! | `echojson` | `EchoJson` |
//! | `estimaterawfee` | `EstimateRawFee` |
//! | `generate` | `Generate` |
//! | `generateblock` | `GenerateBlock` |
//! | `generatetoaddress` | `GenerateToAddress` |
//! | `generatetodescriptor` | `GenerateToDescriptor` |
//! | `getmempoolfeeratediagram` | `GetMempoolFeeratediagram` |
//! | `getorphantxs` | `GetOrphanTxs` |
//! | `getrawaddrman` | `GetRawAddrman` |
//! | `invalidateblock` | returns nothing |
//! | `mockscheduler` | returns nothing |
//! | `reconsiderblock` | returns nothing |
//! | `sendmsgtopeer` | `SendmsgToPeer` |
//! | `setmocktime` | returns nothing |
//! | `syncwithvalidationinterfacequeue` | returns nothing |
//!
//! </details>
//!
//! <details>
//! <summary> == Mining == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `getblocktemplate` | `GetBlocktemplate` |
//! | `getmininginfo` | `GetMiningInfo` |
//! | `getnetworkhashps` | returns number |
//! | `getprioritisedtransactions` | `GetPrioritisedTransactions` |
//! | `prioritisetransaction` | returns boolean |
//! | `submitblock` | `SubmitBlock` |
//! | `submitheader` | returns nothing |
//!
//! </details>
//!
//! <details>
//! <summary> == Network == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `addnode` | returns nothing |
//! | `clearbanned` | returns nothing |
//! | `disconnectnode` | returns nothing |
//! | `getaddednodeinfo` | `GetAddedNodeInfo` |
//! | `getaddrmaninfo` | `GetAddrmanInfo` |
//! | `getconnectioncount` | returns number |
//! | `getnettotals` | `GetNetTotals` |
//! | `getnetworkinfo` | `GetNetworkInfo` |
//! | `getnodeaddresses` | `GetNodeAddresses` |
//! | `getpeerinfo` | `GetPeerInfo` |
//! | `listbanned` | `ListBanned` |
//! | `ping` | returns nothing |
//! | `setban` | returns nothing |
//! | `setnetworkactive` | returns boolean |
//!
//! </details>
//!
//! <details>
//! <summary> == RawTransactions == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `analyzepsbt` | `AnalyzePsbt` |
//! | `combinepsbt` | returns string |
//! | `combinerawtransaction` | returns string |
//! | `converttopsbt` | returns string |
//! | `createpsbt` | returns string |
//! | `createrawtransaction` | returns string |
//! | `decodepsbt` | `DecodePsbt` |
//! | `decoderawtransaction` | `DecodeRawTransaction` |
//! | `decodescript` | `DecodeScript` |
//! | `descriptorprocesspsbt` | `DescriptorProcessPsbt` |
//! | `finalizepsbt` | `FinalizePsbt` |
//! | `fundrawtransaction` | `FundRawTransaction` |
//! | `getrawtransaction` | `GetRawTransaction` |
//! | `joinpsbts` | returns string |
//! | `sendrawtransaction` | returns string |
//! | `signrawtransactionwithkey` | `SignRawTransactionwithKey` |
//! | `submitpackage` | `SubmitPackage` |
//! | `testmempoolaccept` | `TestMempoolaccept` |
//! | `utxoupdatepsbt` | returns string |
//!
//! </details>
//!
//! <details>
//! <summary> == Signer == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `enumeratesigners` | `EnumerateSigners` |
//!
//! </details>
//!
//! <details>
//! <summary> == Util == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `createmultisig` | `CreateMultisig` |
//! | `deriveaddresses` | `DeriveAddresses` |
//! | `estimatesmartfee` | `EstimateSmartFee` |
//! | `getdescriptorinfo` | `GetDescriptorInfo` |
//! | `getindexinfo` | `GetIndexInfo` |
//! | `signmessagewithprivkey` | returns string |
//! | `validateaddress` | `ValidateAddress` |
//! | `verifymessage` | returns boolean |
//!
//! </details>
//!
//! <details>
//! <summary> == Wallet == </summary>
//!
//! | Method | Returns |
//! |:---|:---|
//! | `abandontransaction` | returns nothing |
//! | `abortrescan` | returns boolean |
//! | `backupwallet` | returns nothing |
//! | `bumpfee` | `BumpFee` |
//! | `createwallet` | `CreateWallet` |
//! | `createwalletdescriptor` | `CreateWalletDescriptor` |
//! | `encryptwallet` | returns string |
//! | `getaddressesbylabel` | `GetAddressesbyLabel` |
//! | `getaddressinfo` | `GetAddressInfo` |
//! | `getbalance` | returns string |
//! | `getbalances` | `GetBalances` |
//! | `gethdkeys` | `GetHdKeys` |
//! | `getnewaddress` | returns string |
//! | `getrawchangeaddress` | returns string |
//! | `getreceivedbyaddress` | returns string |
//! | `getreceivedbylabel` | returns string |
//! | `gettransaction` | `GetTransaction` |
//! | `getwalletinfo` | `GetWalletInfo` |
//! | `importdescriptors` | `ImportDescriptors` |
//! | `importprunedfunds` | returns nothing |
//! | `keypoolrefill` | returns nothing |
//! | `listaddressgroupings` | `ListAddressGroupings` |
//! | `listdescriptors` | `ListDescriptors` |
//! | `listlabels` | `ListLabels` |
//! | `listlockunspent` | `ListLockUnspent` |
//! | `listreceivedbyaddress` | `ListReceivedbyAddress` |
//! | `listreceivedbylabel` | `ListReceivedbyLabel` |
//! | `listsinceblock` | `ListSinceBlock` |
//! | `listtransactions` | `ListTransactions` |
//! | `listunspent` | `ListUnspent` |
//! | `listwalletdir` | `ListWalletDir` |
//! | `listwallets` | `ListWallets` |
//! | `loadwallet` | `LoadWallet` |
//! | `lockunspent` | returns boolean |
//! | `migratewallet` | `MigrateWallet` |
//! | `psbtbumpfee` | `PsbtBumpFee` |
//! | `removeprunedfunds` | returns nothing |
//! | `rescanblockchain` | `RescanBlockchain` |
//! | `restorewallet` | `RestoreWallet` |
//! | `send` | `Send` |
//! | `sendall` | `SendAll` |
//! | `sendmany` | `Sendmany` |
//! | `sendtoaddress` | `SendToAddress` |
//! | `setlabel` | returns nothing |
//! | `settxfee` | returns boolean |
//! | `setwalletflag` | `SetWalletflag` |
//! | `signmessage` | returns string |
//! | `signrawtransactionwithwallet` | `SignRawTransactionwithWallet` |
//! | `simulaterawtransaction` | `SimulateRawTransaction` |
//! | `unloadwallet` | `UnloadWallet` |
//! | `walletcreatefundedpsbt` | `WalletCreateFundedPsbt` |
//! | `walletdisplayaddress` | `WalletDisplayAddress` |
//! | `walletlock` | returns nothing |
//! | `walletpassphrase` | returns nothing |
//! | `walletpassphrasechange` | returns nothing |
//! | `walletprocesspsbt` | `WalletProcessPsbt` |
//!
//! </details>
//!

pub mod blockchain;
pub mod control;
pub mod hidden;
pub mod mining;
pub mod network;
pub mod raw_transactions;
pub mod signer;
pub mod util;
pub mod wallet;

pub use self::blockchain::{DumpTxoutSet, GetBestBlockHash, GetBlockVerboseZero, GetBlockVerboseOne, GetBlockVerboseTwoTxItem, GetBlockVerboseTwo, GetBlockVerboseThreeTxItem, GetBlockVerboseThreeTxItemVinItem, GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey, GetBlockVerboseThreeTxItemVinItemPrevout, GetBlockVerboseThree, GetBlockchainInfo, GetBlockCount, GetBlockFilter, GetBlockFromPeer, GetBlockHash, GetBlockHeaderVerboseOne, GetBlockHeaderVerboseZero, GetBlockstats, GetChainstates, GetChainstatesChainstatesItem, GetChainTips, GetChainTipsItem, GetChainTxstats, GetDeploymentInfo, GetDeploymentInfoDeployments, GetDeploymentInfoDeploymentsBip9Statistics, GetDeploymentInfoDeploymentsBip9, GetDescriptorActivity, GetDifficulty, GetMempoolAncestorsVerboseZero, GetMempoolAncestorsVerboseOneEntry, GetMempoolAncestorsVerboseOneEntryFees, GetMempoolAncestorsVerboseOne, GetMempoolcluster, GetMempoolclusterChunksItem, GetMempoolDescendantsVerboseZero, GetMempoolDescendantsVerboseOneEntry, GetMempoolDescendantsVerboseOneEntryFees, GetMempoolDescendantsVerboseOne, GetMempoolEntry, GetMempoolEntryFees, GetMempoolInfo, GetRawMempoolVerboseZero, GetRawMempoolVerboseOneEntry, GetRawMempoolVerboseOneEntryFees, GetRawMempoolVerboseOne, GetRawMempoolVerboseTwo, GetTxoutVerboseOne, GetTxoutVerboseOneScriptPubKey, GetTxoutProof, GetTxoutSetInfo, GetTxoutSetInfoBlockInfoUnspendables, GetTxoutSetInfoBlockInfo, GetTxspendingprevOut, GetTxspendingprevOutItem, ImportMempool, LoadTxoutSet, PruneBlockchain, SaveMempool, ScanBlocksVerboseOne, ScanBlocksVerboseTwo, ScanBlocksVerboseThree, ScanTxoutSetVerboseZero, ScanTxoutSetVerboseZeroUnspentsItem, ScanTxoutSetVerboseOne, ScanTxoutSetVerboseTwo, VerifyChain, VerifyTxoutProof, WaitForBlock, WaitForBlockHeight, WaitForNewBlock};
pub use self::control::{GetMemoryInfoVerboseZero, GetMemoryInfoVerboseZeroLocked, GetMemoryInfoVerboseOne, GetopenRpc, GetRpcInfo, GetRpcInfoActiveCommandsItem, Help, Logging, Stop, Uptime};
pub use self::hidden::{AddConnection, AddPeerAddress, Echo, EchoIpc, EchoJson, EstimateRawFee, EstimateRawFeeLong, EstimateRawFeeMedium, EstimateRawFeeShortFail, EstimateRawFeeShortPass, EstimateRawFeeShort, Generate, GenerateBlock, GenerateToAddress, GenerateToDescriptor, GetMempoolFeeratediagram, GetMempoolFeeratediagramItem, GetOrphanTxsVerboseZero, GetOrphanTxsVerboseOneItem, GetOrphanTxsVerboseOne, GetOrphanTxsVerboseTwoItem, GetOrphanTxsVerboseTwo, GetRawAddrman, GetRawAddrmanEntryEntry, SendmsgToPeer};
pub use self::mining::{GetBlocktemplateVerboseOne, GetBlocktemplateVerboseTwoTransactionsItem, GetBlocktemplateVerboseTwo, GetMiningInfo, GetMiningInfoNext, GetNetworkHashps, GetPrioritisedTransactions, GetPrioritisedTransactionsEntry, PrioritiseTransaction, SubmitBlockVerboseOne};
pub use self::network::{GetAddedNodeInfo, GetAddedNodeInfoItem, GetAddedNodeInfoItemAddressesItem, GetAddrmanInfo, GetAddrmanInfoEntry, GetConnectionCount, GetNetTotals, GetNetTotalsUploadtarget, GetNetworkInfo, GetNetworkInfoLocaladdressesItem, GetNetworkInfoNetworksItem, GetNodeAddresses, GetNodeAddressesItem, GetPeerInfo, GetPeerInfoItem, ListBanned, ListBannedItem, SetNetworkactive};
pub use self::raw_transactions::{AnalyzePsbt, AnalyzePsbtInputsItem, AnalyzePsbtInputsItemMissing, CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreatePsbt, CreateRawTransaction, DecodePsbt, DecodePsbtGlobalXpubsItem, DecodePsbtInputsItem, DecodePsbtInputsItemBip32DerivsItem, DecodePsbtInputsItemFinalScriptSig, DecodePsbtInputsItemMusig2PartialSigsItem, DecodePsbtInputsItemMusig2ParticipantPubkeysItem, DecodePsbtInputsItemMusig2PubnoncesItem, DecodePsbtInputsItemNonWitnessUtxo, DecodePsbtInputsItemProprietaryItem, DecodePsbtInputsItemRedeemScript, DecodePsbtInputsItemTaprootBip32DerivsItem, DecodePsbtInputsItemTaprootScriptPathSigsItem, DecodePsbtInputsItemTaprootScriptsItem, DecodePsbtInputsItemWitnessScript, DecodePsbtInputsItemWitnessUtxoScriptPubKey, DecodePsbtInputsItemWitnessUtxo, DecodePsbtOutputsItem, DecodePsbtOutputsItemBip32DerivsItem, DecodePsbtOutputsItemMusig2ParticipantPubkeysItem, DecodePsbtOutputsItemProprietaryItem, DecodePsbtOutputsItemRedeemScript, DecodePsbtOutputsItemTaprootBip32DerivsItem, DecodePsbtOutputsItemTaprootTreeItem, DecodePsbtOutputsItemWitnessScript, DecodePsbtProprietaryItem, DecodePsbtTx, DecodeRawTransaction, DecodeRawTransactionVinItem, DecodeRawTransactionVinItemScriptSig, DecodeRawTransactionVoutItem, DecodeRawTransactionVoutItemScriptPubKey, DecodeScript, DecodeScriptSegwit, DescriptorProcessPsbt, FinalizePsbt, FundRawTransaction, GetRawTransactionVerboseZero, GetRawTransactionVerboseOneVinItem, GetRawTransactionVerboseOneVinItemScriptSig, GetRawTransactionVerboseOneVoutItem, GetRawTransactionVerboseOneVoutItemScriptPubKey, GetRawTransactionVerboseOne, GetRawTransactionVerboseTwoVinItem, GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey, GetRawTransactionVerboseTwoVinItemPrevout, GetRawTransactionVerboseTwo, JoinPsbts, SendRawTransaction, SignRawTransactionwithKey, SignRawTransactionwithKeyErrorsItem, SubmitPackage, SubmitPackageTxResults, SubmitPackageTxResultsFees, TestMempoolaccept, TestMempoolacceptItem, TestMempoolacceptItemFees, UtxoUpdatePsbt};
pub use self::signer::{EnumerateSigners, EnumerateSignersSignersItem};
pub use self::util::{CreateMultisig, DeriveAddressesVerboseZero, DeriveAddressesVerboseOne, EstimateSmartFee, GetDescriptorInfo, GetIndexInfo, GetIndexInfoEntry, SignMessagewithPrivKey, ValidateAddress, VerifyMessage};
pub use self::wallet::{AbortRescan, BumpFee, CreateWallet, CreateWalletDescriptor, EncryptWallet, GetAddressesbyLabel, GetAddressesbyLabelEntry, GetAddressInfo, GetAddressInfoEmbedded, GetBalance, GetBalances, GetBalancesLastprocessedblock, GetBalancesMine, GetHdKeys, GetHdKeysItem, GetHdKeysItemDescriptorsItem, GetNewAddress, GetRawChangeAddress, GetReceivedbyAddress, GetReceivedbyLabel, GetTransaction, GetTransactionDecodedVinItem, GetTransactionDecodedVinItemScriptSig, GetTransactionDecodedVoutItem, GetTransactionDecodedVoutItemScriptPubKey, GetTransactionDecoded, GetTransactionDetailsItem, GetTransactionLastprocessedblock, GetWalletInfo, GetWalletInfoLastprocessedblock, GetWalletInfoScanning, ImportDescriptors, ImportDescriptorsItem, ImportDescriptorsItemError, ListAddressGroupings, ListDescriptors, ListDescriptorsDescriptorsItem, ListLabels, ListLockUnspent, ListLockUnspentItem, ListReceivedbyAddress, ListReceivedbyAddressItem, ListReceivedbyLabel, ListReceivedbyLabelItem, ListSinceBlock, ListSinceBlockTransactionsItem, ListTransactions, ListTransactionsItem, ListUnspent, ListUnspentItem, ListWalletDir, ListWalletDirWalletsItem, ListWallets, LoadWallet, LockUnspent, MigrateWallet, PsbtBumpFee, RescanBlockchain, RestoreWallet, Send, SendAll, SendmanyVerboseZero, SendmanyVerboseOne, SendToAddressVerboseZero, SendToAddressVerboseOne, SetTxFee, SetWalletflag, SignMessage, SignRawTransactionwithWallet, SignRawTransactionwithWalletErrorsItem, SimulateRawTransaction, UnloadWallet, WalletCreateFundedPsbt, WalletDisplayAddress, WalletProcessPsbt};
