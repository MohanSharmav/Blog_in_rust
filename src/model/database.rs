
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};

pub(crate) async fn selecting() ->Result<Vec<String> ,()>{



    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    select_all_from_table().await.expect("cant select");

    let mut vect=Vec::new();
    let  rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(&pool)
        .await.expect("Unable to");

    for row in rows{
        let name: String=row.get("name");
        vect.push(name.clone());
        println!("name is ⭐: {}",name)
    }


   println!(" ⭐⭐⭐⭐⭐⭐        ok {:?}",vect);
Ok(vect)
}


async fn select_all_from_table() -> Result<(),Error> {



    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let rows = sqlx::query("SELECT title,description,name FROM posts")
          .fetch_all(&pool)
          .await?;
    for row in rows{
        let title:String=row.get("title");
       let description: String = row.get("description");
        let name:String= row.get("name");
        println!("{}", title);
        println!("{}", description);
        println!("{}", name);
    }

    Ok(())
}