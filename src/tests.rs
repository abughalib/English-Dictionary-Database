#[cfg(test)]
mod tests {

    use crate::models::{NewDefinition, NewMeaning};
    use crate::routes::*;
    use crate::{database_op::establish_connection, models::QueryWord};
    use actix_web::{http::StatusCode, web::Json};
    use diesel::result::Error;
    use diesel::Connection;
    use serial_test::serial;

    #[actix_rt::test]
    #[serial]
    async fn test_index() {
        let resp = index().await;

        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "Check Used port is not used by other application \
    or this this application config"
        );
    }

    #[actix_rt::test]
    #[serial]
    async fn test_json_query_page_meaning_not_found() {
        let info = QueryWord {
            word: "ejgiofdpsoigs8943t34543".to_string(),
        };

        let info_json = Json::<QueryWord>(info);
        let resp = query_meaning(info_json).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    #[serial]
    fn test_database_insertion() {
        let mut conn = establish_connection();
        let mut conn2 = establish_connection();

        let new_meaning = NewMeaning {
            word: "some_unknown_word",
            def: vec!["Unknown", "Definition not known"],
            keywords: vec!["unknown"],
        };

        use crate::schema::definition::dsl::definition;
        use crate::schema::meaning::dsl::meaning;
        use diesel::RunQueryDsl;

        conn.test_transaction::<_, Error, _>(|_| {
            diesel::insert_into(meaning)
                .values(new_meaning)
                .execute(&mut conn2)
                .ok();
            Ok(())
        });

        let new_def = NewDefinition {
            word: "some_unknown_word",
            meaning_id: &1i32,
            synonyms: vec!["if any"],
            antonyms: vec!["if any"],
        };

        conn.test_transaction::<_, Error, _>(|_| {
            diesel::insert_into(definition)
                .values(new_def)
                .execute(&mut conn2)
                .ok();

            Ok(())
        });
    }

    #[test]
    #[serial]
    fn test_database_deletion() {
        let mut conn = establish_connection();
        let mut conn2 = establish_connection();

        use crate::schema::definition::dsl::{definition, word_id};
        use crate::schema::meaning::dsl::{meaning, meaning_id};
        use diesel::prelude::*;

        conn.test_transaction::<_, Error, _>(|_| {
            diesel::delete(meaning)
                .filter(meaning_id.eq(1))
                .execute(&mut conn2)
                .ok();

            Ok(())
        });

        conn.test_transaction::<_, Error, _>(|_| {
            diesel::delete(definition)
                .filter(word_id.eq(1))
                .execute(&mut conn2)
                .ok();

            Ok(())
        });
    }
}
