# Procedural Macro Collection

## #[timestamps]

This macro will automatically append the following fields to a struct:

```rust
pub created_by: String,
pub created_at: NaiveDateTime,
pub updated_by: String,
pub updated_at: NaiveDateTime,
```

Within the framework, this macro is only applied to database models.

```rust
#[timestamps]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
```
