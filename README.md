# Simd::Json

This is an experiment in writing a Ruby wrapper over a simd-json Rust library. It's not intended for production use.

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'simd-json'
```

And then execute:

    $ bundle install

Or install it yourself as:

    $ gem install simd-json

## Usage

```ruby
json = '{ "a": 1, "b": { "c": 2 } }'
Simd::Json.parse(json)
```
