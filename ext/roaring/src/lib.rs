use std::cell::RefCell;

use magnus::{define_module, function, method, prelude::*, Error};
use roaring::RoaringBitmap;

struct Wrapper {
    _data: roaring::RoaringBitmap,
}

#[magnus::wrap(class = "Roaring::Bitmap")]

struct MutWrapper(RefCell<Wrapper>);

impl MutWrapper {
    fn new() -> Self {
        Self(RefCell::new(Wrapper {
            _data: RoaringBitmap::new(),
        }))
    }

    fn insert(&self, item: u32) -> Result<bool, Error> {
        Ok(self.0.borrow_mut()._data.insert(item))
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Roaring")?;
    let bitmap_class = module.define_class("Bitmap", Default::default())?;
    bitmap_class.define_singleton_method("new", function!(MutWrapper::new, 0))?;
    bitmap_class.define_method("insert", method!(MutWrapper::insert, 1))?;
    Ok(())
}
