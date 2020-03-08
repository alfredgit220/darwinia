use sc_network::{
	config::{NonReservedPeerMode, TransportConfig},
	multiaddr::Protocol,
};
use sc_service::{Configuration, RuntimeGenesis};
use serde::Deserialize;
use std::iter;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::error;
use crate::params::node_key_params::NodeKeyParams;

/// Parameters used to create the network configuration.
#[derive(Clone, Debug, Deserialize, StructOpt)]
pub struct NetworkConfigurationParams {
	/// Specify a list of bootnodes.
	#[structopt(long = "bootnodes", value_name = "URL")]
	pub bootnodes: Vec<String>,

	/// Specify a list of reserved node addresses.
	#[structopt(long = "reserved-nodes", value_name = "URL")]
	pub reserved_nodes: Vec<String>,

	/// Whether to only allow connections to/from reserved nodes.
	///
	/// If you are a validator your node might still connect to other validator
	/// nodes regardless of whether they are defined as reserved nodes.
	#[structopt(long = "reserved-only")]
	pub reserved_only: bool,

	/// Specify a list of sentry node public addresses.
	#[structopt(
		long = "sentry-nodes",
		value_name = "URL",
		conflicts_with_all = &[ "sentry" ]
	)]
	pub sentry_nodes: Vec<String>,

	/// Listen on this multiaddress.
	#[structopt(long = "listen-addr", value_name = "LISTEN_ADDR")]
	pub listen_addr: Vec<String>,

	/// Specify p2p protocol TCP port.
	///
	/// Only used if --listen-addr is not specified.
	#[structopt(long = "port", value_name = "PORT")]
	pub port: Option<u16>,

	/// Forbid connecting to private IPv4 addresses (as specified in
	/// [RFC1918](https://tools.ietf.org/html/rfc1918)), unless the address was passed with
	/// `--reserved-nodes` or `--bootnodes`.
	#[structopt(long = "no-private-ipv4")]
	pub no_private_ipv4: bool,

	/// Specify the number of outgoing connections we're trying to maintain.
	#[structopt(long = "out-peers", value_name = "COUNT", default_value = "25")]
	pub out_peers: u32,

	/// Specify the maximum number of incoming connections we're accepting.
	#[structopt(long = "in-peers", value_name = "COUNT", default_value = "25")]
	pub in_peers: u32,

	/// Disable mDNS discovery.
	///
	/// By default, the network will use mDNS to discover other nodes on the
	/// local network. This disables it. Automatically implied when using --dev.
	#[structopt(long = "no-mdns")]
	pub no_mdns: bool,

	/// Maximum number of peers to ask the same blocks in parallel.
	///
	/// This allows downlading announced blocks from multiple peers. Decrease to save
	/// traffic and risk increased latency.
	#[structopt(long = "max-parallel-downloads", value_name = "COUNT", default_value = "5")]
	pub max_parallel_downloads: u32,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub node_key_params: NodeKeyParams,

	/// Experimental feature flag.
	#[structopt(long = "use-yamux-flow-control")]
	pub use_yamux_flow_control: bool,
}

impl NetworkConfigurationParams {
	/// Fill the given `NetworkConfiguration` by looking at the cli parameters.
	pub fn update_config<G, E>(
		&self,
		mut config: &mut Configuration<G, E>,
		config_path: PathBuf,
		client_id: String,
		is_dev: bool,
	) -> error::Result<()>
	where
		G: RuntimeGenesis,
	{
		config.network.boot_nodes.extend(self.bootnodes.clone());
		config.network.config_path = Some(config_path.clone());
		config.network.net_config_path = Some(config_path.clone());

		config.network.reserved_nodes.extend(self.reserved_nodes.clone());
		if self.reserved_only {
			config.network.non_reserved_mode = NonReservedPeerMode::Deny;
		}

		config.network.sentry_nodes.extend(self.sentry_nodes.clone());

		for addr in self.listen_addr.iter() {
			let addr = addr.parse().ok().ok_or(error::Error::InvalidListenMultiaddress)?;
			config.network.listen_addresses.push(addr);
		}

		if config.network.listen_addresses.is_empty() {
			let port = match self.port {
				Some(port) => port,
				None => 30333,
			};

			config.network.listen_addresses = vec![iter::once(Protocol::Ip4(Ipv4Addr::new(0, 0, 0, 0)))
				.chain(iter::once(Protocol::Tcp(port)))
				.collect()];
		}

		config.network.client_version = client_id;
		self.node_key_params.update_config(&mut config, Some(&config_path))?;

		config.network.in_peers = self.in_peers;
		config.network.out_peers = self.out_peers;

		config.network.transport = TransportConfig::Normal {
			enable_mdns: !is_dev && !self.no_mdns,
			allow_private_ipv4: !self.no_private_ipv4,
			wasm_external_transport: None,
			use_yamux_flow_control: self.use_yamux_flow_control,
		};

		config.network.max_parallel_downloads = self.max_parallel_downloads;

		Ok(())
	}
}

impl Default for NetworkConfigurationParams {
	fn default() -> Self {
		Self {
			bootnodes: vec![],
			reserved_nodes: vec![],
			reserved_only: false,
			sentry_nodes: vec![],
			listen_addr: vec![],
			port: None,
			no_private_ipv4: false,
			out_peers: 25,
			in_peers: 25,
			no_mdns: false,
			max_parallel_downloads: 5,
			node_key_params: Default::default(),
			use_yamux_flow_control: false,
		}
	}
}
