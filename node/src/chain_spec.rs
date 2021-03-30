use sp_core::{Pair, Public, sr25519, ed25519};
use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature, JackBlockConfig,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Off-chain authorities
			vec![
				hex!["ae70ee67f0b51dacc666f0d17f46fbead307846682385f69bb1c9d5d77701616"].into(),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				(
					sr25519::Public::from_slice(&hex!("8c95143789a7de6b1081b8de5d2882797f4b8d7e49b7d18bed2c9dfba0093749")).into(), // node-0
					ed25519::Public::from_slice(&hex!("3ba3259226032472d23bdf929e87ca64a67a0d6bae1590026f2878f390bfe46e")).into(),
				),
				(
					sr25519::Public::from_slice(&hex!("1c012da62a15bc32bddcc446ead445e6e3db649718b75f7564e52110f9d16259")).into(), // node-1
					ed25519::Public::from_slice(&hex!("d673017b1fe89ae315440ef77fbe126a5c73dfef550e51621cf464e9d4f2f77e")).into(),
				),
			],
			// Off-chain authorities
			vec![
				hex!("8c95143789a7de6b1081b8de5d2882797f4b8d7e49b7d18bed2c9dfba0093749").into(), // node-0
				hex!("1c012da62a15bc32bddcc446ead445e6e3db649718b75f7564e52110f9d16259").into(), // node-1
			],
			// Sudo account
			sr25519::Public::from_slice(&hex!("8c95143789a7de6b1081b8de5d2882797f4b8d7e49b7d18bed2c9dfba0093749")).into(), // node-0
			// Pre-funded accounts
			vec![
				hex!("8c95143789a7de6b1081b8de5d2882797f4b8d7e49b7d18bed2c9dfba0093749").into(), // node-0
				hex!("1c012da62a15bc32bddcc446ead445e6e3db649718b75f7564e52110f9d16259").into(), // node-1
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn public_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Jackblock Testnet",
		// ID
		"jackblock_testnet",
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				(
					sr25519::Public::from_slice(&hex!("f6de5cd7b974141f359efd875f56d261ceb1c7af567fc92787e6912d1ebe875b")).into(), // rafal
					ed25519::Public::from_slice(&hex!("8425afe90642a0aa98cccdf5d49404fe1141e34b98c30516b303df5b6f3e6f36")).into(),
				),
				(
					sr25519::Public::from_slice(&hex!("286818e712a21ffb2efd5af0b38e1ddd5525cd03788b4f62f9a85429a9238078")).into(), // miko
					ed25519::Public::from_slice(&hex!("02a51935ea51f45a72f176e1f3f542fdbb790ccaf8f5b357faaa8b6882147024")).into(),
				),
				(
					sr25519::Public::from_slice(&hex!("3e0d36d4f8e06c6f92a71acd57ea6d56c5b55347c8883a172a732ff0dd24211d")).into(), // tomek
					ed25519::Public::from_slice(&hex!("76961074863c9ff54eec83b534cb2e17f1316af5e692a5d1021603884128d2c9")).into(),
				),
			],
			// Off-chain authorities
			vec![
				hex!("f6de5cd7b974141f359efd875f56d261ceb1c7af567fc92787e6912d1ebe875b").into(), // rafal
				hex!("286818e712a21ffb2efd5af0b38e1ddd5525cd03788b4f62f9a85429a9238078").into(), // miko
				hex!("3e0d36d4f8e06c6f92a71acd57ea6d56c5b55347c8883a172a732ff0dd24211d").into(), // tomek
			],
			// Sudo account
			sr25519::Public::from_slice(&hex!("f6de5cd7b974141f359efd875f56d261ceb1c7af567fc92787e6912d1ebe875b")).into(), // node-0
			// Pre-funded accounts
			vec![
				hex!("f6de5cd7b974141f359efd875f56d261ceb1c7af567fc92787e6912d1ebe875b").into(), // rafal
				hex!("286818e712a21ffb2efd5af0b38e1ddd5525cd03788b4f62f9a85429a9238078").into(), // miko
				hex!("3e0d36d4f8e06c6f92a71acd57ea6d56c5b55347c8883a172a732ff0dd24211d").into(), // tomek
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	offchain_authorities: Vec<AccountId>, // TO BE REMOVED --------------------------
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_jackblock: Some(JackBlockConfig {
			offchain_authorities,
		}),
	}
}
