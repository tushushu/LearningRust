import time
import random

if __name__ == "__main__":
    # Duration of Python: 77.534 ms

    n = 10000000
    vec = [random.uniform(-1.0, 1.0) for _ in range(n)]

    start = time.time()
    result = sum(vec)
    duration = round((time.time() - start) * 1000, 3)

    print(f"Duration of Python: {duration} ms")
    print("The result is: ", result)
