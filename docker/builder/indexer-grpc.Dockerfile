### Indexer GRPC Image ###

FROM debian-base AS indexer-grpc

COPY --link --from=indexer-builder /libra2/dist/libra2-indexer-grpc-cache-worker /usr/local/bin/libra2-indexer-grpc-cache-worker
COPY --link --from=indexer-builder /libra2/dist/libra2-indexer-grpc-file-store /usr/local/bin/libra2-indexer-grpc-file-store
COPY --link --from=indexer-builder /libra2/dist/libra2-indexer-grpc-data-service /usr/local/bin/libra2-indexer-grpc-data-service
COPY --link --from=indexer-builder /libra2/dist/libra2-indexer-grpc-file-checker /usr/local/bin/libra2-indexer-grpc-file-checker
COPY --link --from=indexer-builder /libra2/dist/libra2-indexer-grpc-data-service-v2 /usr/local/bin/libra2-indexer-grpc-data-service-v2
COPY --link --from=indexer-builder /libra2/dist/libra2-indexer-grpc-manager /usr/local/bin/libra2-indexer-grpc-manager

# The health check port
EXPOSE 8080
# The gRPC non-TLS port
EXPOSE 50052
# The gRPC TLS port
EXPOSE 50053

ENV RUST_LOG_FORMAT=json

# add build info
ARG GIT_TAG
ENV GIT_TAG ${GIT_TAG}
ARG GIT_BRANCH
ENV GIT_BRANCH ${GIT_BRANCH}
ARG GIT_SHA
ENV GIT_SHA ${GIT_SHA}
