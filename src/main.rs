use anyhow::{Context};
use anyhow::Error;
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{AtomEnum, ConnectionExt, Window},
    },
};

// Image grabbing logic based on https://github.com/neXromancers/shotgun and
// https://www.apriorit.com/dev-blog/672-lin-how-to-take-multi-monitor-screenshots-on-linux
// Pixmap grabbing based on https://github.com/polybar/polybar

fn main() -> anyhow::Result<()> {
        // TODO: Exit code
        // Rust makes this very complicated
        // The only stable way to set the exit code is to use std::process::exit(), which has the
        // issue of not running destructors.
        // Setting the exit code via regular control flow requires the unstable Termination trait,
        // which is provided for Result, but always calls Debug::fmt in the error case, so there's
        // no way to silently exit with an error status without custom types.

    let (c, screen_num) = x11rb::connect(None)?;
    let root = c.setup().roots[screen_num].root;
    println!("{:?}", root);
    get_background(&c, root).context("Failed to get background image.")?;
    Ok(())
}

fn get_background(c: &impl Connection, root: Window) -> Result<u8, Error>   {
    let bg_atom = c
        .intern_atom(true, b"_XROOTPMAP_ID")
        .context("Failed to create cookie to retrieve background atom ID.")?
        .reply()
        .context("Failed to get background atom ID.")?
        .atom;

    let prop = c
        .get_property(false, root, bg_atom, AtomEnum::PIXMAP, 0, 1)
        .context("Failed to create cookie to get background pixmap.")?
        .reply()
        .context("Failed to get background pixmap.")?;

    // This is what Polybar does and it works
    let _value_iter = prop
        .value32()
        .with_context(|| format!("Unexpected pixmap reply format {}.", prop.format))?;
  
    return Ok(0);
}
