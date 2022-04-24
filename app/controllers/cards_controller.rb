class CardsController < ApplicationController

  def new
    @card = Card.new
    @token_id = "f_nft_" + Time.now.to_f.to_s.gsub('.', '_') + '_' + ('a'..'z').to_a.shuffle[0, 5].join
  end

  # Index test fail, because params[:token_id] is undefined. 
  # We didn't create a database, so this doesn't work anyways! 
  # In the future, we might want to remove it, though. 
  def index  
    @card = Card.find_by(token_id: params[:token_id]);
    redirect_to @card
  end

  def show
    @token_id = token_id
  end

  # def create
  # end

  # private
    # def card_params 
    #   params.require(:card).permit(:token_id)
    # end

    # def token_id 
    #   params.require(:token_id)
    # end
end
