[![Build Status](https://travis-ci.com/efharkin/swc2dot.svg?token=w2Bu6kMAWz66WkG555u7&branch=master)](https://travis-ci.com/efharkin/swc2dot)

# swc2dot

A simple command line tool for converting neuron morphologies in SWC format to
DOT graph description language.

## Description

Neurons have complex and
[beautiful](https://www.nytimes.com/2018/01/18/arts/design/brain-neuroscience-santiago-ramon-y-cajal-grey-gallery.html)
dendritic arbours that capture synaptic inputs and shape their functional
properties. The SWC file format
[widely](https://alleninstitute.github.io/AllenSDK/cell_types.html#morphology-swc-files)
[used](http://www.neuromorpho.org) to register dendritic morphology is simpler
and [not as
beautiful](http://www.neuronland.org/NLMorphologyConverter/MorphologyFormats/SWC/Spec.html).

The SWC format provides a minimal description of the morphology of a neuron as
an acyclic graph of tapered cylinders arranged in 3D space. While SWC is well
supported by specialized software packages for neuron modeling, it is not
supported by more general programs for visualizing and manipulating graphs.

swc2dot converts neuron morphologies stored in SWC to human-readable [DOT
format](https://graphs.grevian.org/example) supported by
[Graphviz](https://www.graphviz.org), [Gephi](https://gephi.org),
[NetworkX](https://networkx.github.io), and others.

## Example

swc2dot can be invoked from the command line as follows
```sh
swc2dot -o morphological_graph.dot morphology.swc
```

Contents of `morphology.swc` input file:
```
1 1 0 0 0 6.86102 -1
2 1 6.31 2.7 0 6.86102 1
3 1 -6.3 -2.7 0 6.86102 1
4 3 -13.75 -29.68 0 0.590534 1
5 3 -29.05 -71.43 0 0.409091 4
6 3 -45.53 -83.33 0 0.806474 5
7 3 -24.64 -77.78 0 0.519971 5
8 3 -29.34 -88.69 0 0.476807 7
9 3 -44.64 -96.09 0 0.519312 8
10 3 -51.02 -96.62 0 0.356949 9
11 3 -52.12 -100.5 0 0.549713 10
12 3 -63.58 -104 0 0.3125 11
...
```

Contents of `morphological_graph.dot` output file:
```dot
graph{
    1 -- {2, 3, 4, 14, 28, 45, 76, 101};
    4 -- 5;
    5 -- {6, 7};
    7 -- 8;
    8 -- 9;
    9 -- 10;
    10 -- 11;
    11 -- 12;
    12 -- 13;
    14 -- 15;
    15 -- {16, 21};
    ...
}
```

## Installation

### Build from source

Unfortunately, for now the only way to install swc2dot is to build it from
source. Fortunately, this is very easy.

You'll need a working Rust compiler together with the build manager Cargo to
build swc2dot. The easiest way to get set up is with
[rustup](https://rustup.rs). Try running `cargo  --version` in a
terminal afterwards to make sure everything worked.

Next, get a copy of the swc2dot git repository and open it in a terminal. On
MacOS or Linux, you can do this by pasting `git clone --depth 1
https://github.com/efharkin/swc2dot.git && cd swc2dot` into your terminal.

Finally, compile swc2dot into an executable by running `cargo build --release`
(note that this will download a couple of swc2dot's dependencies over the
internet). You can find the resulting binary in `./target/release/swc2dot`. Put
it somewhere in your `PATH` and you're done! Try running `swc2dot --help` to
make sure everything worked.

