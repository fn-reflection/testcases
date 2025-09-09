# frozen_string_literal: true

RSpec.describe 'foo2' do
  it do
    sleep(2)
    expect(0 + 2).to eq 2
  end
end
