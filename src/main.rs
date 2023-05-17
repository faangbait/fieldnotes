use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tera::{Tera,Context};
use warp::Filter;
use comrak::{markdown_to_html, ComrakOptions};
use chrono::prelude::{DateTime, NaiveDateTime};
use chrono::Utc;

const CONTENT_DIR: Dir = include_dir!("src/content");
const METADATA: &str = include_str!("fieldnotes.json");
const STATIC_DIR: Dir = include_dir!("src/static");

#[derive(Serialize, Clone)]
pub struct FieldNote {
    date_epoch: u64,
    title: String,
    body: String,
    slug: String
}

impl FieldNote {
    pub fn render(&self, maybe_context: Option<Context>) -> impl warp::Reply {
        let tera = preload_templates();
        let mut ctx = match Context::from_serialize(&self) {
            Ok(c) => match maybe_context {
                Some(mut cx) => {
                    cx.extend(c);
                    cx
                },
                None => c
            },
            Err(e) => panic!("Serialization error(s): {}", e),
        };

        let naive = NaiveDateTime::from_timestamp(self.date_epoch.try_into().unwrap(),0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        ctx.insert("date", &datetime.format("%B %-d, %Y").to_string());

        match tera.render("note.html", &ctx) {
            Ok(rend) => warp::reply::html(rend),
            Err(e) => warp::reply::html(e.to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct FieldMetadata {
    title: String,
    date: u64,
    slug: String,
}

#[tokio::main]
async fn main() {
    STATIC_DIR.extract("/tmp/fieldnotes").expect("Couldn't extract field notes.");
    
    let staticfiles = warp::path("static").and(warp::fs::dir("/tmp/fieldnotes"));
    let favicon = warp::path("favicon.ico").and(warp::fs::dir("/tmp/fieldnotes/favicon.ico"));
    
    let tera = preload_templates();
    let fieldnotes = preload_notes();
    let notes_sliced = fieldnotes.iter().map(|(_,v)| { v }).collect::<Vec<&FieldNote>>();
    
    let mut context = get_default_context().unwrap();
    context.insert("entries", &notes_sliced);
    context.insert("title", "Field Notes");
    
    let index = warp::path::end().map(move || warp::reply::html(tera.render("index.html", &context).unwrap()));
    
    let notes = warp::path::param()
        .map(move |path:String| {
            let note_context = get_default_context();
            match fieldnotes.get(&path) {
                Some(note) => note.render(note_context),
                None => FieldNote {
                    date_epoch: 0,
                    title: "End of the Road".to_string(),
                    slug: "end-of-the-road".to_string(),
                    body: String::new()
                }.render(note_context)
            }
        });
    
    let routes = warp::get().and(
        staticfiles
        .or(favicon)
        .or(index)
        .or(notes)
    );

    warp::serve(routes).run(([127,0,0,1],3030)).await;
}

fn get_default_context() -> Option<Context> {
    Some(Context::new())
}

fn preload_templates() -> Tera {
    let mut tera = Tera::default();

    tera.add_raw_templates(vec![
        ("base.html", include_str!("templates/base.html")),
        ("index.html", include_str!("templates/index.html")),
        ("footer.html", include_str!("templates/footer.html")),
        ("nav.html", include_str!("templates/nav.html")),
        ("note.html", include_str!("templates/note.html")),
    ])
    .expect("Error: couldn't preload templates");

    tera
}

fn preload_notes() -> HashMap<String, FieldNote> {
    let mut notes_hash = HashMap::<String, FieldNote>::new();
    
    let notes_metadata: Vec<FieldMetadata> =
        serde_json::from_str(METADATA).expect("JSON not well formatted");
    
    for entry in notes_metadata {
        let preloaded = CONTENT_DIR.get_file(entry.slug.clone() + ".md");
        let entry_body = match preloaded {
            Some(f) => match f.contents_utf8() {
                Some(b) => markdown_to_html(b, &ComrakOptions::default()),
                None => String::new(),
            },
            None => String::new(),
        };
        
        notes_hash.insert(
            entry.slug.clone(),
            FieldNote {
                date_epoch: entry.date,
                title: entry.title,
                body: entry_body,
                slug: entry.slug.clone()
            },
        );
    }

    notes_hash
}
