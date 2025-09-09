# frozen_string_literal: true

RSpec.describe 'foo4' do
  it do
    sleep(4)
    expect(0 + 4).to eq 4
  end
end
