########################################
# 1️⃣ Build Rust binary
########################################
FROM rust:latest AS builder

WORKDIR /app
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release


########################################
# 2️⃣ Runtime (Python + gTTS + Rust binary)
########################################
FROM python:3.11-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    ffmpeg \
    espeak \
    && rm -rf /var/lib/apt/lists/*

# Install gTTS
RUN pip install --no-cache-dir gTTS

# Copy Rust binary (use YOUR actual binary name)
COPY --from=builder /app/target/release/hf-rust-inference /app/hf-rust-inference

# Copy Python script
COPY speech.py .

# Expose backend port
EXPOSE 8000

# Run Rust backend
CMD ["./hf-rust-inference"]
