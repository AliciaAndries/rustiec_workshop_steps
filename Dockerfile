FROM ubuntu:latest
LABEL maintainer='imec-DistriNet'
#TO BUILD THE IMAGE RUN THE FOLLOWING COMMAND:
#open a terminal in the same directory as the downloaded Dockerfile
#docker build -t c2rustworkshop:<v1> . 

# install git
RUN apt-get -y update
RUN apt-get -y install git
RUN apt-get -y install sudo
# get code
RUN git clone  https://github.com/AliciaAndries/rustiec_workshop_steps.git

#install Rust
RUN apt-get -y install curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN . ~/.bashrc
RUN echo $PATH
ENV PATH=$PATH:~/.cargo/
ENV . ~/.bashrc
RUN echo $PATH
RUN . ~/.bashrc
#RUN ls home

#RUN /root/.cargo/bin/rustup -v
#install C2Rust
RUN apt install build-essential llvm clang libclang-dev cmake libssl-dev pkg-config python3 git -y
RUN /root/.cargo/bin/cargo install c2rust
# install cbindgen
RUN /root/.cargo/bin/cargo install --force cbindgen

WORKDIR "/rustiec_workshop_steps"
#bootstrap
RUN yes | /rustiec_workshop_steps/bootstrap.sh


