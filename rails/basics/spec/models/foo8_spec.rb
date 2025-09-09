# frozen_string_literal: true

RSpec.describe 'foo8' do
  it do
    sleep(8)
    expect(0 + 8).to eq 8
  end
end
