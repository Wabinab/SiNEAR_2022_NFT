Rails.application.routes.draw do
  get 'users', to: 'users#new'
  post 'users', to: 'users#new'
  root 'users#new'

  get '@:username', to: 'users#show'

  resources :users
end
