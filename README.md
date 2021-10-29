# `glifpathlen`

**This program is deprecated and its function has been assumed by [MFEKmetadata](https://github.com/MFEK/metadata), where it is called `glyphpathlen`.**

There are many automated operations for which knowing the length of the contours inside a UFO `.glif` file can come in handy.

Raph Levien wrote a nice blog post about his implementation of path length functions in `kurbo`: [How long is that Bézier?](https://raphlinus.github.io/curves/2018/12/28/bezier-arclength.html) (2018).

However, he didn't provide an implementation of a script to use his work…so I wrote this one.

## Usage
```
cargo run -- ~/Workspace/FRBAmericanCursive/FRBAmericanCursive-SOURCE.ufo/glyphs/numbersign.glif
```

Output:
```
308.9999084472656
309
597.2795966745455
597.2795966745455
```

One output line per `<contour>` element in `.glif` `<outline>`. :-)
