// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	pub const NANO: Balance = 1;
	pub const MICRO: Balance = 1_000 * NANO;
	pub const MILLI: Balance = 1_000 * MICRO;
	pub const COIN: Balance = 1_000 * MILLI;
}

/// Time.
pub mod time {
	use node_primitives::{BlockNumber, Moment};

	/// Since BABE is probabilistic this is the average expected block time that
	/// we are targetting. Blocks will be produced at a minimum duration defined
	/// by `SLOT_DURATION`, but some slots will not be allocated to any
	/// authority and hence no block will be produced. We expect to have this
	/// block time on average following the defined slot duration and the value
	/// of `c` configured for BABE (where `1 - c` represents the probability of
	/// a slot being empty).
	/// This value is only used indirectly to define the unit constants below
	/// that are expressed in blocks. The rest of the code should use
	/// `SLOT_DURATION` instead (like the timestamp module for calculating the
	/// minimum period).
	///
	/// If using BABE with secondary slots (default) then all of the slots will
	/// always be assigned, in which case `MILLISECS_PER_BLOCK` and
	/// `SLOT_DURATION` should have the same value.
	///
	/// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>

	// Develop
	//	pub const MILLISECS_PER_BLOCK: Moment = 1000;
	// Production
	pub const MILLISECS_PER_BLOCK: Moment = 3000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	// Develop
	//	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10;
	//	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = MINUTES;
	// Production
	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
	pub const EPOCH_DURATION_IN_SLOTS: u64 = {
		const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

		(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
	};

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	pub const SESSION_DURATION: BlockNumber = EPOCH_DURATION_IN_SLOTS as _;
	// Develop
	//	pub const SESSION_PER_ERA: sr_staking_primitives::SessionIndex = 3;
	// Production
	pub const SESSION_PER_ERA: sr_staking_primitives::SessionIndex = 5;
}

// CRITICAL NOTE: The system module maintains two constants: a _maximum_ block weight and a _ratio_
// of it yielding the portion which is accessible to normal transactions (reserving the rest for
// operational ones). `TARGET_BLOCK_FULLNESS` is entirely independent and the system module is not
// aware of if, nor should it care about it. This constant simply denotes on which ratio of the
// _maximum_ block weight we tweak the fees. It does NOT care about the type of the dispatch.
//
// For the system to be configured in a sane way, `TARGET_BLOCK_FULLNESS` should always be less than
// the ratio that `system` module uses to find normal transaction quota.
/// Fee-related.
pub mod fee {
	pub use sr_primitives::Perbill;

	/// The block saturation level. Fees will be updates based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);
}
