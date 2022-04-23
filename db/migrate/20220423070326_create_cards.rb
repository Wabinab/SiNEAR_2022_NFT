class CreateCards < ActiveRecord::Migration[7.0]
  def change
    create_table :cards do |t|
      t.string :token_id

      t.timestamps
    end
    add_index :cards, :token_id, unique: true
  end
end
