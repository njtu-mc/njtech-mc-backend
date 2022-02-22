table! {
    users (id) {
        id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        mc_id -> Varchar,
        mc_name -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        referrer_id -> Nullable<Integer>,
    }
}
