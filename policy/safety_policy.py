from dataclasses import dataclass
from enum import Enum, auto
from typing import List, Optional


class RiskLevel(Enum):
    LOW = auto()
    MEDIUM = auto()
    HIGH = auto()


class Action(Enum):
    ALLOW = auto()
    DENY = auto()
    ESCALATE = auto()


@dataclass
class SafetyDecision:
    action: Action
    reason: str
    risk: RiskLevel
    requires_consent: bool = False
    requires_payment: bool = False


class SafetyPolicy:
    """
    Lightweight safety policy used for gating requests before running Creative or Verified flows.
    """

    def __init__(self, allow_network: bool = False, allow_verified: bool = True, require_payment: bool = False):
        self.allow_network = allow_network
        self.allow_verified = allow_verified
        self.require_payment = require_payment
        self.blocklist = ["self-harm", "bioweapon", "malware", "child exploitation", "terrorism"]
        self.high_risk_terms = ["financial advice", "medical diagnosis", "legal judgment"]

    def evaluate(self, prompt: str, mode: str, axiom_set: Optional[str] = None, payment_token: Optional[str] = None) -> SafetyDecision:
        lower = prompt.lower()
        if any(term in lower for term in self.blocklist):
            return SafetyDecision(Action.DENY, "Blocked content", RiskLevel.HIGH)

        if mode == "verified" and not self.allow_verified:
            return SafetyDecision(Action.DENY, "Verified mode disabled by policy", RiskLevel.MEDIUM)

        requires_consent = any(term in lower for term in self.high_risk_terms)

        if mode == "verified" and self.require_payment and not payment_token:
            return SafetyDecision(Action.ESCALATE, "Payment required for verified mode", RiskLevel.MEDIUM, requires_consent, True)

        # If network is disabled, flag if axiom_set implies remote fetch or proof sync.
        if not self.allow_network and axiom_set and "remote" in axiom_set:
            return SafetyDecision(Action.ESCALATE, "Network disabled; remote axiom set not allowed", RiskLevel.MEDIUM, requires_consent)

        return SafetyDecision(Action.ALLOW, "Allowed", RiskLevel.LOW, requires_consent, self.require_payment and mode == "verified")
