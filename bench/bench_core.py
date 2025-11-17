from src.ace import Ace
import time

def bench():
    ace = Ace()
    start = time.time()
    ace.process("benchmark test")
    print("Time:", time.time() - start)

if __name__ == "__main__":
    bench()