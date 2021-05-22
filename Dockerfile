FROM rustlang/rust:nightly-stretch-slim as builder
# Current 1.5x does not start the application correctly
#FROM rust:1.52.1-slim-bullseye as builder

RUN apt-get update \
    && apt-get install -y musl-tools \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/webapp
COPY src src
COPY Cargo.* ./

# compile with musl and strip afterwards to reduce size
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/pandoc-rustful-api

###############
# Web Container
###############
FROM pandoc/latex:latest

ENV ROCKET_PROFILE="release"
ENV ROCKET_PORT=8000

ARG TEMPLATE_DIR=/home/webapp/.pandoc/templates/
ARG EISVOGEL_GIT=https://raw.githubusercontent.com/Wandmalfarbe/pandoc-latex-template
ARG EISVOGEL_VERSION=2.0.0

RUN tlmgr update --self && \
    tlmgr install \
        # Eisvogel: https://github.com/Wandmalfarbe/pandoc-latex-template/blob/master/.travis.yml \
        adjustbox \
        awesomebox \
        background \
        bidi \
        collectbox \
        csquotes \
        everypage \
        etoolbox \
        environ \
        filehook \
        fontawesome5 \
        footmisc \
        footnotebackref \
        framed \
        fvextra \
        letltxmacro \
        ly1 \
        mdframed \
        mweights \
        needspace \
        pagecolor \
        pgf \
        sourcecodepro \
        sourcesanspro \
        tcolorbox \
        titling \
        trimspaces \
        ucharcat \
        ulem \
        unicode-math \
        upquote \
        xecjk \
        xurl \
        zref

RUN addgroup -g 1000 webapp \
    && adduser -D -s /bin/sh -u 1000 -G webapp webapp

WORKDIR /home/webapp/app/
COPY --chown=webapp:webapp --from=builder /usr/src/webapp/target/x86_64-unknown-linux-musl/release/pandoc-rustful-api .
COPY Rocket.toml .

# Switch to user and start the webservice
USER webapp

RUN mkdir -p ${TEMPLATE_DIR} \
    && mkdir -p /home/webapp/pandoc

# Deploy templates
RUN wget ${EISVOGEL_GIT}/v${EISVOGEL_VERSION}/eisvogel.tex -O ${TEMPLATE_DIR}/eisvogel.latex

EXPOSE ${ROCKET_PORT}

ENTRYPOINT ["/home/webapp/app/pandoc-rustful-api"]