def process_nlp(message: str):
    tokens = message.split()
    context = "retrieved"
    reasoned = "reasoned"
    return tokens, context, reasoned