pub mod config;

use std::fmt::Write;
use std::path::Path;
use std::path::PathBuf;

use actix_web::{ Result, HttpRequest, HttpResponse, fs, App, middleware, server };

fn file_handler(req: &HttpRequest<config::Config>) -> Result<fs::NamedFile> {
    let path = &req.state().path;
    Ok(fs::NamedFile::open(path)?)
}

fn directory_displayer<S>(dir: &fs::Directory, req: &HttpRequest<S>) -> Result<HttpResponse, std::io::Error> {
    let index = format!("Index Of {}", req.path());
    let mut body = String::new();
    let base = Path::new(req.path());
    for entry in dir.path.read_dir()? {
        if dir.is_visible(&entry) {
            let entry = entry.unwrap();
            let file_url = match entry.path().strip_prefix(&dir.path) {
                Ok(path) => base.join(path),
                Err(_) => continue,
            };

            let file_name = &entry.file_name();
            let file_name = file_name.to_str().unwrap();

            if let Ok(_meta_data) = entry.metadata() {
                let _ = write!(body, "<li><a href=\"{}\">{}/</a></li>", file_url.to_str().unwrap(), file_name);
            }
        }
    }

    let html = format!(
        "<html>\
        <head><title>{}</title></head>\
        <body><h1>{}</h1>\
        <ul>{}</ul>\
        </body></html>
        ",
        index, index, body
    );

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

fn configure_app(app: App<config::Config>) -> App<config::Config> {
    let s = {
        let path = &app.state().path;
        println!("path = {}", path.to_str().unwrap());
        if path.is_file() {
            None
        } else {
            Some(
                fs::StaticFiles::new(path)
                    .expect(&format!("Can't create the director: {:?}!", path))
                    .show_files_listing()
                    .files_listing_renderer(|dir, req| {
                        directory_displayer(dir, req)
                    })
            )
        }
    };

    if let Some(s) = s {
        app.handler("/", s)
    } else {
        app.resource("/", |r| r.f(file_handler))
    }
}

pub fn start_server() {
    let sys = actix::System::new("simple-http-server");

    let config = config::Config{
        path: PathBuf::from("D://")
    };

    let _server = server::new(move || {
        App::with_state(config.clone())
            .middleware(middleware::Logger::default())
            .configure(configure_app)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();

    let _ = sys.run();
}