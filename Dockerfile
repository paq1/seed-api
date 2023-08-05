FROM rust:latest
COPY . .
WORKDIR /
RUN cargo build --release
EXPOSE 8000
# todo mettre le nom de votre executable ici "./target/release/<executable_name>"
CMD ["./target/release/seed-api"]