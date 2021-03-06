require 'near_api'

module UsersHelper

  def gravatar_for(user)
    gravatar_id = Digest::MD5::hexdigest(user.account_id)
    gravatar_url = "https://secure.gravatar.com/avatar/#{gravatar_id}?d=identicon&r=PG"
    image_tag(gravatar_url, class: "gravatar")
  end

  def get_tokens(user)
    account_id = user.account_id
    JSON.parse(@query.function(
      @contract,
      'nft_tokens_for_owner',
      {
        "account_id": account_id,
        "limit": 10
      }
    )["result"]["result"].pack('c*'))
  end

  def to_percentage(json_info, user)
    account_id = user.account_id
    percentage = json_info["all_owners"][account_id]
    percentage.to_f / 100
  end
end
