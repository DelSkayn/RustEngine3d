extern crate crossbeam;

use self::crossbeam::mem::epoch::{self,Owned,Atomic};
use std::sync::atomic::{Ordering,AtomicUsize};
use std::sync::Arc;

use super::metadata::Container as MetaDataContainer;

struct ContainerData<T>{
    data: Atomic<T>,
    changed: AtomicUsize,
    meta: MetaDataContainer,
}

pub struct Container<T>{
    data: Arc<ContainerData<T>>,
    // Identifier used to determin if data
    // was changed in the mean time without checking
    // the whole asset.
    changed: usize,
}

impl<T> Clone for Container<T>{
    fn clone(&self) -> Self{
        Container{
            data: self.data.clone(),
            changed: self.changed,
        }
    }
}

impl<T> Container<T>{
    pub fn new(data: T) -> Self{
        let d = ContainerData{
            data: Atomic::null(),
            changed: AtomicUsize::new(0),
            meta: MetaDataContainer::new(),
        };
        d.data.store(Some(Owned::new(data))
                     ,Ordering::Release);
        Container{
            data: Arc::new(d),
            changed: 0
        }
    }

    pub fn empty() -> Self{
        Container{
            data: Arc::new(
              ContainerData{
                  data: Atomic::null(),
                  changed: AtomicUsize::new(0),
                  meta: MetaDataContainer::new(),
              }),
            changed: 0,
        }
    }

    pub fn change(&self,data: T){
        // TODO check for memory leak.
        self.data.data.store(Some(Owned::new(data))
                             ,Ordering::Release);
        self.data.changed.fetch_add(1,Ordering::AcqRel);
    }

    pub fn meta(&self) -> &MetaDataContainer{
        &self.data.meta
    }

    pub fn changed(&self) -> bool{
        self.changed == self.data.changed.load(Ordering::Acquire)
    }

    pub fn loaded(&self) -> bool{
        self.data.data.cas(None,None,Ordering::AcqRel).is_err()
    }

    pub fn use_data<F,R>(&self,func: F) -> R
        where F: FnOnce(&T) -> R
        {
            func(*self.data.data.load(Ordering::Acquire,&epoch::pin()).unwrap())
        }
}
