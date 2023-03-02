# frozen_string_literal: true

require_relative "roaring/version"

# Tries to require the extension for the given Ruby version first
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "roaring/#{Regexp.last_match(1)}/roaring_rb"
rescue LoadError
  require "roaring/roaring_rb"
end

module Roaring
  class Error < StandardError; end

  class Bitmap
    include Enumerable

    alias_method :size, :cardinality
    alias_method :count, :cardinality
    alias_method :length, :cardinality

    alias_method :&, :intersection
    alias_method :and, :intersection
    alias_method :and_len, :intersection_len

    alias_method :|, :union
    alias_method :or, :union
    alias_method :or_len, :union_len

    alias_method :-, :difference
    alias_method :and_not, :difference
    alias_method :and_not_len, :difference_len

    alias_method :^, :symmetric_difference
    alias_method :xor, :symmetric_difference
    alias_method :xor_len, :symmetric_difference_len

    alias_method :<<, :insert

    alias_method :delete, :remove

    alias_method :include?, :contains
    alias_method :member?, :contains
    alias_method :contains?, :contains

    alias_method :reset, :clear

    alias_method :first, :min
    alias_method :last, :max

    alias_method :==, :eql?

    def hash
      to_a.hash
    end

    def self._load(args)
      deserialize(args)
    end

    def _dump(_level)
      serialize
    end
  end
end
