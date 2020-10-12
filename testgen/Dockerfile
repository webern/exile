FROM maven:3-jdk-14
ENV CARGO_HOME=/rust/cargo_home
ENV RUSTUP_HOME=/rust/rustup_home
ENV PATH=$PATH:$CARGO_HOME/bin
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN cargo version
COPY ./entrypoint.sh /entrypoint.sh
ENTRYPOINT [ "/entrypoint.sh" ]
CMD [ "echo", "done" ]
