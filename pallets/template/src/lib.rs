#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	pub type Book<T: Config> = StorageMap<_, Twox64Concat, (T::AccountId, u32), BoundedVec<u8, MaxStringLength>, OptionQuery>;

	type MaxStringLength = ConstU32<256>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BooksStored {
			book_id: u32,
			book: BoundedVec<u8, MaxStringLength>,
			who: T::AccountId,
		},
		BooksRetrieved {
			book_id: u32,
			book: Option<BoundedVec<u8, MaxStringLength>>,
			who: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		BookNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::create_book())]
		pub fn create_book(origin: OriginFor<T>, book_id: u32, book: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bounded_string = BoundedVec::try_from(book.clone())
				.map_err(|_| Error::<T>::StorageOverflow)?;

            Book::<T>::insert((who.clone(), book_id), bounded_string.clone());

			Self::deposit_event(Event::BooksStored { book_id, book: bounded_string, who });

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn get_book(origin: OriginFor<T>, book_id: u32) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let book = Book::<T>::get((who.clone(), book_id));
			Self::deposit_event(Event::BooksRetrieved { book_id, book, who });

			Ok(().into())
		}
	}
}
