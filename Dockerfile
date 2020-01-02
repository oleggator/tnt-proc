FROM tarantool/tarantool:2.x-centos7 as base
RUN yum install -y cmake make gcc gcc-c++ openssl
RUN mkdir /opt/build


FROM base as rust-build

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /opt/rust
COPY rust/Cargo.toml .
COPY rust/Cargo.lock .
RUN cargo fetch
COPY rust .
RUN cargo build --release \
    && mv /opt/rust/target/release/librustproc.so /opt/build


FROM base as c-build
WORKDIR /opt/c
COPY c .
RUN cmake . && cmake --build . && mv libcproc.so /opt/build/libcproc.so


FROM tarantool/tarantool:2.x-centos7
WORKDIR /opt/tarantool
COPY --from=rust-build /opt/build/librustproc.so .
COPY --from=c-build /opt/build/libcproc.so .
COPY tarantool .
ENV RUST_BACKTRACE 1
CMD ["tarantool", "init.lua"]
