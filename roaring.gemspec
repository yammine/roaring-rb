# frozen_string_literal: true

require_relative "lib/roaring/version"

Gem::Specification.new do |spec|
  spec.name = "roaring"
  spec.version = Roaring::VERSION
  spec.authors = ["Chris Yammine"]
  spec.email = ["chris@yammine.dev"]

  spec.summary = "Ruby bindings for the Roaring Bitmap library"
  spec.description = "Ruby bindings for the Roaring Bitmap library"
  spec.homepage = "https://github.com/yammine/roaring-rb"
  spec.required_ruby_version = ">= 2.6.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/yammine/roaring-rb"
  spec.metadata["changelog_uri"] = "https://github.com/yammine/roaring-rb/blob/master/CHANGELOG.md"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|circleci)|appveyor)})
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/roaring/Cargo.toml"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
