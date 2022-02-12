table! {
    user (id) {
        id -> Integer,
        create_at -> Datetime,
        update_at -> Datetime,
        name -> Varchar,
        email -> Varchar,
        referrer_id -> Integer,
        mc_id -> Varchar,
    }
}
