table! {
    associations (id) {
        id -> Integer,
        created_at -> Text,
        updated_at -> Text,
        name -> Text,
    }
}

table! {
    configs (id) {
        id -> Integer,
        created_at -> Text,
        updated_at -> Text,
        base_path -> Text,
    }
}

table! {
    creators (id) {
        id -> Integer,
        created_at -> Text,
        updated_at -> Text,
        name -> Text,
        association_id -> Nullable<Integer>,
    }
}

table! {
    tags (id) {
        id -> Integer,
        created_at -> Text,
        updated_at -> Text,
        name -> Text,
    }
}

table! {
    works (id) {
        id -> Integer,
        created_at -> Text,
        updated_at -> Text,
        name -> Text,
        path -> Text,
        thumbnail -> Nullable<Text>,
        rate -> Integer,
    }
}

table! {
    works_creators (work_id, creator_id) {
        work_id -> Integer,
        creator_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    works_tags (work_id, tag_id) {
        work_id -> Integer,
        tag_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

joinable!(creators -> associations (association_id));
joinable!(works_creators -> creators (creator_id));
joinable!(works_creators -> works (work_id));
joinable!(works_tags -> tags (tag_id));
joinable!(works_tags -> works (work_id));

allow_tables_to_appear_in_same_query!(
    associations,
    configs,
    creators,
    tags,
    works,
    works_creators,
    works_tags,
);
