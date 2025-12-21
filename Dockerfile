# Usar imagen oficial de Rust para compilación
FROM rust:1.75-slim as builder

WORKDIR /app

# Copiar archivos de configuración de Cargo
COPY Cargo.toml Cargo.lock ./

# Crear un proyecto dummy para cachear dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar el código fuente
COPY src ./src

# Compilar la aplicación
RUN cargo build --release

# Imagen final más pequeña
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar el binario compilado
COPY --from=builder /app/target/release/blockchain-indexer /app/blockchain-indexer

# Ejecutar la aplicación
CMD ["./blockchain-indexer"]

