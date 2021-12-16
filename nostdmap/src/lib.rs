#![no_std]
extern crate alloc;

pub mod soramap {
    use core::ops::Index;

    use alloc::vec::Vec;

    #[derive(Debug)]
    pub struct SoraMap<K, V> {
        keys: Vec<K>,
        values: Vec<V>,
    }

    impl<K, V> SoraMap<K, V>
    where
        K: PartialEq,
    {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn get(&self, key: K) -> Option<&V> {
            if let Some(key_index) = self.keys.iter().position(|k| k.eq(&key)) {
                Some(self.values.index(key_index))
            } else {
                None
            }
        }

        pub fn set(&mut self, key: K, value: V) -> Result<(), &'static str> {
            if let Some(key_index) = self.keys.iter().position(|k| k.eq(&key)) {
                self.values.insert(key_index, value);
                Ok(())
            } else if self.keys.len() != self.values.len() {
                Err("Size of keys differ from values size")
            } else {
                let index = self.keys.len();
                self.keys.insert(index, key);
                self.values.insert(index, value);
                Ok(())
            }
        }
    }

    impl<K, V> Default for SoraMap<K, V> {
        fn default() -> Self {
            Self {
                keys: Vec::<K>::new(),
                values: Vec::<V>::new(),
            }
        }
    }
}
