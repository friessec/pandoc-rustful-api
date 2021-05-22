FROM pandoc/latex:latest

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

# Switch to user and start the webservice
USER webapp

RUN mkdir -p ${TEMPLATE_DIR} \
    && mkdir -p /home/webapp/pandoc

# Deploy templates
RUN wget ${EISVOGEL_GIT}/v${EISVOGEL_VERSION}/eisvogel.tex -O ${TEMPLATE_DIR}/eisvogel.latex

WORKDIR /home/webapp/pandoc
VOLUME /home/webapp/pandoc