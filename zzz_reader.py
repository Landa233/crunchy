import numpy as np

# read zzz.npy


for i in range(1, 10):
    # how to check if a file exists?
    try:
        zzz = np.load(f'_test/1/backup_{i}.npy')
        print(zzz['run_info']['recordings'])
    except:
        continue


# a = read_zzz()
# for field in a.dtype.fields:
#     print(field)
#     print(a[field])
#     print()
