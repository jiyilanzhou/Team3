use crate::{Module, Trait};
use sp_core::H256;
use crate::*;
use frame_support::{impl_outer_origin, impl_outer_event, parameter_types, weights::Weight,
					traits::{OnInitialize, OnFinalize, TestRandomness},
};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;

impl_outer_origin! {
	pub enum Origin for Test where system = frame_system {}
}

mod kitties {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum Event for Test {
		frame_system<T>,
		pallet_balances<T>,
		kitties<T>,
	}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}


impl pallet_balances::Trait for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const KittyReserve: u64 = 5_000;
	pub const ExistentialDeposit: u64 = 1;
}

impl Trait for Test {
	type Event = Event;
	type Randomness = TestRandomness;
	type Currency = pallet_balances::Module<Self>;
	type KittyIndex = u32;
	type KittyReserve = KittyReserve;
}


pub type Kitties = Module<Test>;
pub type System = frame_system::Module<Test>;


pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		Kitties::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Kitties::on_initialize(System::block_number());
	}
}


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test>{
		// Total issuance will be 200 with treasury account initialized at ED.
		balances: vec![(0, 100000), (1, 100000), (2, 100000)],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}

pub fn last_event() -> RawEvent<u64> {
	System::events().into_iter().map(|r| r.event)
		.filter_map(|e| {
			if let Event::kitties(inner) = e { Some(inner) } else { None }
		})
		.last()
		.unwrap()
}