from matplotlib.collections import LineCollection
from matplotlib.colors import LinearSegmentedColormap
from matplotlib.gridspec import GridSpec
import numpy as np
import matplotlib.pyplot as plt


def plot_data(ax, betas, tin_data, david_data):
    ax.plot(betas, tin_data, 'go', label="Tin", )
    ax.plot(betas, david_data, 'bo', label="David", )


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

    ax.plot([0, number_of_recordings], 2*[tin_data[index]], color='orange')
    ax.set_ylim(0, 1)


def plot_scatters(columns, indices_to_scatter, time_series, fig):
    white_viridis = LinearSegmentedColormap.from_list('white_viridis', [
        (0, '#ffffff'),
        (1e-20, '#440053'),
        (0.2, '#404388'),
        (0.4, '#2a788e'),
        (0.6, '#21a784'),
        (0.8, '#78d151'),
        (1, '#fde624'),
    ], N=256)

    axes = []

    rows = min(1 + len(indices_to_scatter) // columns, 5)
    gs = GridSpec(rows, columns, figure=fig)

    for i in range(rows):
        for j in range(columns):
            if i*columns + j >= len(indices_to_scatter):
                break

            index = i * columns + j

            time_serie = time_series[index]

            # Concatenate the time series with its rotation by 2pi/z_order

            time_serie_rotated = time_serie.copy()
            rotation_matrix = np.array([[np.cos(2 * np.pi / 3), -np.sin(2 * np.pi / 3)], [
                                       np.sin(2 * np.pi / 3), np.cos(2 * np.pi / 3)]])

            time_serie_rotated = np.dot(time_serie, rotation_matrix)
            time_serie = np.concatenate([time_serie, time_serie_rotated])

            time_serie_rotated = np.dot(time_serie_rotated, rotation_matrix)
            time_serie = np.concatenate([time_serie, time_serie_rotated])

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
