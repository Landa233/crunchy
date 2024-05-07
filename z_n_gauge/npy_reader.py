import os
import numpy as np
import matplotlib.pyplot as plt

folder_name = "_recordings/Correlators_2024-05-02--16-56-11/correlators.npz"

data = np.load(folder_name)

# for key in data.keys():
#     print(data["12"][0]["corr"].shape)


# Sort the keys
keys = list(data.keys())
keys = [int(key) for key in keys]
keys.sort()

normaliser = 12
fig = plt.figure()

for key in keys:
    if key == 8:
        continue
    correlators = data[str(key)][0]["corr"]
    # average along axis 1
    correlators = np.mean(correlators, axis=1)
    real_part = correlators[:, 0]

    Ls = np.arange(len(real_part))
    Ls -= 1
    max_L = np.max(Ls)

    Ls = Ls / max_L
    print(Ls)

    plt.plot(Ls[2:-1], np.log(real_part[2:-1]), '-o', label=f"L={key}")


# Turn off x axis numbers
plt.gca().axes.get_xaxis().set_visible(False)

plt.legend()
plt.show()
