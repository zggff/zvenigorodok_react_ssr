use std::path::PathBuf;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    get, http::StatusCode, middleware, web, web::scope, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use clap::Parser;
use mongodb::Client;
use once_cell::sync::OnceCell;

mod api;
mod cache;
mod ssr;
static SSR: OnceCell<ssr::Ssr> = OnceCell::new();
static DIST: OnceCell<PathBuf> = OnceCell::new();

#[derive(Debug, Parser)]
struct Args {
    /// port for the application
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    /// frontend dist directory
    #[arg(short, long, default_value_t = String::from("./client/dist"))]
    dir: String,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let args = Args::parse();
    let client_path = PathBuf::from(args.dir);
    DIST.set(client_path.clone()).unwrap();

    {
        // initialize Server side rendering
        let polyfill = r##"function TextEncoder(){}function TextDecoder(){}TextEncoder.prototype.encode=function(e){for(var o=[],t=e.length,r=0;r<t;){var n=e.codePointAt(r),c=0,f=0;for(n<=127?(c=0,f=0):n<=2047?(c=6,f=192):n<=65535?(c=12,f=224):n<=2097151&&(c=18,f=240),o.push(f|n>>c),c-=6;c>=0;)o.push(128|n>>c&63),c-=6;r+=n>=65536?2:1}return o},TextDecoder.prototype.decode=function(e){for(var o="",t=0;t<e.length;){var r=e[t],n=0,c=0;if(r<=127?(n=0,c=255&r):r<=223?(n=1,c=31&r):r<=239?(n=2,c=15&r):r<=244&&(n=3,c=7&r),e.length-t-n>0)for(var f=0;f<n;)c=c<<6|63&(r=e[t+f+1]),f+=1;else c=65533,n=e.length-t;o+=String.fromCodePoint(c),t+=n+1}return o};"##;
        let code = std::fs::read_to_string(client_path.as_path().join("index.js"))
            .expect("no js file found");
        let entrypoint = "SSR";
        let result = format!("{};{};{}", polyfill, code, entrypoint);

        let script = std::fs::read_dir(client_path.as_path().join("scripts"))
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|f| {
                f.path().to_str().and_then(|d| {
                    if d.ends_with("bundle.js") {
                        Some(f.file_name().to_string_lossy().to_string())
                    } else {
                        None
                    }
                })
            })
            .next()
            .expect("no js file");
        let style = std::fs::read_dir(client_path.as_path().join("styles"))
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|f| {
                f.path().to_str().and_then(|d| {
                    if d.ends_with("ssr.css") {
                        Some(f.file_name().to_string_lossy().to_string())
                    } else {
                        None
                    }
                })
            })
            .next()
            .expect("no css file");

        let result = result
            .replace("ssr.css", &style)
            .replace("bundle.js", &script);

        ssr::Ssr::initialize();
        SSR.set(ssr::Ssr::new(result))
            .expect("failed to set global variable");
    }

    let uri = std::env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    let coll_name = std::env::var("COLL_NAME").unwrap_or("reviews".into());
    let db_name = std::env::var("DB_NAME").unwrap_or("zvenigorodok".into());
    let collection: mongodb::Collection<api::Review> =
        client.database(&db_name).collection(&coll_name);

    HttpServer::new(move || {
        let cors = if cfg!(debug_assertions) {
            Cors::permissive()
        } else {
            Cors::default().allowed_methods(vec!["GET", "POST"])
        };

        App::new()
            .app_data(web::Data::new(collection.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .service(
                scope("/styles")
                    .wrap(cache::CacheInterceptor::new(7))
                    .service(
                        fs::Files::new("", client_path.as_path().join("styles/"))
                            .show_files_listing(),
                    ),
            )
            .service(
                scope("/images")
                    .wrap(cache::CacheInterceptor::new(31))
                    .service(
                        fs::Files::new("", client_path.as_path().join("images/"))
                            .show_files_listing(),
                    ),
            )
            .service(
                scope("/scripts")
                    .wrap(cache::CacheInterceptor::new(7))
                    .service(
                        fs::Files::new("", client_path.as_path().join("scripts/"))
                            .show_files_listing(),
                    ),
            )
            .service(sitemap)
            .service(api::api("/api"))
            .service(index)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await?;
    Ok(())
}

#[get("/sitemap")]
async fn sitemap() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open(
        DIST.get().unwrap().as_path().join("sitemap.xml"),
    )?)
}

#[get("{url}*")]
async fn index(req: HttpRequest) -> impl Responder {
    let props = format!(
        r##"{{
            "location": "{}",
            "context": {{}}
        }}"##,
        req.uri()
    );

    let html = SSR.get().unwrap().render_to_string(Some(&props));

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}
