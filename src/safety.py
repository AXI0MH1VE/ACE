import osqp
import scipy.sparse as sp
import numpy as np

def safety_qp_project(a_star):
    n = len(a_star)
    P = sp.eye(n, format="csc")
    q = -np.array(a_star)
    A = sp.eye(n, format="csc")
    l = np.ones(n) * -1.0
    u = np.ones(n) * 1.0
    prob = osqp.OSQP()
    prob.setup(P, q, A, l, u, verbose=False)
    res = prob.solve()
    return res.x if res.info.status == "solved" else [0] * n