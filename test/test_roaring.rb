# frozen_string_literal: true

require "test_helper"

class TestRoaring < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Roaring::VERSION
  end

  def test_insert
    bitmap = Roaring::Bitmap.new
    assert bitmap.insert(1) == true
    assert bitmap.insert(1) == false
  end

  def test_insert_out_of_range
    bitmap = Roaring::Bitmap.new
    assert_raises(RangeError) do 
      bitmap.insert(2**64)
    end
  end
end
