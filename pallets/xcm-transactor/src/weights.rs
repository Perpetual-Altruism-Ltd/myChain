

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for xcm_transactor.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn register() -> Weight;
	#[rustfmt::skip]
	fn deregister() -> Weight;
	#[rustfmt::skip]
	fn set_transact_info() -> Weight;
	#[rustfmt::skip]
	fn remove_transact_info() -> Weight;
}

/// Weights for xcm_transactor using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: XcmTransactor IndexToAccount (r:1 w:1)
	#[rustfmt::skip]
	fn register() -> Weight {
		(19_401_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor IndexToAccount (r:0 w:1)
	#[rustfmt::skip]
	fn deregister() -> Weight {
		(16_181_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:0 w:1)
	#[rustfmt::skip]
	fn set_transact_info() -> Weight {
		(19_595_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:0 w:1)
	#[rustfmt::skip]
	fn remove_transact_info() -> Weight {
		(18_691_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: XcmTransactor IndexToAccount (r:1 w:1)
	#[rustfmt::skip]
	fn register() -> Weight {
		(19_401_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor IndexToAccount (r:0 w:1)
	#[rustfmt::skip]
	fn deregister() -> Weight {
		(16_181_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:0 w:1)
	#[rustfmt::skip]
	fn set_transact_info() -> Weight {
		(19_595_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmTransactor TransactInfoWithWeightLimit (r:0 w:1)
	#[rustfmt::skip]
	fn remove_transact_info() -> Weight {
		(18_691_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
