# frozen_string_literal: true

RSpec.describe 'foo3' do
  it do
    sleep(3)
    expect(0 + 3).to eq 3
  end
end
