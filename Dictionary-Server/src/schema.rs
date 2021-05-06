table! {
    definition (word_id) {
        word_id -> Int4,
        word -> Text,
        meaning_id -> Int4,
        antonyms -> Array<Text>,
        synonyms -> Array<Text>,
    }
}

table! {
    meaning (id) {
        id -> Int4,
        def -> Array<Text>,
        keywords -> Array<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    definition,
    meaning,
);
