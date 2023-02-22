use std::{
    cell::RefCell,
    ops::{BitAndAssign, BitXorAssign},
    ops::{BitOrAssign, SubAssign},
};

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

    fn new_full() -> Self {
        Self(RefCell::new(Wrapper {
            _data: RoaringBitmap::full(),
        }))
    }

    /// Inserts an item into the bitmap. Returns true if the item was not already present, false otherwise.
    fn insert(&self, item: u32) -> Result<bool, Error> {
        Ok(self.0.borrow_mut()._data.insert(item))
    }

    /// Removes an item from the bitmap. Returns true if the item was present, false otherwise.
    fn remove(&self, item: u32) -> Result<bool, Error> {
        Ok(self.0.borrow_mut()._data.remove(item))
    }

    /// Returns true if the item is present in the bitmap, false otherwise.
    fn contains(&self, item: u32) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.contains(item))
    }

    /// Returns an array of all the items in the bitmap.
    fn to_vec(&self) -> Result<Vec<u32>, Error> {
        Ok(self.0.borrow()._data.iter().collect())
    }

    /// Clears the bitmap, removing all items.
    fn clear(&self) -> Result<(), Error> {
        self.0.borrow_mut()._data.clear();
        Ok(())
    }

    /// Returns the number of items in the bitmap.
    fn len(&self) -> Result<u64, Error> {
        Ok(self.0.borrow()._data.len())
    }

    /// Returns true if the bitmap is empty, false otherwise.
    fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_empty())
    }

    /// Returns true if bitmap is full, false otherwise.
    fn is_full(&self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_full())
    }

    /// Returns the maximum value in the bitmap, or nil if the bitmap is empty.
    fn max(&self) -> Result<Option<u32>, Error> {
        Ok(self.0.borrow()._data.max())
    }

    /// Returns the minimum value in the bitmap, or nil if the bitmap is empty.
    fn min(&self) -> Result<Option<u32>, Error> {
        Ok(self.0.borrow()._data.min())
    }

    /// Returns the nth integer in the bitmap, or nil if the bitmap is empty or if n is out of bounds.
    fn select(&self, item: u32) -> Result<Option<u32>, Error> {
        Ok(self.0.borrow()._data.select(item))
    }

    /// Returns true if the bitmap is disjoint with another bitmap, false otherwise.
    fn is_disjoint(&self, other: &Self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_disjoint(&other.0.borrow()._data))
    }

    /// Returns true if the bitmap is a subset of another bitmap, false otherwise.
    fn is_subset(&self, other: &Self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_subset(&other.0.borrow()._data))
    }

    /// Returns true if the bitmap is a superset of another bitmap, false otherwise.
    fn is_superset(&self, other: &Self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_superset(&other.0.borrow()._data))
    }

    /// Union the bitmap with another bitmap. Bitwise OR.
    fn union(&self, other: &Self) -> Result<(), Error> {
        Ok(self
            .0
            .borrow_mut()
            ._data
            .bitor_assign(&other.0.borrow()._data))
    }

    /// Computes the union of the bitmap with another bitmap and returns the cardinality of the result.
    fn union_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self.0.borrow()._data.union_len(&other.0.borrow()._data))
    }

    /// Intersects the bitmap with another bitmap. Bitwise AND.
    fn intersection(&self, other: &Self) -> Result<(), Error> {
        Ok(self
            .0
            .borrow_mut()
            ._data
            .bitand_assign(&other.0.borrow()._data))
    }

    /// Computes the intersection of the bitmap with another bitmap and returns the cardinality of the result.
    fn intersection_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self
            .0
            .borrow()
            ._data
            .intersection_len(&other.0.borrow()._data))
    }

    /// A difference between the two bitmaps. Bitwise AND NOT.
    fn difference(&self, other: &Self) -> Result<(), Error> {
        Ok(self
            .0
            .borrow_mut()
            ._data
            .sub_assign(&other.0.borrow()._data))
    }

    /// Computes the difference of the bitmap with another bitmap and returns the cardinality of the result.
    fn difference_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self
            .0
            .borrow()
            ._data
            .difference_len(&other.0.borrow()._data))
    }

    /// A symmetric difference between the two bitmaps. This is equivalent to the union of the two bitmaps minus the intersection. Bitwise XOR.
    fn symmetric_difference(&self, other: &Self) -> Result<(), Error> {
        Ok(self
            .0
            .borrow_mut()
            ._data
            .bitxor_assign(&other.0.borrow()._data))
    }

    /// Computes the symmetric difference of the bitmap with another bitmap and returns the cardinality of the result.
    fn symmetric_difference_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self
            .0
            .borrow()
            ._data
            .symmetric_difference_len(&other.0.borrow()._data))
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Roaring")?;
    let bitmap_class = module.define_class("Bitmap", Default::default())?;
    bitmap_class.define_singleton_method("new", function!(MutWrapper::new, 0))?;
    bitmap_class.define_singleton_method("full", function!(MutWrapper::new_full, 0))?;

    bitmap_class.define_method("insert", method!(MutWrapper::insert, 1))?;
    bitmap_class.define_alias("<<", "insert")?;
    bitmap_class.define_alias("push", "insert")?;

    bitmap_class.define_method("remove", method!(MutWrapper::remove, 1))?;
    bitmap_class.define_alias("delete", "remove")?;

    bitmap_class.define_method("contains", method!(MutWrapper::contains, 1))?;
    bitmap_class.define_alias("include?", "contains")?;
    bitmap_class.define_alias("member?", "contains")?;
    bitmap_class.define_alias("contains?", "contains")?;

    bitmap_class.define_method("to_a", method!(MutWrapper::to_vec, 0))?;

    bitmap_class.define_method("clear", method!(MutWrapper::clear, 0))?;
    bitmap_class.define_alias("reset", "clear")?;

    bitmap_class.define_method("len", method!(MutWrapper::len, 0))?;
    bitmap_class.define_alias("size", "len")?;

    bitmap_class.define_method("empty?", method!(MutWrapper::is_empty, 0))?;

    bitmap_class.define_method("full?", method!(MutWrapper::is_full, 0))?;

    bitmap_class.define_method("max", method!(MutWrapper::max, 0))?;

    bitmap_class.define_method("min", method!(MutWrapper::min, 0))?;

    bitmap_class.define_method("nth", method!(MutWrapper::select, 1))?;

    bitmap_class.define_method("disjoint?", method!(MutWrapper::is_disjoint, 1))?;

    bitmap_class.define_method("subset?", method!(MutWrapper::is_subset, 1))?;

    bitmap_class.define_method("superset?", method!(MutWrapper::is_superset, 1))?;

    bitmap_class.define_method("union", method!(MutWrapper::union, 1))?;
    bitmap_class.define_alias("|", "union")?;
    bitmap_class.define_method("union_len", method!(MutWrapper::union_len, 1))?;

    bitmap_class.define_method("intersection", method!(MutWrapper::intersection, 1))?;
    bitmap_class.define_alias("&", "intersection")?;
    bitmap_class.define_method("intersection_len", method!(MutWrapper::intersection_len, 1))?;

    bitmap_class.define_method("difference", method!(MutWrapper::difference, 1))?;
    bitmap_class.define_alias("-", "difference")?;
    bitmap_class.define_method("difference_len", method!(MutWrapper::difference_len, 1))?;

    bitmap_class.define_method(
        "symmetric_difference",
        method!(MutWrapper::symmetric_difference, 1),
    )?;
    bitmap_class.define_alias("^", "symmetric_difference")?;
    bitmap_class.define_method(
        "symmetric_difference_len",
        method!(MutWrapper::symmetric_difference_len, 1),
    )?;

    Ok(())
}
