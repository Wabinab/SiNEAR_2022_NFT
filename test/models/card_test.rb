require "test_helper"

class CardTest < ActiveSupport::TestCase
  def setup
    @card = Card.new(token_id: "token-1")
  end

  test "should be valid" do 
    assert @card.valid?
  end

  test "token_id should be present" do 
    @card.token_id = "    "
    assert_not @card.valid?
  end

  test "token_id should be unique" do 
    dup_card = @card.dup
    @card.save
    assert_not dup_card.valid?
  end
end
