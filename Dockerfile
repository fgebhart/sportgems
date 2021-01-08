FROM ubuntu:latest

# set apt to noninteractive mode  (for installing tzdata)
ENV DEBIAN_FRONTEND='noninteractive'

RUN apt-get update && \
    apt-get install -y python3-dev \
                       python3-pip \
                       vim \
                       git \
                       build-essential \
                       zsh \
                       wget \
                       curl \
                       tzdata

RUN echo "Europe/Berlin" > /etc/timezone && \
    ln -fs /usr/share/zoneinfo/`cat /etc/timezone` /etc/localtime && \
    dpkg-reconfigure -f noninteractive tzdata

# install oh-my-zsh
RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true

RUN echo "alias python=python3" >> ~/.zshrc
RUN echo "alias pip=pip3" >> ~/.zshrc

# install rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY requirements.txt /sportgems/requirements.txt
WORKDIR /sportgems

# install pip packages
RUN pip3 install -r /sportgems/requirements.txt

COPY . /sportgems
WORKDIR /sportgems

RUN maturin build
