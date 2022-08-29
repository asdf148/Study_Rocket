table! {
    comments (id) {
        id -> Int4,
        comment -> Nullable<Varchar>,
        user_id -> Int4,
        post_id -> Int4,
    }
}