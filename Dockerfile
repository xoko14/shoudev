FROM rust AS builder

WORKDIR /app

COPY . .

RUN cargo build --release
RUN mv target/release/shoudev-site ./
RUN cargo clean

FROM node:lts-slim

WORKDIR /app

COPY --from=builder /app/frontend ./frontend
COPY --from=builder /app/static ./static
COPY --from=builder /app/content ./content
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/shoudev-site .

WORKDIR /app/frontend
RUN npm config set registry https://registry.npmjs.org
RUN npm i
RUN npm run build

WORKDIR /app

EXPOSE 3000

ENV SERVE_ENV prod
ENV RUST_LOG info

CMD ["./shoudev-site"]