#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod calculator{
    
    #[ink(storage)]
   pub struct Record{
        current_value: u32,
        previous_value: u32,
        current_approx: u64
    }

    impl Record{
        
        #[ink(constructor)]
        pub fn init(init_value: u32) -> Self{
            return Self{current_value:init_value, previous_value:0, current_approx: (init_value as u64)};
        }

        #[ink(constructor)]
        pub fn default() -> Self{
            return Self::init(Default::default());
        }

        #[ink(message)]
        pub fn update_record(&mut self, current_value: u32){
            self.previous_value =  self.current_value;
            self.current_value = current_value;
            self.current_approx = current_value as u64;
        }


        #[ink(message)]
        pub fn add(&mut self, a:u32, b: u32) -> u32{
           let c = a + b;
           self.update_record(c);
           return c;
        }
    }

    #[cfg(test)]
    mod tests{

        use super::*;
        
        #[ink::test]
        pub fn init_works(){
            let record = Record::init(10);
            assert_eq!(record.current_value, 10);
            assert_eq!(record.current_approx, 10);
            assert_eq!(record.previous_value, 0);
        }

        #[ink::test]
        pub fn default_works(){
            let record = Record::default();
            assert_eq!(record.current_value, 0);
            assert_eq!(record.current_approx, 0);
            assert_eq!(record.previous_value, 0);
        }

        #[ink::test]
        pub fn add_works(){
            let mut record = Record::init(10);
            record.add(4, 5);
            assert_eq!(record.current_value, 9);
            assert_eq!(record.current_approx, 9);
            assert_eq!(record.previous_value, 10);
        }
    }
}