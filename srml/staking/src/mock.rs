#![cfg(test)]

use primitives::BuildStorage;
use primitives::{traits::{IdentityLookup, Convert}, testing::{Digest, DigestItem, Header}, Perbill};
use primitives::testing::{ UintAuthorityId, ConvertUintAuthorityId};
use substrate_primitives::{H256, Blake2Hasher};
use srml_support::impl_outer_origin;
use crate::{GenesisConfig, Module, Trait, StakerStatus};

impl_outer_origin!{
	pub enum Origin for Test {}
}
pub type AccountIdType = u64;

/// Simple structure that exposes how u64 currency can be represented as... u64.
pub struct CurrencyToVoteHandler;
impl Convert<u64, u64> for CurrencyToVoteHandler {
	fn convert(x: u64) -> u64 { x }
}
impl Convert<u128, u64> for CurrencyToVoteHandler {
	fn convert(x: u128) -> u64 {
		x as u64
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;

impl system::Trait for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = ::primitives::traits::BlakeTwo256;
	type Digest = Digest;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type Log = DigestItem;
}

impl timestamp::Trait for Test {
	type Moment = u64;
	type OnTimestampSet = ();
}

impl consensus::Trait for Test {
	type Log = DigestItem;
	type SessionKey = UintAuthorityId;
	type InherentOfflineReport = ();
}

impl ring::Trait for Test {
	type Balance = u64;
	type OnFreeBalanceZero = ();
	type OnNewAccount = ();
	type Event = ();
	type TransactionPayment = ();
	type TransferPayment = ();
	type DustRemoval = ();
}

impl kton::Trait for Test {
	type Balance = u64;
	type Currency = ring::Module<Self>;
	type Event = ();
	type SystemPayment = ();
	type SystemRefund = ();
}

impl session::Trait for Test {
	type ConvertAccountIdToSessionKey = ConvertUintAuthorityId;
	type OnSessionChange = Staking;
	type Event = ();
}

impl Trait for Test {
	type Currency = kton::Module<Self>;
	type RewardCurrency = ring::Module<Self>;
	type CurrencyToVote = CurrencyToVoteHandler;
	type OnRewardMinted = ();
	type Event = ();
	type Slash = ();
	type Reward = ();

}

pub struct ExtBuilder {
	existential_deposit: u64,
	session_length: u64,
	sessions_per_era: u64,
	current_era: u64,
	reward: u64,
	validator_pool: bool,
	nominate: bool,
	validator_count: u32,
	minimum_validator_count: u32,
	fair: bool,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			existential_deposit: 0,
			session_length: 1,
			sessions_per_era: 1,
			current_era: 0,
			reward: 10,
			validator_pool: false,
			nominate: true,
			validator_count: 2,
			minimum_validator_count: 0,
			fair: true
		}
	}
}

impl ExtBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}


	pub fn build(self) -> runtime_io::TestExternalities<Blake2Hasher> {

		let (mut t, mut c) = system::GenesisConfig::<Test>::default().build_storage().unwrap();
		let balance_factor = if self.existential_deposit > 0 {
			1000
		} else {
			1
		};

		let _ = timestamp::GenesisConfig::<Test> {
			minimum_period: 5,
		}.assimilate_storage(&mut t, &mut c);

		let _ = ring::GenesisConfig::<Test> {
			balances: vec![
				(1, 10 * balance_factor),
				(2, 20 * balance_factor),
				(3, 300 * balance_factor),
				(4, 400 * balance_factor),
				(10, balance_factor),
				(11, balance_factor * 1000), // 1 m
				(20, balance_factor),
				(21, balance_factor * 2000), // 2 m
				(30, balance_factor),
				(31, balance_factor * 2000), // 2 m
				(40, balance_factor),
				(41, balance_factor * 2000), // 2 m
				(100, 2000000 * balance_factor),
				(101, 2000000 * balance_factor),
				(102, 2000000 * balance_factor),
				(103, 2000000 * balance_factor),
				(104, 2000000 * balance_factor),
			],
			transaction_base_fee: 0,
			transaction_byte_fee: 0,
			existential_deposit: self.existential_deposit,
			transfer_fee: 0,
			creation_fee: 0,
			vesting: vec![],
		}.assimilate_storage(&mut t, &mut c);

		let _ = kton::GenesisConfig::<Test> {
			sys_account: 42,
			claim_fee: balance_factor,
			balances: vec![],
			vesting: vec![],
		}.assimilate_storage(&mut t, &mut c);

		let _ = consensus::GenesisConfig::<Test>{
			code: vec![],
			authorities: vec![],
		}.assimilate_storage(&mut t, &mut c);

		let _ = session::GenesisConfig::<Test>{
			session_length: self.session_length,
			// NOTE: if config.nominate == false then 100 is also selected in the initial round.
			validators: if self.validator_pool { vec![10, 20, 30, 40] }  else { vec![10, 20] },
			keys: vec![],
		}.assimilate_storage(&mut t, &mut c);

		let _ = GenesisConfig::<Test>{
			sessions_per_era: self.sessions_per_era,
			current_era: self.current_era,
			stakers: if self.validator_pool {
				vec![
					(100, 1, balance_factor * 10, StakerStatus::<AccountIdType>::Validator),
					(101, 2, balance_factor * if self.fair { 10 } else { 20 }, StakerStatus::<AccountIdType>::Validator),
					(102, 3, balance_factor * 20, if self.validator_pool { StakerStatus::<AccountIdType>::Validator } else { StakerStatus::<AccountIdType>::Idle }),
					// nominator
					(103, 4, balance_factor * 20, if self.nominate { StakerStatus::<AccountIdType>::Nominator(vec![1, 2]) } else { StakerStatus::<AccountIdType>::Nominator(vec![]) })
				]
			} else {
				vec![
					(100, 1, balance_factor * 1000, StakerStatus::<AccountIdType>::Validator),
					(101, 2, balance_factor * if self.fair { 1000 } else { 2000 }, StakerStatus::<AccountIdType>::Validator),
					(102, 3, 1, StakerStatus::<AccountIdType>::Validator),
					// nominator
					(103, 4, balance_factor * 500, if self.nominate { StakerStatus::<AccountIdType>::Nominator(vec![11, 21]) } else { StakerStatus::<AccountIdType>::Nominator(vec![]) })
				]
			},
			validator_count: self.validator_count,
			minimum_validator_count: self.minimum_validator_count,
			bonding_duration: self.sessions_per_era * self.session_length * 3,
			session_reward: Perbill::from_millionths((1000000 * self.reward / balance_factor) as u32),
			offline_slash: Perbill::from_percent(5),
			current_session_reward: self.reward,
			offline_slash_grace: 0,
			invulnerables: vec![],
		}.assimilate_storage(&mut t, &mut c);
		let _ = timestamp::GenesisConfig::<Test>{
			minimum_period: 5,
		}.assimilate_storage(&mut t, &mut c);
		t.into()
	}



}


pub type System = system::Module<Test>;
pub type Ring = ring::Module<Test>;
pub type Timestamp = timestamp::Module<Test>;
pub type Kton = kton::Module<Test>;
pub type Session = session::Module<Test>;
pub type Staking = Module<Test>;