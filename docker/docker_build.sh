#!/bin/sh

set -e
set -o nounset

export script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd ${script_dir}/..

docker_image="spellrs_builder"
git_sha1="`git rev-parse HEAD`"
git_describe="`git describe --tags --always`"
git_branch="`git rev-parse --abbrev-ref HEAD`"

# ***********************************************************************
run() {
    args="${@}"
    printf "\033[32m${args}\n\033[0m"
    eval ${@}
    echo ""
}
# ***********************************************************************

# Build docker image
run "docker build
    --file docker/Dockerfile
    --tag ${docker_image}:${git_sha1}
    ."
