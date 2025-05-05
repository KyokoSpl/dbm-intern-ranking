# dbm-intern-ranking
**this is a rust based API to collect and manage results of tourney matches and display them in a nice dashboard with grafana**

### API features:
- collect results of tourney matches
- store results in a database
- intended to use with the according discord bot [found here](https://github.com/KyokoSpl/dbm-intern-ranking-bot)

### ui features:
- display the results of the tourney matches
- visualize winratees in a clean diagram
- display playerspecific stats in a separate dashboard


### Dependencies

- Server with os of your choice
- have nginx installed

## nginx
nxing is used to annonymize the Ip addresses so no public information is leaked in the grafana ui
also used for cleaner grafana url

### install nginx:
install nginx according to the [official nginx guide](https://www.nginx.com/resources/wiki/start/topics/tutorials/install/)

#### nginx setup:

add replace the `nginx.conf` in `/etc/nginx/` with the `nginx.conf` of this repo

# Setup:

start with cloning the repo
```bash
git clone https://github.com/KyokoSpl/dbm-intern-ranking
```

## SQL Database
Install and setup MYSQL server to your preference
refer to the [official mysql guide](https://dev.mysql.com/doc/mysql-getting-started/en/) for help

## Database setup:

1. Create the database and the tables according to the `ranking.sql` file
2. change the .env file to match your database settings

## API setup:
### Prepare the API

1. Install Rust according to the [official rust guide](https://www.rust-lang.org/tools/install)
2. make sure to have the `cargo` command available
   - if you have installed rust via `rustup` you can run `cargo --version` to check if it is installed
3. make sure that the Port of the API is open (default is `8000`)


#### for single use: 
```bash
cargo run
```
#### for longterm ussage:
copy the `ranking- api.service` file to `/etc/systemd/system/` and run `systemctl enable ranking-api.service` to enable the service and start at system boot

run the following commands to check the status of the service
```bash
systemctl status ranking-api.service
systemctl start ranking-api.service
systemctl stop ranking-api.service
```

# Grafana
Grafana is used to visualize the data from the API

## Grafana setup:
- install and setup Grafana to your preference
    - refer to the [official grafana guide](https://grafana.com/docs/grafana/latest/installation/) for help

- use the grafana ui to import the dashboards from the `dashboards` folder
- change the `datasource` to match your database settings


