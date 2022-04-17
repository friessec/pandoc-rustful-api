FROM rust:1.60-slim as builder

RUN apt-get update \
    && apt-get install -y musl-tools \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/webapp

# Pre-Build Dependencies
COPY Cargo.toml Cargo.toml
COPY api/Cargo.toml api/Cargo.toml
RUN mkdir -p api/src/ \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > api/src/main.rs \
    && mkdir -p cli-tool/src  \
    && echo "[package]\nname = \"pandoc-rustful-cli\"\nversion = \"0.1.0\"" > cli-tool/Cargo.toml \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > cli-tool/src/main.rs

RUN cargo build --package pandoc-rustful-api --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/pandoc-rustful-api*

# Build Main Application
COPY . .

# compile with musl and strip afterwards to reduce size
RUN cargo build --package pandoc-rustful-api --release --target=x86_64-unknown-linux-musl

###############
# Web Container
###############
FROM pandoc/latex:2.18

ENV ACTIX_PROFILE="production"
ENV ACTIX_PORT=8000

ARG TEMPLATE_DIR=/home/webapp/.pandoc/templates/
ARG EISVOGEL_GIT=https://raw.githubusercontent.com/Wandmalfarbe/pandoc-latex-template
#ARG EISVOGEL_VERSION=v2.0.0
ARG EISVOGEL_VERSION=master

RUN tlmgr update --self && \
    tlmgr install \
        # Eisvogel: https://github.com/Wandmalfarbe/pandoc-latex-template/blob/master/.travis.yml \
        adjustbox \
        awesomebox \
        babel-german \
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
        koma-script \
        letltxmacro \
        lineno \
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
COPY --chown=webapp:webapp static static
COPY Settings.toml .

# Switch to user and start the webservice
USER webapp

RUN mkdir -p ${TEMPLATE_DIR} \
    && mkdir -p /home/webapp/pandoc

# Deploy templates
RUN wget ${EISVOGEL_GIT}/${EISVOGEL_VERSION}/eisvogel.tex -O ${TEMPLATE_DIR}/eisvogel.tex

EXPOSE ${ACTIX_PORT}
ENTRYPOINT ["/home/webapp/app/pandoc-rustful-api"]