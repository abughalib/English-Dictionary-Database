// @generated automatically by Diesel CLI.

diesel::table! {
    definition (word_id) {
        word_id -> Int4,
        word -> Text,
        meaning_id -> Int4,
        antonyms -> Array<Nullable<Text>>,
        synonyms -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    meaning (meaning_id) {
        meaning_id -> Int4,
        word -> Text,
        def -> Array<Nullable<Text>>,
        keywords -> Array<Nullable<Text>>,
    }
}

diesel::joinable!(definition -> meaning (meaning_id));

diesel::allow_tables_to_appear_in_same_query!(
    definition,
    meaning,
);
