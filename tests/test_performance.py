from src.performance import execute_performance
def test_performance():
    assert "recommended" in execute_performance("test", "reasoned")