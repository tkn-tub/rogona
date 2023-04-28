#from https://webgit.ccs-labs.org/git/CCS/mamoko2020pogona/src/branch/nanocom2021air/scripts/common.py
from typing import Tuple, Iterable
import io
import itertools
import numpy as np
import scipy.stats
import matplotlib as mpl
# import matplotlib.pyplot as plt
import seaborn as sns


LATEX_PAPER_WIDTH_M = 0.216  # "Portrait Letter / ANSI A", not DIN A4
LATEX_PAPER_WIDTH_IN = LATEX_PAPER_WIDTH_M * 39.37
LATEX_PAPER_PAPERWIDTH_PT = 614.295
LATEX_PAPER_DPI = LATEX_PAPER_PAPERWIDTH_PT / LATEX_PAPER_WIDTH_IN
"""
Dots per inch of a LaTeX document.
Can be calculated by dividing the \\the\\paperwidth from LaTeX
by the reported width of the document (converted to inches).
"""
LATEX_PAPER_TEXTWIDTH_PT = 506.295
LATEX_PAPER_COLUMN_WIDTH_PT = 241.14749
PRESENTATION_DPI = 96  # PowerPoint?
PPT_WIDTH_PT = 960  # 4:3 PowerPoints are 10 times 7.5 inches
PRESENTATION_WIDTH_PT = 1279.968  # PowerPoint?

LINE_STYLES_PAPER = ['-', '--', '-.', ':'] * 4
LINE_STYLES_PRESENTATION = ['-'] * 10


style_dpi = LATEX_PAPER_DPI
"""Dots per inch; replaced in set_plotting_style."""
style_default_width_pt = LATEX_PAPER_COLUMN_WIDTH_PT
"""Default width in pt; replaced in set_plotting_style."""
style_default_width_large_pt = LATEX_PAPER_TEXTWIDTH_PT
style_color_float_palette_name = 'viridis'
"""Color palette to use for more than a handful of categories."""
# TODO: replace depending on set style
style_color_cat_palette = sns.color_palette('colorblind')
style_line_styles = LINE_STYLES_PAPER


def set_plotting_style(style: str, enable_pgf=True):
    """
    :param style: One of 'paper', 'paper-color', 'presentation'.
    """
    global style_dpi
    global style_default_width_pt
    global style_default_width_large_pt
    global style_color_cat_palette
    global style_line_styles

    sns.set(
        context='paper' if style.startswith('paper') else 'talk',
        style='ticks',
        # color_palette='Set2',  # doesn't work
    )
    font_size = 11 if style == 'presentation' else 8
    settings = {  # setup matplotlib to use latex for output
        # Settings adapted from
        # https://gist.github.com/bougui505/b844feb1bdfba3072ca92a6d6961f71b

        # Change this if using xelatex or lualatex:
        "pgf.texsystem": "pdflatex",
        "text.usetex": True,  # use LaTeX to write all text
        "text.latex.preamble": r"\usepackage{amsmath}",
        "font.family": 'sans-serif' if style == 'presentation' else 'serif',
        # Blank entries should cause plots to inherit fonts from the document:
        "font.serif": [],  # Use LaTeX default
        "font.sans-serif": ["DejaVu Sans"],
        "font.monospace": [],
        "axes.labelsize": font_size,  # LaTeX default is 10pt font.
        "axes.axisbelow": True,
        "font.size": font_size,
        "legend.fontsize": font_size,
        "xtick.labelsize": font_size,
        "ytick.labelsize": font_size,
        "legend.fontsize": font_size,
        "legend.title_fontsize": font_size,
        "errorbar.capsize": 9,  # error bar cap length in pt
        # "figure.figsize": figsize(0.9),  # default fig size of 0.9 textwidth
        "pgf.preamble": (
            r"\usepackage{csquotes}"
            r"\usepackage{amsmath}"
            r"\usepackage[utf8x]{inputenc}"
            r"\usepackage[T1]{fontenc}"
        ),
        "savefig.dpi": 300
    }
    # if style == 'presentation':
    #     settings["pgf.preamble"] += r"\usepackage{sansmath}"
    if enable_pgf:
        # More information on *.pgf output:
        # https://matplotlib.org/users/pgf.html
        mpl.use("pgf")
    # For all options, see https://matplotlib.org/1.5.1/users/customizing.html
    mpl.rcParams.update(settings)

    # if style == 'presentation':
    #     mpl.rcParams.update({
    #         "lines.color": COLOR_UPB_BLUE,
    #         "text.color": COLOR_UPB_BLUE,
    #         "axes.labelcolor": COLOR_UPB_BLUE,
    #         "axes.edgecolor": COLOR_UPB_BLUE,
    #         "xtick.color": COLOR_UPB_BLUE,
    #         "ytick.color": COLOR_UPB_BLUE,
    #         "grid.color": COLOR_UPB_GRAY
    #     })

    if style.startswith('paper'):
        style_dpi = LATEX_PAPER_DPI
        style_default_width_pt = LATEX_PAPER_COLUMN_WIDTH_PT
        style_default_width_large_pt = LATEX_PAPER_TEXTWIDTH_PT
        style_color_cat_palette = ['k'] * 20
        style_line_styles = LINE_STYLES_PAPER
    if style == 'paper-color':
        style_color_cat_palette = ['#2649ab', 'k', '#a31869']
    if style == 'presentation':
        style_dpi = PRESENTATION_DPI
        style_default_width_pt = PPT_WIDTH_PT
        style_default_width_large_pt = style_default_width_pt
        style_color_cat_palette = sns.color_palette('colorblind')
        style_line_styles = LINE_STYLES_PRESENTATION


def add_argparse_arguments_plotting(parser):
    parser.add_argument(
        '--plotting-style',
        choices=['paper', 'paper-color', 'presentation'],
        help="Plotting style",
        default='paper',
    )
    parser.add_argument(
        '--plotting-disable-pgf',
        action='store_true',
    )


def set_plotting_style_from_args(args):
    set_plotting_style(
        style=args.plotting_style,
        enable_pgf=not args.plotting_disable_pgf,
    )


def get_latex_fig_size(
         scale,
         ratio=(np.sqrt(5.0) - 1.0) / 2.0,
         text_width_pt=LATEX_PAPER_COLUMN_WIDTH_PT,
         dpi=LATEX_PAPER_DPI
) -> Tuple[float, float]:
    """
    Modified from
    https://gist.github.com/bougui505/b844feb1bdfba3072ca92a6d6961f71b
    :param scale:
    :param text_width_pt: Maximum figure width in pt.
        Can be obtained from LaTeX via \\the\\textwidth or \\the\\paperwidth.
    :param ratio: Ratio of the image; h = ratio * width. Default: golden ratio.
    :param dpi: Dots per inch of the document.
        Can be calculated by dividing the \\the\\paperwidth from LaTeX
        by the reported width of the document (converted to inches).
    :return: a matplotlib-compatible figsize (i.e., in inches) as a fraction of
        \\textwidth in LaTeX.
    """
    fig_width = text_width_pt * scale / dpi  # width in inches
    fig_height = fig_width * ratio  # height in inches
    fig_size = (fig_width, fig_height)
    # print("Fig size: {}".format(fig_size))
    return fig_size


def camel(
        snake_str,
        upper=False,
        keep_other_case=False,
        del_non_alphanum=True
) -> str:
    """
    Convert snake case to camel case (e.g. "c_low_mean" to "cLowMean").
    From https://stackoverflow.com/a/42450252/1018176
    :param snake_str:
    :param upper:
    :param keep_other_case: Instead of 'c_low_MeAn' -> 'CLowMean',
        do 'CLowMeAn'.
    :param del_non_alphanum: Delete non-alphanumeric characters.
    :return:
    """
    if del_non_alphanum:
        snake_str = ''.join([c for c in snake_str if c.isalnum() or c == '_'])
    first, *others = snake_str.split('_')
    if not keep_other_case:
        return ''.join([
            first.title() if upper else first.lower(), *map(str.title, others)
        ])
    else:
        return ''.join([
            (first[0].upper() if upper else first[0].lower()) + first[1:],
            *[other[0].upper() + other[1:] for other in others]
        ])


def latex_newcommand(cmd, val) -> str:
    digits = '0123456789'
    # Digits to letters b/c LaTeX doesn't like numbers in variable names:
    cmd = ''.join([
        c if c not in digits
        else 'ABCDEFGHIJ'[digits.index(c)]
        for c in cmd
    ])
    if isinstance(val, float):
        val = f'{val:.10f}'
    return f"\\newcommand{{\\{cmd}}}{{{val}}}"


def stats_to_latex(
        stats: dict,
        filename: str,
        cmd_prefix="Sensors",
):
    """
    Write contents of a dictionary to LaTeX `\\newcommand`s.
    """
    def _write_recursively(
            _f: io.TextIOWrapper,
            _prefix: str,
            _stats: dict
    ):
        _prefix = camel(_prefix, upper=True, keep_other_case=True)
        exclude_keys = set(_stats.get('exclude_from_latex', set()))
        exclude_keys.add('exclude_from_latex')
        for k, v in _stats.items():
            if k in exclude_keys or str(k) in exclude_keys:
                continue
            k = str(k)  # allow int keys, for example
            cmd = f"{_prefix}{camel(k, upper=True, keep_other_case=True)}"
            if isinstance(v, dict):
                _write_recursively(_f=_f, _prefix=cmd, _stats=v)
                continue
            _f.write(latex_newcommand(cmd, v) + "\n")

    with open(filename, 'w') as f:
        _write_recursively(_f=f, _prefix=cmd_prefix, _stats=stats)


def get_ci(data_array, lvl=.95):
    if len(data_array) == 0:
        return [-np.inf, np.inf]
    return scipy.stats.t.interval(
        lvl,
        len(data_array) - 1,
        loc=np.mean(data_array),
        scale=scipy.stats.sem(data_array)
    )


def describe(data_array):
    if len(data_array) == 0:
        print("NO DATA")
        return
    interval = get_ci(data_array)
    print(f"Min: {np.min(data_array)}")
    print(f"1st quartile: {np.quantile(data_array, q=.25)}")
    print(f"Mean: {np.mean(data_array)}")
    print(f"Standard deviation: {np.std(data_array)}")
    print(f"Median: {np.median(data_array)}")
    print(f"3rd quartile: {np.quantile(data_array, q=.75)}")
    print(f"Max: {np.max(data_array)}")
    print(f"95% Confidence interval: {interval}")


def describe_to_dict(data_array):
    if len(data_array) == 0:
        return dict()
    interval = get_ci(data_array)
    # (Make sure all numbers are internal Python data types
    # so they won't cause trouble when writing to YAML.)
    return dict(
        min_val=float(np.min(data_array)),
        first_quartile=float(np.quantile(data_array, q=.25)),
        mean=float(np.mean(data_array)),
        standard_deviation=float(np.std(data_array)),
        median=float(np.median(data_array)),
        third_quartile=float(np.quantile(data_array, q=.75)),
        max_val=float(np.max(data_array)),
        ci_95_low=float(interval[0]),
        ci_95_high=float(interval[1]),
        data=[float(f) for f in data_array],
        # Specify which attributes not to export as LaTeX:
        exclude_from_latex=['data'],
    )


def group_list(l, step):
    """
    Example: l=[1, 2, 3, 4, 5, 6], step=2:
    [[1, 2], [3, 4], [5, 6]]
    Might be less efficient than `grouper` (see below)
    """
    return [l[i:i+step] for i in range(0, len(l), step)]


def is_power_of_two(n):
    """
    Check if a number is a power of 2.
    Based on https://stackoverflow.com/a/57025941/1018176
    :param n:
    :return: True iff n is a power of 2.
    """
    return (n != 0) and (n & (n-1) == 0)


def grouper(iterable: Iterable, n: int, fillvalue=None):
    """
    Iterate over an iterable in n-sized chunks.
    Based on https://stackoverflow.com/a/434411/1018176
    :param iterable:
    :param n:
    :param fillvalue:
    :return:
    """
    args = [iter(iterable)] * n
    return itertools.zip_longest(*args, fillvalue=fillvalue)
