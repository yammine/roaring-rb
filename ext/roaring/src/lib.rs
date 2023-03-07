#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::bare_urls)]
use std::{
    cell::RefCell,
    ops::{BitAnd, BitOr, BitXor, Sub},
};

use magnus::{
    block::*, define_module, function, method, prelude::*, typed_data::Obj, DataTypeFunctions,
    Error, RArray, RString, TypedData, Value,
};
use roaring::RoaringBitmap;

struct Wrapper {
    _data: roaring::RoaringBitmap,
}

/// @yard
/// Roaring::Bitmap is a fast, compressed bitmap implementation.
#[derive(TypedData)]
#[magnus(class = "Roaring::Bitmap", free_immediately, size)]
struct MutWrapper(RefCell<Wrapper>);

impl DataTypeFunctions for MutWrapper {
    fn size(&self) -> usize {
        self.byte_size().unwrap_or(0)
    }
}

impl MutWrapper {
    /// @yard
    ///
    /// Initializes a new empty bitmap.
    ///
    /// @return [Roaring::Bitmap] A new empty bitmap.
    fn new() -> Self {
        Self(RefCell::new(Wrapper {
            _data: RoaringBitmap::new(),
        }))
    }

    /// @yard
    /// @def full
    ///
    /// Initializes a new bitmap with all bits set to 1.
    ///
    /// @example Initialize a full bitmap.
    ///     rb = Roaring::Bitmap.full
    ///     rb.full?    #=> true
    ///
    /// @return [Roaring::Bitmap] A new full bitmap.
    fn new_full() -> Self {
        Self(RefCell::new(Wrapper {
            _data: RoaringBitmap::full(),
        }))
    }

    /// @yard
    /// @def from_a(array)
    ///
    /// Initializes a new bitmap from an array of integers.
    ///
    /// @param [Array<Integer>] array An array of integers to initialize the bitmap.
    ///
    /// @example Initialize a bitmap from an array of integers
    ///     rb = Roaring::Bitmap.from_a([1, 2, 3])
    ///     rb.to_a     #=> [1, 2, 3]
    ///
    /// @return [Roaring::Bitmap] A new bitmap initialized from the array.
    fn from_array(array: RArray) -> Self {
        let values = array.to_vec::<u32>().unwrap();
        Self(RefCell::new(Wrapper {
            _data: RoaringBitmap::from_iter(values),
        }))
    }

    /// @yard
    /// @def insert(item)
    ///
    /// Inserts an item into the bitmap.
    ///
    /// @param [Integer] item The item to insert.
    ///
    /// @example Insert an item into the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert(1)    #=> true
    ///
    /// @example Insert an item that is already present into the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert(1)    #=> true
    ///     rb.insert(1)    #=> false
    ///
    /// @return [Boolean] true if the item was not already present, false otherwise.
    fn insert(&self, item: u32) -> Result<bool, Error> {
        Ok(self.0.borrow_mut()._data.insert(item))
    }

    /// @yard
    /// @def insert_many(items)
    ///
    /// Inserts multiple items into the bitmap.
    ///
    /// @param [Array<Integer>] items An array of items to insert.
    ///
    /// @example Insert multiple items into the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])   #=> 3
    ///     rb.to_a                     #=> [1, 2, 3]
    ///
    /// @return [Integer] The number of items that were inserted.
    fn insert_many(&self, items: RArray) -> Result<u32, Error> {
        let values = items.to_vec::<u32>()?;
        let mut inserted = 0;
        for value in values {
            if self.0.borrow_mut()._data.insert(value) {
                inserted += 1;
            }
        }

        Ok(inserted)
    }

    /// @yard
    /// @def remove(item)
    ///
    /// Removes an item from the bitmap.
    ///
    /// @param [Integer] item The item to remove.
    ///
    /// @example Remove an item from the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert(1)
    ///     rb.remove(1)    #=> true
    ///
    /// @example Remove an item that is not present in the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.remove(1)    #=> false
    ///
    /// @return [Boolean] true if the item was present, false otherwise.
    fn remove(&self, item: u32) -> Result<bool, Error> {
        Ok(self.0.borrow_mut()._data.remove(item))
    }

    /// @yard
    /// @def contains(item)
    ///
    /// Check the bitmap for the presence of an item.
    ///
    /// @param [Integer] item The item to check for.
    ///
    /// @example Check the bitmap for the presence of an item.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert(1)
    ///     rb.contains(1)  #=> true
    ///
    /// @example Check the bitmap for the presence of an item that is not present.
    ///     rb = Roaring::Bitmap.new
    ///     rb.contains(1)  #=> false
    ///
    /// @return [Boolean] true if the item is present, false otherwise.
    fn contains(&self, item: u32) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.contains(item))
    }

    /// @yard
    /// @def to_a
    ///
    /// Returns an array of all the items in the bitmap.
    ///
    /// @example Return an array of all the items in the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])
    ///     rb.to_a     #=> [1, 2, 3]
    ///
    /// @return [Array<Integer>] An array of all the items in the bitmap.
    fn to_vec(&self) -> Result<Vec<u32>, Error> {
        Ok(self.0.borrow()._data.iter().collect())
    }

    /// @yard
    /// @def clear
    ///
    /// Clears the bitmap, removing all items.
    ///
    /// @example Clear the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])
    ///     rb.to_a     #=> [1, 2, 3]
    ///     rb.clear
    ///     rb.to_a     #=> []
    ///
    /// @return [nil]
    fn clear(&self) -> Result<(), Error> {
        self.0.borrow_mut()._data.clear();
        Ok(())
    }

    /// @yard
    /// @def cardinality
    ///
    /// Returns the number of items in the bitmap.
    ///
    /// @example Return the number of items in the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])
    ///     rb.cardinality  #=> 3
    ///
    /// @return [Integer] The number of items in the bitmap.
    fn len(&self) -> Result<u64, Error> {
        Ok(self.0.borrow()._data.len())
    }

    /// @yard
    /// @def empty?
    ///
    /// Checks if the bitmap is empty.
    ///
    /// @example Checking an empty bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.empty?   #=> true
    ///
    /// @example Checking a non-empty bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert(1)
    ///     rb.empty?   #=> false
    ///
    /// @return [Boolean] true if the bitmap is empty, false otherwise.
    fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_empty())
    }

    /// @yard
    /// @def full?
    ///
    /// Checks if the bitmap is full.
    ///
    /// @example Checking a full bitmap.
    ///     rb = Roaring::Bitmap.full
    ///     rb.full?    #=> true
    ///
    /// @example Checking a non-full bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert(1)
    ///     rb.full?    #=> false
    ///
    /// @return [Boolean] true if the bitmap is full, false otherwise.
    fn is_full(&self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_full())
    }

    /// @yard
    /// @def max
    ///
    /// Retrieves the maximum value in the bitmap.
    ///
    /// @example Retrieving the maximum value in the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])
    ///     rb.max  #=> 3
    ///
    /// @example Retrieving the maximum value in an empty bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.max  #=> nil
    ///
    /// @return [Integer, nil] The maximum value in the bitmap, or nil if the bitmap is empty.
    fn max(&self) -> Result<Option<u32>, Error> {
        Ok(self.0.borrow()._data.max())
    }

    /// @yard
    /// @def min
    ///
    /// Retrieves the minimum value in the bitmap.
    ///
    /// @example Retrieving the minimum value in the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])
    ///     rb.min  #=> 1
    ///
    /// @example Retrieving the minimum value in an empty bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.min  #=> nil
    ///
    /// @return [Integer, nil] The minimum value in the bitmap, or nil if the bitmap is empty.
    fn min(&self) -> Result<Option<u32>, Error> {
        Ok(self.0.borrow()._data.min())
    }

    /// @yard
    /// @def nth(item)
    ///
    /// Retrieves the nth integer in the bitmap, or nil if the bitmap is empty or if n is out of bounds.
    ///
    /// @example Retrieving the nth integer in the bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])
    ///     rb.nth(1)   #=> 2
    ///
    /// @example Retrieving the nth integer in an empty bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.nth(1)   #=> nil
    ///
    /// @return [Integer, nil] The nth integer in the bitmap, or nil if the bitmap is empty or if n is out of bounds.
    fn select(&self, item: u32) -> Result<Option<u32>, Error> {
        Ok(self.0.borrow()._data.select(item))
    }

    /// @yard
    /// @def disjoint?(other)
    ///
    /// Checks if the bitmaps are disjoint.
    ///
    /// @param other [Roaring::Bitmap] The other bitmap to check.
    ///
    /// @example When the bitmaps are disjoint.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([4, 5, 6])
    ///     rb1.disjoint?(rb2)  #=> true
    ///
    /// @example When the bitmaps are not disjoint.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.disjoint?(rb2)  #=> false
    ///
    /// @return [Boolean] true if the bitmaps are disjoint, false otherwise.
    fn is_disjoint(&self, other: &Self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_disjoint(&other.0.borrow()._data))
    }

    /// @yard
    /// @def subset?(other)
    ///
    /// Checks if the bitmap is a subset of another bitmap.
    ///
    /// @param other [Roaring::Bitmap] The other bitmap.
    ///
    /// @example When the bitmap is a subset of another bitmap.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([1, 2, 3, 4, 5])
    ///     rb1.subset?(rb2)    #=> true
    ///
    /// @example When the bitmap is not a subset of another bitmap.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.subset?(rb2)    #=> false
    ///
    /// @return [Boolean] true if the bitmap is a subset of another bitmap, false otherwise.
    fn is_subset(&self, other: &Self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_subset(&other.0.borrow()._data))
    }

    /// @yard
    /// @def superset?(other)
    ///
    /// Checks if the bitmap is a superset of another bitmap.
    ///
    /// @param other [Roaring::Bitmap] The other bitmap.
    ///
    /// @example When the bitmap is a superset of the other bitmap.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3, 4, 5])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([1, 2, 3])
    ///     rb1.superset?(rb2)  #=> true
    ///
    /// @example When the bitmap is not a superset of the other bitmap.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([3, 4, 5])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([1, 2, 3])
    ///     rb1.superset?(rb2)  #=> false
    ///
    /// @return [Boolean] true if the bitmap is a superset of another bitmap, false otherwise.
    fn is_superset(&self, other: &Self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data.is_superset(&other.0.borrow()._data))
    }

    /// @yard
    /// @def union(other)
    ///
    /// Union the bitmap with another bitmap. Bitwise OR.
    ///
    /// @param other [Roaring::Bitmap] The other bitmap to union with.
    ///
    /// @example Unioning two bitmaps.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.union(rb2).to_a #=> [1, 2, 3, 4, 5]
    ///     rb1.or(rb2).to_a    #=> [1, 2, 3, 4, 5]
    ///     (rb1 | rb2).to_a    #=> [1, 2, 3, 4, 5]
    ///
    /// @return [Roaring::Bitmap] The union of the bitmap with another bitmap.
    fn union(&self, other: &Self) -> Result<Self, Error> {
        let lhs = &self.0.borrow()._data;
        let rhs = &other.0.borrow()._data;
        let d = lhs.bitor(rhs);

        Ok(Self(RefCell::new(Wrapper { _data: d })))
    }

    /// @yard
    /// @def union_len(other)
    ///
    /// Computes the union of the bitmap with another bitmap and returns the cardinality of the result.
    /// Useful for when you want to know the length of the result but not create a new bitmap.
    ///
    /// @param other [Roaring::Bitmap] The bitmap to compute the union length with.
    ///
    /// @example Computing the union length of two bitmaps.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.union_len(rb2) #=> 5
    ///
    /// @return [Integer] The cardinality of the union of the bitmap with another bitmap.
    fn union_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self.0.borrow()._data.union_len(&other.0.borrow()._data))
    }

    /// @yard
    /// @def intersection(other)
    ///
    /// Intersects the bitmap with another bitmap. Bitwise AND.
    ///
    /// @param other [Roaring::Bitmap] The bitmap to intersect with.
    ///
    /// @example Intersecting two bitmaps.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.intersection(rb2).to_a #=> [3]
    ///     rb1.and(rb2).to_a          #=> [3]
    ///     (rb1 & rb2).to_a           #=> [3]
    ///
    /// @return [Roaring::Bitmap] The intersection of the bitmap with another bitmap.
    fn intersection(&self, other: &Self) -> Result<Self, Error> {
        let lhs = &self.0.borrow()._data;
        let rhs = &other.0.borrow()._data;
        let d = lhs.bitand(rhs);

        Ok(Self(RefCell::new(Wrapper { _data: d })))
    }

    /// @yard
    /// @def intersection_len(other)
    ///
    /// Computes the intersection of the bitmap with another bitmap and returns the cardinality of the result.
    /// Useful for when you want to know the length of the result but not create a new bitmap.
    ///
    /// @param other [Roaring::Bitmap] The other bitmap compute the intersection length with.
    ///
    /// @example Computing the intersection length of two bitmaps.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.intersection_len(rb2)   #=> 1
    ///
    /// @return [Integer] The cardinality of the intersection of the bitmap with another bitmap.
    fn intersection_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self
            .0
            .borrow()
            ._data
            .intersection_len(&other.0.borrow()._data))
    }

    /// @yard
    /// @def difference(other)
    ///
    /// A difference between the two bitmaps. Bitwise AND NOT.
    ///
    /// @param other [Roaring::Bitmap] The other bitmap to compute the difference with.
    ///
    /// @example Computing the difference of two bitmaps.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.difference(rb2).to_a    #=> [1, 2]
    ///     rb1.and_not(rb2).to_a       #=> [1, 2]
    ///     (rb1 - rb2).to_a            #=> [1, 2]
    ///
    /// @return [Roaring::Bitmap] The difference of the bitmap with another bitmap.
    fn difference(&self, other: &Self) -> Result<Self, Error> {
        let lhs = &self.0.borrow()._data;
        let rhs = &other.0.borrow()._data;
        let d = lhs.sub(rhs);

        Ok(Self(RefCell::new(Wrapper { _data: d })))
    }

    /// @yard
    /// @def difference_len(other)
    ///
    /// Computes the difference of the bitmap with another bitmap and returns the cardinality of the result.
    /// Useful for when you want to know the length of the result but not create a new bitmap.
    ///
    /// @param [Roaring::Bitmap] other The other bitmap to compute the difference length with.
    ///
    /// @example Computing the difference length of two bitmaps.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.difference_len(rb2)     #=> 2
    ///
    /// @return [Integer] The cardinality of the difference of the bitmap with another bitmap.
    fn difference_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self
            .0
            .borrow()
            ._data
            .difference_len(&other.0.borrow()._data))
    }

    /// @yard
    /// @def symmetric_difference(other)
    ///
    /// A symmetric difference between the two bitmaps. This is equivalent to the union of the two bitmaps minus the intersection. Bitwise XOR.
    ///
    /// @param [Roaring::Bitmap] other The other bitmap to compute the symmetric difference with.
    ///
    /// @example Computing the symmetric difference of two bitmaps.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.symmetric_difference(rb2).to_a #=> [1, 2, 4, 5]
    ///     rb1.xor(rb2).to_a                  #=> [1, 2, 4, 5]
    ///     (rb1 ^ rb2).to_a                   #=> [1, 2, 4, 5]
    ///
    /// @return [Roaring::Bitmap] The symmetric difference of the bitmap with another bitmap.
    fn symmetric_difference(&self, other: &Self) -> Result<Self, Error> {
        let lhs = &self.0.borrow()._data;
        let rhs = &other.0.borrow()._data;
        let d = lhs.bitxor(rhs);
        Ok(Self(RefCell::new(Wrapper { _data: d })))
    }

    /// @yard
    /// @def symmetric_difference_len(other)
    ///
    /// Computes the symmetric difference of the bitmap with another bitmap and returns the cardinality of the result.
    /// Useful for when you want to know the length of the result but not create a new bitmap.
    ///
    /// @param [Roaring::Bitmap] other The other bitmap to compute the symmetric difference length with.
    ///
    /// @example Computing the symmetric difference length of two bitmaps.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([3, 4, 5])
    ///     rb1.symmetric_difference_len(rb2)   #=> 4
    ///
    /// @return [Integer] The cardinality of the symmetric difference of the bitmap with another bitmap.
    fn symmetric_difference_len(&self, other: &Self) -> Result<u64, Error> {
        Ok(self
            .0
            .borrow()
            ._data
            .symmetric_difference_len(&other.0.borrow()._data))
    }

    /// @yard
    /// @def rank(item)
    ///
    /// Returns the number of integers that are <= value. rank(u32::MAX) == len(). This is also known as the rank or rank-select idiom.
    ///
    /// @example Computing the rank of an item in a bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3, 4, 5])
    ///     rb.rank(3) #=> 3
    ///
    /// @return [Integer] The number of integers that are <= value.
    /// @raise [Roaring::Error] If the item is greater than u32::MAX.
    /// @raise [Roaring::Error] If the item is less than 0.
    fn rank(&self, item: u32) -> Result<u64, Error> {
        Ok(self.0.borrow()._data.rank(item))
    }

    /// @yard
    ///
    /// @overload each(&block)
    ///
    ///     Iterates over the bitmap and yields each item.
    ///
    ///     @example Iterating over a bitmap.
    ///         rb = Roaring::Bitmap.new
    ///         rb.insert_many([1, 2, 3, 4, 5])
    ///         rb.each do |i|
    ///             puts i
    ///         end
    ///
    ///     @yield [Integer] The item in the bitmap.
    ///
    ///     @return [Roaring::Bitmap] The bitmap.
    ///
    /// @overload each
    ///
    ///     Returns an enumerator if no block is given.
    ///
    ///     @example Receiving an Enumerator if no block is given.
    ///         rb = Roaring::Bitmap.new
    ///         rb.insert_many([1, 2, 3, 4, 5])
    ///         rb.each #=> #<Enumerator: ...>
    ///
    ///     @return [Enumerator] An enumerator.
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

    /// @yard
    /// @def byte_size
    ///
    /// Returns the size of the bitmap in bytes.
    ///
    /// @example Computing the size of a bitmap in bytes.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3, 4, 5])
    ///     rb.byte_size #=> 26
    ///
    /// @return [Integer] The size of the bitmap in bytes.
    fn byte_size(&self) -> Result<usize, Error> {
        Ok(self.0.borrow()._data.serialized_size())
    }

    /// @yard
    /// @def eql?(other)
    ///
    /// Checks if the bitmap is equal to another bitmap.
    ///
    /// @param other [Roaring::Bitmap] The other bitmap to compare to.
    ///
    /// @example Checking if two bitmaps are equal.
    ///     rb1 = Roaring::Bitmap.new
    ///     rb1.insert_many([1, 2, 3])
    ///     rb2 = Roaring::Bitmap.new
    ///     rb2.insert_many([1, 2, 3])
    ///     rb1.eql?(rb2) #=> true
    ///
    /// @return [Boolean] True if the bitmap is equal to another bitmap, false otherwise.
    fn eql(&self, other: &Self) -> Result<bool, Error> {
        Ok(self.0.borrow()._data == other.0.borrow()._data)
    }

    /// @yard
    /// @def serialize
    ///
    /// Serializes the bitmap into its binary representation.
    ///
    /// @example Serializing a bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])
    ///     rb.serialize    #=> ":0\x00\x00\x01\x00\x00\x00\x00\x00\x04\x00\x10\x00\x00\x00\x01\x00\x02\x00\x03\x00\x04\x00\x05\x00"
    ///
    /// @return [String] The binary representation of the bitmap.
    fn serialize(&self) -> Result<RString, Error> {
        let mut buf = vec![];
        self.0.borrow()._data.serialize_into(&mut buf).unwrap();
        Ok(RString::from_slice(&buf))
    }

    /// @yard
    /// @def deserialize(rstr)
    ///
    /// Deserializes a bitmap from its binary representation.
    ///
    /// @param rstr [String] The binary representation of the bitmap.
    ///
    /// @example Deserializing a bitmap.
    ///     rb = Roaring::Bitmap.new
    ///     rb.insert_many([1, 2, 3])
    ///     dumped = rb.serialize   #=> ":0\..."
    ///
    ///     from_binary = Roaring::Bitmap.deserialize(dumped)
    ///     rb.eql?(from_binary)    #=> true
    ///
    /// @return [Roaring::Bitmap] The bitmap.
    fn deserialize(rstr: RString) -> Result<Self, Error> {
        let buf = unsafe { rstr.as_slice() };
        let d = RoaringBitmap::deserialize_from(&mut &buf[..]).unwrap();
        Ok(Self(RefCell::new(Wrapper { _data: d })))
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

    bitmap_class.define_method("insert_many", method!(MutWrapper::insert_many, 1))?;

    bitmap_class.define_method("remove", method!(MutWrapper::remove, 1))?;

    bitmap_class.define_method("contains", method!(MutWrapper::contains, 1))?;

    bitmap_class.define_method("to_a", method!(MutWrapper::to_vec, 0))?;

    bitmap_class.define_method("clear", method!(MutWrapper::clear, 0))?;

    bitmap_class.define_method("cardinality", method!(MutWrapper::len, 0))?;

    bitmap_class.define_method("empty?", method!(MutWrapper::is_empty, 0))?;

    bitmap_class.define_method("full?", method!(MutWrapper::is_full, 0))?;

    bitmap_class.define_method("max", method!(MutWrapper::max, 0))?;

    bitmap_class.define_method("min", method!(MutWrapper::min, 0))?;

    bitmap_class.define_method("nth", method!(MutWrapper::select, 1))?;

    bitmap_class.define_method("disjoint?", method!(MutWrapper::is_disjoint, 1))?;

    bitmap_class.define_method("subset?", method!(MutWrapper::is_subset, 1))?;

    bitmap_class.define_method("superset?", method!(MutWrapper::is_superset, 1))?;

    bitmap_class.define_method("union", method!(MutWrapper::union, 1))?;
    bitmap_class.define_method("union_len", method!(MutWrapper::union_len, 1))?;

    bitmap_class.define_method("intersection", method!(MutWrapper::intersection, 1))?;
    bitmap_class.define_method("intersection_len", method!(MutWrapper::intersection_len, 1))?;

    bitmap_class.define_method("difference", method!(MutWrapper::difference, 1))?;
    bitmap_class.define_method("difference_len", method!(MutWrapper::difference_len, 1))?;

    bitmap_class.define_method(
        "symmetric_difference",
        method!(MutWrapper::symmetric_difference, 1),
    )?;
    bitmap_class.define_method(
        "symmetric_difference_len",
        method!(MutWrapper::symmetric_difference_len, 1),
    )?;

    bitmap_class.define_method("rank", method!(MutWrapper::rank, 1))?;

    bitmap_class.define_method("each", method!(MutWrapper::each, 0))?;

    bitmap_class.define_method("byte_size", method!(MutWrapper::byte_size, 0))?;

    bitmap_class.define_method("eql?", method!(MutWrapper::eql, 1))?;

    bitmap_class.define_method("serialize", method!(MutWrapper::serialize, 0))?;
    bitmap_class.define_singleton_method("deserialize", function!(MutWrapper::deserialize, 1))?;

    Ok(())
}
