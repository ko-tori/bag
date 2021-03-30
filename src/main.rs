use actix_web::{web, App, HttpServer, HttpResponse, web::Form};
use std::sync::Mutex;
use rand::prelude::*;
use serde::Deserialize;

static CSS: &str = r#"
<link rel="preconnect" href="https://fonts.gstatic.com">
<link href="https://fonts.googleapis.com/css2?family=New+Tegomin&display=swap" rel="stylesheet">
<style>
body {
	font-family: 'New Tegomin', serif;
	padding: 5px;
	font-size: 2em;
	text-align: center;
}
</style>
"#;

struct AppState {
	list: Mutex<Vec<String>>,
}

#[derive(Deserialize)]
struct FormData {
	question: String,
}

async fn home() -> HttpResponse {
    HttpResponse::Ok()
    	.content_type("text/html")
    	.body(CSS.to_owned() + "<p>Enter your question here:</p><form method='POST'><input name='question' size='40'>&nbsp;<input type='submit'></form>")
}

async fn stuff(form: Form<FormData>, data: web::Data<AppState>) -> HttpResponse {
	let mut qlist = data.list.lock().unwrap();
    qlist.push(form.question.clone());
    println!("New question submitted! Currently {:?} total.", qlist.len());
	HttpResponse::Ok()
		.content_type("text/html")
		.body(CSS.to_owned() + "<p>Question Submitted!</p><form method='POST' action='/q'><input type='submit' value='Get Random Question!'></form>")
}

async fn getq(data: web::Data<AppState>) -> HttpResponse {
    let mut qlist = data.list.lock().unwrap();
    let l = qlist.len();
    if l == 0 {
    	HttpResponse::Ok()
    		.content_type("text/html")
    		.body(CSS.to_owned() + "No questions left!")
    } else {
    	let mut rng = rand::thread_rng();
    	HttpResponse::Ok()
    		.content_type("text/html")
    		.body(CSS.to_owned() + &qlist.remove(rng.gen_range(0..l)))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let qlist = web::Data::new(AppState {
        list: Mutex::new(Vec::new()),
    });
    HttpServer::new(move || {
        App::new()
        	.app_data(qlist.clone())
            .route("/", web::get().to(home))
            .route("/", web::post().to(stuff))
            .route("/q", web::post().to(getq))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}