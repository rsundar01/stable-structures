#![no_main]
use libfuzzer_sys::{fuzz_target, fuzz_mutator, arbitrary::Arbitrary, arbitrary};
use std::collections::HashMap;
use std::cell::RefCell;
use libfuzzer_sys::arbitrary::Unstructured;
use ic_stable_structures::{BoundedStorable, GrowFailed, Storable};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bldob,
    DefaultMemoryImpl, StableBTreeMap, Vec as StableVec,
};

const FUZZ_MEMORY_ID: MemoryId = MemoryId::new(0);
const KEY_SIZE: usize = 8;
const VALUE_SIZE: usize = 24;

#[derive(Arbitrary, Debug)]
enum StableBTreeOperation {
    Insert {
        key: Blob<KEY_SIZE>,
        value: Blob<VALUE_SIZE>
    },
    RemoveRandom {
        key: Blob<KEY_SIZE>
    },
    RemoveExisting {
        key: Blob<KEY_SIZE>
    }
}


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static FUZZ_STRUCT_STABLE_BTREE: RefCell<StableBTreeMap<Blob<KEY_SIZE>, Blob<VALUE_SIZE>, VirtualMemory<DefaultMemoryImpl>>> =
        MEMORY_MANAGER.with(|memory_manager| RefCell::new(StableBTreeMap::init(memory_manager.borrow().get(FUZZ_MEMORY_ID))));

    static STORE: RefCell<Vec<Blob<KEY_SIZE>>> = RefCell::new(Vec::new());

    static STORE_INDEX_MAP: RefCell<HashMap<Blob<KEY_SIZE>, usize>> = RefCell::new(HashMap::new());
}

fuzz_target!(|ops: std::vec::Vec<StableBTreeOperation>| {
FUZZ_STRUCT_STABLE_BTREE.with(|r| {
        let fuzz_struct_stable_btree = *r.borrow_mut();
        match ops {
            StableBTreeOperation::Insert { key, value } => {
                fuzz_struct_stable_btree.stable_btree.insert(key, value);
                assert_eq!(fuzz_struct_stable_btree.stable_btree.get(&key), Some(&value));
            },
            StableBTreeOperation::RemoveRandom { key } => {
                if fuzz_struct_stable_btree.stable_btree.remove(key).is_some() {
                    remove_from_store(key);
                }
            },
            StableBTreeOperation::RemoveRandom { key} => {
                if fuzz_struct_stable_btree.stable_btree.remove(key).is_none() { panic!("Inconsistent state"); }
                remove_from_store(key);
            }
        }

    })
});

fn arbitrary_remove_existing(u: &mut Unstructured) -> arbitrary::Result<Blob<KEY_SIZE>> {
    STORE.with(|v|{
        let store = v.borrow();
        match store.is_empty(){
            true => {
                u.arbitrary()
            },
            false => {
                let e = store.len() - 1;
                let index = u.int_in_range(0..=e).unwrap();
                Ok(*store.get(index).unwrap())
            }
        }
    })
}

fn remove_from_store(key: Blob<KEY_SIZE>) {
    STORE_INDEX_MAP.with(|v1| {
        let mut store_index_map = v1.borrow_mut();
        let index = store_index_map.remove(&key).unwrap();
        STORE.with(|v2| {
            let mut store = v2.borrow_mut();
            assert_eq!(store.remove(index), key);
        })
    })
}

fn add_to_store(key: Blob<KEY_SIZE>) {
    STORE.with(|v1| {
        let mut store = v1.borrow_mut();
        store.push(key);
        let index = store.len() - 1;
        assert_eq!(store.get(index), key);
        STORE_INDEX_MAP.with(|v2|{
            let mut store_index_map = v2.borrow_mut();
            store_index_map.insert(key, index);
            assert_eq!(store_index_map.get(&key), index);
        })
    })
}
