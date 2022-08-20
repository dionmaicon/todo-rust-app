# Backend

Using Tokio Runtime, WARP (Web Server), SQLX (ORM) and Database (PostgreSQL).

## Development 

### Watch Model Tests


```sh
## watch model_db tests (database connection and seed data)
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'

```
```sh
## watch model_todo tests
cargo watch -q -c -w src/ -x 'test model_todo_ -- --test-threads=1 --nocapture'

```

### Watch Web Tests
```sh
## Watch Frontend Static and API
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'
```

## Database Instance

```sh
#postgress run
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14
```
```sh
# postgres execution
docker exec -it -u postgres pg psql

```
