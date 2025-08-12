### Forge Image ###

FROM debian-base as forge

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get install --no-install-recommends -y \
        awscli \
        busybox \
        git \
        openssh-client \
        unzip \
        wget

WORKDIR /aptos

# copy helm charts from source
COPY --link --from=tools-builder /aptos/terraform/helm /aptos/terraform/helm
COPY --link --from=tools-builder /aptos/testsuite/forge/src/backend/k8s/helm-values/libra2-node-default-values.yaml /aptos/terraform/libra2-node-default-values.yaml

RUN cd /usr/local/bin && wget "https://storage.googleapis.com/kubernetes-release/release/v1.18.6/bin/linux/amd64/kubectl" -O kubectl && chmod +x kubectl
RUN cd /usr/local/bin && wget "https://get.helm.sh/helm-v3.8.0-linux-amd64.tar.gz" -O- | busybox tar -zxvf - && mv linux-amd64/helm . && chmod +x helm
ENV PATH "$PATH:/root/bin"

WORKDIR /aptos
COPY --link --from=node-builder /aptos/dist/forge /usr/local/bin/forge

### Get Libra2 Framework Release for forge framework upgrade testing
COPY --link --from=tools-builder /aptos/libra2-move/framework/ /aptos/libra2-move/framework/
COPY --link --from=tools-builder /aptos/libra2-move/libra2-release-builder/ /aptos/libra2-move/libra2-release-builder/

ENV RUST_LOG_FORMAT=json

# add build info
ARG BUILD_DATE
ENV BUILD_DATE ${BUILD_DATE}
ARG GIT_TAG
ENV GIT_TAG ${GIT_TAG}
ARG GIT_BRANCH
ENV GIT_BRANCH ${GIT_BRANCH}
ARG GIT_SHA
ENV GIT_SHA ${GIT_SHA}

ENTRYPOINT ["/tini", "--", "forge"]
