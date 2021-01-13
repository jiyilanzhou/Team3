#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, ensure, sp_runtime,
	StorageMap, StorageValue, Parameter,
	traits::{Randomness, Currency, ReservableCurrency, Get},
};
use frame_system::ensure_signed;
use sp_io::hashing::{blake2_128};
use sp_runtime::{DispatchError, traits::{AtLeast32Bit, Bounded, Member}};
use sp_std::{
	prelude::*,
};

type KittyIndex = u32;
#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Randomness: Randomness<Self::Hash>;
	type KittyIndex: Parameter + Member + AtLeast32Bit + Bounded + Default + Copy;
	/// The currency mechanism.
	type Currency: ReservableCurrency<Self::AccountId>;
	/// The minimum amount kitty required to reserve.
	type KittyReserve: Get<BalanceOf<Self>>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) KittyIndex => Option<Kitty>;
        pub KittiesCount get (fn kitties_count): KittyIndex;
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) KittyIndex => Option<T::AccountId>;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait>{
        KittiesCountOverFlow,
        InvalidKittyId,
        RequireDifferentParent,
        NotKittyOwner,
    }
}

decl_event! {
    pub enum Event<T> where <T as frame_system::Trait>::AccountId,{

        Created(AccountId, KittyIndex),
        Transferred(AccountId, AccountId, KittyIndex),
    }

}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        type Error = Error<T>;
        fn deposit_event() = default;

        const KittyReserve: BalanceOf<T> = T::KittyReserve::get();

        #[weight = 0]
        pub fn create(origin){
            let sender = ensure_signed(origin)?;

			T::Currency::reserve(&sender, T::KittyReserve::get())
					.map_err(|_| "locker can't afford to lock the amount requested")?;

            let kitty_id = Self::next_kitty_id()?;
            let dna = Self::random_value(&sender);

            let kitty = Kitty(dna);
            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));
        }

        #[weight = 0]
        pub fn transfer(origin, to: T::AccountId, kitty_id: KittyIndex){
            let sender = ensure_signed(origin)?;

			let kitties = Self::kitty_owner(kitty_id);
			if let Some(v) = kitties {
				ensure!(v == sender , Error::<T>::NotKittyOwner);
			}

			T::Currency::reserve(&to, T::KittyReserve::get())
					.map_err(|_| "locker can't afford to lock the amount requested")?;
			T::Currency::unreserve(&sender, T::KittyReserve::get());

            <KittyOwners<T>>::insert(kitty_id, to.clone());
            Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id))
        }

        #[weight = 0]
        pub fn breed(origin, kitty_id1: KittyIndex, kitty_id2: KittyIndex){
            let sender = ensure_signed(origin)?;

			T::Currency::reserve(&sender, T::KittyReserve::get())
					.map_err(|_| "locker can't afford to lock the amount requested")?;

            let new_kitty_id = Self::do_breed(&sender, kitty_id1, kitty_id2)?;
            Self::deposit_event(RawEvent::Created(sender,new_kitty_id));
        }
    }
}
fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
	fn insert_kitty(owner: &T::AccountId, kitty_id: KittyIndex, kitty: Kitty) {
		Kitties::insert(kitty_id, kitty);
		KittiesCount::put(kitty_id + 1);
		<KittyOwners<T>>::insert(kitty_id, owner);
	}

	fn next_kitty_id() -> Result<KittyIndex, DispatchError> {
		let kitty_id = Self::kitties_count();
		if kitty_id == KittyIndex::max_value() {
			return Err(Error::<T>::KittiesCountOverFlow.into());
		}
		Ok(kitty_id)
	}

	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}

	// fn do_breed(
	//     sender: &T::AccountId,
	//     kitty_id_1: KittyIndex,
	//     kitty_id_2: KittyIndex,
	// ) -> sp_std::result::Result<KittyIndex, DispatchError> {
	fn do_breed(sender: &T::AccountId, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> Result<KittyIndex, DispatchError> {
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));
		Ok(kitty_id)
	}

}