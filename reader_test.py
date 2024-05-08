import numpy as np

# data = np.load("_test/2024-05-07--20-39-57/4-L_4_4_4_4-b_1p57-l_0p00/1.npy")


# for key in data['ploop_corr_recordings'].dtype.names:
#     print(key, data['ploop_corr_recordings'][key].shape)


# print(np.linspace(0.5, 3.0, 8))

data = np.load(
    "_recordings/Z_7_all_measurements/1-L_6_6_6_6-b_0p50-l_0p00/1.npy")


print(data['ploop_corr_recordings'].dtype)
