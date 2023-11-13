#Setting up:

## Get the code
    $ git clone  https://github.com/AliciaAndries/rustiec_workshop_steps.git

To download all dependencies of the code run:

    cd rustiec_workshop_steps
    ./bootstrap.sh


## Install Rust    
If you are using Linux or macOS the installation instructions can be found here: \
    https://www.rust-lang.org/tools/install 

If you are using Windows the installation instructions can be found here: \
    https://forge.rust-lang.org/infra/other-installation-methods.html#which 

### Install C2Rust
run the following:

    apt install build-essential llvm clang libclang-dev cmake libssl-dev pkg-config python3 git
    /root/.cargo/bin/cargo install c2rust

### Install cbindgen
    RUN /root/.cargo/bin/cargo install --force cbindgen