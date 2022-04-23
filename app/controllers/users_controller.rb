class UsersController < ApplicationController
  def new
    @user = User.new
  end

  def show
    @user = User.find(params[:id])
  end

  def index
    @user = User.find_by(username: search_params[:account_id].gsub(".", "-"))
    redirect_to @user
  end
  

  def create
    user_params[:username] = user_params[:account_id]
    @user = User.new(user_params)

    if @user.save
      flash[:success] = "User successfully created"
      redirect_to @user
    else
      @user = User.find_by(username: user_params[:account_id])
      @user.public_key = user_params[:public_key]
      @user.all_keys = user_params[:all_keys]
      @user.save
    end

    redirect_to @user
  end
  
  private

    def user_params
      @params = params.require(:user).permit(:account_id, :public_key, :all_keys)
      @params[:account_id] = @params[:account_id].gsub(".", "-")
      @params
    end

    def search_params
      params.require(:user).permit(:account_id)
    end
end
