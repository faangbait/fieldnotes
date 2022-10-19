mod mn;

use std::os::linux::fs::MetadataExt;
use glob::glob;
use tera::{Tera, Context};
use warp::Filter;
use lazy_static::lazy_static;
use comrak::{markdown_to_html, ComrakOptions};

#[tokio::main]
async fn main() {
    let css = warp::path("css").and(warp::fs::dir("css"));
    let js = warp::path("js").and(warp::fs::dir("js"));
    let staticfiles = warp::path("static").and(warp::fs::dir("static"));
    let favicon = warp::path("favicon.ico").and(warp::fs::file("favicon.ico"));

    let index = warp::path::end().map(|| {
        let mut context = get_default_context();
        context.insert("entries", &get_notes());
        render_base("index.html", &context)
    });

    let notes = warp::path::param()
    .map(|path:String| {
        let mut context = get_inner_context();
        let rendered = render_note(path, &context);
        context.insert("note", &rendered);
        render_base("note.html", &context)
    });

    let routes = warp::get().and(
        staticfiles
        .or(favicon)
        .or(css)
        .or(js)
        .or(index)
        .or(notes)
        );
    
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn get_default_context() -> Context {
    let mut ctx = Context::new();
    ctx.insert("entries", &get_notes());
    ctx
}

fn get_inner_context() -> Context {
    Context::new()
}

fn get_notes() -> Vec<String> {
    let paths: Result<Vec<_>,_> = glob("src/notes/*").expect("Couldn't read glob pattern").collect();
    if let Ok(mut path) = paths {
        path.sort_by(|a,b| {
            let a = a.metadata().expect("metadata call failed");
            let b = b.metadata().expect("metadata call failed");
            return a.st_ctime().cmp(&b.st_ctime())
        });

        path.iter().map(|p| String::from(p.file_name().unwrap().to_str().unwrap())).collect()
    } else {
        vec![]
    }
}

/// Renders the content of a note given its path
fn render_note(path: String, context: &Context) -> String {
    match Tera::new(r#"src/notes/**/*"#) {
        Ok(t) => match t.render(&path, context) {
            Ok(rend) => markdown_to_html(&rend, &ComrakOptions::default()),
            Err(e) => {
                println!("Rendering error(s): {}", e);
                String::new()
            },
        },
        Err(e) => {
            println!("Rendering error(s): {}", e);
            String::new()
        },
    }
}

/// Renders a full reply, including wrapper html
fn render_base(template_name: &str, context: &Context) -> warp::reply::Html<String> {
    let base = TEMPLATES.render(template_name, context);
    warp::reply::html(
        match base {
            Ok(b) => b,
            Err(b) => {
                println!("Rendering error(s): {}", b);
                String::new()
            },
        }
    )
}

// templating
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}
