# frozen_string_literal: true

RSpec.describe 'foo6' do
  it do
    sleep(6)
    expect(0 + 6).to eq 6
  end
end
