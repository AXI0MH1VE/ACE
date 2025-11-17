import pytest
from src.triage import triage
def test_triage():
    assert triage("test", {})[1] == "act"