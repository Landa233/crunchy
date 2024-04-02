import numpy as np

# read zzz.npy


def read_zzz():
    zzz = np.load('zzz.npy')
    return zzz


a = read_zzz()
print(a.dtype.fields)

print(a[0]['a'])
print(a[0]['b'])
