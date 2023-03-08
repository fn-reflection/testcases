require_relative "boot"

require "rails"
require "active_record/railtie"
require "action_controller/railtie"
require "action_view/railtie"
require "action_mailer/railtie"
# require "active_job/railtie"
# require "action_cable/engine"
# require "active_storage/engine"
# require "rails/test_unit/railtie"
# require "sprockets/railtie"

Bundler.require(*Rails.groups)

module Abc
  class Application < Rails::Application
    config.load_defaults 7.0
  end
end
