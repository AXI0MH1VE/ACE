from src.broker import Broker
def test_broker():
    b = Broker()
    assert "auto_approved" in b.request([0.1,0.1])["status"]