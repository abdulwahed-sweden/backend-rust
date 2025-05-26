# 🦀 Rust Backend with Actix Web

A blazing fast, secure, and lightweight backend built with **Rust** and **Actix Web**, containerized using **Docker** for easy deployment.

## 🚀 Features

- ⚡ **High Performance** - Built with Rust for maximum speed and safety
- 🌐 **Modern Web Framework** - Powered by Actix Web 4
- 🐳 **Docker Ready** - One command deployment
- 🛡️ **Memory Safe** - Zero-cost abstractions with Rust
- 📱 **Responsive UI** - Beautiful Arabic/English interface
- 🔧 **Production Ready** - Optimized for deployment

## 📦 Quick Start

### Pull and Run
```bash
docker pull mansourhub/backend
docker run -p 8080:8080 mansourhub/backend
```

Then open: **http://localhost:8080**

### Using Docker Compose
```yaml
version: "3.8"
services:
  backend:
    image: mansourhub/backend
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
    restart: unless-stopped
```

```bash
docker-compose up -d
```

## 🌟 What's Inside

- **Rust 1.82** - Latest stable version
- **Actix Web 4** - High-performance web framework  
- **Static File Serving** - CSS, JavaScript, images
- **API Endpoints** - RESTful API ready
- **Health Checks** - Built-in monitoring
- **Multilingual Support** - Arabic & English

## 🧪 API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Main application page |
| `/api/status` | GET | Health check endpoint |
| `/static/*` | GET | Static files (CSS, JS, images) |

Example:
```bash
curl http://localhost:8080/api/status
```

Response:
```json
{
  "status": "ok",
  "message": "Rust Backend API is running",
  "version": "0.1.0"
}
```

## 🔧 Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level (debug, info, warn, error) |
| `SERVER_PORT` | `8080` | Server port |

## 🏗️ Development

### Local Development
```bash
git clone https://github.com/YOUR_USERNAME/backend-rust
cd backend-rust
cargo run
```

### Build from Source
```bash
docker build -t mansourhub/backend .
```

## 🧰 Tech Stack

- ![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white) **Rust 1.82**
- ![Actix](https://img.shields.io/badge/Actix%20Web-000000?style=flat&logo=rust&logoColor=white) **Actix Web 4**
- ![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat&logo=docker&logoColor=white) **Docker**

## 📊 Image Info

- **Base Image**: `rust:1.82-slim`
- **Size**: ~100MB (optimized multi-stage build)
- **Architecture**: `linux/amd64`, `linux/arm64`
- **Security**: Non-root user, minimal attack surface

## 🔍 Health Check

The container includes built-in health checks:
```bash
docker run --health-cmd="curl -f http://localhost:8080/api/status || exit 1" \
           --health-interval=30s \
           --health-timeout=10s \
           --health-retries=3 \
           -p 8080:8080 mansourhub/backend
```

## 📂 Source Code

👉 **GitHub**: [https://github.com/YOUR_USERNAME/backend-rust](https://github.com/YOUR_USERNAME/YOUR_REPO_NAME)

## 🧑‍💻 Maintainer

**Abdulwahed Mansour**  
Full-Stack Developer specializing in Django, Rust, and AI Integration

- 🌐 Portfolio: www.me.com
- 💼 LinkedIn: www.them.com
- 📧 Email: abdulwahed.sweden@gmail.com

## 📝 License

This project is licensed under the MIT License.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 💬 Support

- 🐛 **Found a bug?** Open an issue on GitHub
- ❓ **Need help?** Contact me directly
- ⭐ **Like this project?** Give it a star on GitHub!

---

**Built with ❤️ using Rust 🦀**