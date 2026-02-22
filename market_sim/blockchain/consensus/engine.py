from abc import ABC, abstractmethod
from typing import Any, Dict, Optional
import json
import os
import subprocess
import sys

_MARKET_SIM_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
_REPO_ROOT = os.path.abspath(os.path.join(_MARKET_SIM_ROOT, ".."))
_CRATE = os.path.join(_REPO_ROOT, "blockchain_consensus_rust")
_BINARY = os.path.join(_CRATE, "target", "debug", "blockchain_consensus")


class ConsensusEngine(ABC):
    # pluggable consensus backend for market_sim.

    @abstractmethod
    def run_consensus(self, num_rounds: int) -> Dict[str, Any]:
        pass

    def name(self) -> str:
        return self.__class__.__name__


class NoOpConsensus(ConsensusEngine):
    # no-op; simulation unchanged.

    def run_consensus(self, num_rounds: int) -> Dict[str, Any]:
        return {"chain_lengths_per_round": [], "final_log": [], "final_length": 0}


class RustConsensusAdapter(ConsensusEngine):
    # invokes Rust binary, parses JSON into simulation results.

    def __init__(self, binary_path: Optional[str] = None, crate_path: Optional[str] = None):
        self._binary = binary_path or _BINARY
        self._crate = crate_path or _CRATE

    def _invoke_rust(self) -> Dict[str, Any]:
        if os.path.isfile(self._binary):
            result = subprocess.run(
                [self._binary],
                capture_output=True,
                text=True,
                timeout=15,
                cwd=_REPO_ROOT,
            )
        else:
            result = subprocess.run(
                ["cargo", "run", "--quiet", "--manifest-path", os.path.join(self._crate, "Cargo.toml")],
                cwd=self._crate,
                capture_output=True,
                text=True,
                timeout=60,
            )
        result.check_returncode()
        return json.loads(result.stdout)

    def run_consensus(self, num_rounds: int) -> Dict[str, Any]:
        data = self._invoke_rust()
        return {
            "chain_lengths_per_round": data.get("chain_lengths_per_round", []),
            "final_log": data.get("final_log", []),
            "final_length": data.get("final_length", 0),
        }


def get_engine(backend: str) -> ConsensusEngine:
    if backend == "rust":
        return RustConsensusAdapter()
    return NoOpConsensus()
