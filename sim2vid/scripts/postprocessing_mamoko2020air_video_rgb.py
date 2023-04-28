# from: https://webgit.ccs-labs.org/git/CCS/mamoko2020pogona/src/branch/nanocom2021air/scripts/postprocessing_mamoko2020air_video_rgb.py 

#!/usr/bin/env python3

from typing import Tuple
import os
import argparse
import contextlib
import numpy as np
import pandas as pd
import cv2
import alive_progress


def main():
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Count the number of red, green, and blue pixels "
                    "for each frame of a video; export to CSV.",
    )
    parser.add_argument('-i', default="video.mp4",)
    parser.add_argument('-o', default="rgb.csv",)
    parser.add_argument('--fps', type=float,)
    parser.add_argument(
        '--progress',
        action='store_true',
        help="Show progress bar",
    )
    parser.add_argument(
        '--noise-threshold',
        type=float,
        help="Subpixel values below this threshold (between 0 and 255) "
             "will be ignored.",
        default=5,
    )
    parser.add_argument(
        '--window-v-px-start',
        type=int,
        help="Ignore pixels outside this window. "
             "Vertical start, top to bottom.",
        default=0,
    )
    parser.add_argument(
        '--window-v-px-end',
        type=int,
        help="Ignore pixels outside this window. "
             "Vertical end, top to bottom. -1 => Use full height.",
        default=-1,
    )
    parser.add_argument(
        '--window-h-px-start',
        type=int,
        help="Ignore pixels outside this window. "
             "Horizontal start, top to bottom.",
        default=0
    )
    parser.add_argument(
        '--window-h-px-end',
        type=int,
        help="Ignore pixels outside this window. "
             "Horizontal end, top to bottom. -1 => Use full width.",
        default=-1,
    )
    args = parser.parse_args()

    df = analyze_video(
        filename=args.i,
        noise_threshold=args.noise_threshold,
        fps=args.fps,
        show_progress=args.progress,
        window=(
            args.window_h_px_start,
            args.window_h_px_end,
            args.window_v_px_start,
            args.window_v_px_end,
        ),
    )
    df.to_csv(args.o)


def analyze_video(
        filename: str,
        noise_threshold: int,
        fps: float,
        show_progress=False,
        window: Tuple[int, int, int, int] = (0, -1, 0, -1),
) -> pd.DataFrame:
    if not os.path.exists(filename):
        raise FileNotFoundError(f"Video \"{filename}\" does not exist.")
    video = cv2.VideoCapture(filename)
    num_frames = int(video.get(cv2.CAP_PROP_FRAME_COUNT))
    video_w = int(video.get(cv2.CAP_PROP_FRAME_WIDTH))
    video_h = int(video.get(cv2.CAP_PROP_FRAME_HEIGHT))
    norm_factor = 1 / (video_w * video_h * 255)

    win_h_start, win_h_end, win_v_start, win_v_end = window
    if win_h_end < 0:
        win_h_end = video_w
    if win_v_end < 0:
        win_v_end = video_h

    init_frame = None
    """The very first frame, used for background extraction."""

    data = dict(time_s=[], r=[], g=[], b=[])

    with alive_progress.alive_bar(
        total=num_frames,
        title="Reading frames…",
        bar='filling',
    ) if show_progress else contextlib.nullcontext() as bar:
        # Iterate through video frames:
        frame_i = 0
        while True:
            ret_val, frame = video.read()
            if not ret_val:
                break
            if show_progress:
                bar()

            # All frames converted to 16 bit R, G, B channels to prevent
            # clipping issues
            frame = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
            frame = frame.astype('int16')
            if init_frame is None:
                init_frame = frame.astype('int16')

            frame_filtered = np.clip(frame - init_frame, 0, 255)
            frame_filtered[frame_filtered < noise_threshold] = 0

            avg_r_normalized = frame_filtered[
                win_v_start:win_v_end,
                win_h_start:win_h_end,
                0
            ].sum() * norm_factor
            avg_g_normalized = frame_filtered[
                win_v_start:win_v_end,
                win_h_start:win_h_end,
                1
            ].sum() * norm_factor
            avg_b_normalized = frame_filtered[
                win_v_start:win_v_end,
                win_h_start:win_h_end,
                2
            ].sum() * norm_factor

            data['time_s'].append(frame_i / fps)
            data['r'].append(avg_r_normalized)
            data['g'].append(avg_g_normalized)
            data['b'].append(avg_b_normalized)
            frame_i += 1

    return pd.DataFrame(data)


if __name__ == '__main__':
    main()
