// SPDX-License-Identifier: CC0-1.0

//! Identifier conversion: turn JSON-RPC method names and field names into idiomatic Rust idents.
//!
//! Bitcoin Core's RPC names are all-lowercase compounds (`getblockheader`, `bestblockhash`).
//! Splitting them into idiomatic Rust requires a curated word list — there's no rule we can
//! derive from the names alone.
//!
//! Two design rules drive the contents of [`METHOD_WORDS`] and [`FIELD_WORDS`]:
//!
//! 1. **Plurals are listed only when they cannot be reconstructed by appending `s`.** For words
//!    ending in a consonant + `s` (`blocks`, `txs`, `times`), the plural is *omitted* because
//!    keeping it shadows real word boundaries — `blocks` + `tats` would otherwise win over the
//!    correct `block` + `stats` split in `getblockstats`.
//! 2. **Single-word forms preferred over compound forms.** `txout` is split as `tx` + `out` so
//!    we get `GetTxOut`, matching `corepc-types`. We pay a one-off cost for any compound that
//!    cannot decompose cleanly (`prevout` is its own entry).

/// Words used to split RPC method names like `getblockheader` into `Get` + `Block` + `Header`.
///
/// The list is sorted longest-first at module-init time so `blockchain` wins over `block`.
pub static METHOD_WORDS: &[&str] = &[
    "abandon", "abort", "activity", "add", "address", "addresses", "addrman", "all", "analyze",
    "ancestors", "api", "backup", "balance", "balances", "banned", "best", "bip125", "block",
    "blockchain", "bump", "chain", "change", "clear", "cluster", "combine", "connection",
    "control", "convert", "count", "create", "decode", "delete", "deployment", "derive",
    "descendants", "descriptor", "descriptors", "diagram", "difficulty", "dir", "disconnect",
    "display", "dump", "echo", "encrypt", "entry", "enumerate", "estimate", "fee", "filter",
    "finalize", "for", "from", "fund", "funded", "generate", "get", "group", "groupings", "hash",
    "hashps", "hd", "header", "height", "help", "hex", "import", "index", "info", "interface",
    "invalidate", "ipc", "join", "json", "key", "keys", "label", "labels", "list", "load",
    "loaded", "lock", "logging", "median", "mempool", "memory", "message", "mining", "mock",
    "multisig", "net", "network", "new", "node", "open", "orphan", "out", "package",
    "passphrase", "peer", "peers", "pool", "precious", "prevout", "prioritise", "prioritised",
    "priv", "process", "proof", "prune", "psbt", "psbts", "queue", "rate", "raw", "received",
    "recipient", "reconsider", "rescan", "restore", "rpc", "save", "scan", "scheduler", "script",
    "send", "set", "sign", "signer", "signers", "simulate", "since", "smart", "spending",
    "stamp", "state", "states", "stats", "status", "stop", "submit", "sync", "template", "test",
    "time", "timestamp", "tips", "to", "totals", "transaction", "transactions", "tx", "unload",
    "unlock", "unspent", "update", "upgrade", "uptime", "utxo", "validate", "validation",
    "verify", "wait", "wallet", "wallets", "with", "zmq",
];

/// Words used to split JSON field names like `bestblockhash` into `best_block_hash`.
pub static FIELD_WORDS: &[&str] = &[
    "absolute", "accept", "address", "addresses", "amount", "amounts", "ancestor", "ancestors",
    "automatic", "balance", "balances", "banned", "bare", "block", "burn", "bytes",
    "carrier", "chain", "chains", "chainstate", "chainstates", "chainwork", "challenge", "change",
    "changes", "cluster", "coin", "coins", "conf", "confirmations", "connections", "count",
    "counts", "current", "data", "delta", "deltas", "depth", "depths", "desc", "descendant",
    "descendants", "descriptor", "descriptors", "diagram", "difficulty", "disk", "download",
    "effective", "entries", "entry", "error", "errors", "estimated", "fee", "fees", "filter",
    "filters", "final", "finalize", "first", "full", "hash", "hashps", "hd", "header", "headers",
    "height", "heights", "hex", "included", "incremental", "index", "info", "initial",
    "interface", "internal", "key", "keys", "label", "labels", "last", "limit", "limits",
    "loaded", "lock", "locks", "locktime", "locktimes", "log", "max", "median", "mediantime",
    "mempool", "message", "messages", "min", "mining", "mock", "modified", "multisig", "name",
    "names", "nchaintx", "network", "networks", "new", "next", "node", "nodes", "nonce", "num",
    "only", "open", "orphan", "orphans", "out", "output", "outputs", "path", "paths", "peer",
    "peers", "permit", "pooled", "prev", "prevout", "previous", "prioritised", "priority",
    "processed", "progress", "proof", "proofs", "prune", "pruned", "pruneheight", "pruning",
    "pubkey", "pubkeys", "queue", "rate", "raw", "rbf", "reachable", "received", "relay",
    "required", "result", "results", "scan", "scanning", "script", "scripts", "sequence",
    "sequences", "signet", "size", "sizes", "spending", "stamp", "start", "state", "states",
    "stats", "status", "stripped", "success", "sync", "target", "targets", "template", "time",
    "timestamp", "tips", "total", "totals", "transaction", "transactions", "tx", "txid", "txids",
    "type", "type_", "types", "unbroadcast", "unconfirmed", "unlock", "unspent", "used", "utxo",
    "utxos", "validate", "validation", "value", "values", "verification", "version", "versions",
    "vsize", "wallet", "wallets", "warnings", "watch", "weight", "weights", "with", "witness",
    "work", "written",
];

/// Rust keywords that need an underscore suffix when they appear as identifiers.
const RUST_KEYWORDS: &[(&str, &str)] = &[
    ("type", "type_"),
    ("match", "match_"),
    ("ref", "ref_"),
    ("self", "self_"),
    ("mod", "mod_"),
    ("async", "async_"),
    ("await", "await_"),
    ("use", "use_"),
];

/// Convert an RPC method name to PascalCase using the [`METHOD_WORDS`] list.
///
/// Algorithm: at each position, consume the longest known word that matches; if no word
/// matches, consume the longest unmatched run of characters until the next position where
/// some word *does* match, and treat that whole run as a single piece. This means an unknown
/// token like `reconsider` produces `Reconsider`, not character soup.
pub fn method_to_pascal(name: &str) -> String {
    let mut sorted = METHOD_WORDS.to_vec();
    sorted.sort_by_key(|w| std::cmp::Reverse(w.len()));
    pascal_pieces(&name.to_ascii_lowercase(), &sorted)
}

/// Convert an RPC method name to snake_case (the Rust function-name convention).
pub fn method_to_snake(name: &str) -> String {
    let pascal = method_to_pascal(name);
    let mut out = String::with_capacity(pascal.len() + 4);
    for (i, ch) in pascal.char_indices() {
        if i > 0 && ch.is_ascii_uppercase() {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    rust_keyword_safe(&out)
}

/// Convert a JSON field name to snake_case using the [`FIELD_WORDS`] list.
///
/// Three input shapes are handled:
/// * already-separated forms (`-`/`_`) are split, lowercased and rejoined;
/// * camelCase has underscores inserted at lower→upper transitions;
/// * all-lowercase compounds use the field word list to find boundaries.
pub fn to_rust_field(name: &str) -> String {
    let cleaned = name.replace('-', "_");
    if cleaned.contains('_') {
        return rust_keyword_safe(&cleaned.to_ascii_lowercase());
    }
    let de_camel = decamel(&cleaned);
    if de_camel.contains('_') {
        return rust_keyword_safe(&de_camel);
    }
    let mut sorted = FIELD_WORDS.to_vec();
    sorted.sort_by_key(|w| std::cmp::Reverse(w.len()));
    let parts = greedy_split(&cleaned.to_ascii_lowercase(), &sorted);
    rust_keyword_safe(&parts.join("_"))
}

/// Convert a free-form name (often a JSON field) to PascalCase, used for nested-type idents.
///
/// Re-uses [`method_to_pascal`] for all-lowercase inputs so nested type names stay consistent
/// with top-level type names.
pub fn to_pascal(s: &str) -> String {
    let cleaned = s.replace('-', "_");
    let de_camel = decamel(&cleaned);
    if de_camel.contains('_') {
        return de_camel
            .split('_')
            .filter(|p| !p.is_empty())
            .map(capitalise_first)
            .collect();
    }
    if cleaned.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) && !cleaned.is_empty() {
        return method_to_pascal(&cleaned);
    }
    capitalise_first(&cleaned)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn pascal_pieces(name: &str, sorted_words: &[&str]) -> String {
    let mut out = String::new();
    let bytes = name.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if let Some(w) = match_word(name, i, sorted_words) {
            out.push_str(&capitalise_first(w));
            i += w.len();
            continue;
        }
        // No word matches at `i`. Walk forward until some word matches again.
        let mut j = i + 1;
        while j < bytes.len() && match_word(name, j, sorted_words).is_none() {
            j += 1;
        }
        out.push_str(&capitalise_first(&name[i..j]));
        i = j;
    }
    out
}

fn greedy_split(name: &str, sorted_words: &[&str]) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let bytes = name.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if let Some(w) = match_word(name, i, sorted_words) {
            parts.push(w.to_owned());
            i += w.len();
            continue;
        }
        // No word matches at `i`. Walk forward until we find the next position where some
        // word matches; everything in between is a single unrecognised piece. This is the
        // same lookahead `method_to_pascal` uses, applied here so a name like `bestblockhash`
        // splits as `best` + `block` + `hash` even though `best` isn't in the word list.
        let mut j = i + 1;
        while j < bytes.len() && match_word(name, j, sorted_words).is_none() {
            j += 1;
        }
        parts.push(name[i..j].to_owned());
        i = j;
    }
    if parts.is_empty() {
        parts.push(name.to_owned());
    }
    parts
}

fn match_word<'a>(haystack: &'a str, at: usize, sorted_words: &[&'a str]) -> Option<&'a str> {
    sorted_words
        .iter()
        .copied()
        .find(|w| haystack.len() >= at + w.len() && &haystack[at..at + w.len()] == *w)
}

fn capitalise_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

fn decamel(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            if !out.is_empty() && out.chars().last().is_some_and(|p| p.is_ascii_lowercase()) {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn rust_keyword_safe(s: &str) -> String {
    for &(kw, replacement) in RUST_KEYWORDS {
        if s == kw {
            return replacement.to_owned();
        }
    }
    s.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_pascal_simple_compounds() {
        assert_eq!(method_to_pascal("getbestblockhash"), "GetBestBlockHash");
        assert_eq!(method_to_pascal("getblockcount"), "GetBlockCount");
        assert_eq!(method_to_pascal("getblockheader"), "GetBlockHeader");
    }

    #[test]
    fn method_pascal_resists_plural_shadow() {
        // `blocks` is deliberately NOT in the word list — would shadow `block` + s-words.
        assert_eq!(method_to_pascal("getblockstats"), "GetBlockStats");
        assert_eq!(method_to_pascal("getchaintxstats"), "GetChainTxStats");
    }

    #[test]
    fn method_pascal_handles_split_compounds() {
        assert_eq!(method_to_pascal("gettxout"), "GetTxOut");
        assert_eq!(method_to_pascal("gettxoutsetinfo"), "GetTxOutSetInfo");
        assert_eq!(method_to_pascal("loadtxoutset"), "LoadTxOutSet");
    }

    #[test]
    fn method_pascal_handles_unknown_chunks() {
        // `reconsider` is not in the word list; the chunk-fallback path keeps it as one piece.
        assert_eq!(method_to_pascal("reconsiderblock"), "ReconsiderBlock");
        assert_eq!(method_to_pascal("syncwithvalidationinterfacequeue"),
                   "SyncWithValidationInterfaceQueue");
    }

    #[test]
    fn method_snake_round_trips() {
        assert_eq!(method_to_snake("getbestblockhash"), "get_best_block_hash");
        assert_eq!(method_to_snake("sendrawtransaction"), "send_raw_transaction");
        assert_eq!(method_to_snake("invalidateblock"), "invalidate_block");
        assert_eq!(method_to_snake("setmocktime"), "set_mock_time");
        assert_eq!(method_to_snake("getopenrpc"), "get_open_rpc");
    }

    #[test]
    fn fields_split_correctly() {
        assert_eq!(to_rust_field("bestblockhash"), "best_block_hash");
        assert_eq!(to_rust_field("maxfeerate"), "max_fee_rate");
        assert_eq!(to_rust_field("maxburnamount"), "max_burn_amount");
        assert_eq!(to_rust_field("verificationprogress"), "verification_progress");
        // Already snake_case stays snake_case.
        assert_eq!(to_rust_field("max_burn_amount"), "max_burn_amount");
        // camelCase gets de-camelised.
        assert_eq!(to_rust_field("maxBurnAmount"), "max_burn_amount");
    }

    #[test]
    fn fields_avoid_keyword_collisions() {
        assert_eq!(to_rust_field("type"), "type_");
        assert_eq!(to_rust_field("ref"), "ref_");
    }

    #[test]
    fn pascal_helper_uses_word_list_for_lowercase_compounds() {
        assert_eq!(to_pascal("chainstates"), "ChainStates");
        assert_eq!(to_pascal("blockheader"), "BlockHeader");
    }
}
