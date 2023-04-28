mod model;
mod controller;

use std::env::Args;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::io::Read;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, web};
use actix_web::http::StatusCode;
use tokio::select;
use warp::reply::with_status;
use controller::home_page::get_all_posts;
use model::database::selecting;
use warp::{get, Rejection, Reply};

async fn index(req: HttpRequest)-> Result<NamedFile>{
     let path= Path::new("templates/index.hbs");
     Ok(NamedFile::open(path)?)
}

#[tokio::main]
async fn main() -> Result<()>{

//test start
     get_all_posts().await;


selecting();
//
//      let mut vector=Vec::new();
// vector= selecting();
//      print_type_of(vector);
    //  vector=selecting();
    // 
    // 
    // println!({})

//       let mut x =selecting();
//      x.read_to_string().expect("TODO: panic message");
// //      let string1: String = x.await.iter().collect::<String>();
// println!("come onn{}",string1);
     // println!("{:?}",x);
     // vector.push(selecting);
     // println!("{}", vector);
// test end
     HttpServer::new(|| {
          App::new()
              .service(web::resource("/").to(index))
              .service(web::resource("/hi").to(index))
              .service(web::resource("/hello").to(get_all_posts))
     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
