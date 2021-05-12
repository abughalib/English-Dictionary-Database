# English-Dictionary-Database
It's an opensource English Dictionary definition Database

# Installation
## On Local Machine
**Clone the entire repo or just download the dictionary-server**
Configure the .env file<br>
```
DB_URL=localhost/dictionary
DB_USER=abughalib
DB_PASS=abughalib
```
DB_URL -> Database url.<br>
DB_USER -> Database user.<br>
DB_PASS -> Database password<br>

Test if everything is working fine.<br>
`cargo test`
## On Server
**Using Docker**
Download the image dependencies <br>
`docker build .`

Test if everything is working fine.<br>
`docker-compose up`

Coming soon.
