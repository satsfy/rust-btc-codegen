// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - network.
//! Types for methods found under the `== Network ==` section.
//! Auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `getaddednodeinfo`.
///
/// > getaddednodeinfo
/// >
/// > Returns information about the given added node, or all added nodes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddedNodeInfo(pub Vec<GetAddedNodeInfoItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddedNodeInfoItem {
    /// The node IP address or name (as provided to addnode)
    pub addednode: String,
    /// Only when connected = true
    pub addresses: Vec<GetAddedNodeInfoItemAddressesItem>,
    /// If connected
    pub connected: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddedNodeInfoItemAddressesItem {
    /// The bitcoin server IP and port we're connected to
    pub address: String,
    /// connection, inbound or outbound
    pub connected: String,
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

/// Result of the JSON-RPC method `getconnectioncount`.
///
/// > getconnectioncount
/// >
/// > Returns the number of connections to other nodes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetConnectionCount(pub i64);

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
    pub uploadtarget: GetNetTotalsUploadtarget,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotalsUploadtarget {
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
    pub localaddresses: Vec<GetNetworkInfoLocaladdressesItem>,
    /// true if transaction relay is requested from peers
    pub localrelay: bool,
    /// the services we offer to the network
    pub localservices: String,
    /// the services we offer to the network, in human-readable form
    pub localservicesnames: Vec<String>,
    /// whether p2p networking is enabled
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// information per network
    pub networks: Vec<GetNetworkInfoNetworksItem>,
    /// the protocol version
    pub protocolversion: i64,
    /// minimum relay fee rate for transactions in BTC/kvB
    #[serde(rename = "relayfee")]
    pub relay_fee: i64,
    /// the server subversion string
    pub subversion: String,
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
pub struct GetNetworkInfoLocaladdressesItem {
    /// network address
    pub address: String,
    /// network port
    pub port: i64,
    /// relative score
    pub score: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoNetworksItem {
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
    pub conntime: i64,
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
    pub pingtime: Option<i64>,
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
    pub servicesnames: Vec<String>,
    /// The session ID for this connection, or "" if there is none ("v2" transport protocol only).
/// 
    pub session_id: String,
    /// (DEPRECATED, returned only if config option -deprecatedrpc=startingheight is passed) The starting height (block) of the peer
    #[serde(rename = "startingheight")]
    pub start_ingheight: Option<i64>,
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

/// Result of the JSON-RPC method `setnetworkactive`.
///
/// > setnetworkactive
/// >
/// > Disable/enable all p2p network activity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetNetworkactive(pub bool);

