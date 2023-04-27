use std::collections::HashMap;
use std::fs;
use actix_web::HttpResponse;

pub async fn get_all_posts()-> HttpResponse
{
    println!("⭐⭐⭐⭐⭐⭐  Getting all posts");

let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();

let vec=vec![1,2,2,2];

    let mut data =HashMap::new();
    // data.insert("name","boss");
    // data.insert("1","jos");
    data.insert("2",vec);
    println!("{:?}",data);
    let html = handlebars.render("index", &data).unwrap();

    //test  start
//test end
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}