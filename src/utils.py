import hashlib

def hash_value(v):
    return hashlib.sha256(str(v).encode()).hexdigest()