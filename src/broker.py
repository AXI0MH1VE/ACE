import numpy as np

class Broker:
    def request(self, u_safe):
        risk = np.linalg.norm(u_safe)
        if risk < 0.9:
            return {"status": "auto_approved"}
        return {"status": "queued"}