from matplotlib.colors import LinearSegmentedColormap
from matplotlib.gridspec import GridSpec
import numpy as np
import matplotlib.pyplot as plt
import mpl_scatter_density
from z_n_gauge.data_analysis.plot_times_series import draw_times_series
from z_n_gauge.data_analysis.python_scripts.plotters import plot_data, plot_scatters


data = np.load("_recordings/CLUSTER_2024-04-18--17-27-02/archive.npz")


sorted_list = sorted(
    data.values(), key=lambda x: x["run_info"]["run_id"][0])
combined = np.concatenate(sorted_list)

x = 40
left = 40+x
right = 60+x
combined = combined[left:right]

translated_combines = combined.copy()


number_of_betas = len(combined)
recordings, x_dim, y_dim, z_dim = combined['backup_data'][0].shape

time_series = np.zeros((number_of_betas, recordings, 2))
david_data = np.zeros(number_of_betas)
tin_data = np.zeros(number_of_betas)

betas = combined["reboot_seed"]["experiment_parameters"]["beta"]
z_order = combined["reboot_seed"]["experiment_parameters"]["z_order"][0]
roots_of_unity = np.array([[np.cos(2 * np.pi * i / z_order),
                            np.sin(2 * np.pi * i / z_order)] for i in range(z_order)])


for (i, data_step) in enumerate(combined):
    print(i)
    loops = data_step['backup_data']
    loops = roots_of_unity[loops]

    spacial_dimensions = x_dim * y_dim * z_dim
    normalization = loops[..., 0].size

    # Time series
    temp = loops[...].sum(axis=(1, 2, 3)) / spacial_dimensions
    time_series[i] = temp

    # Tin Data
    temp = loops.sum(axis=(1, 2, 3))
    tin_snapshot = np.linalg.norm(temp, axis=-1).sum()
    tin_data[i] = tin_snapshot/normalization

    # David data
    temp = loops[:, :, :, :, :].sum(axis=0)
    david_snapshot = np.linalg.norm(temp, axis=-1).sum()
    david_data[i] = david_snapshot/normalization

print("Data Processed")


# Draw David Data and Tin Data, draw time series
fig, ax = plt.subplots(1, 2)
plot_data(ax[0], betas, tin_data, david_data)
draw_times_series(0, time_series, tin_data, ax[1])


# Draw scatters
indices_to_scatter = range(len(time_series))
columns = 8
fig = plt.figure(layout="constrained", figsize=(16, 8))
plot_scatters(columns, indices_to_scatter, time_series, fig)


plt.show()
