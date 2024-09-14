FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src

COPY src ./src

RUN cargo build --release

COPY . .

# To'g'ri fayl nomini ko'rsating
CMD ["./target/release/Hayvonlarni-ozuqasini-hisoblovchi"]



