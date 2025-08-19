#!/bin/bash -xe

TEMP="$(mktemp)"

curl https://fullnode.devnet.libra2.org/v1 > "$TEMP"

COMMIT="$(jq -r .git_hash "$TEMP")"
CHAIN_ID="$(jq -r .chain_id "$TEMP")"

DIGEST="$(crane digest aptoslabs/validator:devnet_"$COMMIT")"
GENESIS_SHA="$(curl https://devnet.libra2.org/genesis.blob | shasum -a 256 | awk '{print $1}')"
WAYPOINT="$(curl https://devnet.libra2.org/waypoint.txt)"

cat <<EOF

Hey @everyone devnet finished release, please update your fullnodes now!

For upgrade, make sure you pulled the latest docker image, or build the rust binary from the latest devnet branch. To confirm:

- Devnet branch commit: $COMMIT
- Docker image tag: devnet_$COMMIT
- Docker image digest: $DIGEST
- genesis.blob sha256: $GENESIS_SHA
- waypoint: $WAYPOINT
- Chain ID: $CHAIN_ID
You can follow the instructions here for upgrade: https://docs.libra2.org/nodes/full-node/update-fullnode-with-new-devnet-releases

EOF
