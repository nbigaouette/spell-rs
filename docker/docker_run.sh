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

# Cleanup
run "rm -fr python/{build,dist,spellrs.egg-info,spell/_native*}"

docker_home="/home/rust/home"
docker_src="/home/rust/src"

docker_cmd="docker run
    -it --rm
    --env HOME=${docker_home}
    --env CARGO_HOME=${docker_src}/docker/cache/cargo
    --env CARGO_TARGET_DIR=${docker_src}/docker/cache/target
    --workdir ${docker_src}/python
    --user $(id -u):$(id -g)
    --volume "$PWD":${docker_src}
    --volume "$PWD"/docker/cache/home:${docker_home}
    ${docker_image}:${git_sha1}"

run "${docker_cmd} pip3 install --user --verbose --editable ."
run "${docker_cmd} python3 setup.py --verbose bdist_wheel"
