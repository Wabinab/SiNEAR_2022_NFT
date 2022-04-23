class CardsController < ApplicationController

  def new
    @card = Card.new
    @token_id = "f_nft_" + Time.now.to_f.to_s.gsub('.', '_')
  end

  def index
    respond_to do |format|
      format.js 
    end
    
  end

  def show
    redirect_to "/users/1"
  end

  def create
    # @token_id = "f_nft_" + Time.now.to_f.to_s.gsub('.', '_')
    # @card = Card.new(@token_id)

    # respond_to do |format|
      # if @card.save 
      #   format.js
      #   format.html { redirect_to @card }
      # else
      #   format.html { render :new }
      #   # format.turbo_stream { render :form_update, status: :unprocessable_entity }
      # end
    # end

    respond_to do |format|
        format.js 
        format.html { redirect_to "/users/1" }
    end

    # redirect_to "/users/1"

  end
end
