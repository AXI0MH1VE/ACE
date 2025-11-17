import numpy as np
from .config import load_config
from .triage import triage
from .performance import execute_performance
from .nlp import process_nlp
from .safety import safety_qp_project
from .broker import Broker
from .ledger import Ledger
from .utils import hash_value

class Ace:
    def __init__(self):
        self.cfg = load_config()
        self.broker = Broker()
        self.ledger = Ledger()

    def process(self, message: str):
        # NLP
        tokens, context, reasoned = process_nlp(message)
        # Triage
        confidence, mode = triage(message, self.cfg)
        if mode == "ask":
            return {"status": "ask", "clarify": "Need clarification"}
        # Propose action
        a_star = np.random.randn(4)  # planner stub
        u_safe = safety_qp_project(a_star)
        # Broker
        broker_res = self.broker.request(u_safe)
        # Performance protocol
        solutions = execute_performance(message, reasoned)
        # Ledger
        self.ledger.append({"message": message, "solutions_hash": hash_value(solutions)})
        return {"reply": solutions["recommended"], "broker": broker_res}