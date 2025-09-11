# frozen_string_literal: true

RSpec.describe 'foo1' do
  it do
    sleep(1)
    expect(0+1).to eq 1
  end
end
