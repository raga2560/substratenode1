// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Identity Pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! A federated naming system, allowing for multiple registrars to be added from a specified origin.
//! Registrars can set a fee to provide identity-verification service. Anyone can put forth a
//! proposed identity for a fixed deposit and ask for review by any number of registrars (paying
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! #### For general users
//! * `set_identity` - Set the associated identity of an account; a small deposit is reserved if not
//!   already taken.
//! #### For general users with sub-identities
//! * `quit_sub` - Remove a sub-identity of an identity (called by the sub-identity).
//!
//! #### For registrars
//! * `set_fee` - Set the fee required to be paid for a judgement to be given by the registrar.
//! * `set_fields` - Set the fields that a registrar cares about in their judgements.
//! * `provide_judgement` - Provide a judgement to an identity.
//!
//! #### For super-users
//! * `add_registrar` - Add a new registrar to the system.
//! * `kill_identity` - Forcibly remove the associated identity; the deposit is lost.
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
#[cfg(test)]
mod tests;
mod types;
pub mod weights;

use codec::{Decode, Encode, MaxEncodedLen};

use sp_io::hashing::{sha2_256, blake2_128, blake2_256, twox_128, twox_256, twox_64};

use frame_support::traits::{BalanceStatus, Currency, OnUnbalanced, ReservableCurrency};
use sp_runtime::traits::{AppendZerosInput, Saturating,  AtLeast32BitUnsigned,
    AccountIdConversion,
        AtLeast32Bit,
        MaybeSerializeDeserialize,
        Member,
    StaticLookup, Zero};
use sp_runtime::{RuntimeDebug,DispatchError };

use sp_std::prelude::*;
use scale_info::TypeInfo;

pub use weights::WeightInfo;
use sp_std::{
    fmt::Debug,
    prelude::*,
};

pub use pallet::*;
pub use types::{
	Data, IdentityField, IdentityFields, IdentityInfo, IdentityInfoSel, Judgement, RegistrarIndex, RegistrarInfo,
	Registration, RegistrationSel
};

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

pub type UseridentityIndex = u32;

/// Token info
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct TokenInfo<AccountId, Data, TokenMetadataOf> {
	/// Token metadata
	pub metadata: TokenMetadataOf,
	/// Token owner
	pub owner: AccountId,
	/// Token Properties
	pub data: Data,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The amount held on deposit for a registered identity
		#[pallet::constant]
		type BasicDeposit: Get<BalanceOf<Self>>;

		/// The maximum size of a class's metadata
		type MaxAccessTokenMetadata: Get<u32>;

		/// The amount held on deposit per additional field for a registered identity.
		#[pallet::constant]
		type FieldDeposit: Get<BalanceOf<Self>>;

		/// The amount held on deposit for a registered subaccount. This should account for the fact
		/// that one storage item's value will increase by the size of an account ID, and there will
		/// be another trie item whose value is the size of an account ID plus 32 bytes.
		#[pallet::constant]
		type SubAccountDeposit: Get<BalanceOf<Self>>;

		/// The maximum number of sub-accounts allowed per identified account.
		#[pallet::constant]
		type MaxSubAccounts: Get<u32>;

		/// Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O
		/// required to access an identity, but can be pretty high.
		#[pallet::constant]
		type MaxAdditionalFields: Get<u32>;

		/// Maxmimum number of registrars allowed in the system. Needed to bound the complexity
		/// of, e.g., updating judgements.
		#[pallet::constant]
		type MaxRegistrars: Get<u32>;

		type MaxEmailsize: Get<u32>;

		type MaxTokenid: Get<u32>;

		type MaxUseridentities: Get<u32>;
		/// What to do with slashed funds.
		type Slashed: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// The origin which may forcibly set or remove a name. Root can always do this.
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		/// The origin which may add or remove registrars. Root can always do this.
		type RegistrarOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

//	 type AccountId: frame_system::Config::AccountId;

	}




    pub type TokenId<T> = BoundedVec<u8, <T as Config>::MaxTokenid> ;

    pub type Email<T> =  BoundedVec<u8, <T as  Config>::MaxEmailsize>;

	pub type TokenMetadataOf<T> = BoundedVec<u8, <T as Config>::MaxAccessTokenMetadata>;
	pub type TokenInfoOf<T> =
		TokenInfo<<T as frame_system::Config>::AccountId,  Data, TokenMetadataOf<T>>;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
    #[pallet::generate_storage_info]
	pub struct Pallet<T>(_);



	/// testing
	/// Access tokens if needed, access for which services
	/// Returns `None` if token info not set or removed.
	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	pub type Tokens<T: Config> =
		StorageMap<_, Twox64Concat,  TokenId<T>, TokenInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn emailid)]
	pub type EmailId<T: Config> =
		StorageMap<_, Twox64Concat,  T::AccountId, Email<T>>;

	/// Information that is pertinent to identify the entity behind an account.
	///
	/// TWOX-NOTE: OK ― `AccountId` is a secure hash.
	#[pallet::storage]
	#[pallet::getter(fn identity)]
	pub(super) type IdentityOf<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		Registration<BalanceOf<T>, T::MaxRegistrars, T::MaxAdditionalFields>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn identity1)]
	pub(super) type Identity1Of<T: Config> = StorageMap<
		_,
		Twox64Concat,
		BoundedVec<u8, T::MaxEmailsize> ,
		RegistrationSel<BalanceOf<T>, T::AccountId, T::MaxRegistrars, T::MaxAdditionalFields>,
		OptionQuery,
	>;

	/// The super-identity of an alternative "sub" identity together with its name, within that
	/// context. If the account is not some other account's sub-identity, then just `None`.
	#[pallet::storage]
	#[pallet::getter(fn super_of)]
	pub(super) type SuperOf<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, (T::AccountId, Data), OptionQuery>;

	/// Alternative "sub" identities of this account.
	///
	/// The first item is the deposit, the second is a vector of the accounts.
	///
	/// TWOX-NOTE: OK ― `AccountId` is a secure hash.
	#[pallet::storage]
	#[pallet::getter(fn subs_of)]
	pub(super) type SubsOf<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		(BalanceOf<T>, BoundedVec<T::AccountId, T::MaxSubAccounts>),
		ValueQuery,
	>;

	/// The set of registrars. Not expected to get very big as can only be added through a
	/// special origin (likely a council motion).
	///
	/// The index into this can be cast to `RegistrarIndex` to get a valid value.
	#[pallet::storage]
	#[pallet::getter(fn registrars)]
	pub(super) type Registrars<T: Config> = StorageValue<
		_,
		BoundedVec<Option<RegistrarInfo<BalanceOf<T>, T::AccountId>>, T::MaxRegistrars>,
		ValueQuery,
	>;


	#[pallet::storage]
	#[pallet::getter(fn useridentities)]
	pub(super) type Useridentities<T: Config> = StorageValue<
		_,
		BoundedVec<Option  <IdentityInfo<T::MaxAdditionalFields>>, T::MaxUseridentities>,
		ValueQuery,
	>;




	#[pallet::error]
	pub enum Error<T> {

        IdentityAlreadyClaimed,

        SignerNotmatching,

        ReferalFailed,

        LoginFailed,

        MaxMetadataExceeded,

        ServiceAccessFailed,

		/// Too many subs-accounts.
		TooManySubAccounts,

		TooManyUseridentities,
		/// Account isn't found.
		NotFound,
		/// Account isn't named.
		NotNamed,
		/// Empty index.
		EmptyIndex,
		/// Fee is changed.
		FeeChanged,
		/// No identity found.
		NoIdentity,
		/// Sticky judgement.
		StickyJudgement,
		/// Judgement given.
		JudgementGiven,
		/// Invalid judgement.
		InvalidJudgement,
		/// The index is invalid.
		InvalidIndex,
		/// The target is invalid.
		InvalidTarget,
		/// Too many additional fields.
		TooManyFields,
		/// Maximum amount of registrars reached. Cannot add any more.
		TooManyRegistrars,
		/// Account ID is already named.
		AlreadyClaimed,
		/// Sender is not a sub-account.
		NotSub,
		/// Sub-account isn't owned by sender.
		NotOwned,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A name was set or reset (which will remove all judgements).
		UserRegistered { who: Vec<u8>      },
		UserExists { who: Vec<u8>      },
		UserInvalid { who: Vec<u8>      },
		UserRegisterfailed { who: Vec<u8>      },

		UserWeb3registered { who: Vec<u8>      },
		UserWeb3registerfailed { who: Vec<u8>      },

		UserLoginsuccess { who: Vec<u8>, blocksession: Vec<u8>      },
		UserLoginfailed { who: Vec<u8>      },
		UserDoesnotexist { who: Vec<u8>      },

		UserWeb3loginsuccess { who: Vec<u8>, blocksession: Vec<u8>      },
		UserWeb3loginfailed { who: Vec<u8>      },
		UserWeb3doesnotexist { who: Vec<u8>      },

		IdentitySet { who: T::AccountId },
		/// A name was cleared, and the given balance returned.
		IdentityCleared { who: T::AccountId, deposit: BalanceOf<T> },
		/// A name was removed and the given balance slashed.
		IdentityKilled { who: T::AccountId, deposit: BalanceOf<T> },
		/// A judgement was asked from a registrar.
		JudgementRequested { who: T::AccountId, registrar_index: RegistrarIndex },
		/// A judgement request was retracted.
		JudgementUnrequested { who: T::AccountId, registrar_index: RegistrarIndex },
		/// A judgement was given by a registrar.
		JudgementGiven { target: T::AccountId, registrar_index: RegistrarIndex },
        
		/// A useridentity was added.
		UseridentityAdded { useridentity_index: UseridentityIndex },

		/// A registrar was added.
		RegistrarAdded { registrar_index: RegistrarIndex },
		/// A sub-identity was added to an identity and the deposit paid.
		SubIdentityAdded { sub: T::AccountId, main: T::AccountId, deposit: BalanceOf<T> },
		/// A sub-identity was removed from an identity and the deposit freed.
		SubIdentityRemoved { sub: T::AccountId, main: T::AccountId, deposit: BalanceOf<T> },
		/// A sub-identity was cleared, and the given deposit repatriated from the
		/// main identity account to the sub-identity account.
		SubIdentityRevoked { sub: T::AccountId, main: T::AccountId, deposit: BalanceOf<T> },
	}

	#[pallet::call]
	/// Identity pallet declaration.
	impl<T: Config> Pallet<T> {


        #[pallet::weight(1_000)]
		pub fn request_registration_sel11(
			origin: OriginFor<T>,
			email: Vec<u8>,
			password: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

         //   let _x2 = Pallet::<T>::test1(email.clone());
            let emailx : BoundedVec<_, T::MaxEmailsize> = email.clone().try_into().unwrap();
        
            ensure!(!Identity1Of::<T>::contains_key(&emailx), Error::<T>::IdentityAlreadyClaimed);



              let add: BoundedVec<_, T::MaxAdditionalFields> = vec![
                    (
                        Data::Raw(b"number".to_vec().try_into().unwrap()),
                        Data::Raw(10u32.encode().try_into().unwrap())
                    ),
                    (
                        Data::Raw(b"text".to_vec().try_into().unwrap()),
                        Data::Raw(b"10".to_vec().try_into().unwrap())
                    ),
                ]
                .try_into()
                .unwrap();


        let info =  IdentityInfoSel {
        display: Data::Raw(b"ten".to_vec().try_into().unwrap()),
        legal: Data::Raw(b"The Right Ordinal Ten, Esq.".to_vec().try_into().unwrap()),
        image: Data::Raw(b"The Right Ordinal Ten, Esq.".to_vec().try_into().unwrap()),
        web: Data::Raw(b"The Right Ordinal Ten, Esq.".to_vec().try_into().unwrap()),
        referalhash: Data::Raw(b"The Right Ordinal Ten, Esq.".to_vec().try_into().unwrap()),
        email: Data::Raw(email.clone().try_into().unwrap()),
        passwordhash: Data::BlakeTwo256(blake2_256(&password.clone())),
        pgp_fingerprint: None,
        account: Data::Raw(b"The Right Ordinal Ten, Esq.".to_vec().try_into().unwrap()),
        additional: add
        };


        let reg = RegistrationSel {
                    accountId:sender, 
                    info: info,
                    judgements: BoundedVec::default(),
                    deposit: Zero::zero(),
        };

            
			<Identity1Of<T>>::insert(emailx, reg);

			Ok(())
		}



        #[pallet::weight(1_000)]
		pub fn login_access_sel12(
			origin: OriginFor<T>,
			email: Vec<u8>,
			password: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

            let emailx : BoundedVec<_, T::MaxEmailsize> = email.clone().try_into().unwrap();

			let id = <Identity1Of<T>>::get(&emailx).ok_or(Error::<T>::NoIdentity)?;

            let info = id.info;
        
            //let r = blake2_128(password.clone()).unwrap();

            let passtocheck = Data::BlakeTwo256(blake2_256(&password.clone()));

            ensure!(info.passwordhash == passtocheck , Error::<T>::LoginFailed);
			Ok(())
		}


        #[pallet::weight(1_000)]
		pub fn change_password_sel13(
			origin: OriginFor<T>,
			email: Vec<u8>,
			password: Vec<u8>,
		) -> DispatchResult {

			let sender = ensure_signed(origin)?;

            let emailx : BoundedVec<_, T::MaxEmailsize> = email.clone().try_into().unwrap();

			let id = <Identity1Of<T>>::get(&emailx).ok_or(Error::<T>::NoIdentity)?;

            let mut info = id.info;

            let newpassword = Data::BlakeTwo256(blake2_256(&password.clone()));

            info.passwordhash  = newpassword;
            info.account  =  Data::Raw(b"The Right Ordinal Ten, Esq.".to_vec().try_into().unwrap());

            let reg = RegistrationSel {
                    accountId: sender,
                    info: info,
                    judgements: BoundedVec::default(),
                    deposit: Zero::zero(),

            };

            
			<Identity1Of<T>>::insert(emailx, reg);

			Ok(())
		}

        #[pallet::weight(1_000)]
		pub fn set_referal_sel12(
			origin: OriginFor<T>,
			email: Vec<u8>,
			referal: Vec<u8>,
		) -> DispatchResult {
            
			let sender = ensure_signed(origin)?;

            let emailx : BoundedVec<_, T::MaxEmailsize> = email.clone().try_into().unwrap();

			let id = <Identity1Of<T>>::get(&emailx).ok_or(Error::<T>::NoIdentity)?;

            let hashtoset = Data::Sha256(sha2_256(&referal.clone()));

            let mut info = id.info;

            info.referalhash  =  hashtoset;


            let reg = RegistrationSel {
                    accountId: sender,
                    info: info,
                    judgements: BoundedVec::default(),
                    deposit: Zero::zero(),
            };

            
			<Identity1Of<T>>::insert(emailx, reg);

			Ok(())
		}


        #[pallet::weight(1_000)]
		pub fn create_web3link_sel15(
			origin: OriginFor<T>,
			email: Vec<u8>,
			idtolink: T::AccountId,
			referal: Vec<u8>,
		) -> DispatchResult {
            // We check if the signer is same as in email-id
            // Then update account-id of that record
            // That user would have received referal key, onlythen he can link 
            // After use referal is removed
            // No more linking possible     
            // We check if the person signing is same as origin
            
			let sender = ensure_signed(origin)?;

            ensure!(sender == idtolink , Error::<T>::SignerNotmatching);

            let emailx : BoundedVec<_, T::MaxEmailsize> = email.clone().try_into().unwrap();

			let id = <Identity1Of<T>>::get(&emailx).ok_or(Error::<T>::NoIdentity)?;

            let hashtocheck = Data::Sha256(sha2_256(&referal.clone()));

            let mut info = id.info;

            ensure!(info.referalhash == hashtocheck , Error::<T>::ReferalFailed);


            // Remove referal 
            info.referalhash  =  Data::Raw(b"null".to_vec().try_into().unwrap());


            let reg = RegistrationSel {
                    accountId: sender.clone(),
                    info: info,
                    judgements: BoundedVec::default(),
                    deposit: Zero::zero(),
            };

            
			<Identity1Of<T>>::insert(emailx.clone(), reg);
            <EmailId<T>>::insert(sender, emailx);

			Ok(())
		}

        #[pallet::weight(1_000)]
		pub fn login_web3_sel16(
			origin: OriginFor<T>,
			email: Vec<u8>,
		) -> DispatchResult{
            // let key = Origin::signed(1);
            
			let sender = ensure_signed(origin)?;


            let emailx : BoundedVec<_, T::MaxEmailsize> = email.clone().try_into().unwrap();

			let id = <Identity1Of<T>>::get(&emailx).ok_or(Error::<T>::NoIdentity)?;
            ensure!(sender == id.accountId , Error::<T>::LoginFailed);

            let tokenid : TokenId<T> =   email.clone().try_into().unwrap();


			Ok(())
		}


        #[pallet::weight(1_000)]
        pub fn set_accessservice_sel17(
            origin: OriginFor<T>,
			idtoaccess: T::AccountId,
            service: Vec<u8>,
        ) -> DispatchResult {

            let sender = ensure_signed(origin)?;

            let email = <EmailId<T>>::get(idtoaccess.clone()).ok_or(Error::<T>::NoIdentity)?;

            let emailx : BoundedVec<_, T::MaxEmailsize> = email.clone().try_into().unwrap();

            let id = <Identity1Of<T>>::get(&emailx).ok_or(Error::<T>::NoIdentity)?;

            let web = Data::Raw(service.try_into().unwrap());

            let mut info = id.info;

            info.web  =  web;


            let reg = RegistrationSel {
                    accountId: id.accountId,
                    info: info,
                    judgements: BoundedVec::default(),
                    deposit: Zero::zero(),
            };


            <Identity1Of<T>>::insert(emailx, reg);

            Ok(())
        }

        #[pallet::weight(1_000)]
        pub fn check_web3access_sel18(
            origin: OriginFor<T>,
            service: Vec<u8>,
        ) -> DispatchResult{
            // let key = Origin::signed(1);

            let sender = ensure_signed(origin)?;

            let email = <EmailId<T>>::get(sender.clone()).ok_or(Error::<T>::NoIdentity)?;

            let emailx : BoundedVec<_, T::MaxEmailsize> = email.clone().try_into().unwrap();

            let id = <Identity1Of<T>>::get(&emailx).ok_or(Error::<T>::NoIdentity)?;
          
            let servicetocheck = Data::Raw(service.try_into().unwrap());
            let info = id.info;
            
            ensure!(servicetocheck == info.web , Error::<T>::ServiceAccessFailed);

            Ok(())
        }

    }
}

