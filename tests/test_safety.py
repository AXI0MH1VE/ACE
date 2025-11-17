from src.safety import safety_qp_project
def test_safety():
    assert len(safety_qp_project([1,2])) == 2