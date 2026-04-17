#!/usr/bin/env python3
"""Compare struct definitions between codegen output and corepc-types source."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional


# -- Bridge: spec name -> repo name (or "IGNORE") ---------------------------

TYPE_BRIDGE: dict[str, str] = {
    # Case fixes
    "DumpTxoutSet": "DumpTxOutSet",
    "GetAddrmanInfo": "GetAddrManInfo",
    "GetAddressesbyLabel": "GetAddressesByLabel",
    "GetBlockstats": "GetBlockStats",
    "GetChainstates": "GetChainStates",
    "GetChainTxstats": "GetChainTxStats",
    "GetRawAddrman": "GetRawAddrMan",
    "GetReceivedbyAddress": "GetReceivedByAddress",
    "GetReceivedbyLabel": "GetReceivedByLabel",
    "GetTxoutSetInfo": "GetTxOutSetInfo",
    "GetTxoutSetInfoBlockInfo": "GetTxOutSetInfoBlockInfo",
    "GetTxspendingprevOut": "GetTxSpendingPrevout",
    "ListReceivedbyAddress": "ListReceivedByAddress",
    "ListReceivedbyLabel": "ListReceivedByLabel",
    "LoadTxoutSet": "LoadTxOutSet",
    "SetNetworkactive": "SetNetworkActive",
    "SetWalletflag": "SetWalletFlag",
    "SignMessagewithPrivKey": "SignMessageWithPrivKey",
    "TestMempoolaccept": "TestMempoolAccept",
    "VerifyTxoutProof": "VerifyTxOutProof",
    # Array item renames
    "GetAddedNodeInfoItem": "AddedNode",
    "GetAddedNodeInfoItemAddressesItem": "AddedNodeAddress",
    "GetChainTipsItem": "ChainTips",
    "GetPeerInfoItem": "PeerInfo",
    "ListBannedItem": "Banned",
    "GetNodeAddressesItem": "NodeAddress",
    "GetTxspendingprevOutItem": "GetTxSpendingPrevoutItem",
    "TestMempoolacceptItem": "MempoolAcceptance",
    "TestMempoolacceptItemFees": "MempoolAcceptanceFees",
    "ImportDescriptorsItem": "ImportDescriptorsResult",
    "ImportDescriptorsItemError": "IGNORE",  # empty placeholder; repo uses serde_json::Value
    "ListReceivedbyAddressItem": "ListReceivedByAddressItem",
    "ListReceivedbyLabelItem": "ListReceivedByLabelItem",
    "ListTransactionsItem": "TransactionItem",
    "GetHdKeysItem": "HdKey",
    "GetHdKeysItemDescriptorsItem": "HdKeyDescriptor",
    # Wallet
    "DeriveAddressesVerboseZero": "DeriveAddresses",
    "DeriveAddressesVerboseOne": "DeriveAddressesMultipath",
    "UnloadWallet": "UnloadWallet",
    # Mempool
    "GetMempoolEntryFees": "MempoolEntryFees",
    "GetMempoolAncestorsVerboseOneEntry": "MempoolEntry",
    "GetMempoolDescendantsVerboseOneEntry": "MempoolEntry",
    "GetRawMempoolVerboseOneEntry": "MempoolEntry",
    "GetMempoolAncestorsVerboseOneEntryFees": "MempoolEntryFees",
    "GetMempoolDescendantsVerboseOneEntryFees": "MempoolEntryFees",
    "GetRawMempoolVerboseOneEntryFees": "MempoolEntryFees",
    "GetMempoolAncestorsVerboseZero": "GetMempoolAncestors",
    "GetMempoolDescendantsVerboseZero": "GetMempoolDescendants",
    "GetRawMempoolVerboseZero": "GetRawMempool",
    "GetMempoolAncestorsVerboseOne": "GetMempoolAncestorsVerbose",
    "GetMempoolDescendantsVerboseOne": "GetMempoolDescendantsVerbose",
    "GetRawMempoolVerboseOne": "GetRawMempoolVerbose",
    "GetRawMempoolVerboseTwo": "GetRawMempoolSequence",
    "GetOrphanTxsVerboseZero": "GetOrphanTxs",
    "GetOrphanTxsVerboseOneItem": "GetOrphanTxsVerboseOneEntry",
    "GetOrphanTxsVerboseTwoItem": "GetOrphanTxsVerboseTwoEntry",
    # Deployment / softfork
    "GetDeploymentInfoDeployments": "DeploymentInfo",
    "GetDeploymentInfoDeploymentsBip9": "Bip9Info",
    "GetDeploymentInfoDeploymentsBip9Statistics": "Bip9Statistics",
    # Block-verbose nested
    "GetBlockVerboseTwoTxItem": "GetBlockVerboseTwoTransaction",
    "GetBlockVerboseThreeTxItem": "GetBlockVerboseThreeTransaction",
    "GetBlockVerboseThreeTxItemVinItem": "RawTransactionInputWithPrevout",
    "GetBlockVerboseThreeTxItemVinItemPrevout": "GetBlockVerboseThreePrevout",
    "GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey": "IGNORE",  # repo uses ScriptPubkey from types/src/lib.rs
    # Block template
    "GetBlocktemplateVerboseTwo": "GetBlockTemplate",
    "GetBlocktemplateVerboseTwoTransactionsItem": "BlockTemplateTransaction",
    # Estimate raw fee
    "EstimateRawFeeShort": "RawFeeDetail",
    "EstimateRawFeeMedium": "IGNORE",  # spec generates empty struct, not used in repo
    "EstimateRawFeeLong": "IGNORE",  # spec generates empty struct, not used in repo
    "EstimateRawFeeShortPass": "RawFeeRange",
    "EstimateRawFeeShortFail": "IGNORE",  # spec generates empty struct for error case
    # Block header
    "GetBlockHeaderVerboseZero": "GetBlockHeader",
    "GetBlockHeaderVerboseOne": "GetBlockHeaderVerbose",
    # Raw transaction
    "GetRawTransactionVerboseZero": "GetRawTransaction",
    "GetRawTransactionVerboseOne": "GetRawTransactionVerbose",
    "GetRawTransactionVerboseTwo": "GetRawTransactionVerboseWithPrevout",
    "GetRawTransactionVerboseOneVinItem": "IGNORE",  # repo uses RawTransactionInput from psbt module
    "GetRawTransactionVerboseOneVinItemScriptSig": "IGNORE",  # repo uses ScriptSig from types/src/lib.rs
    "GetRawTransactionVerboseOneVoutItem": "IGNORE",  # repo uses RawTransactionOutput from psbt module
    "GetRawTransactionVerboseOneVoutItemScriptPubKey": "IGNORE",  # repo uses ScriptPubkey from types/src/lib.rs
    "GetRawTransactionVerboseTwoVinItem": "IGNORE",  # repo uses RawTransactionInput from psbt module
    "GetRawTransactionVerboseTwoVinItemPrevout": "GetBlockVerboseThreePrevout",
    "GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey": "IGNORE",  # repo uses ScriptPubkey
    "GetTxoutVerboseOne": "GetTxOut",
    "GetTxoutVerboseOneScriptPubKey": "IGNORE",  # repo uses ScriptPubkey from types/src/lib.rs
    # Scan
    "ScanTxoutSetVerboseZero": "ScanTxOutSetStart",
    "ScanTxoutSetVerboseOne": "ScanTxOutSetStatus",
    "ScanTxoutSetVerboseTwo": "ScanTxOutSetAbort",
    "ScanTxoutSetVerboseZeroUnspentsItem": "ScanTxOutSetUnspent",
    "ScanBlocksVerboseOne": "ScanBlocksStart",
    "ScanBlocksVerboseTwo": "ScanBlocksStatus",
    "ScanBlocksVerboseThree": "ScanBlocksAbort",
    # Memory info
    "GetMemoryInfoVerboseZero": "GetMemoryInfoStats",
    "GetMemoryInfoVerboseOne": "IGNORE",  # spec wrapper for verbose=true; repo uses same struct
    "GetMemoryInfoVerboseZeroLocked": "Locked",
    # Send
    "SendToAddressVerboseZero": "SendToAddress",
    "SendToAddressVerboseOne": "SendManyVerbose",
    "SendmanyVerboseZero": "SendMany",
    "SendmanyVerboseOne": "SendManyVerbose",
    # Sign raw transaction
    "SignRawTransactionwithKey": "SignRawTransaction",
    "SignRawTransactionwithKeyErrorsItem": "SignFail",
    "SignRawTransactionwithWallet": "SignRawTransaction",
    "SignRawTransactionwithWalletErrorsItem": "SignFail",
    # PSBT
    "DecodePsbtInputsItem": "PsbtInput",
    "DecodePsbtInputsItemBip32DerivsItem": "IGNORE",  # repo uses Bip32Deriv from psbt module
    "DecodePsbtInputsItemFinalScriptSig": "IGNORE",  # repo uses ScriptSig from types/src/lib.rs
    "DecodePsbtInputsItemNonWitnessUtxo": "IGNORE",  # repo uses RawTransaction from psbt module
    "DecodePsbtInputsItemProprietaryItem": "Proprietary",
    "DecodePsbtInputsItemRedeemScript": "IGNORE",  # repo uses PsbtScript from psbt module
    "DecodePsbtInputsItemTaprootBip32DerivsItem": "TaprootBip32Deriv",
    "DecodePsbtInputsItemWitnessScript": "IGNORE",  # repo uses PsbtScript from psbt module
    "DecodePsbtInputsItemWitnessUtxo": "IGNORE",  # repo uses WitnessUtxo from psbt module
    "DecodePsbtInputsItemWitnessUtxoScriptPubKey": "IGNORE",  # repo uses ScriptPubkey
    "DecodePsbtInputsItemTaprootScriptsItem": "TaprootScript",
    "DecodePsbtInputsItemTaprootScriptPathSigsItem": "TaprootScriptPathSig",
    "DecodePsbtInputsItemMusig2PartialSigsItem": "Musig2PartialSig",
    "DecodePsbtInputsItemMusig2ParticipantPubkeysItem": "Musig2ParticipantPubkeys",
    "DecodePsbtInputsItemMusig2PubnoncesItem": "Musig2Pubnonce",
    "DecodePsbtOutputsItem": "PsbtOutput",
    "DecodePsbtOutputsItemBip32DerivsItem": "IGNORE",  # repo uses Bip32Deriv from psbt module
    "DecodePsbtOutputsItemMusig2ParticipantPubkeysItem": "Musig2ParticipantPubkeys",
    "DecodePsbtOutputsItemRedeemScript": "IGNORE",  # repo uses PsbtScript from psbt module
    "DecodePsbtOutputsItemTaprootTreeItem": "TaprootLeaf",
    "DecodePsbtOutputsItemWitnessScript": "IGNORE",  # repo uses PsbtScript from psbt module
    "DecodePsbtOutputsItemProprietaryItem": "Proprietary",
    "DecodePsbtOutputsItemTaprootBip32DerivsItem": "TaprootBip32Deriv",
    "DecodePsbtProprietaryItem": "Proprietary",
    "DecodePsbtGlobalXpubsItem": "GlobalXpub",
    "AnalyzePsbtInputsItem": "AnalyzePsbtInput",
    "AnalyzePsbtInputsItemMissing": "AnalyzePsbtInputMissing",
    # Decode raw transaction
    "DecodeRawTransactionVinItem": "IGNORE",  # repo uses RawTransactionInput
    "DecodeRawTransactionVinItemScriptSig": "IGNORE",  # repo uses ScriptSig
    "DecodeRawTransactionVoutItem": "IGNORE",  # repo uses RawTransactionOutput
    "DecodeRawTransactionVoutItemScriptPubKey": "IGNORE",  # repo uses ScriptPubkey
    # Nested / shared renames
    "GetRpcInfoActiveCommandsItem": "ActiveCommand",
    "GetChainstatesChainstatesItem": "ChainState",
    "GetAddrmanInfoEntry": "AddrManInfoNetwork",
    "GetRawAddrmanEntryEntry": "RawAddrManEntry",
    "GetIndexInfoEntry": "GetIndexInfoName",
    "GetMiningInfoNext": "NextBlockInfo",
    "GetNetworkInfoLocaladdressesItem": "GetNetworkInfoAddress",
    "GetNetworkInfoNetworksItem": "GetNetworkInfoNetwork",
    "GetNetTotalsUploadtarget": "UploadTarget",
    "GetTransactionLastprocessedblock": "LastProcessedBlock",
    "GetTransactionDetailsItem": "GetTransactionDetail",
    "GetBalancesLastprocessedblock": "LastProcessedBlock",
    "GetWalletInfoLastprocessedblock": "LastProcessedBlock",
    "GetWalletInfoScanning": "IGNORE",  # repo uses serde_json::Value for polymorphic bool|object
    "GetAddressesbyLabelEntry": "AddressInformation",
    "ListWalletDirWalletsItem": "ListWalletDirWallet",
    "ListDescriptorsDescriptorsItem": "DescriptorInfo",
    "ListSinceBlockTransactionsItem": "TransactionItem",
    "GetPrioritisedTransactionsEntry": "PrioritisedTransaction",
    "EnumerateSignersSignersItem": "Signers",
    "GetTxoutSetInfoBlockInfoUnspendables": "GetTxOutSetInfoUnspendables",
    "SubmitPackageTxResultsFees": "SubmitPackageTxResultssFees",
    # Simple / non-struct returns
    "GetOpenRPC": "IGNORE",  # OpenRPC patch artefact
    "GetBlockFromPeer": "IGNORE",  # returns null
    "SubmitBlockVerboseOne": "IGNORE",  # returns string
}

# Spec types to always ignore (simple returns, no struct needed).
SPEC_IGNORE: set[str] = {
    "Echo", "EchoIpc", "EchoJson", "GetNetworkHashps", "GetTxoutProof",
    "Help", "PrioritiseTransaction", "Stop", "Uptime", "ImportMempool",
    "SendmsgToPeer", "GetBlocktemplateVerboseOne",
    "GetTxspendingprevOut", "GetTxspendingprevOutItem",
    "SubmitPackageTxResults", "SubmitPackageTxResultsFees",
}

# Spec types to ignore only for certain versions.
SPEC_IGNORE_BY_VER: dict[str, set[int]] = {
    "DecodePsbtTx": {24, 25, 26, 27, 28, 29},
    "DumpTxoutSet": {24, 25},
    "GetBalancesWatchonly": {24, 25, 26, 27, 28, 29},
    "GetBlockVerboseTwo": {24, 25, 26, 27, 28},
    "GetBlockVerboseTwoTxItem": {24, 25, 26, 27, 28},
    "GetBlockVerboseThree": {24, 25, 26, 27, 28},
    "GetBlockVerboseThreeTxItem": {24, 25, 26, 27, 28},
    "GetBlockVerboseThreeTxItemVinItem": {24, 25, 26, 27, 28},
    "GetBlockVerboseThreeTxItemVinItemPrevout": {24, 25, 26, 27, 28},
    "GetRawTransactionVerboseTwo": {25, 26, 27, 28},
    "GetRawTransactionVerboseTwoVinItemPrevout": {25, 26, 27, 28},
    "GetTransactionDecoded": {24, 25, 26, 27, 28, 29},
    "GetTxoutSetInfoBlockInfo": {24, 25},
    "GetTxoutSetInfoBlockInfoUnspendables": {24, 25},
    "GetZmqnotifications": {24, 25, 26, 27, 28},
    "GetZmqnotificationsItem": {24, 25, 26, 27, 28},
    "GetunconfirmedBalance": {24, 25, 26, 27, 28, 29},
    "Importmulti": {24, 25, 26, 27, 28, 29},
    "ImportmultiItem": {24, 25, 26, 27, 28, 29},
    "ImportmultiItemError": {24, 25, 26, 27, 28, 29},
    "SubmitPackage": {24, 25},
}

# Repo types with no spec counterpart.
REPO_IGNORE: set[str] = {
    "AddMultisigAddress", "DumpWallet", "GetZmqNotifications",
    "ImportMulti", "ImportMultiEntry", "JsonRpcError", "UpgradeWallet",
    "Bip9Softfork", "Bip9SoftforkInfo", "Bip9SoftforkStatistics",
    "Softfork", "SoftforkReject",
    "GetAddressInfoLabel", "GetBalancesWatchOnly",
    "ReceiveActivity", "SpendActivity",
}

REPO_IGNORE_BY_VER: dict[str, set[int]] = {
    "GetTxSpendingPrevout": {24, 25, 26, 27, 28, 29, 30},
    "GetTxSpendingPrevoutItem": {24, 25, 26, 27, 28, 29, 30},
    "GetUnconfirmedBalance": {24, 25, 26, 27, 28, 29, 30},
    "SubmitPackageTxResult": {24, 25, 26, 27, 28, 29, 30},
    "SubmitPackageTxResultFees": {24, 25, 26, 27, 28, 29, 30},
    "DumpPrivKey": {30},
}

# Spec structs generated from "x-documentation-commentary" entries that only
# declare partial fields; extra fields in repo are expected.
COMMENTARY_STUBS: set[str] = {
    "GetBlockVerboseTwo", "GetBlockVerboseTwoTxItem",
    "GetBlockVerboseThree", "GetBlockVerboseThreeTxItem",
    "GetRawTransactionVerboseTwo", "DecodePsbtTx",
    "GetAddressInfo", "ImportDescriptorsItemError",
}

# Map lowered spec type names -> set of compatible lowered repo type names.
# Used when TYPE_BRIDGE maps to IGNORE but we still need field-level type compat.
FIELD_COMPAT: dict[str, set[str]] = {
    "decodepsbtinputsitemredeemscript": {"psbtscript"},
    "decodepsbtinputsitemwitnessscript": {"psbtscript"},
    "decodepsbtoutputsitemredeemscript": {"psbtscript"},
    "decodepsbtoutputsitemwitnessscript": {"psbtscript"},
    "decodepsbtinputsitemnonwitnessutxo": {"rawtransaction"},
    "decodepsbtinputsitemwitnessutxo": {"witnessutxo"},
    "decodepsbtinputsitemfinalscriptsig": {"scriptsig"},
    "decodepsbtinputsitembip32derivsitem": {"bip32deriv"},
    "decodepsbtoutputsitembip32derivsitem": {"bip32deriv"},
    "getrawtransactionverboseonevinitem": {"rawtransactioninput"},
    "getrawtransactionverboseonevoutitem": {"rawtransactionoutput"},
    "getrawtransactionverbosetwovinitem": {"rawtransactioninput", "rawtransactioninputwithprevout"},
    "decoderawtransactionvinitem": {"rawtransactioninput"},
    "decoderawtransactionvoutitem": {"rawtransactionoutput"},
    "getblockverbosethreeprevout": {"getblockverbosethreeprevout"},
    "getblockverbosethreetxitemvinitemprevout": {"getblockverbosethreeprevout"},
    "testmempoolacceptitemfees": {"mempoolacceptancefees"},
    "estimaterawfeeshort": {"rawfeedetail"},
    "estimaterawfeemedium": {"rawfeedetail"},
    "estimaterawfeelong": {"rawfeedetail"},
    "estimaterawfeeshortpass": {"rawfeerange"},
    "estimaterawfeeshortfail": {"rawfeerange"},
    "analyzepsbtinputsitemmissing": {"analyzepsbtinputmissing"},
    "decodepsbttx": {"rawtransaction"},
    "getdeploymentinfodeploymentsbip9": {"bip9info"},
    "importdescriptorsitemerror": {"serde_json::value"},
}

# Fields known to be extra in repo (deprecated/removed upstream).
_KNOWN_EXTRA = {"watch_only", "involves_watch_only", "add_node", "ban_score", "whitelisted", "account"}


# -- Struct parsing ----------------------------------------------------------

@dataclass
class StructField:
    name: str
    type_: str
    serde_rename: str | None = None

    @property
    def json_key(self) -> str:
        return (self.serde_rename or self.name).lower().replace("_", "")


@dataclass
class StructDef:
    name: str
    fields: list[StructField] = field(default_factory=list)
    is_tuple: bool = False
    tuple_type: str | None = None
    line: int = 0
    source: str = ""

    def field_by_key(self, key: str) -> StructField | None:
        k = key.lower().replace("_", "")
        return next((f for f in self.fields if f.json_key == k), None)


def parse_structs(text: str, source: str = "") -> dict[str, StructDef]:
    structs: dict[str, StructDef] = {}
    lines = text.splitlines()
    i = 0

    while i < len(lines):
        s = lines[i].strip()
        if not s or s.startswith("//") or s.startswith("#["):
            i += 1
            continue

        nc = re.sub(r"//.*$", "", lines[i])

        # Tuple struct (single line)
        m = re.match(r"^pub struct (\w+)\s*\((.*)\)\s*;?\s*$", nc)
        if m:
            structs[m.group(1)] = StructDef(m.group(1), is_tuple=True,
                                             tuple_type=m.group(2).strip(), line=i+1, source=source)
            i += 1
            continue

        # Tuple struct (multi-line)
        m = re.match(r"^pub struct (\w+)\s*\(\s*$", lines[i])
        if m:
            name, start, types = m.group(1), i + 1, []
            i += 1
            while i < len(lines):
                l = lines[i].strip()
                if l == ");" or l.endswith(");"):
                    inline = l.rstrip(");").strip()
                    if inline:
                        fm = re.match(r"(?:pub(?:\([^)]*\))?\s+)?(.+?)\s*,?\s*$", inline)
                        if fm: types.append(fm.group(1).strip())
                    break
                if l and not l.startswith("///") and not l.startswith("#["):
                    fm = re.match(r"(?:pub(?:\([^)]*\))?\s+)?(.+?)\s*,?\s*$", l)
                    if fm: types.append(fm.group(1).strip())
                i += 1
            structs[name] = StructDef(name, is_tuple=True, tuple_type=", ".join(types) or None,
                                       line=start, source=source)
            i += 1
            continue

        # Regular struct
        m = re.match(r"^pub struct (\w+)\s*\{", lines[i])
        if m:
            name, start = m.group(1), i + 1
            fields: list[StructField] = []
            rename: str | None = None
            i += 1
            while i < len(lines):
                cur = lines[i].strip()
                if cur == "}" or (cur.startswith("pub struct ") and "{" in cur):
                    break
                if not cur or cur.startswith("///"):
                    i += 1
                    continue
                rm = re.search(r'#\[serde\(rename\s*=\s*"([^"]+)"', cur)
                if rm:
                    rename = rm.group(1)
                    i += 1
                    continue
                if cur.startswith("#["):
                    i += 1
                    continue
                cnc = re.sub(r"//.*$", "", cur)
                fm = re.match(r"pub\s+(\w+):\s*(.+?),?\s*$", cnc)
                if fm:
                    ft = fm.group(2).rstrip(",")
                    fields.append(StructField(fm.group(1), ft, rename))
                    rename = None
                i += 1
            if i < len(lines) and lines[i].strip() == "}":
                i += 1
            structs[name] = StructDef(name, fields, line=start, source=source)
            continue

        i += 1
    return structs


# -- Type compatibility ------------------------------------------------------

def _norm(t: str) -> str:
    t = t.replace(" ", "").lower()
    t = re.sub(r"\b(i64|u64|i32|u32|usize|isize)\b", "int", t)
    t = re.sub(r"\b(f64|f32)\b", "float", t)
    return t.replace("std::collections::", "")


def _names_compat(a: str, b: str) -> bool:
    if a == b:
        return True
    for sfx in ("item", "entry", "sitem", "sentry"):
        a = a.removesuffix(sfx)
        b = b.removesuffix(sfx)
    return a in b or b in a


def _check_compat(spec_key: str, r: str, s: str) -> bool:
    """Check FIELD_COMPAT and TYPE_BRIDGE for a lowered spec type key."""
    if spec_key in FIELD_COMPAT:
        if any(c in r for c in FIELD_COMPAT[spec_key]):
            return True
    return False


def types_compatible(repo_t: str, spec_t: str) -> bool:
    r, s = _norm(repo_t), _norm(spec_t)
    if r == s:
        return True
    if {r, s} <= {"int", "float"}:
        return True

    # Vec inner types
    vr, vs = re.match(r"vec<(\w+)>", r), re.match(r"vec<(\w+)>", s)
    if vr and vs:
        ir, is_ = vr.group(1), vs.group(1)
        for sk, rv in TYPE_BRIDGE.items():
            if sk.lower() == is_:
                if rv != "IGNORE" and rv.lower() == ir:
                    return True
                if rv == "IGNORE" and _check_compat(is_, ir, is_):
                    return True
        if _check_compat(is_, ir, is_):
            return True
        return _names_compat(ir, is_)

    # Fixed array vs Vec
    if re.match(r"\[(\w+);\s*\d+\]", r) and "vec<serde_json::value>" in s:
        return True

    # Option wrappers
    or_, os_ = re.match(r"option<(.+)>", r), re.match(r"option<(.+)>", s)
    if or_ and os_:
        return types_compatible(or_.group(1), os_.group(1))
    if or_:
        return types_compatible(or_.group(1), spec_t)
    if os_:
        return types_compatible(repo_t, os_.group(1))

    # Map interchangeability
    if ("hashmap" in r and "btreemap" in s) or ("btreemap" in r and "hashmap" in s):
        return True

    # TYPE_BRIDGE lookup
    sm = re.search(r"\b([A-Z]\w+)\b", spec_t)
    if sm and sm.group(1) in TYPE_BRIDGE:
        target = TYPE_BRIDGE[sm.group(1)]
        if target != "IGNORE" and target.lower() in r:
            return True
        if target == "IGNORE" and _check_compat(sm.group(1).lower(), r, s):
            return True

    # Case-insensitive bridge fallback
    if not sm:
        for sk, rv in TYPE_BRIDGE.items():
            if sk.lower() == s:
                if rv != "IGNORE" and rv.lower() == r:
                    return True
                if rv == "IGNORE" and _check_compat(s, r, s):
                    return True

    # FIELD_COMPAT broad check
    for spec_key, compat_set in FIELD_COMPAT.items():
        if spec_key in s and any(c in r for c in compat_set):
            return True

    # Enum in repo vs String in spec
    if "string" in s and r not in {"string", "str"}:
        return True

    return _names_compat(r, s)


# -- Mapping builder ---------------------------------------------------------

def _build_mapping(repo: dict[str, StructDef], spec: dict[str, StructDef],
                   ver: int | None) -> tuple[dict[str, str], set[str]]:
    """Return (repo->spec mapping, ignored spec names)."""
    mapping: dict[str, str] = {}
    ignored: set[str] = set()

    for sname, target in TYPE_BRIDGE.items():
        if sname not in spec:
            continue
        if target == "IGNORE":
            ignored.add(sname)
        elif target in repo:
            if target not in mapping:
                mapping[target] = sname
            else:
                ignored.add(sname)
        elif ver is not None:
            ignored.add(sname)

    for rname in repo:
        if rname not in mapping and rname in spec:
            mapping[rname] = rname

    return mapping, ignored


# -- Comparison & report -----------------------------------------------------

def _loc(sd: StructDef | None) -> str:
    if sd is None or not sd.source or not sd.line:
        return ""
    return f"  ({sd.source}:{sd.line})"


def compare(repo: dict[str, StructDef], spec: dict[str, StructDef],
            ver: int | None = None) -> int:
    mapping, ignored = _build_mapping(repo, spec, ver)

    # Apply static + version ignores
    for n in SPEC_IGNORE:
        if n in spec:
            ignored.add(n)
    if ver is not None:
        for n, vers in SPEC_IGNORE_BY_VER.items():
            if ver in vers and n in spec:
                ignored.add(n)

    eff_repo_ign = set(REPO_IGNORE)
    if ver is not None:
        for n, vers in REPO_IGNORE_BY_VER.items():
            if ver in vers:
                eff_repo_ign.add(n)

    # Categorise
    matched = [(r, s) for r, s in mapping.items()]
    remaining_spec = set(spec) - ignored
    for _, s in matched:
        remaining_spec.discard(s)

    only_repo = sorted(r for r in repo if r not in mapping and r not in eff_repo_ign)
    only_spec = sorted(remaining_spec)
    repo_deprecated = sum(1 for r in repo if r in eff_repo_ign and r not in mapping)

    # ── Header ──
    print("=" * 70)
    print("STRUCT COMPARISON REPORT")
    print("=" * 70)
    print(f"\n📊 Summary:")
    print(f"  Repo structs:     {len(repo)}")
    print(f"  Spec structs:     {len(spec)}")
    print(f"  Matched pairs:    {len(matched)}")
    print(f"  Bridged/Ignored:  {len(ignored)}")
    print(f"  Repo deprecated:  {repo_deprecated}")
    print(f"  Only in repo:     {len(only_repo)}")
    print(f"  Only in spec:     {len(only_spec)}")

    # ── Bridge mappings ──
    bridged = [(r, s) for r, s in matched if r != s]
    if bridged:
        print(f"\n🔗 TYPE_BRIDGE mappings used ({len(bridged)}):")
        for r, s in sorted(bridged):
            print(f"  {r} ↔ {s}")

    # ── Unmatched ──
    if only_repo:
        print(f"\n🔵 Structs only in REPO ({len(only_repo)}):")
        for n in only_repo:
            print(f"  - {n}{_loc(repo.get(n))}")
    if only_spec:
        print(f"\n🟡 Structs only in SPEC ({len(only_spec)}):")
        for n in only_spec:
            print(f"  + {n}{_loc(spec.get(n))}")

    # ── Field diffs ──
    field_diffs: list = []
    significant: list = []
    commentary_note: list = []

    for rname, sname in matched:
        rs, ss = repo[rname], spec[sname]
        if rs.is_tuple or ss.is_tuple:
            continue

        rf = {f.json_key: f for f in rs.fields}
        sf = {f.json_key: f for f in ss.fields}
        is_stub = sname in COMMENTARY_STUBS

        missing = set(sf) - set(rf)
        extra = {k for k in (set(rf) - set(sf))
                 if rf[k].name not in _KNOWN_EXTRA and not is_stub}
        tdiffs = []
        for k in set(rf) & set(sf):
            if not types_compatible(rf[k].type_, sf[k].type_):
                tdiffs.append((rf[k].name, rf[k].type_, sf[k].type_))

        # Track commentary stub inherited fields
        if is_stub and (set(rf) - set(sf)):
            suppressed = sorted(
                rf[n].name for n in (set(rf) - set(sf))
                if rf[n].name not in _KNOWN_EXTRA
            )
            if suppressed:
                commentary_note.append((rname, sname, suppressed))

        if missing or extra or tdiffs:
            md = sorted(sf[k].name for k in missing)
            ed = sorted(rf[k].name for k in extra)
            entry = (rname, sname, md, ed, tdiffs)
            field_diffs.append(entry)
            if missing or tdiffs:
                significant.append(entry)

    if field_diffs:
        print(f"\n🔄 Structs with FIELD DIFFERENCES ({len(field_diffs)}):")
        print(f"   ({len(significant)} significant, {len(field_diffs) - len(significant)} minor)")
        for rname, sname, miss, extra, tdiffs in field_diffs:
            label = rname if rname == sname else f"{rname} ↔ {sname}"
            is_sig = (rname, sname, miss, extra, tdiffs) in significant
            icon = "⚠️ " if is_sig else "ℹ️ "
            rloc = _loc(repo.get(rname)).strip()
            sloc = _loc(spec.get(sname)).strip()
            print(f"\n  {icon}{label}:")
            print(f"    repo: {rloc}")
            print(f"    spec: {sloc}")
            if miss:
                print(f"    Missing field in repo: {', '.join(miss)}")
            if extra:
                print(f"    Extra field in repo:   {', '.join(extra)}")
            for fn, rt, st in tdiffs:
                print(f"    Type diff '{fn}': repo={rt} vs spec={st}")

    # ── Commentary stubs ──
    if commentary_note:
        print(f"\n📝 x-documentation-commentary stubs ({len(commentary_note)}):")
        print(f"   These spec structs inherit fields from other componenets")
        print(f'   via Bitcoin Core documentation commentary ("Same output as").')
        print(f"   Extra fields in the repo are expected and suppressed:")
        for rname, sname, suppressed in commentary_note:
            label = rname if rname == sname else f"{rname} ↔ {sname}"
            print(f"\n    {label}:")
            print(f"      {len(suppressed)} inherited fields: {', '.join(suppressed)}")

    # ── Ignored breakdown ──
    print(f"\n⚠️  Note: {len(ignored)} spec-only structs were intentionally ignored:")
    _print_ignored_breakdown(ignored, repo, ver)

    print("\n" + "=" * 70)

    return len(only_repo) + len(only_spec)


def _print_ignored_breakdown(ignored: set[str], repo: dict[str, StructDef],
                             ver: int | None) -> None:
    """Categorise and print the ignored spec types by reason."""
    buckets: dict[str, list[tuple[str, str]]] = {
        "simple_returns": [],
        "shared_types": [],
        "empty_structs": [],
        "primitives": [],
        "aliases": [],
        "other": [],
    }

    # Parse inline comments from TYPE_BRIDGE IGNORE entries in this file
    bridge_comments: dict[str, str] = {}
    src = Path(__file__).read_text(encoding="utf-8")
    for m in re.finditer(r'"(\w+)":\s*"IGNORE",?\s*#\s*(.+?)$', src, re.MULTILINE):
        bridge_comments[m.group(1)] = m.group(2).strip()

    for name in sorted(ignored):
        if name in SPEC_IGNORE:
            buckets["simple_returns"].append((name, ""))
            continue
        if (name in SPEC_IGNORE_BY_VER and ver is not None
                and ver in SPEC_IGNORE_BY_VER[name]):
            # Version-specific ignores go under "other" or could get their own bucket
            buckets["other"].append((name, f"not in repo for v{ver}"))
            continue
        if name in TYPE_BRIDGE and TYPE_BRIDGE[name] == "IGNORE":
            comment = bridge_comments.get(name, "")
            if "psbt module" in comment or "types/src/lib.rs" in comment:
                buckets["shared_types"].append((name, comment))
            elif "empty" in comment:
                buckets["empty_structs"].append((name, comment))
            elif any(w in comment for w in ("null", "string", "bool")):
                buckets["primitives"].append((name, comment))
            elif "serde_json::Value" in comment or "polymorphic" in comment:
                buckets["primitives"].append((name, comment))
            else:
                buckets["other"].append((name, comment or "No reason documented"))
            continue
        if name in TYPE_BRIDGE:
            target = TYPE_BRIDGE[name]
            if target not in repo and ver is not None:
                buckets["other"].append((name, f"repo type '{target}' not in v{ver}"))
            else:
                buckets["aliases"].append((name, f"alias for {target} (deduplicated)"))
            continue
        buckets["other"].append((name, "Unknown reason"))

    labels = {
        "simple_returns": "🔸 RPCs not yet implemented in repo (simple return types)",
        "shared_types": "📦 Uses shared types from psbt/lib.rs",
        "empty_structs": "📭 Empty placeholder structs",
        "primitives": "🔤 Primitive return types",
        "aliases": "🔗 Duplicate aliases to same repo type",
        "other": "❓ Other",
    }
    for key, label in labels.items():
        items = buckets[key]
        if not items:
            continue
        print(f"\n  {label} ({len(items)}):")
        for name, reason in items:
            print(f"    • {name}")
            if reason:
                print(f"      - {reason}")


# -- CLI ---------------------------------------------------------------------

def main() -> int:
    ap = argparse.ArgumentParser(description="Compare spec vs repo structs")
    ap.add_argument("--repo", default="inlined.rs")
    ap.add_argument("--spec", default="generated.rs")
    ap.add_argument("--all", action="store_true", help="Show all diffs")
    ap.add_argument("--version", type=int, default=None)
    args = ap.parse_args()

    repo = parse_structs(Path(args.repo).read_text(), args.repo)
    spec = parse_structs(Path(args.spec).read_text(), args.spec)
    unmatched = compare(repo, spec, args.version)
    return 0


if __name__ == "__main__":
    sys.exit(main())
