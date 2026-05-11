ARG RUST_VERSION=1.95
ARG APP_NAME=multitarget-poem

# Stage 1: Build
FROM rust:${RUST_VERSION}-slim AS build
ARG APP_NAME
WORKDIR /app

# Установка системных зависимостей для сборки (pkg-config и libssl-dev нужны для большинства web-фреймворков)
RUN apt-get update && apt-get install -y pkg-config libssl-dev git && rm -rf /var/lib/apt/lists/*

# Использование BuildKit Cache вместо cargo-chef.
# Это позволяет сохранять target и registry между сборками контейнера.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

# Stage 2: Runtime
FROM debian:bookworm-slim AS final

# Установка корневых сертификатов (необходимо для исходящих HTTPS-запросов)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Создание несистемного пользователя (безопасность)
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/usr/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser

# Копирование готового бинарного файла
COPY --from=build /bin/server /bin/server

EXPOSE 3000

# Исправленный запуск: указываем точный путь к переименованному файлу
CMD ["/bin/server"]