use rustorm::{
    DbError,
    FromDao,
    Pool,
    ToColumnNames,
    ToTableName,
};

#[derive(Debug, FromDao, ToColumnNames, ToTableName)]
struct Actor {
    actor_id: i32,
    first_name: String,
}

fn main() {
 //   #[cfg(feature = "with-mysql")]
    let db_url = "mysql://app:app123@127.0.0.1/app";
    let mut pool = Pool::new();
    let mut em = pool
        .em(db_url)
        .expect("Should be able to get a connection here..");
    let sql = "SELECT * FROM actor LIMIT 10";
    let actors: Result<Vec<Actor>, DbError> = em.execute_sql_with_return(sql, &[]);
    println!("Actor: {:#?}", actors);
    let actors = actors.unwrap();
    assert_eq!(actors.len(), 10);
    for actor in actors {
        println!("actor: {:?}", actor);
    }
}