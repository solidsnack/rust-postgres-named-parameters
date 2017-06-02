# Named Parameters For `postgres` Queries

This crate provides for named parameters for [`postgres`] queries.

[`postgres`]: https://docs.rs/postgres

```rust
#[macro_use]
extern crate postgres_named_parameters;

use chrono::Duration;
use chrono::prelude::UTC;

let now = UTC.now();
let q = query!("SELECT * FROM log WHERE t BETWEEN {lo} AND {hi}",
               lo = now - Duration.minutes(6),
               hi = now - Duration.minutes(1));

conn.execute(q.text(), q.parameters());
```

## Parameters & Idenitifers

* Use `{...}` for parameters:

```sql
SELECT * FROM log WHERE t BETWEEN {lo} AND {hi}
```

* Use `{&...}` for variables that will be treated as identifiers, which is to
  say, as the names of columns, types or tables:

```sql
INSERT INTO {&current_partition} VALUES (now(), {tag}, {location})
```
