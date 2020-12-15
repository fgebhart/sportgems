FROM ubuntu:latest

# set apt to noninteractive mode  (for installing firefox)
ENV DEBIAN_FRONTEND='noninteractive'
# install sqlite3 package for the use of djangos db shell
RUN apt-get update
RUN apt-get install -y python3-dev \
                       python3-pip \
                       vim \
                       git \
                       build-essential \
                       zsh \
                       wget \
                       curl

# install oh-my-zsh
RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true

# install rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
