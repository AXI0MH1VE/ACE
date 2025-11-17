def triage(message: str, cfg):
    confidence = 0.95  # stub
    if confidence > 0.9:
        return confidence, "act"
    return confidence, "ask"