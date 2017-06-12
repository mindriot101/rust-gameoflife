#!/usr/bin/env python


import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.animation as manimation
import argparse


def parse_file(filename):
    states = []
    with open(filename) as infile:
        header = infile.readline()
        width, height = tuple(map(int, header.strip().split()))

        nextline = infile.readline()
        assert nextline.strip() == '---'

        newstate = []
        for line in infile:
            line = line.strip()
            if line == '---':
                states.append(newstate)
                newstate = []
            else:
                x, y = tuple(map(int, line.split()))
                newstate.append((x, y))

    return states, width, height


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('filename')
    parser.add_argument('-o', '--output', required=True)
    args = parser.parse_args()

    states, width, height = parse_file(args.filename)
    nstates = len(states)
    print(width, height)
    print('Found {} states'.format(nstates))

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
        for state in states:
            xvals = [row[0] + 0.5 for row in state]
            yvals = [row[1] + 0.5 for row in state]
            l.set_data(xvals, yvals)
            writer.grab_frame()
