FROM debian:buster

RUN apt-get update && apt-get install -y \
    wget \
    xz-utils \
    curl

RUN /bin/bash -c " \
    cd /opt \
    && wget https://nodejs.org/dist/v16.17.1/node-v16.17.1-linux-x64.tar.xz \
    && tar --no-same-owner -xvf node-v16.17.1-linux-x64.tar.xz \
    && rm node-v16.17.1-linux-x64.tar.xz"

ENV PATH /opt/node-v16.17.1-linux-x64/bin:$PATH

RUN /bin/bash -c "npm install --global yarn"

ARG UNAME=user
ARG UID=1000
ARG GID=1000
RUN groupadd -g $GID -o $UNAME
RUN useradd -m -u $UID -g $GID -o -s /bin/bash $UNAME
USER $UNAME

WORKDIR /home/user

RUN /bin/bash -c " \
    wget -O rustup.sh https://sh.rustup.rs \
    && chmod +x rustup.sh \
    && ./rustup.sh -y"

ENV PATH /home/user/.cargo/bin:$PATH


RUN /bin/bash -c "rustup install 1.63.0"

RUN /bin/bash -c " \
    rustup default 1.63.0 \
    && rustup target add wasm32-unknown-unknown"
    


WORKDIR /pwd

USER root
RUN apt-get install -y build-essential libssl-dev pkg-config
USER $UNAME

RUN cargo install --locked trunk
RUN cargo install --force cargo-make

RUN /bin/bash -c "\
    mkdir /home/user/bin \
    && (echo ''> /home/user/bin/xdg-open) \
    && chmod +x /home/user/bin/xdg-open"
    
ENV PATH /home/user/bin:$PATH



CMD exec /bin/bash -c "trap : TERM INT; sleep infinity & wait"






# ENTRYPOINT ["pebble"]

# CMD exec /bin/bash -c "trap : TERM INT; sleep infinity & wait"
