import numpy as np

# read zzz.npy


def read_zzz():
    zzz = np.load('zzz.npy')
    return zzz


a = read_zzz()
for field in a.dtype.fields:
    print(field)
    print(a[field])
    print()
