#!/usr/bin/env bash
set -euo pipefail

export MYAPP_HOME="$(cd "$(dirname "$0")/.." && pwd)"
export PROJ_HOME="${MYAPP_HOME}/${CRATE}"
export PROJ_NAME="myapp"
export CARGO_PATH="${PROJ_HOME}/core/Cargo.toml"

# TODO: bump dotfiles + remove these functions
log::note() { log::info "$@"; }
command_exists() { has "$@"; }
export -f log::note command_exists

dot::clone() {
  git clone 'https://github.com/denisidoro/dotfiles' "$DOTFILES"
  cd "$DOTFILES"
  git checkout 'v2022.07.16'
}

dot::clone_if_necessary() {
  if [ -z "${DOTFILES:-}" ]; then
    export DOTFILES="${MYAPP_HOME}/target/dotfiles"
  fi
  if [ -x "${DOTFILES}/bin/dot" ]; then
    return 0
  fi
  dot::clone
}

dot::clone_if_necessary

source "${DOTFILES}/scripts/core/main.sh"
