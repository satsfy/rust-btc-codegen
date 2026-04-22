// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - control.
//! Types for methods found under the `== Control ==` section.
//! Auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

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
    pub lock_ed: GetMemoryInfoVerboseZeroLocked,
}

/// Information about locked memory manager
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoVerboseZeroLocked {
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

/// Return an OpenRPC document describing the RPC API.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetopenRpc {

}

/// Result of the JSON-RPC method `help`.
///
/// > help
/// >
/// > List all commands, or get help for a specified command.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Help(pub String);

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

/// Result of the JSON-RPC method `stop`.
///
/// > stop
/// >
/// > Request a graceful shutdown of Bitcoin Core.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Stop(pub String);

/// Result of the JSON-RPC method `uptime`.
///
/// > uptime
/// >
/// > Returns the total uptime of the server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Uptime(pub i64);

