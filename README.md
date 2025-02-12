# dbm-intern-ranking

## Setup

---

#### Dependencies

- running MySQL server with Database setup according to the api programm
- have nginx installed

### nginx

Is used to make cleaner url for Grafana setup (.json for Grafana will be added soon) and to anonymize ip addresses in the Profile overview and sytstem logs

#### nginx setup:

add replace the `nginx.conf` in `/etc/nginx/` with the `nginx.conf` of this repo

# Installation:

### Windows:

- Download .exe von [Releases](https://github.com/KyokoSpl/dbm-intern-ranking/releases/tag/1.0.0) und ausführen

### Linux:

- Download Binary Datei von [Releases](https://github.com/KyokoSpl/dbm-intern-ranking/releases/tag/1.0.0) und ausführen

### Manually

```sh
git clone https://github.com/KyokoSpl/dbm-intern-ranking
cd dbm-intern-ranking/dbm-intern-ranking-gui/
```

- Than replace Placeholder in .rs and .env files after that run

```sh
cargo run
```
