import sys
import matplotlib as mpl
from matplotlib.collections import LineCollection
from matplotlib.colors import LinearSegmentedColormap
from matplotlib.gridspec import GridSpec
import numpy as np
import matplotlib.pyplot as plt
import mpl_scatter_density

# from data_analysis.plot_times_series import plot_region

# from data_analysis.python_scripts.plot_times_series import draw_times_series, plot_region, plot_selected_ts


# data = np.load('data_analysis/temp/transfer_zip.npz')


load_data_flag = False

if not load_data_flag:
    print("Producing data from scratch")
    print("Loading data")

    data = np.load(
        'z_n_gauge/data_analysis/temp/restructure_test.npz')

    raw_data = data["loops"]
    meta_data = data["meta_data"]

    recordings, x_dim, y_dim, z_dim, _ = loops = raw_data['data'][0].shape
    number_of_betas = len(raw_data)

    time_series = np.zeros((number_of_betas, recordings, 2))
    david_data = np.zeros(number_of_betas)
    tin_data = np.zeros(number_of_betas)
    betas = np.zeros(number_of_betas)

    for (i, data_step) in enumerate(raw_data):
        # print(i)
        betas[i] = (data_step['beta'])
        loops = raw_data['data'][i]

        recordings, x_dim, y_dim, z_dim, _ = loops.shape
        spacial_dimensions = x_dim * y_dim * z_dim
        normalization = loops[..., 0].size

        # Time series
        temp = loops[...].sum(axis=(1, 2, 3)) / spacial_dimensions
        # temp = np.linalg.norm(temp, axis = -1)
        time_series[i] = temp

        # Tin Data
        temp = loops.sum(axis=(1, 2, 3))
        tin_snapshot = np.linalg.norm(temp, axis=-1).sum()
        tin_data[i] = tin_snapshot/normalization
        # tin_data.append(tin_snapshot/normalization)

        # David data
        temp = loops[:, :, :, :, :].sum(axis=0)
        david_snapshot = np.linalg.norm(temp, axis=-1).sum()
        david_data[i] = david_snapshot/normalization
        # david_data.append(david_snapshot/normalization)

    # Save the arrays to a file
    np.savez('z_n_gauge/data_analysis/temp/arrays.npz', tin_data=tin_data,
             david_data=david_data, time_series=time_series, betas=betas)

else:
    print("Loading data from file")
    arrays = np.load('data_analysis/temp/transfer_zip.npz')
    tin_data = arrays['tin_data']
    david_data = arrays['david_data']
    time_series = arrays['time_series']
    betas = arrays['betas']

z_order = 3

plt.plot(betas, tin_data, 'go', label="Tin", )
plt.plot(betas, david_data, 'bo', label="David", )
plt.show()

# plt.savefig(
#     f"z_n_gauge/data_analysis/figs/z_3_beta_sweep.pdf")


n = 8
m = 8
region = range(len(time_series))

fig = plt.figure(layout="constrained", figsize=(16, 8))
gs = GridSpec(n, m, figure=fig)

axes = []

if len(region) >= n*m:
    step = len(region) // (n*m)
    indices = np.linspace(0, len(region) - 1, n*m, dtype=int)
    selected_ts = [region[i] for i in indices]
else:
    print("Not enough elements in the list")


white_viridis = LinearSegmentedColormap.from_list('white_viridis', [
    (0, '#ffffff'),
    (1e-20, '#440053'),
    (0.2, '#404388'),
    (0.4, '#2a788e'),
    (0.6, '#21a784'),
    (0.8, '#78d151'),
    (1, '#fde624'),
], N=256)


for i in range(n):
    for j in range(m):
        if i*m+j < len(region):
            index = region[i*m+j]

            time_serie = time_series[index]

            x = time_serie[:, 0]
            y = time_serie[:, 1]

            axes.append(fig.add_subplot(
                gs[i, j], projection='scatter_density'))
            axes[-1].scatter_density(x, y, cmap=white_viridis)
            axes[-1].set_xlim(-1, 1)
            axes[-1].set_ylim(-1, 1)
            axes[-1].set_aspect('equal', 'box')
            axes[-1].set_xticks([])
            axes[-1].set_yticks([])
            axes[-1].set_title(f"{index}")
            circle = plt.Circle((0, 0), 1, color='black', fill=False)
            axes[-1].add_artist(circle)

plt.gca().set_adjustable("box")
# plt.savefig(
#     f"z_n_gauge/data_analysis/figs/z_3_discs.pdf")

plt.show()


# outliers = []
# for (i, beta) in enumerate(betas):
#     if abs(tin_data[i] - david_data[i]) > 0.12:
#         outliers.append(i)
# outliers_betas = [betas[i] for i in outliers]
# outliers_tin_data = [tin_data[i] for i in outliers]
# max_outlier = min(5, len(outliers))

# outliers_left = outliers[0]
# outliers_right = outliers[-1]

# all_indices = range(len(betas))
# regions = []
# regions.append(all_indices[0:outliers_left])
# regions.append(all_indices[outliers_left: outliers_right+2])
# regions.append(all_indices[outliers_right+2:])
# for (i, region) in enumerate(regions):
#     plot_region(outliers, max_outlier, betas, tin_data,
#                 david_data, time_series, z_order, region, f"region{i+1}")


# white_viridis = LinearSegmentedColormap.from_list('white_viridis', [
#     (0, '#ffffff'),
#     (1e-20, '#440053'),
#     (0.2, '#404388'),
#     (0.4, '#2a788e'),
#     (0.6, '#21a784'),
#     (0.8, '#78d151'),
#     (1, '#fde624'),
# ], N=256)

# index = outliers_right


# n, m = 4, 7
# all_indices = range(len(betas))
# regions = []
# regions.append(all_indices[0:outliers_left])
# regions.append(all_indices[outliers_left: outliers_right+2])
# regions.append(all_indices[outliers_right+2:])

# all_selected = []

# for (region_index, region) in enumerate(regions):
#     fig = plt.figure(layout="constrained", figsize=(16, 8))
#     gs = GridSpec(n, m, figure=fig)

#     axes = []

#     if len(region) >= n*m:
#         step = len(region) // (n*m)
#         indices = np.linspace(0, len(region) - 1, n*m, dtype=int)
#         selected_ts = [region[i] for i in indices]
#     else:
#         print("Not enough elements in the list")

#     all_selected += selected_ts

#     for i in range(n):
#         for j in range(m):
#             index = selected_ts[i*m+j]

#             time_serie = time_series[index]

#             x = time_serie[:, 0]
#             y = time_serie[:, 1]

#             axes.append(fig.add_subplot(
#                 gs[i, j], projection='scatter_density'))
#             axes[-1].scatter_density(x, y, cmap=white_viridis)
#             axes[-1].set_xlim(-1, 1)
#             axes[-1].set_ylim(-1, 1)
#             axes[-1].set_aspect('equal', 'box')
#             axes[-1].set_xticks([])
#             axes[-1].set_yticks([])
#             axes[-1].set_title(f"{index}")
#             circle = plt.Circle((0, 0), 1, color='black', fill=False)
#             axes[-1].add_artist(circle)

#     plt.gca().set_adjustable("box")
#     plt.savefig(
#         f"data_analysis/figs/recording_z7_polyakov/region_{region_index+1}_disc.pdf")
#     # plt.show()
