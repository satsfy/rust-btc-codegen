// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - signer.
//! Types for methods found under the `== Signer ==` section.
//! Auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

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

