require 'near_api'

module UsersHelper

  # def define_constants
  #   @node_url = 'https://rpc.testnet.near.org/'
  #   @conf = NearApi::Config.new(node_url: @node_url)
  #   @query = NearApi::Query.new(config = @conf)

  #   @contract = 'f_nft.wabinab.testnet'
  # end

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
end
