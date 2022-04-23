Rails.application.routes.draw do

  get 'user', to: 'users#new'
  post 'user', to: 'users#new'
  # get 'home', to: 'static_pages#home'

  # root 'static_pages#home'
  root 'users#new'
  get '@:id', to: 'users#show'

  resources :users
end
