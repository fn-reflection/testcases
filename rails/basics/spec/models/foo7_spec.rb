# frozen_string_literal: true

RSpec.describe 'foo7' do
  it do
    sleep(7)
    expect(0 + 7).to eq 7
  end
end
