# frozen_string_literal: true

require "test_helper"

class Simd::JsonTest < Minitest::Test
  def test_it_parses
    json = '{ "foo": "bar", "null" : null, "number" : 1, "array": [1, 2, 3], "nested": { "inside": { "deep": "cuts" } } }'
    hash = { "foo" => "bar", "null" => nil, "number" => 1, "array" => [1, 2, 3], "nested" => { "inside" => { "deep" => "cuts" }} }
    assert_equal(hash, Simd::Json.parse(json))
  end
end
