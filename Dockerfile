FROM rust:latest as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates


# Create appuser
ENV USER=myip
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN mkdir app
WORKDIR /app
ADD . /app
COPY templates /app/templates
COPY static /app/static
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine
ARG APP=/usr/src/app

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/simpleblogrust ${APP}/
COPY --from=builder /app/templates ${APP}/templates
COPY --from=builder /app/static ${APP}/static

RUN chown -R myip:myip ${APP}

USER myip:myip

WORKDIR ${APP}

CMD ["./simpleblogrust"]