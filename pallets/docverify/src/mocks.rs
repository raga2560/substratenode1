#![cfg(test)]

use crate as pallet_docverify;
//use crate as pallet_template;
use sp_core::H256;
use frame_support::{parameter_types, ord_parameter_types};
use frame_support::traits::{StorageMapShim};

use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
// use frame_system as system;

/// Balance of an account.
pub type Balance = u128;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TestingPallet: pallet_docverify::{Pallet, Call, Storage, Event<T>},
        Identity: pallet_studentid::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},

	}
);

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = StorageMapShim<
    pallet_balances::Account<Test>,
    frame_system::Provider<Test>,
    Self::AccountId,
    pallet_balances::AccountData<Balance>,>;
    type WeightInfo = ();
}


parameter_types! {
    pub const BasicDeposit: u64 = 10;
    pub const FieldDeposit: u64 = 10;
    pub const SubAccountDeposit: u64 = 10;
    pub const MaxSubAccounts: u32 = 2;
    pub const MaxUseridentities: u32 = 2;
    pub const MaxAdditionalFields: u32 = 2;
    pub const MaxRegistrars: u32 = 20;
    pub const MaxEmailsize: u32 = 30;
    pub const MaxTokenid: u32 = 30;

}
ord_parameter_types! {
    pub const One: u64 = 1;
    pub const Two: u64 = 2;
    pub const MaxAccessTokenMetadata: u32 = 1;

}
//type EnsureOneOrRoot = EnsureOneOf<EnsureRoot<u64>, EnsureSignedBy<One, u64>>;
//type EnsureTwoOrRoot = EnsureOneOf<EnsureRoot<u64>, EnsureSignedBy<Two, u64>>;
impl pallet_studentid::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type Slashed = ();
    type BasicDeposit = BasicDeposit;
    type FieldDeposit = FieldDeposit;
    type SubAccountDeposit = SubAccountDeposit;
    type MaxSubAccounts = MaxSubAccounts;
    type MaxUseridentities = MaxUseridentities;
    type MaxAdditionalFields = MaxAdditionalFields;
    type MaxRegistrars = MaxRegistrars;
    type MaxEmailsize = MaxEmailsize;
    type MaxAccessTokenMetadata = MaxAccessTokenMetadata;
    type MaxTokenid = MaxTokenid;

    type RegistrarOrigin = frame_system::EnsureRoot<Self::AccountId> ;
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>  ;
    type WeightInfo = ();
}


parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter =  frame_support::traits::AllowAll;
	type BlockWeights = ();
    type OnSetCode = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

/* impl pallet_template::Config for Test {
	type Event = Event;
}
*/

parameter_types! {
    pub const MaxValue: u32 = 50;
}


impl pallet_docverify::Config for Test {
    type Event = Event;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

