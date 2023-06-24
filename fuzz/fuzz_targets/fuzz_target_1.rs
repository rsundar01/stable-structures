#![no_main]
use std::fmt;
use libfuzzer_sys::{fuzz_target, fuzz_mutator};
use std::cell::RefCell;
use ic_stable_structures::{BoundedStorable, GrowFailed, Storable};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Blob,
    DefaultMemoryImpl, StableBTreeMap, Vec as StableVec
};
use arbitrary;

const FUZZ_MEMORY_ID: MemoryId = MemoryId::new(0);
const KEY_SIZE: usize = 8;
const VALUE_SIZE: usize = 24;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static FUZZ_STRUCT_STABLE_BTREE: RefCell<StableBTreeMap<[u8; KEY_SIZE], [u8; VALUE_SIZE], VirtualMemory<DefaultMemoryImpl>>> =
        MEMORY_MANAGER.with(|memory_manager| RefCell::new(StableBTreeMap::init(memory_manager.borrow().get(FUZZ_MEMORY_ID))));

}


#[derive(arbitrary::Arbitrary)]
enum StableBTreeOperation {
    Insert {
        key: [u8; KEY_SIZE],
        value: [u8; VALUE_SIZE]
    },
    RemoveRandom {
        key: [u8; KEY_SIZE]
    },
    RemoveExisting {
        key: [u8; KEY_SIZE]
    }
}

impl fmt::Debug for StableBTreeOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            StableBTreeOperation::Insert { key, value } => {
                write!(f, "Insert[ key = {}  value = {} ]", hex::encode(key.as_slice()), hex::encode(value.as_slice()))
            },
            StableBTreeOperation::RemoveRandom { key } => {
                write!(f, "RemoveRandom[ key = {} ]", hex::encode(key.as_slice()))
            },
            StableBTreeOperation::RemoveExisting { key } => {
                write!(f, "RemoveExisting[ key = {} ]", hex::encode(key.as_slice()))
            }
        }
    }
}

/*fuzz_target!(|data: &[u8]| {
    println!("Inside fuzz_target: Size of data {}", data.len());
    println!("Data: {:?}", &data[0..data.len()]);
});*/

fuzz_target!(|ops: std::vec::Vec<StableBTreeOperation>| {

    FUZZ_STRUCT_STABLE_BTREE.with(|r| {
        let mut fuzz_struct_stable_btree = r.borrow_mut();

        ops.iter().for_each(|op| {
            println!("{:?}", op);

            match op {
                StableBTreeOperation::Insert { key, value } => {
                    fuzz_struct_stable_btree.insert(key.clone(), value.clone());
                    assert_eq!(fuzz_struct_stable_btree.get(&key), Some(*value));
                },
                StableBTreeOperation::RemoveRandom { key } => {
                },
                StableBTreeOperation::RemoveExisting { key} => {
                }
            }

        });
        println!("size of the btree: {}",fuzz_struct_stable_btree.len()); 
        fuzz_struct_stable_btree.iter().for_each(|x|{
            println!("{:?}", x);
        });
    });

});

fuzz_mutator!(|data: &mut [u8], size: usize, max_size: usize, seed: u32| {
    let max_size = 8192;
    let d = &data;
    //println!("{}", hex::encode(d));
    let mutated_size = libfuzzer_sys::fuzzer_mutate(data, size, max_size);
    println!("\n{} {}", size, mutated_size);
    mutated_size
});
