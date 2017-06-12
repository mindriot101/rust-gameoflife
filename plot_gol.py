#!/usr/bin/env python


import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.animation as manimation
import argparse
import tqdm


def parse_file(filename):
    with open(filename) as infile:
        header = infile.readline()
        width, height = tuple(map(int, header.strip().split()))

        yield width
        yield height

        nstates = int(infile.readline().strip())
        yield nstates

        nextline = infile.readline()
        assert nextline.strip() == '---'

        newstate = []
        for line in infile:
            line = line.strip()
            if line == '---':
                yield newstate
                newstate = []
            else:
                x, y = tuple(map(int, line.split()))
                newstate.append((x, y))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('filename')
    parser.add_argument('-o', '--output', required=True)
    args = parser.parse_args()

    parser = parse_file(args.filename)
    width = next(parser)
    height = next(parser)
    nstates = next(parser)

    FFMpegWriter = manimation.writers['ffmpeg']
    metadata = dict()
    writer = FFMpegWriter(fps=10, metadata=metadata)

    fig = plt.figure()
    ax = fig.add_subplot(111)
    l, = ax.plot([], [], 'ks')

    ax.set(
        xlim=(0, width),
        ylim=(0, height),
    )

    ax.tick_params(axis='both', which='both', bottom='off', left='off',
                   labelbottom='off', labelleft='off')

    with writer.saving(fig, args.output, nstates):
        pbar = tqdm.tqdm(total=nstates)
        for state in parser:
            xvals = [row[0] + 0.5 for row in state]
            yvals = [row[1] + 0.5 for row in state]
            l.set_data(xvals, yvals)
            writer.grab_frame()
            pbar.update(1)
