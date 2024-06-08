pub mod db;

use std::{net::SocketAddr, path::PathBuf};

use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use clap::{arg, command, Parser};
use db::{Book, Database};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use tower::util::ServiceExt;
use tower_http::services::ServeFile;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Ip and port for the webserver
    host: SocketAddr,
    /// Directory for the static assets
    #[arg(short, long, default_value = "./assets")]
    assets: PathBuf,
}

#[tokio::main]
async fn main() {
    logging();

    let Args { host, assets } = Args::parse();

    let db = db::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/book", post(add_book).with_state(db.clone()))
        .route("/book/add", get(get_add_book))
        .route(
            "/book/*id",
            get(show_book)
                .put(edit_book)
                .delete(delete_book)
                .with_state(db.clone()),
        )
        .route("/search", post(fetch_books).with_state(db))
        .route("/*file", get(static_assets).with_state(assets));

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Initialize tracing
fn logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Header of the Page
fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="UTF-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        title { (page_title) }
        script src="/static/content/node_modules/htmx.org/dist/htmx.min.js" {}
        link href="/static/content/dist/output.css" rel="stylesheet";
    }
}

/// Navbar of the Page
fn navbar() -> Markup {
    html! {
        div class="navbar bg-base-300 shadow-xl" {
            div class="flex justify-between w-full" {
                a class="btn btn-ghost text-xl" href="/" {
                    "db"
                }
                a  title="Github" class="btn btn-ghost btn-square text-xl" href="https://github.com/nwrenger/" target="_blank" {
                    svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                        path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" {}
                        path d="M9 18c-4.51 2-5-2-7-2" {}
                    }
                }
            }
        }
    }
}

/// The Page, useful when wanting to use multiple Pages
fn page(title: &str, html: Markup) -> Markup {
    html! {
        head {
            (header(title))
        }
        body {
            div class="grid grid-rows-[64px_auto] h-full" {
                div {
                    (navbar())
                }
                div class="h-full overflow-scroll" {
                    div class="container space-y-8 flex flex-col items-center !max-w-6xl mx-auto p-4" {
                        (html)
                    }
                }
            }
        }
    }
}

/// The Index Page
async fn root() -> Markup {
    page(
        "db",
        html! {
            section class="p-2 w-full" {
                div class="label" {
                        span class="label-text-alt" {
                           "Search"
                        }
                }
                    div class="join w-full" {
                        input
                            id="search"
                            class="input input-bordered join-item w-full"
                            type="search"
                            name="search"
                            placeholder="Begin Typing To Search Books..."
                            hx-post="/search"
                            hx-trigger="input changed delay:500ms, search, load, update from:body"
                            hx-target="#search-results"
                            hx-indicator=".htmx-indicator";
                        button class="btn join-item" hx-get="/book/add" hx-target="#book-display" hx-indicator=".htmx-indicator" {"Add Book"}
                    }
                    div class="label" {
                        span class="htmx-indicator label-text-alt" {
                            "Loading..."
                        }
                    }
                    div id="search-results" {}
                    div id="book-display" class="overflow-x-scroll" {}
            }
        },
    )
}

/// Search Form, needed for Axum to serialize it's forms
#[derive(Deserialize)]
struct SearchForm {
    search: String,
}

/// Get the books and show them in a list
async fn fetch_books(State(db): State<Database>, Form(form): Form<SearchForm>) -> Markup {
    let books = db.search_book(&form.search);
    html! {
        table class="table table-pin-rows table-pin-cols" {
            thead {
                tr {
                    th { "ID" }
                    th {" Title" }
                }
            }
            tbody {
                @for book in &books {
                    tr class="hover" hx-get={ "/book/" (book.id)} hx-target="#book-display" hx-indicator=".htmx-indicator" {
                        td { (book.id) }
                        td { (book.title) }
                    }
                }
                @if books.is_empty() {
                    tr class="opacity-50" {
                        th { "No results!" }
                    }
                }
            }
        }
    }
}

/// Show a Single Book
async fn show_book(State(db): State<Database>, Path(path): Path<String>) -> Markup {
    let id = path.parse::<usize>().unwrap_or_default();
    let book = db.get_book(&id).unwrap_or_default();
    book_with_edit_buttons(&book)
}

/// Show an Empty Book with an Add Button
async fn get_add_book() -> Markup {
    book_with_add_button()
}

/// Action when a book is added, opens it directly
async fn add_book(State(db): State<Database>, Form(book): Form<Book>) -> impl IntoResponse {
    if let Some(new_book) = db.add_book(book) {
        let mut headers = HeaderMap::default();
        headers.insert("HX-Trigger", HeaderValue::from_static("update"));

        (StatusCode::OK, headers, book_with_edit_buttons(&new_book))
    } else {
        (
            StatusCode::OK,
            HeaderMap::default(),
            html! {
                (book_with_add_button())
                div role="alert" class="alert alert-error" {
                    svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24" { path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z";}
                    span { "Error! Adding Book failed: 'Bad Request'." }
                }
            },
        )
    }
}

/// Action when a book is edited, opens it directly
async fn edit_book(
    State(db): State<Database>,
    Path(path): Path<String>,
    Form(book): Form<Book>,
) -> impl IntoResponse {
    let id = path.parse::<usize>().unwrap_or_default();

    if let Some(new_book) = db.edit_book(&id, book) {
        let mut headers = HeaderMap::default();
        headers.insert("HX-Trigger", HeaderValue::from_static("update"));

        (StatusCode::OK, headers, book_with_edit_buttons(&new_book))
    } else {
        if let Some(old_book) = db.get_book(&id) {
            (
                StatusCode::OK,
                HeaderMap::default(),
                html! {
                    (book_with_edit_buttons(&old_book))
                    div role="alert" class="alert alert-error" {
                        svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24" { path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z";}
                        span { "Error! Editing Book failed: 'Bad Request'." }
                    }
                },
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::default(),
                html!(),
            )
        }
    }
}

/// Action when a book is deleted, closes it directly
async fn delete_book(
    State(db): State<Database>,
    Path(path): Path<String>,
    Form(book): Form<Book>,
) -> impl IntoResponse {
    let id = path.parse::<usize>().unwrap_or_default();
    if db.delete_book(&id).is_some() {
        let mut headers = HeaderMap::default();
        headers.insert("HX-Trigger", HeaderValue::from_static("update"));

        (StatusCode::OK, headers, html!())
    } else {
        (
            StatusCode::OK,
            HeaderMap::default(),
            html! {
                (book_with_edit_buttons(&book))
                    div role="alert" class="alert alert-error" {
                        svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24" { path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z";}
                        span { "Error! Deleting Book failed: 'Bad Request'." }
                    }
            },
        )
    }
}

/// Book Display with `Add` Button
fn book_with_add_button() -> Markup {
    html! {
        form hx-post="/book" hx-target="#book-display" hx-indicator=".htmx-indicator" {
            (book_display(&Book::default()))
            button class="btn btn-outline btn-primary mt-2" type="submit"{ "Add" }
        }
    }
}

/// Book Display with `Edit` && `Delete` Button
fn book_with_edit_buttons(book: &Book) -> Markup {
    html! {
        form hx-put={ "/book/" (book.id) } hx-target="#book-display" hx-indicator=".htmx-indicator" {
            (book_display(&book))
            div  class="space-x-2 mt-2" {
                button class="btn btn-outline btn-primary" type="submit" { "Submit" }
                button class="btn btn-outline btn-danger" type="button" hx-Delete={ "/book/" (book.id) } hx-target="#book-display" hx-indicator=".htmx-indicator"{ "Delete" }
            }
        }
    }
}

/// General Book Display
fn book_display(book: &Book) -> Markup {
    html! {
        label class="form-control w-full" for="search" {
            div class="label" {
                span class="label-text" { "Title" }
            }
            input
                name="title"
                class="input input-bordered w-full"
                type="text"
                placeholder="Title"
                value={ (book.title) };
        }
        label class="form-control w-full" for="search" {
             div class="label" {
                span class="label-text" { "ID" }
             }
            input
                name="id"
                class="input input-bordered w-full"
                type="number"
                placeholder="ID"
                value={ (book.id) };
        }
        label class="form-control w-full" for="search" {
            div class="label" {
                span class="label-text" { "Author" }
            }
            input
                name="author"
                class="input input-bordered w-full"
                type="text"
                placeholder="Author"
                value={ (book.author) };
        }
        label class="form-control w-full" for="search" {
            div class="label" {
                span class="label-text" {"Price"}
            }
            input
                name="price"
                class="input input-bordered w-full"
                type="text"
                pattern="[0-9]*[.,]?[0-9]+"
                placeholder="Price"
                value={ (book.price) };
        }
    }
}

/// Mounting Static Files
async fn static_assets(
    State(dir): State<PathBuf>,
    Path(file): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let path = dir.join(&file);
    ServeFile::new(path)
        .oneshot(req)
        .await
        .unwrap()
        .into_response()
}
