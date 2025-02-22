# xbgdump

[![CI](https://github.com/FallenWarrior2k/xbgdump/actions/workflows/ci.yml/badge.svg)](https://github.com/FallenWarrior2k/xbgdump/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/xbgdump)](https://crates.io/crates/xbgdump)

`xbgdump` is a simple tool to dump the current X11 background to an image file.

You can use it like `xbgdump file.png` or `xbgdump -` to send data to stdout. By default, it writes to the file `bg.png` in the current directory. For efficiency reasons, data sent to stdout is encoded as PAM instead of PNG. Beside a noticeable speedup, this should not make any difference when piping into ImageMagick or similar.

When given the `-m` (or `--mask`) flag, `xbgdump` will query the current screen layout with RandR and mask off-screen areas with transparency. For more details, consult the help with `xbgdump -h`.

For now, only PNG and PAM are supported, but in theory, it should be easy to expand support to all formats supported by [image-rs](https://github.com/image-rs/image).

## Motivation

I made this because I use [nitrogen](https://github.com/l3ib/nitrogen) and [i3lock](https://github.com/i3/i3lock) as my screen locker. I wanted a blurred version of my background for my lock screen, but i3lock only takes a single image, which I didn't have, as nitrogen generates it on the fly.

I knew [polybar](https://github.com/polybar/polybar) inspects the background to implement pseudo-transparency, which is where I took the initial idea from. I then tried using [xprop](https://gitlab.freedesktop.org/xorg/app/xprop), but to the best of my knowledge, it appears to only let me retrieve the ID of the pixmap used, not its contents. Which then led to me making this.

## Internals

`xbgdump` works by retrieving the pixmap attached to the X root window under the property `_XROOTPMAP_ID`. This property is set by [feh](https://github.com/derf/feh) and nitrogen; I have not tested this with other wallpaper-setting tools or desktop environments yet.

For 8-bit RGB, the contents of this pixmap are returned by X11 as BGR0—I don't know if this is actually documented somewhere; I found out through trial and error—which is then converted to RGB before being encoded as PNG and output to the given file or stdout.
