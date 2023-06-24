#![no_main]
use libfuzzer_sys::{fuzz_target, fuzz_mutator, arbitrary::Arbitrary};
use std::cell::RefCell;
use ic_stable_structures::{BoundedStorable, GrowFailed, Storable};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Blob,
    DefaultMemoryImpl, StableBTreeMap, Vec as StableVec,
};

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


fuzz_target!(|data: &[u8]| {
    println!("hi from fuzz target");
});

fuzz_mutator!(|data: &mut [u8], size: usize, max_size: usize, seed: u32| {
    let max_size = 8192;
    let d = &data;
    println!("{}", hex::encode(d));
    let mutated_size = libfuzzer_sys::fuzzer_mutate(data, size, max_size);
    println!("{} {}", size, mutated_size);
    mutated_size
});
