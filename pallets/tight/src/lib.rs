#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;




#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec; // Step 3.1 will include this in `Cargo.toml`

	#[pallet::config]  // <-- Step 2. code block will replace this.
	pub trait Config: frame_system::Config + pallet_balances::Config + orml_nft::Config    {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;


	}


	#[pallet::event]   // <-- Step 3. code block will replace this.
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a proof has been claimed. [who, claim]
		ClaimCreated(T::AccountId, Vec<u8>),
		/// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimRevoked(T::AccountId, Vec<u8>),

		UpdateCreated(T::AccountId, Vec<u8>),

		ClaimLocked(T::AccountId, Vec<u8>),

        PshAddressCreated(T::AccountId, Vec<u8>),

        SomethingStored(u32, T::AccountId),
		TokenIssuedBy(T::AccountId),
		TokenMinted(T::AccountId),
	}

	#[pallet::error]   // <-- Step 4. code block will replace this.
	pub enum Error<T> {
		/// The proof has already been claimed.
		ProofAlreadyClaimed,
		/// The proof does not exist, so it cannot be revoked.
		NoSuchProof,
		/// The proof is claimed by another account, so caller can't revoke it.
		NotProofOwner,

		ProofAlreadyLocked,

		ProofNotClaimed,

		InvalidLock,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage] // <-- Step 5. code block will replace this.
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::storage] // <-- Step 5. code block will replace this.
	pub(super) type P2shaddress<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::storage] // <-- Step 5. code block will replace this.
	pub(super) type Prooflocks<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, Vec<u8> ), ValueQuery>;

    /*
	#[pallet::storage] // <-- Step 5. code block will replace this.
	pub(super) type NftClassId<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, Vec<u8> ), ValueQuery>;

	#[pallet::storage] // <-- Step 5. code block will replace this.
	pub(super) type NftTokenMetaData<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, Vec<u8>   ), ValueQuery>;
*/
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
   

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]   // <-- Step 6. code block will replace this.
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;



            // Verify that the specified proof has not already been claimed.
        ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

        // Get the block number from the FRAME System pallet.
        let current_block = <frame_system::Pallet<T>>::block_number();

        // Store the proof with the sender and block number.
        Proofs::<T>::insert(&proof, (&sender, current_block));



			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated(sender, proof));

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn update_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
			sentlock: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has not already been claimed.
			if Proofs::<T>::contains_key(&proof) {

			  // Get owner of the claim.
			  let (owner, current_block) = Proofs::<T>::get(&proof);

			  let (_, lock) = Prooflocks::<T>::get(&proof);

			ensure!(sentlock == lock, Error::<T>::InvalidLock);

			  if sender != owner {
//			    Proofs::<T>::remove(&proof);
			    Proofs::<T>::insert(&proof, (&sender, current_block));
//			    Prooflocks::<T>::remove(&proof);
		        Prooflocks::<T>::insert(&proof, (&sender, &lock));
              }

            }
            else {
            //
			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&proof, (&sender, current_block));

            }

			// Emit an event that the claim was created.
			Self::deposit_event(Event::UpdateCreated(sender, proof));

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn lock_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
			lock: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;

        ensure!(!Prooflocks::<T>::contains_key(&proof), Error::<T>::ProofAlreadyLocked);
        ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::ProofNotClaimed);

			let (owner, _) = Proofs::<T>::get(&proof);

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);

		    Prooflocks::<T>::insert(&proof, (&sender, &lock));
                

			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimLocked(sender, lock));

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn create_pshaddress(
			origin: OriginFor<T>,
			pshaddress: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has not already been claimed.
			ensure!(!P2shaddress::<T>::contains_key(&pshaddress), Error::<T>::ProofAlreadyClaimed);

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the proof with the sender and block number.
			P2shaddress::<T>::insert(&pshaddress, (&sender, current_block));

			// Emit an event that the claim was created.
			Self::deposit_event(Event::PshAddressCreated(sender, pshaddress));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;

            let _total_balance = pallet_balances::Pallet::<T>::total_issuance();


			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);

			// Remove claim from storage.
			Proofs::<T>::remove(&proof);

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked(sender, proof));

			Ok(())
		}


		#[pallet::weight(20_000)]
		pub fn create_nft(origin  : OriginFor<T>, 
                    metadata: Vec<u8> , 
                    data: () ) -> DispatchResult{

			let who = ensure_signed(origin)?;
			let res = orml_nft::Pallet<T>::create_class(&who,metadata.clone(),data);
			//NftClassId::<T>::insert(&who, (&who, &metadata ));
			//NftTokenMetaData::<T>::insert(&who, (&who, metadata.clone()));
			// <NftClassId<T>>::insert(&who,res.unwrap());
			// <NftTokenMetaData<T>>::insert(&who,metadata);
			Self::deposit_event(Event::TokenIssuedBy(who));
			Ok(())
		}


/*
#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn mint_nft(origin, data: <T as orml_nft::Trait>::TokenData) -> dispatch::DispatchResult{
			let who = ensure_signed(origin)?;
			let metadata = <NftTokenMetaData<T>>::get(&who);
			let classid = <NftClassId<T>>::get(&who);
			
			let _res = <orml_nft::Module<T>>::mint(&who,classid,metadata,data);
			
			Self::deposit_event(RawEvent::TokenMinted(who));
			Ok(())
		}

*/



	}
}
