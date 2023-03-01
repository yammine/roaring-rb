<div align="center">
  <h1><code>roaring-rb</code></h1>

  <p>
    <strong>Ruby embedding of
    <a href="https://github.com/RoaringBitmap/roaring-rs">roaring-rs</a></strong>
  </p>

  <p>
    <a href="https://github.com/yammine/roaring-rb/actions?query=workflow%3ACI">
      <img src="https://github.com/yammine/roaring-rb/actions/workflows/ci.yml/badge.svg" alt="CI status"/>
    </a>
  </p>
</div>

## Goal

`roaring-rb`'s goal is to expose the full power of Roaring Bitmaps in Ruby with minimal overhead.

## Installation

Add the `roaring-rb` gem to your Gemfile and run `bundle install`:

```ruby
gem "roaring-rb"
```

Alternatively, you can install the gem manually:

```sh
gem install roaring-rb
```

### Precompiled gems

We recommend installing the `roaring-rb` precompiled gems available for Linux, macOS, and Windows. Installing a precompiled gem avoids the need to compile from source code, which is generally slower and less reliable.

When installing the `roaring-rb` gem for the first time using `bundle install`, Bundler will automatically download the precompiled gem for your current platform. However, you will need to inform Bundler of any additional platforms you plan to use.

To do this, lock your Bundle to the required platforms you will need from the list of supported platforms below:

```sh
bundle lock --add-platform x86_64-linux # Standard Linux (e.g. Heroku, GitHub Actions, etc.)
bundle lock --add-platform x86_64-linux-musl # MUSL Linux deployments (i.e. Alpine Linux)
bundle lock --add-platform aarch64-linux # ARM64 Linux deployments (i.e. AWS Graviton2)
bundle lock --add-platform x86_64-darwin # Intel MacOS (i.e. pre-M1)
bundle lock --add-platform arm64-darwin # Apple Silicon MacOS  (i.e. M1)
bundle lock --add-platform x64-mingw-ucrt # Windows 
bundle lock --add-platform x64-mingw32 # Different Windows?
```

## Usage

Example usage:

```ruby
require "roaring-rb"

rb = Roaring::Bitmap.new

rb.insert 1
rb.insert 2
rb.min # => 1
rb.max # => 2

rb.insert_many(5, 10, 100)
rb.to_a # => [1, 2, 5, 10, 100]

dump = rb.serialize
loaded = Roaring::Bitmap.deserialize(dump)
rb == loaded # => true
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake` to compile & run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. Version releases are automated by GH Actions.
## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/yammine/roaring.
