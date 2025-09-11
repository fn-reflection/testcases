# frozen_string_literal: true

RSpec.describe 'foo1' do
  it do
    sleep(1)
    expect([true, false].sample).to be true
  end
end
