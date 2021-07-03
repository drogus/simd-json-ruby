# frozen_string_literal: true

require_relative "json/version"
require 'rutie'

module RutieRubyExample
  target = ENV['CARGO_PROFILE'] || 'release'
  Rutie.new('simd_json', lib_path: "../../target/#{target}").init 'Init_simd_json', __dir__
end

module Simd
  module Json
    class Error < StandardError; end
    # Your code goes here...

    def self.parse(string)
      SimdJsonRust.parse(string)
    end
  end
end
