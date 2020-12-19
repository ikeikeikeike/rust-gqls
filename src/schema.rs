table! {
    organizations (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        identifier -> Varchar,
        role -> Integer,
        is_authed -> Bool,
        is_published -> Bool,
        is_deleted -> Bool,
        deleted_at -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        who -> Nullable<Varchar>,
        identifier -> Varchar,
        role -> Integer,
        email -> Varchar,
        password -> Varchar,
        date_joined -> Datetime,
        last_login -> Nullable<Datetime>,
        is_company_receive_unread -> Bool,
        is_company_receive_information -> Bool,
        is_person_receive_information -> Bool,
        is_authed -> Bool,
        is_published -> Bool,
        is_pro -> Bool,
        is_deleted -> Bool,
        deleted_at -> Nullable<Datetime>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users_organizations (user_id, organization_id) {
        user_id -> Integer,
        organization_id -> Integer,
    }
}

joinable!(users_organizations -> organizations (organization_id));
joinable!(users_organizations -> users (user_id));

allow_tables_to_appear_in_same_query!(organizations, users, users_organizations,);
