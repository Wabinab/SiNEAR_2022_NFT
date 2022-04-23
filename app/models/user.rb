class User < ApplicationRecord
  validates :username, presence: true, uniqueness: true
  validates :public_key, :all_keys, presence: true
end