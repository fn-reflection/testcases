# frozen_string_literal: true

require 'spec_helper'

ENV['RAILS_ENV'] = 'test'
require_relative '../config/environment'
require 'rspec/rails'
begin
  ActiveRecord::Migration.maintain_test_schema!
rescue ActiveRecord::PendingMigrationError => e
  abort e.to_s.strip
end
RSpec.configure do |config|
  config.fixture_paths = [
    Rails.root.join('spec/fixtures')
  ]
  config.use_transactional_fixtures = true
  config.filter_rails_from_backtrace!

  if ENV['CI']
    require 'rspec_junit_formatter'
    report_dir = 'test_reports'
    FileUtils.mkdir_p(report_dir)
      config.add_formatter(
        RspecJunitFormatter,
        File.join(
          report_dir,
          "junit-#{ENV['GROUP_INDEX'] || '0'}-#{ENV['TEST_ENV_NUMBER'] || '1'}.xml"
        )
      )
      config.add_formatter(:documentation)
  end
end
