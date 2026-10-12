# Sourced by every script: Python comes only from the repository's uv
# environment (.venv, made by `just venv`: `uv venv --managed-python`,
# on a Python uv installs itself; packages only with `uv pip`), never the
# system's or Homebrew's, whether a script runs from just or on its own.
_venv="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/.venv"
if [ ! -x "$_venv/bin/python3" ]; then
  echo "no .venv: run 'just venv' (uv venv) first" >&2
  exit 1
fi
case ":$PATH:" in *":$_venv/bin:"*) ;; *) export PATH="$_venv/bin:$PATH" ;; esac
unset _venv
