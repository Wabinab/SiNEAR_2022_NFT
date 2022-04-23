class CreateUsers < ActiveRecord::Migration[7.0]
  def change
    create_table :users do |t|
      t.string :username
      t.string :public_key
      t.string :all_keys

      t.timestamps
    end
  end
end
