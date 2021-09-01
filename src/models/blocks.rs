use crate::schema::blocks;

#[derive(Debug, Queryable, Identifiable)]
pub struct Block {
    pub id: i32,
    pub user_id: String,
    pub blocked_id: String
}

#[derive(Debug, Insertable)]
#[table_name = "blocks"]
pub struct NewBlock {
    pub user_id: String,
    pub blocked_id: String
}