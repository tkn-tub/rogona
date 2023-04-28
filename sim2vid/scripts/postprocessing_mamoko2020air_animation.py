# from: https://webgit.ccs-labs.org/git/CCS/mamoko2020pogona/src/branch/nanocom2021air/scripts/postprocessing_mamoko2020air_animation.py

#!/usr/bin/env python3

from typing import List
import glob
import argparse
import alive_progress
import contextlib

import matplotlib
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import matplotlib.animation as animation
import pandas as pd
# This import registers the 3D projection, but is otherwise unused.
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401 unused import
matplotlib.use("Agg")


plt.rcParams.update({
    "axes.facecolor": "black",
    "figure.facecolor": "black",
    "figure.edgecolor": "black",
    "savefig.facecolor": "black",
    "savefig.edgecolor": "black"
})


def init(
        tx_rx_distance_cm: float,
        rx_lat_offset_cm: float,
        viewport_width_cm: float,
        viewport_height_cm: float,
        ax: Axes3D,
        lines_by_tag,
):
    tx_rx_distance_m = tx_rx_distance_cm * 0.01
    rx_lat_offset_m = rx_lat_offset_cm * 0.01
    viewport_width_m = viewport_width_cm * 0.01
    viewport_height_m = viewport_height_cm * 0.01
    # X: Depth
    # Y: Spray direction
    # Z: Height of spray nozzle
    ax.set_xlim3d(-1, 1)
    ax.set_ylim3d(
        tx_rx_distance_m - (viewport_width_m/2),
        tx_rx_distance_m + (viewport_width_m/2)
    )
    ax.set_zlim3d(
        rx_lat_offset_m - viewport_height_m/2,
        rx_lat_offset_m + viewport_height_m/2
    )
    ax.view_init(elev=0.0, azim=0.0)

    # Must return an iterable of artists to be redrawn:
    return list(lines_by_tag.values())


def update(
        frame,
        receiver_dt: float,
        simulation_dt: float,
        processed_data: List[pd.DataFrame],
        lines_by_tag,  # list of Matplotlib Lines3D (Lines2D??)
        bar,
):
    step = receiver_dt / simulation_dt
    frame_cursor = int(frame*step)
    if frame_cursor >= len(processed_data):
        frame_cursor = len(processed_data) - 1

    # print("Time step: " + str(frame_cursor))
    if bar is not None:
        bar.text(f"Time step: {frame_cursor}")
        bar()

    for tag, lines in lines_by_tag.items():
        if tag == "":
            data = processed_data[frame_cursor]
        else:
            data = (
                processed_data[frame_cursor].query(f"tag == \"{tag}\"")
            )
        lines.set_data(data["x"], data["y"])
        lines.set_3d_properties(data["z"])

    # Must return an iterable of all artists that were modified:
    return list(lines_by_tag.values())


def main():
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Create a video from recorded particle positions.",
    )
    parser.add_argument(
        '-i',
        type=str,
        default="molecule_positions",
        help="Folder containing particle position CSVs, one for each "
             "simulation time step.",
    )
    parser.add_argument('-o', type=str, default="video.mp4")
    parser.add_argument('--video-width', type=int, default=1280,)
    parser.add_argument('--video-height', type=int, default=768,)
    parser.add_argument('--tx-rx-distance-cm', type=float, default=42,)
    parser.add_argument(
        '--rx-lat-offset-cm',
        type=float,
        default=0,
        help="Lateral offset of the receiver in z-direction from the "
             "origin.",
    )
    parser.add_argument(
        '--progress',
        action='store_true',
        help="Show progress bar",
    )
    parser.add_argument(
        '--color-by-tag',
        type=str,
        nargs='*',
        default=[":#007623"],  # no tag -> use default color
        help="Define different colors for each possible particle tag. "
             "Expects a list of tag-color pairs separated by colons, e.g.: "
             "`mytag:#00ff33 othertag:#1234ab`.",
    )
    parser.add_argument(
        '--filter-tags',
        nargs='*',
        help="Plot only particles that have one of the listed tags.",
    )
    args = parser.parse_args()

    # Settings of the receiver
    viewport_width_cm = 15  # ! is equal to 30cm at z = 0? SCC works with 15cm
    viewport_height_cm = (
        (args.video_height / args.video_width)
        * viewport_width_cm
    )

    # Settings of the particles
    # particle_color = (0 / 255, 118 / 255, 35 / 255)
    particle_size = 30

    # Timing settings
    simulation_delta_time = 0.002083
    receiver_delta_time = 0.002083

    processed_data = []

    csv_files = args.i + '/*.csv.*'
    video_file = args.o

    print(
        f"Using molecule positions from {csv_files} "
        f"and the following tags: {args.filter_tags}"
    )

    # initialize empty list that we will append dataframes to
    particle_pos_csv_filenames = glob.glob(csv_files)
    list_data = []

    # TODO: This could be more efficient. We probably don't need
    #  all positions all at once.
    for filename in particle_pos_csv_filenames:
        df = pd.read_csv(filename, index_col=0)
        if args.filter_tags is not None and len(args.filter_tags) > 0:
            df = df[df.tag.isin(args.filter_tags)]
        time_step = int(filename.split(".")[-1])
        list_data.append((time_step, df))

    for time_step, data in sorted(list_data, key=lambda tup: tup[0]):
        processed_data.append(data)

    # Setup plot
    dpi = 10
    fig = plt.figure(
        num=None,
        figsize=(args.video_width / dpi, args.video_height / dpi),
        dpi=dpi,
        facecolor='w',
        edgecolor='k',
    )
    plt.autoscale(tight=True)
    plt.axis('off')

    ax = fig.add_subplot(111, projection='3d', label='main')
    ax._axis3don = False

    # Parse colors by tags:
    if len(args.color_by_tag) == 0:
        raise ValueError(
            "Need at least one color. (Use `--color-by-tag :#<color>` "
            "if you are not using tags to set a default color.)"
        )
    colors_by_tag = dict()
    for s in args.color_by_tag:
        split = s.split(':')
        if len(split) != 2:
            raise ValueError(f"\"{s}\" is not a valid tag-color pair.")
        tag, color = split
        if tag == "" and len(args.color_by_tag) > 1:
            raise ValueError(
                "The empty tag \"\", which is supposed to apply to all "
                "particles, was passed to --color-by-tag, "
                "but other tags were specified as well. "
            )
        colors_by_tag[tag] = color

    # Create a Matplotlib Lines instance to update for each frame of the video.
    # Using `ax.plot` rather than `ax.scatter` for efficiency.
    # This means we need a separate call to `plot` for each color, though.
    # -> dict of instances by tag:
    lines_by_tag = dict()
    for tag, color in colors_by_tag.items():
        lines_by_tag[tag], = ax.plot(
            [],
            [],
            [],
            linestyle="",
            marker="o",
            markersize=particle_size,
            markerfacecolor=color,
        )

    step = receiver_delta_time / simulation_delta_time
    num_frames = int(len(processed_data) / step)
    with alive_progress.alive_bar(
        total=num_frames,
        title="Animating…",
        bar='filling',
    ) if args.progress else contextlib.nullcontext() as bar:
        ani = FuncAnimation(
            fig,
            lambda frame: update(
                frame=frame,
                receiver_dt=receiver_delta_time,
                simulation_dt=simulation_delta_time,
                processed_data=processed_data,
                lines_by_tag=lines_by_tag,
                bar=bar,
            ),
            frames=num_frames,
            init_func=lambda: init(
                tx_rx_distance_cm=args.tx_rx_distance_cm,
                rx_lat_offset_cm=args.rx_lat_offset_cm,
                viewport_width_cm=viewport_width_cm,
                viewport_height_cm=viewport_height_cm,
                ax=ax,
                lines_by_tag=lines_by_tag,
            ),
            blit=True,
        )

        writer_cls = animation.writers['ffmpeg']
        writer = writer_cls(
            fps=25,
            metadata=dict(artist='Pogona Simulator'),
            bitrate=-1,
        )
        ani.save(video_file, writer=writer)


if __name__ == "__main__":
    main()

