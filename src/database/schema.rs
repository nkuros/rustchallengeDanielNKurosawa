table! {
    nodes (public_key) {
        public_key -> Text,
        alias -> Text,
        channels -> BigInt,
        capacity -> BigInt,
        first_seen -> BigInt,
        updated_at -> BigInt,
    }
}
