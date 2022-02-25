FROM ubuntu:latest

# set apt to noninteractive mode  (for installing tzdata)
ENV DEBIAN_FRONTEND='noninteractive'

RUN apt-get update && \
    apt-get install -y software-properties-common \
                       vim \
                       git \
                       build-essential \
                       wget \
                       curl \
                       zsh \
                       tzdata

# add deadsnake repo for install python3.10
RUN add-apt-repository ppa:deadsnakes/ppa -y

RUN apt-get update && \
    apt-get install -y python3-dev \
                       python3-pip \
                       virtualenv \
                       python3.9 \
                       python3.10


# install oh-my-zsh
RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true

RUN echo "Europe/Berlin" > /etc/timezone && \
    ln -fs /usr/share/zoneinfo/`cat /etc/timezone` /etc/localtime && \
    dpkg-reconfigure -f noninteractive tzdata

RUN echo "alias python=python3" >> ~/.zshrc
RUN echo "alias pip=pip3" >> ~/.zshrc

# install rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

ENV VIRTUAL_ENV_PATH="/tmp/venv"
# first copy only requirements files to only invalidate the next setps in case of changed requirements
COPY requirements.txt /tmp/requirements.txt
COPY entrypoint.sh /entrypoint.d/entrypoint.sh

# install pip dependencies
RUN virtualenv -p python3.9 $VIRTUAL_ENV_PATH
RUN /bin/bash -c 'source $VIRTUAL_ENV_PATH/bin/activate && pip install -r /tmp/requirements.txt'

# add convenience aliases
RUN echo "alias cargotest='cargo test --no-default-features'" >> /etc/zsh/zshrc
RUN echo "alias maturinbuild-pipinstall='maturin build -i python3.9 && pip install --force pip install target/wheels/sportgems-*-manylinux*_x86_64.whl'" >> /etc/zsh/zshrc
RUN echo "alias maturinpytest='maturin build -i python3.9 && pip install --force pip install target/wheels/sportgems-*-manylinux*_x86_64.whl && pytest tests/ -v'" >> /etc/zsh/zshrc

COPY . /sportgems
WORKDIR /workspaces/sportgems

ENTRYPOINT [ "/entrypoint.d/entrypoint.sh" ]
