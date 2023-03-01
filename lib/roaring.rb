# frozen_string_literal: true

require_relative "roaring/version"

# Tries to require the extension for the given Ruby version first
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "roaring/#{Regexp.last_match(1)}/roaring"
rescue LoadError
  require "roaring/roaring"
end

module Roaring
  class Error < StandardError; end

  class Bitmap
    include Enumerable

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
