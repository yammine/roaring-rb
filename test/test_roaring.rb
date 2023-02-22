# frozen_string_literal: true

require "test_helper"

class TestRoaringBitmap < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Roaring::VERSION
  end

  def test_insert
    bitmap = Roaring::Bitmap.new
    assert bitmap.insert(1) == true
    assert bitmap.insert(1) == false
  end

  def test_insert_aliases
    bitmap = Roaring::Bitmap.new
    assert bitmap << 1 == true
    assert bitmap.push(2) == true
  end

  def test_insert_out_of_range
    bitmap = Roaring::Bitmap.new
    assert_raises(RangeError) do 
      bitmap.insert(2**64)
    end
  end

  def test_remove
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    assert bitmap.remove(1) == true
    assert bitmap.remove(1) == false
  end

  def test_contains
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    assert bitmap.contains(1) == true
    assert bitmap.contains(2) == false
  end

  def test_to_a 
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    assert bitmap.to_a == [1, 2]
  end

  def test_clear
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.clear
    refute bitmap.contains(1)
  end

  def test_len
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    assert bitmap.len == 2
  end

  def test_empty?
    bitmap = Roaring::Bitmap.new
    assert bitmap.empty?
    bitmap.insert(1)
    refute bitmap.empty?
  end

  def test_full?
    bitmap = Roaring::Bitmap.full
    assert bitmap.full?
  end

  def test_max
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    assert bitmap.max == 2
  end

  def test_min
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    assert bitmap.min == 1
  end

  def test_disjoint?
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(3)
    bitmap2.insert(4)
    assert bitmap.disjoint?(bitmap2)
    # refute that bitmap is disjoint with itself
    refute bitmap.disjoint?(bitmap)
  end

  def test_subset?
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(1)
    bitmap2.insert(2)
    bitmap2.insert(3)
    assert bitmap.subset?(bitmap2)
  end

  def test_superset?
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(1)
    bitmap2.insert(2)
    bitmap2.insert(3)
    assert bitmap2.superset?(bitmap)
  end

  def test_union
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(3)
    bitmap2.insert(4)
    bitmap.union(bitmap2)
    assert bitmap.to_a == [1, 2, 3, 4]

    bitmap3 = Roaring::Bitmap.new
    bitmap3.insert(5)
    bitmap | bitmap3
    assert bitmap.to_a == [1, 2, 3, 4, 5]
  end

  def test_union_len
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(3)
    bitmap2.insert(4)
    assert bitmap.union_len(bitmap2) == 4
  end

  def test_intersection
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(2)
    bitmap2.insert(3)
    bitmap.intersection(bitmap2)
    assert bitmap.to_a == [2]

    bitmap3 = Roaring::Bitmap.new
    bitmap3.insert(2)
    bitmap & bitmap3
    assert bitmap.to_a == [2]
  end

  def test_intersection_len
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(2)
    bitmap2.insert(3)
    assert bitmap.intersection_len(bitmap2) == 1
  end

  def test_difference
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(2)
    bitmap2.insert(3)
    bitmap.difference(bitmap2)
    assert bitmap.to_a == [1]

    bitmap3 = Roaring::Bitmap.new
    bitmap3.insert(1)
    bitmap - bitmap3
    assert bitmap.to_a == []
  end

  def test_difference_len
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(2)
    bitmap2.insert(3)
    assert bitmap.difference_len(bitmap2) == 1
  end

  def test_symmetric_difference
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(2)
    bitmap2.insert(3)
    bitmap.symmetric_difference(bitmap2)
    assert bitmap.to_a == [1, 3]

    bitmap3 = Roaring::Bitmap.new
    bitmap3.insert(3)
    bitmap ^ bitmap3
    assert bitmap.to_a == [1]
  end

  def test_symmetric_difference_len
    bitmap = Roaring::Bitmap.new
    bitmap.insert(1)
    bitmap.insert(2)
    bitmap2 = Roaring::Bitmap.new
    bitmap2.insert(2)
    bitmap2.insert(3)
    assert bitmap.symmetric_difference_len(bitmap2) == 2
  end
end
