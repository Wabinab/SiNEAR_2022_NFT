# Spring is NEAR Fractionalized-NFTs

The first thing you did with this, is to get your environmental keys setup. 
For security reasons, rails by default do not export these, and requires you
to deal with it individually. You don't need my keys to play with this example
anyways, so one shall give you instruction in generating new keys. 

Based on [this article](https://stackoverflow.com/questions/54277392/rails-activesupportmessageencryptorinvalidmessage), do these: 

Delete these if exist (usually it don't if you clone from github):
- config/master.key
- config/credentials.yml.enc

provided you're using **VSCode**:

```bash
EDITOR="code ." bin/rails credentials:edit
```

This will open up a file. DOn't need to touch it, just close it. If it asks for saving, ignore it. Most probably you typed a space or soemthing which isn't important. 

Try run this: 

```bash
rails s
```

If it opens the browser and asks you to do migration, do these. With the new key, you should be able to run these without getting into `Rails:ActiveSupport::MessageEncryptor::InvalidMessage` error. 

```bash
rails db:migrate
rails s
```

> `rails s` is equivalent to `rails server`. 

It should work now... 
