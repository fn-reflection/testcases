# frozen_string_literal: true

RSpec.describe 'foo5' do
  it do
    sleep(5)
    expect(0 + 5).to eq 5
  end
end
