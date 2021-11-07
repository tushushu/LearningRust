import time
import random
if __name__ == "__main__":
    # Duration of Python: 94.806 ms
    start = time.time()

    result = 0.0
    m = n = 1000
    for _ in range(m):
        for _ in range(n):
            result += random.uniform(-1.0, 1.0)

    duration = (time.time() - start) * 1000
    print(f"Duration of Python: {duration} ms")
    print("The result is: ", result)
