use actix_web::{get, web, HttpResponse, Responder, Scope};

// API logic will go here

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <title>Home Inventory</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 2em; }
                h1 { color: #2c3e50; }
                .container { max-width: 600px; margin: auto; }
                .item { padding: 0.5em 0; border-bottom: 1px solid #eee; }
            </style>
        </head>
        <body>
            <div class=\"container\">
                <h1>Home Inventory</h1>
                <p>Welcome! This is your starting point for a Rust-based inventory system.</p>
                <ul>
                    <li class=\"item\">Example item 1</li>
                    <li class=\"item\">Example item 2</li>
                </ul>
            </div>
        </body>
        </html>
    "#)
}

pub fn init_routes() -> Scope {
    web::scope("").service(index)
}
