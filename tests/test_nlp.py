from src.nlp import process_nlp
def test_nlp():
    tokens, _, _ = process_nlp("test message")
    assert len(tokens) > 0