import numpy as np

# load _test/2024-04-09--17-10-52/archive.npz

# data = np.load('_test/2024-04-09--17-10-52/archive.npz')

# for key in data.keys():
#     print(key)
#     print(data[key].dtype.names)
#     print(data[key]['run_info'].dtype)


# print(np.linspace(0, 1, 1))


data = np.load('_recordings/CLUSTER-2024-04-23--08-17-20/archive.npz')


for key in data.keys():
    print(key)
    # print(data[key].dtype.names)
    # print(data[key]['run_info'].dtype)
