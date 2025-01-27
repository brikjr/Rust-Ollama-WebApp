# Rust Ollama Web Chat

A sleek web interface for interacting with Ollama language models, built with Rust and modern web technologies.

## Prerequisites

- Rust (latest stable version)
- [Ollama](https://ollama.ai) installed
- Cargo (Rust's package manager)

## Setup

1. Clone the repository:
```bash
git clone <your-repo-url>
cd rust-ollama-webapp
```

2. Create the project structure:
```bash
mkdir static
```

3. Place the files in their respective directories:
- `src/main.rs` - Backend server code
- `static/index.html` - Frontend interface
- `Cargo.toml` - Project dependencies

4. Install the required dependencies:
```bash
cargo build
```

## Running the Application

1. Start the Ollama server in a terminal:
```bash
ollama serve
```

2. In a different terminal, start the web application:
```bash
cargo run
```

3. Open your web browser and navigate to:
```
http://localhost:8080
```

## Usage

1. The application will start with the default model (deepseek-r1:7b)
2. You can change the model in the input field at the top
3. Type your message in the input field at the bottom
4. Press Enter or click Send to chat with the model
5. The chat history will be displayed in the main window

## Project Structure

```
rust-ollama-webapp/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   └── main.rs         # Rust backend server code
└── static/
    └── index.html      # Frontend web interface
```

## Dependencies

### Backend (Rust)
- tokio
- axum
- serde
- reqwest
- tower-http
- anyhow

### Frontend
- TailwindCSS (via CDN)
- Modern browser with JavaScript enabled

## Configuration

- Default port: 8080 (can be modified in both frontend and backend code)
- Default Ollama API endpoint: http://localhost:11434/api

## Error Handling

The application includes comprehensive error handling for:
- Connection issues with Ollama
- Invalid model names
- Network errors
- Invalid responses

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

MIT