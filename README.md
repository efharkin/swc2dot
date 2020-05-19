[![Build Status](https://travis-ci.com/efharkin/swc2dot.svg?token=w2Bu6kMAWz66WkG555u7&branch=master)](https://travis-ci.com/efharkin/swc2dot)

# swc2dot

A simple command line tool for converting neuron morphologies in
SWC format to DOT graph description language.

## Example

```bash
swc2dot -o morphology.dot morphology.swc
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

Contents of `morphology.dot` output file:
```
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
