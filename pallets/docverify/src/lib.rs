#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
//use parity_scale_codec::Encode;

mod mocks;
mod tests;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, SignedExtension, Verify };
use sp_runtime::{ MultiSignature };
//	use pallet_identitysel::*;
	use sp_std::vec::Vec; // Step 3.1 will include this in `Cargo.toml`

	#[pallet::config]  // <-- Step 2. code block will replace this.
	pub trait Config: frame_system::Config + pallet_balances::Config + pallet_identitysel::Config  {

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;


	}

     pub type Verifier<T> = <T as frame_system::Config>::AccountId;
     pub type Submitter<T> = <T as frame_system::Config>::AccountId;
     pub type DocumentHash = Vec<u8>;
     pub type DocumentStatus = Vec<u8>;
     pub type DocumentLink = Vec<u8>;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
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
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber  ), ValueQuery>;

	#[pallet::storage] // <-- Step 5. code block will replace this.
	pub(super) type P2shaddress<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::storage] // <-- Step 5. code block will replace this.
	pub(super) type Documentverify<T: Config> = StorageMap<_, Blake2_128Concat, DocumentHash, (Submitter<T> , Verifier<T>, DocumentLink, DocumentStatus ), ValueQuery>;

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
		pub fn document_submit_sel31(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
//           let signer : AccountId = <<origin as Verify>::Signer as IdentifyAccount>::AccountId;
			let sender = ensure_signed(origin)?;

        //origin.sign(proof);

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
		pub fn document_verify_sel32(
			verifier: OriginFor<T>,
			dochash: Vec<u8>,
		) -> DispatchResult {

            /*
             *
             * Checks if verifier has rights to verify
             * Gets the document hash
             * Then verifies
             * Note:- Document link can be accessed outside this function.
             * Some random person cannot verify
             * */

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn document_reject_sel33(
			verifier: OriginFor<T>,
			dochash: Vec<u8>,
		) -> DispatchResult {

            /*
             *
             * Checks if verifier has rights to verify
             * Gets the document hash
             * Then verifies
             * Note:- Document link can be accessed outside this function.
             *
             * */

			Ok(())
		}


			// Check that the extrinsic was signed and get the signer.
/*
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

        */
        /*
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
            let _total_balance1 = pallet_balances::Pallet::<T>::locks(sender.clone());
            //let _total_balance1 = pallet_balances::total_issuance();


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

        */

		#[pallet::weight(20_000)]
		pub fn create_nft(origin  : OriginFor<T>, 
                    metadata: Vec<u8> , 
                    data: () ) -> DispatchResult{
            let service: Vec<u8> = b"moodle".to_vec();
			let id = pallet_identitysel::Pallet::<T>::check_web3access_sel18(origin.clone(), service);

			let who = ensure_signed(origin.clone())?;
			//let id = pallet_identitysel::Pallet::<T>::set_account_id(origin, 1, who.clone());
			// let res = orml_nft::Pallet::<T>::create_class(&who,vec![1],() as <T as orml_nft::Config>::ClassData);
//             let xx : BoundedVec<_,  pallet_identitysel::Pallet::<T>::Config::MaxEmailsize> = metadata.clone().try_into().unwrap();

			let id = pallet_identitysel::Pallet::<T>::identity(who.clone());

 //           id.accountId =  who.clone();
//            <pallet_identitysel::Pallet::<T> as pallet_identitysel::Pallet::<T>::Config>::Identity1Of::insert(&who, id);


			//let res1 = pallet_identitysel::MaxEmailsize;
            //IdentityOf

			Self::deposit_event(Event::TokenIssuedBy(who));
			Ok(())
		}



  }
}
