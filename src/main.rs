use askama::Template;
use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(home))
        .route("/articles/{id}", get(article))
        .nest_service("/static", ServeDir::new("static"));

    let port = 3000;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

// Wrapper for rendering askama templates
struct HtmlTemplate<T: Template>(T);

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn home() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}

struct ArticleMeta<'a> {
    title: &'a str,
    #[allow(dead_code)]
    date: &'a str,
}

// Article templates
#[derive(Template)]
#[template(path = "articles/choosing-the-best-shape-with-differential-equations.html")]
struct ChoosingTheBestShapeWithDifferentialEquationsTemplate<'a> {
    meta: ArticleMeta<'a>,
}

async fn article(Path(id): Path<String>) -> Response {
    match id.as_str() {
        "choosing-the-best-shape-with-differential-equations" => {
            let template = ChoosingTheBestShapeWithDifferentialEquationsTemplate {
                meta: ArticleMeta {
                    title: "Choosing the best shape with differential equations",
                    date: "11 Aug, 2024",
                },
            };

            HtmlTemplate(template).into_response()
        }
        _ => (StatusCode::NOT_FOUND, "Article not found").into_response(),
    }
}
