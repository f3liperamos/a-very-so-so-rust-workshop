# For some unkown reason docker is PAINFULLY slow when running with volumes
FROM rust AS base
RUN apt update && apt install bash
RUN ["bash", "-c", "cargo install cargo-watch"]

FROM base AS dev
WORKDIR /code
COPY code .
CMD ["bash"]
