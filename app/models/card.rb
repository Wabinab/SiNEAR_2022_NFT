class Card < ApplicationRecord
  validates :token_id, presence: true, uniqueness: true
end
