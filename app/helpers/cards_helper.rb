module CardsHelper
  def get_token_by_id(token_id)
    JSON.parse(@query.function(
      @contract,
      'nft_token',
      {
        "token_id": token_id,
        "limit": 10
      }
    )["result"]["result"].pack('c*'))
  end
end
