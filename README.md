# Burger Backend API

Rust backend API for burger application with a single endpoint to get the list of burgers.

## 📡 API Endpoints

- `GET /get-burgers` - Get list of all burgers
- `GET /B1.png`, `/B2.png`, etc. - Static burger images

## 🏗️ Local Development

```bash
# Start the server
cargo run

# Server will be available at http://localhost:3000
```

## 📦 Project Structure

```
burger-BE/
├── src/
│   ├── main.rs      # Main server file
│   ├── models.rs    # Data models
│   └── handlers.rs  # API handlers
├── static/          # Static files (images)
└── Cargo.toml       # Rust dependencies
```

## 🍔 Available Burgers

The API returns 16 different burgers with the following structure:

```json
{
  "burgers": [
    {
      "id": 1,
      "title": "Classic Beef Supreme",
      "price": "11.50",
      "img": "/B1.png"
    }
    // ... more burgers
  ]
}
```

## 🛠️ Tech Stack

- **Rust** - Programming language
- **Axum** - Web framework
- **Tokio** - Async runtime
- **Serde** - Serialization
- **Tower** - Middleware