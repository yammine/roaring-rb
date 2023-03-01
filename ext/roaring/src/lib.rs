use std::{
    cell::RefCell,
    ops::{BitAnd, BitOr, BitXor, Sub},
};

use magnus::{
    block::*, define_module, function, method, prelude::*, scan_args::scan_args, typed_data::Obj,
    Error, RArray, Value,
};
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

    fn from_array(array: RArray) -> Self {
        let values = array.to_vec::<u32>().unwrap();
        Self(RefCell::new(Wrapper {
            _data: RoaringBitmap::from_iter(values),
        }))
    }

    /// Inserts an item into the bitmap. Returns true if the item was not already present, false otherwise.
    fn insert(&self, item: u32) -> Result<bool, Error> {
        Ok(self.0.borrow_mut()._data.insert(item))
    }

    /// Inserts multiple items into the bitmap.
    fn insert_many(&self, items: &[Value]) -> Result<(), Error> {
        let args = scan_args::<(), (), RArray, (), (), ()>(items)?;
        let values = args.splat.to_vec::<u32>().unwrap();
        self.0.borrow_mut()._data.extend(values);
        Ok(())
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
    fn union(&self, other: &Self) -> Result<Self, Error> {
        let lhs = &self.0.borrow()._data;
        let rhs = &other.0.borrow()._data;
        let d = lhs.bitor(rhs);

        Ok(Self(RefCell::new(Wrapper { _data: d })))
    }

    /// Computes the union of the bitmap with another bitmap and returns the cardinality of the result.
    fn union_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self.0.borrow()._data.union_len(&other.0.borrow()._data))
    }

    /// Intersects the bitmap with another bitmap. Bitwise AND.
    fn intersection(&self, other: &Self) -> Result<Self, Error> {
        let lhs = &self.0.borrow()._data;
        let rhs = &other.0.borrow()._data;
        let d = lhs.bitand(rhs);

        Ok(Self(RefCell::new(Wrapper { _data: d })))
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
    fn difference(&self, other: &Self) -> Result<Self, Error> {
        let lhs = &self.0.borrow()._data;
        let rhs = &other.0.borrow()._data;
        let d = lhs.sub(rhs);

        Ok(Self(RefCell::new(Wrapper { _data: d })))
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
    fn symmetric_difference(&self, other: &Self) -> Result<Self, Error> {
        let lhs = &self.0.borrow()._data;
        let rhs = &other.0.borrow()._data;
        let d = lhs.bitxor(rhs);
        Ok(Self(RefCell::new(Wrapper { _data: d })))
    }

    /// Computes the symmetric difference of the bitmap with another bitmap and returns the cardinality of the result.
    fn symmetric_difference_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self
            .0
            .borrow()
            ._data
            .symmetric_difference_len(&other.0.borrow()._data))
    }

    /// Returns the number of integers that are <= value. rank(u32::MAX) == len(). This is also known as the rank or rank-select idiom.
    fn rank(&self, item: u32) -> Result<u64, Error> {
        Ok(self.0.borrow()._data.rank(item))
    }

    fn each(rb_self: Obj<Self>) -> Result<Value, Error> {
        if block_given() {
            let self_struct = rb_self.get();
            let data = &self_struct.0.borrow()._data;
            let block = block_proc().unwrap();
            for i in data.iter() {
                let rparams = RArray::with_capacity(1);
                rparams.push(i).unwrap();
                match block.call::<RArray, Option<Value>>(rparams) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            Ok(*rb_self)
        } else {
            Ok(*rb_self.enumeratorize("each", ()))
        }
    }

    fn byte_size(&self) -> Result<usize, Error> {
        Ok(self.0.borrow()._data.serialized_size())
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Roaring")?;
    let bitmap_class = module.define_class("Bitmap", Default::default())?;
    bitmap_class.define_singleton_method("new", function!(MutWrapper::new, 0))?;
    bitmap_class.define_singleton_method("full", function!(MutWrapper::new_full, 0))?;
    bitmap_class.define_singleton_method("from_a", function!(MutWrapper::from_array, 1))?;

    bitmap_class.define_method("insert", method!(MutWrapper::insert, 1))?;
    bitmap_class.define_alias("<<", "insert")?;
    bitmap_class.define_alias("push", "insert")?;

    bitmap_class.define_method("insert_many", method!(MutWrapper::insert_many, -1))?;
    bitmap_class.define_alias("bulk_insert", "insert_many")?;

    bitmap_class.define_method("remove", method!(MutWrapper::remove, 1))?;
    bitmap_class.define_alias("delete", "remove")?;

    bitmap_class.define_method("contains", method!(MutWrapper::contains, 1))?;
    bitmap_class.define_alias("include?", "contains")?;
    bitmap_class.define_alias("member?", "contains")?;
    bitmap_class.define_alias("contains?", "contains")?;

    bitmap_class.define_method("to_a", method!(MutWrapper::to_vec, 0))?;

    bitmap_class.define_method("clear", method!(MutWrapper::clear, 0))?;
    bitmap_class.define_alias("reset", "clear")?;

    bitmap_class.define_method("length", method!(MutWrapper::len, 0))?;
    bitmap_class.define_alias("size", "length")?;

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

    bitmap_class.define_method("rank", method!(MutWrapper::rank, 1))?;

    bitmap_class.define_method("each", method!(MutWrapper::each, 0))?;

    bitmap_class.define_method("byte_size", method!(MutWrapper::byte_size, 0))?;

    Ok(())
}
