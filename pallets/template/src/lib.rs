#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
  }

  #[pallet::event] 
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
  /// Event emitted when a claim has been created.
  ClaimCreated { who: T::AccountId, claim: T::Hash },
  /// Event emitted when a claim is revoked by the owner.
  ClaimRevoked { who: T::AccountId, claim: T::Hash },
}

  #[pallet::error] 
  pub enum Error<T> {
	/// The claim already exists.
	AlreadyClaimed,
	/// The claim does not exist, so it cannot be revoked.
	NoSuchClaim,
	/// The claim is owned by another account, so caller can't revoke it.
	NotClaimOwner,
  }
  
  #[pallet::storage] 
  pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;


  #[pallet::call] 
  impl<T: Config> Pallet<T> {
	#[pallet::weight(0)]
	pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
	  // Check that the extrinsic was signed and get the signer.
	  // This function will return an error if the extrinsic is not signed.
	  let sender = ensure_signed(origin)?;
  
	  // Verify that the specified claim has not already been stored.
	  ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);
  
	  // Get the block number from the FRAME System pallet.
	  let current_block = <frame_system::Pallet<T>>::block_number();
  
	  // Store the claim with the sender and block number.
	  Claims::<T>::insert(&claim, (&sender, current_block));
  
	  // Emit an event that the claim was created.
	  Self::deposit_event(Event::ClaimCreated { who: sender, claim });
  
	  Ok(())
	}
  
	#[pallet::weight(0)]
	pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
	  // Check that the extrinsic was signed and get the signer.
	  // This function will return an error if the extrinsic is not signed.
	  let sender = ensure_signed(origin)?;
  
	  // Get owner of the claim, if none return an error.
	  let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;
  
	  // Verify that sender of the current call is the claim owner.
	  ensure!(sender == owner, Error::<T>::NotClaimOwner);
  
	  // Remove claim from storage.
	  Claims::<T>::remove(&claim);
  
	  // Emit an event that the claim was erased.
	  Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
	  Ok(())
	}
  }
}






// #![cfg_attr(not(feature = "std"), no_std)]

// /// Edit this file to define custom logic or remove it if it is not needed.
// /// Learn more about FRAME and the core library of Substrate FRAME pallets:
// /// <https://docs.substrate.io/reference/frame-pallets/>
// pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// #[frame_support::pallet]
// pub mod pallet {
// 	use frame_support::pallet_prelude::*;
// 	use frame_system::pallet_prelude::*;

// 	#[pallet::pallet]
// 	#[pallet::generate_store(pub(super) trait Store)]
// 	pub struct Pallet<T>(_);

// 	/// Configure the pallet by specifying the parameters and types on which it depends.
// 	#[pallet::config]
// 	pub trait Config: frame_system::Config {
// 		/// Because this pallet emits events, it depends on the runtime's definition of an event.
// 		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
// 	}

// 	// The pallet's runtime storage items.
// 	// https://docs.substrate.io/main-docs/build/runtime-storage/
// 	#[pallet::storage]
// 	#[pallet::getter(fn something)]
// 	// Learn more about declaring storage items:
// 	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
// 	pub type Something<T> = StorageValue<_, u32>;

// 	// Pallets use events to inform users when important changes are made.
// 	// https://docs.substrate.io/main-docs/build/events-errors/
// 	#[pallet::event]
// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]
// 	pub enum Event<T: Config> {
// 		/// Event documentation should end with an array that provides descriptive names for event
// 		/// parameters. [something, who]
// 		SomethingStored(u32, T::AccountId),
// 	}

// 	// Errors inform users that something went wrong.
// 	#[pallet::error]
// 	pub enum Error<T> {
// 		/// Error names should be descriptive.
// 		NoneValue,
// 		/// Errors should have helpful documentation associated with them.
// 		StorageOverflow,
// 	}

// 	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// 	// These functions materialize as "extrinsics", which are often compared to transactions.
// 	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
// 	#[pallet::call]
// 	impl<T: Config> Pallet<T> {
// 		/// An example dispatchable that takes a singles value as a parameter, writes the value to
// 		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
// 		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
// 		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			// https://docs.substrate.io/main-docs/build/origins/
// 			let who = ensure_signed(origin)?;

// 			// Update storage.
// 			<Something<T>>::put(something);

// 			// Emit an event.
// 			Self::deposit_event(Event::SomethingStored(something, who));
// 			// Return a successful DispatchResultWithPostInfo
// 			Ok(())
// 		}

// 		/// An example dispatchable that may throw a custom error.
// 		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
// 		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
// 			let _who = ensure_signed(origin)?;

// 			// Read a value from storage.
// 			match <Something<T>>::get() {
// 				// Return an error if the value has not been set.
// 				None => return Err(Error::<T>::NoneValue.into()),
// 				Some(old) => {
// 					// Increment the value read from storage; will error in the event of overflow.
// 					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
// 					// Update the value in storage with the incremented result.
// 					<Something<T>>::put(new);
// 					Ok(())
// 				},
// 			}
// 		}
// 	}
// }
