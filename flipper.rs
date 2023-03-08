#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod flipper{

#[ink(storage)]     
pub struct Flipper{
    value: bool,
}

impl Flipper{

    #[ink(constructor)]
    pub fn new(init: bool) -> Self {
        return Self{value:init};
    }

    #[ink(constructor)]
    pub fn defaulta() -> Self {
        return Self{value: false};
    }

    #[ink(constructor)]
    pub fn default() -> Self {
        return Self::new(Default::default());
    }

    #[ink(message)]
    pub fn set_value(&mut self, val:bool){
        self.value = val;
    }
     
    #[ink(message)]
    pub fn flip(&mut self){
        self.value = !self.value;
    }

    #[ink(message)]
    pub fn get_flip(&self) -> bool{
        return self.value;
    }

}
    #[cfg(test)]
    mod tests{
        use super::*;

        #[ink::test]
        fn new_works(){
            let flep = Flipper::new(true);
            assert_eq!(flep.get_flip(), true);
        }

        #[ink::test]
        fn defaulta_works(){
            let flep = Flipper::defaulta();
            assert_eq!(flep.get_flip(), false);
        }
        #[ink::test]
        fn default_works(){
            let flep = Flipper::default();
            assert_eq!(flep.get_flip(), false);
        }

        #[ink::test]
        fn set_value_works(){
            let mut flep = Flipper::default();
            flep.set_value(true);
            assert_eq!(flep.get_flip(), true);
        }

        #[ink::test]
        fn flip_works(){
            let mut flep = Flipper::default();
            flep.flip();
            assert_eq!(flep.get_flip(), true);
        }
    }


}


