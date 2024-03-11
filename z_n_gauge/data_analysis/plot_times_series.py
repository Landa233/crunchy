from matplotlib import pyplot as plt
from matplotlib.collections import LineCollection
from matplotlib.gridspec import GridSpec
import numpy as np
import matplotlib as mpl


def draw_times_series(index, time_series,  tin_data, ax,):
    time_serie = time_series[index]
    radii = np.linalg.norm(time_serie, axis=-1)

    angles = np.arctan2(time_serie[:, 1], time_serie[:, 0])
    angles = np.mod(angles, 2 * np.pi) / (2 * np.pi)

    number_of_recordings = time_serie.shape[0]

    rec = [i for i in range(number_of_recordings)]
    points = np.column_stack([rec, radii])
    segs = [[points[i], points[i + 1]] for i in range(0, len(radii)-1)]

    line_segments = LineCollection(segs, array=angles, cmap='hsv', linewidth=2)
    ax.add_collection(line_segments)

    # axes[i+1].plot(range(recordings), radii)

    ax.plot([0, number_of_recordings], 2*[tin_data[index]], color='orange')
    ax.set_ylim(0, 1)


def plot_selected_ts(betas, tin_data, david_data, time_series, axes, gs, z_order, fig, region):
    # Ensure there are at least 5 elements
    if len(region) >= 5:
        step = len(region) // 5
        indices = np.linspace(0, len(region) - 1, 5, dtype=int)
        selected_ts = [region[i] for i in indices]
    else:
        print("Not enough elements in the list")

    for (i, index) in enumerate(range(min(5, len(selected_ts)))):
        axes.append(fig.add_subplot(gs[index, 1]))
        index = selected_ts[index]
        draw_times_series(index, time_series, tin_data, axes[-1])
        axes[-1].set_title(f"{i}: β = {betas[index]:.2f}")

    for (i, index) in enumerate(selected_ts):
        axes[0].scatter(betas[index], tin_data[index], color='g',
                        marker='o', s=50, zorder=100)
        offset = [-0.03, 0.02]
        axes[0].annotate(
            f'{i}', (betas[index]+offset[0], tin_data[index]+offset[1]),)

    axes[0].grid(True, linestyle='--', alpha=0.7)
    axes[0].scatter(betas, david_data, color='b', marker='o',
                    label=r'$  \sum_{x,y,z}\ |   \sum_iW^i_{x,y,z}|$', s=31)

    axes[0].set_title(r"$\mathbb{Z}_{%d}$" % z_order)
    axes[0].scatter(betas, tin_data, color='r', marker='o',
                    label=r'$\sum_i \ | \sum_{x,y,z} W^i_{x,y,z}|$', s=10)
    axes[0].set_ylim(0.0, 1.1)

    # Make Color Bar
    cmap = mpl.cm.hsv
    mapa = mpl.cm.ScalarMappable(cmap=cmap)
    cax, kw = mpl.colorbar.make_axes(axes[1:])
    cbar = plt.colorbar(mapa, cax=cax, **kw)
    cbar.set_label('Average Phase')
    cbar.set_ticks([0, 0.5, 1.0])
    cbar.set_ticklabels(['0', '$\pi$', '$2\pi$'])


def plot_region(outliers, max_outlier, betas, tin_data, david_data, time_series, z_order, region, dest_name):
    fig = plt.figure(layout="constrained", figsize=(12, 8))

    if len(outliers) != 0:
        gs = GridSpec(max_outlier, 2, figure=fig)
        ax0 = fig.add_subplot(gs[:, 0])
    else:
        ax0 = fig.gca()
    axes = [ax0]

    # Plot outlier lines
    outliers_left = outliers[0]
    outliers_right = outliers[-1]
    axes[0].plot([betas[outliers_left], betas[outliers_left]],
                 [-2, 2], color='black')
    axes[0].plot([betas[outliers_right], betas[outliers_right]],
                 [-2, 2], color='black')

    plot_selected_ts(betas, tin_data, david_data,
                     time_series, axes, gs, z_order, fig, region)

    plt.savefig(f"data_analysis/figs/recording_z7_polyakov/{dest_name}.pdf")
    plt.show()
