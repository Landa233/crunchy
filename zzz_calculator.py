import numpy as np


def iexp_7(n):
    return np.array([np.cos(2*np.pi*n/7), np.sin(2*np.pi*n/7)])


res = 24*iexp_7(1) + 18*iexp_7(6) + 3*iexp_7(5)
print(1/45 * res)


S = 4
V = 6 * 4**5
print(V)


res += (V-45)*iexp_7(0)

print(1/V * res)
