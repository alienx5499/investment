# unit test: run Rust binary and check JSON output.

import json
import os
import subprocess
import sys

import pytest

_MARKET_SIM_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
_REPO_ROOT = os.path.abspath(os.path.join(_MARKET_SIM_ROOT, ".."))
_CRATE = os.path.join(_REPO_ROOT, "blockchain_consensus_rust")
_BINARY = os.path.join(_CRATE, "target", "debug", "blockchain_consensus")


def _run_rust_binary(args=None):
    args = args or []
    if os.path.isfile(_BINARY):
        cmd = [_BINARY] + args
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=15, cwd=_REPO_ROOT)
    else:
        result = subprocess.run(
            ["cargo", "run", "--quiet", "--manifest-path", os.path.join(_CRATE, "Cargo.toml")] + (["--"] + args if args else []),
            cwd=_CRATE,
            capture_output=True,
            text=True,
            timeout=60,
        )
    result.check_returncode()
    return json.loads(result.stdout)


@pytest.mark.unit
def test_chain_growth_output():
    data = _run_rust_binary([])
    assert "chain_lengths_per_round" in data
    assert "final_log" in data
    lengths = data["chain_lengths_per_round"]
    assert len(lengths) >= 1
    assert lengths[0] >= 1
    assert data["final_log"][0] == "genesis"


@pytest.mark.unit
def test_dolev_strong_output():
    data = _run_rust_binary(["dolev"])
    assert "honest_outputs" in data
    assert data.get("sender_input") == "1"
    outputs = data["honest_outputs"]
    for out in outputs.values():
        assert out == "1"
