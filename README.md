# Named Parameters for `rust-postgres`

* Use `{...}` for parameters:

```sql
SELECT * FROM log WHERE t BETWEEN {lo} AND {hi}
```

* Use `{&...}` for variables that will be treated as identifiers, which is to
  say, as the names of columns, types or tables:

```sql
INSERT INTO {&current_partition} VALUES (now(), {tag}, {location})
```
