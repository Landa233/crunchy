import numpy as np

# data = np.load("_test/2024-05-07--20-39-57/4-L_4_4_4_4-b_1p57-l_0p00/1.npy")


# for key in data['ploop_corr_recordings'].dtype.names:
#     print(key, data['ploop_corr_recordings'][key].shape)


# print(np.linspace(0.5, 3.0, 8))

data = np.load(
    "_test/2024-06-09--11-35-24/8-L_3_3_3_3-b_1p00-l_1p00/1.npy")


print(data['ploop_corr_recordings'].dtype)
