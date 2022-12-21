#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod flipper {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
        owner: AccountId,
    }

    /// The Flipper error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if non-owner is trying to call.
        OnlyOwnerCanFlip,
    }
    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                owner: Self::env().caller(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::OnlyOwnerCanFlip);
            }

            self.value = !self.value;
            ink_env::debug_println!("==\nFlipper::flip called! New value: {:?}", self.value);

            Ok(())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_val(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            // println!("Test flipper: {:?}", flipper);
            assert_eq!(flipper.get_val(), false);
        }

        #[ink::test]
        fn new_works() {
            let flipper = Flipper::new(true);
            assert_eq!(flipper.get_val(), true);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        #[warn(unused_must_use)]
        fn flip_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get_val(), false);
            assert_eq!(flipper.flip(), Ok(())); // check if flip() returns Ok(())
            assert_eq!(flipper.get_val(), true);
        }

        #[ink::test]
        fn flip_fails_if_not_owner() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get_val(), false);

            let bob = AccountId::from([0x2; 32]);
            // set caller to non-owner i.e. bob
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob);
            assert_eq!(flipper.flip(), Err(Error::OnlyOwnerCanFlip));
        }
    }
}
