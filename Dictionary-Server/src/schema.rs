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
    meaning (meaning_id) {
        meaning_id -> Int4,
        word -> Text,
        def -> Array<Text>,
        keywords -> Array<Text>,
    }
}

joinable!(definition -> meaning (meaning_id));

allow_tables_to_appear_in_same_query!(
    definition,
    meaning,
);
