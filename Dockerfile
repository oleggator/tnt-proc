# syntax=docker/dockerfile:experimental

############# base #################
FROM tarantool/tarantool:2.x-centos7 as base

RUN yum install -y centos-release-scl scl-utils \
    && yum-config-manager --enable rhel-server-rhscl-7-rpms \
    && yum install -y cmake3 make devtoolset-8-gcc devtoolset-8-gcc-c++ openssl msgpack-devel msgpuck-devel
RUN mkdir /opt/build
####################################


############# rust #################
FROM base as rust-build

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /opt/rust
COPY rust/Cargo.toml .
COPY rust/Cargo.lock .
RUN cargo fetch
COPY rust .
RUN --mount=type=cache,target=/opt/rust/target \
    source scl_source enable devtoolset-8 \
    && cargo build --release \
    && mv /opt/rust/target/release/librustproc.so /opt/build
####################################


############# c/c++ ################
FROM base as c-cpp-build

WORKDIR /opt
COPY c c
COPY cpp cpp
COPY CMakeLists.txt .

RUN source scl_source enable devtoolset-8 \
    && cmake3 . \
    && cmake3 --build . \
    && mv cpp/libcppproc.so /opt/build/libcppproc.so \
    && mv c/libcproc.so /opt/build/libcproc.so
####################################


############# runtime ##############
FROM tarantool/tarantool:2.x-centos7

WORKDIR /opt/tarantool
COPY --from=rust-build /opt/build/librustproc.so .
COPY --from=c-cpp-build /opt/build/libcproc.so .
COPY --from=c-cpp-build /opt/build/libcppproc.so .

COPY tarantool .
ENV RUST_BACKTRACE 1
CMD ["tarantool", "init.lua"]
####################################
