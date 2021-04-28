table! {
    definition (id) {
        id -> Int4,
        word -> Text,
        meaning -> Nullable<Array<Text>>,
        antonyms -> Nullable<Array<Text>>,
        synonyms -> Nullable<Array<Text>>,
    }
}
