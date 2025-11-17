import hashlib
import time
class Ledger:
    def append(self, entry):
        entry["ts"] = time.time()
        with open("ledger.log", "a") as f:
            f.write(f"{hashlib.sha256(str(entry).encode()).hexdigest()} {entry}\n")