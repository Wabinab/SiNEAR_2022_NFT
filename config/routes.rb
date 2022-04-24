Rails.application.routes.draw do
  get 'cards', to: 'cards#index'
  get '/cards/:token_id', to: 'cards#show'
  # post 'cards', to: 'cards#create'

  get 'user', to: 'users#index'
  post 'user', to: 'users#create'
  # get 'home', to: 'static_pages#home'

  # root 'static_pages#home'
  root 'users#new'
  # get '@:id', to: 'users#show'

  resources :users
end
