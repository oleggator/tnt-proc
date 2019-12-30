FROM tarantool/tarantool:2.x-centos7 as build

RUN yum install -y cmake make gcc gcc-c++ openssl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
ENV PATH=/root/.cargo/bin:$PATH

RUN mkdir /opt/build

WORKDIR /opt/rust

COPY rust/Cargo.toml .
COPY rust/Cargo.lock .
RUN cargo fetch

COPY rust .
RUN cargo build --release \
    && mv /opt/rust/target/release/librustproc.so /opt/build

WORKDIR /opt/c
COPY c .

RUN cmake . && cmake --build . && mv libcproc.so /opt/build/libcproc.so


FROM tarantool/tarantool:2.x-centos7

WORKDIR /opt/tarantool

COPY --from=build /opt/build/librustproc.so .
COPY --from=build /opt/build/libcproc.so .

COPY tarantool .

CMD ["tarantool", "init.lua"]
