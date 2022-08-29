table! {
    posts (id) {
        id -> Int4,
        image -> VarChar,
        title -> Nullable<Varchar>,
        content -> Text,
        user_id -> Int4,
    }
}