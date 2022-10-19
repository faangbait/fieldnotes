use std::collections::HashMap;
use chrono::prelude::{DateTime, NaiveDateTime};
use chrono::Utc;
use comrak::{markdown_to_html, ComrakOptions};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use warp::Filter;

#[tokio::main]
async fn main() {
    let (css, js, staticfiles, favicon) = (
        warp::path("js").and(warp::fs::dir("js")),
        warp::path("static").and(warp::fs::dir("static")),
        warp::path("favicon.ico").and(warp::fs::file("favicon.ico")),
        warp::path("css").and(warp::fs::dir("css")),
    );

    let fieldnotes = get_fieldnotes();
    let mut context = get_default_context().unwrap();
    context.insert("entries", &fieldnotes);
    context.insert("title", "Field Notes");

    let index = warp::path::end().map(move || {
        warp::reply::html(TEMPLATES.render("index.html", &context).unwrap())
    });

    let notes = warp::path::param()
        .map(move |path: String| {
            let note_context = get_default_context();
            
            match fieldnotes.get(&path) {
                Some(note) => note.render(note_context),
                None => FieldNote {
                    date_epoch: 0,
                    title: "End of the Road".to_string(),
                    slug: "end-of-the-road".to_string(),
                    body: String::new(),
                }.render(note_context)
            }
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

fn get_default_context() -> Option<Context> {
    Some(Context::new())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldNote {
    date_epoch: i64,
    title: String,
    slug: String,
    body: String,
}

impl FieldNote {
    pub fn new(
        date_epoch: i64,
        title: String,
        slug: String,
    ) -> Self {
        Self {
            date_epoch,
            title: title.to_string(),
            slug: slug.to_string(),
            body: match Tera::new(r#"src/notes/**/*"#) {
                Ok(t) => match t.render(&slug, &Context::new()) {
                    Ok(rend) => markdown_to_html(&rend, &ComrakOptions::default()),
                    Err(e) => {
                        println!("Rendering error(s): {}", e);
                        String::from("Rendering error(s)")
                    }
                },
                Err(e) => {
                    println!("Rendering error(s): {}", e);
                    String::from("Rendering error(s)")
                }
            },
        }
    }
    pub fn render(&self, maybe_context: Option<Context>) -> impl warp::Reply {
        let mut ctx = match Context::from_serialize(&self) {
            Ok(c) => match maybe_context {
                Some(mut cx) => {
                    cx.extend(c);
                    cx
                },
                None => c,
            },
            Err(e) => panic!("Serialization error(s): {}",e)
        };

        let naive = NaiveDateTime::from_timestamp(self.date_epoch,0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        ctx.insert("date", &datetime.format("%B %-d, %Y").to_string());

        let base  = TEMPLATES.render("note.html", &ctx);
        warp::reply::html(match base {
            Ok(b) => b,
            Err(b) => {
                println!("Rendering error(s): {}", b);
                String::new()
            }
        })
    }
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

/// Metadata entry and data reconstruction
fn get_fieldnotes() -> HashMap<String,FieldNote> {
    
    let notes = vec![
        // slug, title, date epoch
        ("silence", "My repository sits in silence.",1666217144),
        ("test-syntax", "syntax highlighting example ", 1646217544),
    ];
    
    let mut notes_hash: HashMap<String,FieldNote> = HashMap::<String,FieldNote>::new();

    notes.iter().for_each(|f| {
        notes_hash.insert(f.0.to_string(), FieldNote::new(f.2, f.1.to_string(),f.0.to_string()));
    });

    notes_hash
}
