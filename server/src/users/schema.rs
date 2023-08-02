diesel::table! {
    users(id) {
        id -> Integer,
        name -> VarChar,
        favorite_color -> Nullable<VarChar>,
    }
}
