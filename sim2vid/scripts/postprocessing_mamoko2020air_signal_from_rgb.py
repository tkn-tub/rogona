#from https://webgit.ccs-labs.org/git/CCS/mamoko2020pogona/src/branch/nanocom2021air/scripts/postprocessing_mamoko2020air_signal_from_rgb.py

#!/usr/bin/env python3

import argparse
import pandas as pd

import common


def main():
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
        description="Extract the signal from a given CSV file with"
                    "R, G, B values (average subpixel values per frame).",
    )
    parser.add_argument(
        '-i',
        required=True,
        help="CSV file with r, g, b cols.",
    )
    parser.add_argument(
        '-o',
        required=True,
        help="Output CSV with a new column for 'value'",
    )
    parser.add_argument(
        '--value-col-name', default="value",
    )
    common.add_argparse_arguments_plotting(parser)
    args = parser.parse_args()
    common.set_plotting_style_from_args(args)

    df = pd.read_csv(args.i)

    df[args.value_col_name] = df.g  # TODO: more options to filter colors
    df = df.drop(columns=['r', 'g', 'b'])
    df.to_csv(args.o)


if __name__ == '__main__':
    main()
