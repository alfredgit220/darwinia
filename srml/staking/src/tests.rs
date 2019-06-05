// Copyright 2017-2019 Parity Technologies (UK) Ltd.
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

//! Tests for the module.

#![cfg(test)]

use super::*;
use runtime_io::with_externalities;
use phragmen;
use primitives::PerU128;
use srml_support::{assert_ok, assert_noop, assert_eq_uvec, EnumerableStorageMap};
use mock::{Ring, Kton, Session, Staking, System, Timestamp, Test, ExtBuilder, Origin};
use srml_support::traits::{Currency, ReservableCurrency};

#[inline]
fn construct_staking_env() {
	Kton::deposit(Origin::signed(100), 1000000, 12);
	Kton::deposit(Origin::signed(101), 1000000, 12);
	Kton::deposit(Origin::signed(102), 1000000, 12);
	Kton::deposit(Origin::signed(103), 1000000, 12);
}

#[test]
fn basic_work() {
	with_externalities(&mut ExtBuilder::default()
		.existential_deposit(0).build(), || {
		construct_staking_env();
		assert_eq!(Kton::free_balance(&100), 100);
//		assert_eq!(Staking::stakers(&100).total, 10);
//		assert_eq!(Staking::bonded(&101), Some(1)); // Account 11 is stashed and locked, and account 10 is the controller
//		assert_eq!(Staking::validator_count(), 2);


	});
}