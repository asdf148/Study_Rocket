table! {
    comments (id) {
        id -> Integer,
        image -> Varchar,
        title -> Varchar,
        content -> Text,
        user_id -> Integer,
        post_id -> Integer,
    }
}