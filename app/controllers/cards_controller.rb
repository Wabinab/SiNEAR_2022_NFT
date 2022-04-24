class CardsController < ApplicationController

  def new
    @card = Card.new
    @token_id = "f_nft_" + Time.now.to_f.to_s.gsub('.', '_')
  end

  def index  
    @card = Card.find_by(token_id: card_params[:token_id]);
    redirect_to @card
  end

  def show
    @token_id = token_id
  end

  # def create
  # end

  private
    def card_params 
      params.require(:card).permit(:token_id)
    end

    def token_id 
      params.require(:token_id)
    end
end
