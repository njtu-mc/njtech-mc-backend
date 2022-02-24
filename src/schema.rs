table! {
    users (id) {
        id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        mc_id -> Varchar,
        mc_name -> Varchar,
        gender -> Integer,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        njtech_open_id -> Nullable<Varchar>,
        school -> Nullable<Varchar>,
    }
}
