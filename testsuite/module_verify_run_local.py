#!/usr/bin/env python3

# Copyright © A-p-t-o-s Foundation
# SPDX-License-Identifier: Apache-2.0

# Test replay-verify by running it on a public testnet backup
# While the replay-verify composite Github Action is meant to run with libra2-core checked out in the current
# working directory, this test script is meant to be run from this separate repo. The environment variable LIBRA2_CORE_PATH
# is required to be set to the path of your local checkout of libra2-core, which will be used to build and copy over test dependencies.

import os
import subprocess

import module_verify


def local_setup():
    # Take these from the expected replay verify run
    envs = {
        "BUCKET": "libra2-testnet-backup-2223d95b",
        "SUB_DIR": "e1",
        "BACKUP_CONFIG_TEMPLATE_PATH": "terraform/helm/fullnode/files/backup/s3-public.yaml",
    }

    # build backup tools
    subprocess.run(
        [
            "cargo",
            "build",
            "--release",
            "-p",
            "libra2-debugger",
        ],
        check=True,
    )

    # write to environment variables
    for key, value in envs.items():
        os.environ[key] = value


if __name__ == "__main__":
    local_setup()
    module_verify.main()
