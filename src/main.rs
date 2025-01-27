use axum::{
    extract::State,
    routing::post,
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response, sse::{Event, Sse}},
};
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, net::SocketAddr, convert::Infallible};
use tower_http::{cors::CorsLayer, services::ServeDir};
use futures::StreamExt;

const OLLAMA_API_BASE: &str = "http://localhost:11434/api";
const SERVER_PORT: u16 = 8080;

#[derive(Debug, Serialize)]
struct ApiError {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    prompt: String,
}

#[derive(Serialize, Deserialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct OllamaResponse {
    response: String,
    done: bool,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
}

async fn chat(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, ApiError> {
    let ollama_req = GenerateRequest {
        model: req.model,
        prompt: req.prompt,
        stream: false,
    };

    let response = state
        .client
        .post(format!("{}/generate", OLLAMA_API_BASE))
        .json(&ollama_req)
        .send()
        .await
        .map_err(|e| ApiError {
            message: format!("Failed to connect to Ollama: {}. Make sure Ollama is running with 'ollama serve'", e),
        })?;

    let text = response.text().await.map_err(|e| ApiError {
        message: format!("Failed to get response text: {}", e),
    })?;

    // Parse each line and collect responses, handling whitespace
    let full_response: String = text
        .lines()
        .filter_map(|line| {
            serde_json::from_str::<OllamaResponse>(line).ok()
        })
        .map(|res| res.response.trim().to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();

    Ok(Json(ChatResponse {
        response: full_response,
    }))
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        client: reqwest::Client::new(),
    });

    let app = Router::new()
        .route("/api/chat", post(chat))
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], SERVER_PORT));
    println!("Starting server on http://localhost:{}", SERVER_PORT);
    println!("Make sure Ollama is running with 'ollama serve' in another terminal");
    
    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap(),
        app
    )
    .await
    .unwrap();
}