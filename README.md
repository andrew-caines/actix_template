# Actix Webserver with (Sqlx, Postgres, React+Vite, Websocket Echo Server)
#TODO setup working example of Websocket 'channel' So a user can join that channel and recieve messages from it (example IT communication channel that gets messages from servers emitted to it)
#TODO setup SSE at a example route, so a React client can connect to it, and see emitted events (example see will just send out a interval of the size of the log_handler table Select Count(*) from handler_logs)

## To start pull down the template into a directory 
```console
PS $:\> git clone https://github.com/andrew-caines/actix_template.git
PS $:\> cd actix_template 
PS $:\> create/edit a .env file in the root folder of application.
PS $:\> cargo run 

create a new Terminal, if you want to edit the React Front end.
PS $:\> cd client
PS $:\> npm i
PS $:\> npm run dev

```
## Authorization
    This template is using argon2 as a password to db strategy.  /v1/auth
    All Auth routes are accessed @ /v1/auth
    - [POST]    /create - This route is how you add a new users to the DB and create the argon2 hash.
    - [POST]    /login - This route uses BASIC AUTH (like bearer, Authorization: Basic [base64 - string])
    - [GET]     /logout - This basically does nothing, but will blast the TokenClaims and send a 200 to client.

#TODO
