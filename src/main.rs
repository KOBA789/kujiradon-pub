mod db;
mod query;

fn main() -> anyhow::Result<()> {
    let mut db = db::DB::connect("qp.koba789.com:8124")?;
    let user_key = db::UserKey { user_id: 789 };
    let item = db.get_item(db::TABLE_USERS, user_key.into())?.unwrap();
    println!("{}", item.value);
    Ok(())
}
