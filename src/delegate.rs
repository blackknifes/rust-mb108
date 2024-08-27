use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

pub trait IntoDelegateParam<'a, DEST> {
    fn into_param(&'a self) -> DEST;
}

impl<'a, DEST> IntoDelegateParam<'a, DEST> for DEST
where
    DEST: Clone,
{
    fn into_param(&'a self) -> DEST {
        self.clone()
    }
}

impl<'life0, 'life1, DEST> IntoDelegateParam<'life0, &'life1 DEST> for DEST
where
    'life0: 'life1,
{
    fn into_param(&'life0 self) -> &'life1 DEST {
        self
    }
}

pub enum Callback<FNONCE, FN> {
    Once(Box<FNONCE>),
    Func(Box<FN>),
}

pub struct DelegateInner<FN> {
    sequence: usize,
    callbacks: BTreeMap<usize, Box<FN>>,
}

impl<FN> DelegateInner<FN> {
    fn remove(&mut self, id: usize) -> Option<Box<FN>> {
        self.callbacks.remove(&id)
    }

    pub fn add(&mut self, func: FN) -> usize {
        let boxed = Box::new(func);
        self.sequence = self.sequence + 1;
        self.callbacks.insert(self.sequence, boxed);
        self.sequence
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<usize, Box<FN>> {
        self.callbacks.iter()
    }

    pub fn values(&self) -> std::collections::btree_map::Values<usize, Box<FN>> {
        self.callbacks.values()
    }
}

#[macro_export]
macro_rules! DefineMulticastDelegate {
    //同步委托
    ($name:ident, ($($param_name:ident: $param_type:ty),*) -> () $(, $trait1:ident $(+ $traits:ident)*)? ) => {
        DefineMulticastDelegate!($name, ($($param_name: $param_type),*) $(, $trait1 $(+ $traits)*)?);
    };

    //同步委托
    ($name:ident, ($($param_name:ident: $param_type:ty),*) $(,$trait1:ident $(+ $traits:ident)*)? ) => {
        #[derive(Default)]
        pub struct $name {
            sequence: usize,
            callbacks: std::collections::BTreeMap<usize, Box<
                dyn Fn($($param_type),*) ->
                    $crate::error::Result<()> + 'static $(+ $trait1 $(+ $traits)*)?
            >>
        }

        impl $name {
            pub fn add<FN>(&mut self, func: FN) -> usize
            where
                FN: Fn($($param_type), *) ->
                    $crate::error::Result<()> + 'static $(+ $trait1 $(+ $traits)*)?,
            {
                self.sequence = self.sequence + 1;
                let id = self.sequence;
                self.callbacks.insert(id, Box::new(func));
                return id;
            }

            pub fn remove(&mut self, id: usize)
            {
                self.callbacks.remove(&id);
            }

            pub fn emit(&self, $($param_name: $param_type),*) -> $crate::error::Result<()>
            {
                for callback in self.callbacks.values() {
                    callback($($param_name.into_param()), *)?;
                }
                return Ok(());
            }
        }
    };


    //异步委托
    ($name:ident, async ($($param_name:ident: $param_type:ty),*) -> () $(, $trait1:ident $(+ $traits:ident)*)? ) => {
        DefineMulticastDelegate!($name, async ($($param_name: $param_type),*) $(, $trait1 $(+ $traits)*)?);
    };
    //异步委托
    ($name:ident, async ($($param_name:ident: $param_type:ty),*) $(,$trait1:ident $(+ $traits:ident)*)? ) => {
        #[derive(Default)]
        pub struct $name {
            sequence: usize,
            callbacks: std::collections::BTreeMap<usize,
                Box<dyn Fn($($param_type),*) ->
                    std::pin::Pin<Box<
                        dyn std::future::Future<Output = $crate::error::Result<()>> + 'static $(+ $trait1 $(+ $traits)*)?
                    >> + 'static $(+ $trait1 $(+ $traits)*)?
                >
            >
        }

        impl $name {
            pub fn add<FN, FUT>(&mut self, func: FN) -> usize
            where
                FUT: std::future::Future<Output = $crate::error::Result<()>> + 'static $(+ $trait1 $(+ $traits)*)?,
                FN: Fn($($param_type), *) -> FUT + 'static $(+ $trait1 $(+ $traits)*)?,
            {
                self.sequence = self.sequence + 1;
                let id = self.sequence;
                self.callbacks.insert(id, Box::new(move |$($param_name)*| {
                    let fut = func($($param_name)*);
                    Box::pin(fut)
                }));
                return id;
            }

            pub fn remove(&mut self, id: usize)
            {
                self.callbacks.remove(&id);
            }

            pub async fn emit(&self, $($param_name: $param_type),*) -> $crate::error::Result<()>
            {
                for callback in self.callbacks.values() {
                    callback($($param_name.into_param()), *).await?;
                }
                return Ok(());
            }
        }
    };
}
