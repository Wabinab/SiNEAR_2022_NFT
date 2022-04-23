# Requires more test cases. Haven't understand how to test properly, though. 

class UsersController < ApplicationController
  def new
    @user = User.new
  end

  def show 
    @user = User.find(params[:id])
  end

  def index
    @user = User.find_by(account_id: search_params[:account_id])
    redirect_to "/users/#{@user.id}"
  end
  

  def create 
    @user = User.new(user_params)

    @myparam = user_params[:account_id]

    if @user.save 
      # Unfortunately cannot take out this repeat. Only after save we can search. 
      @user =  User.find_by(account_id: user_params[:account_id])
    else
      @user =  User.find_by(account_id: user_params[:account_id])
      @user.public_key = user_params[:public_key]
      @user.all_keys = user_params[:all_keys]
      @user.save
    end

    redirect_to "/users/#{@user.id}"
    
  end

  private 

    def user_params 
      params.require(:user).permit(:account_id, :public_key, :all_keys)
    end

    def search_params 
      params.require(:user).permit(:account_id)
    end
end
