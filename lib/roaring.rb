# frozen_string_literal: true

require_relative "roaring/version"

module Roaring
  class Error < StandardError; end
  # Your code goes here...
end


# Tries to require the extension for the given Ruby version first
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "roaring/#{Regexp.last_match(1)}/roaring"
rescue LoadError
  require "roaring/roaring"
end