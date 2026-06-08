use std::{error::Error, f32::consts::PI};

use tray_icon::Icon;

pub(super) fn tray_icon() -> Result<Icon, Box<dyn Error>> {
    let size = 32;
    let mut rgba = vec![0_u8; size * size * 4];
    let center = (size as f32 - 1.0) / 2.0;

    for y in 0..size {
        for x in 0..size {
            let offset = (y * size + x) * 4;
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let distance = (dx * dx + dy * dy).sqrt();
            let ring = (10.5..=13.6).contains(&distance);
            let exit_arrow = is_exit_arrow(x, y);

            if ring || exit_arrow {
                let angle = (dy.atan2(dx) + PI) / (2.0 * PI);
                rgba[offset] = (28.0 + 44.0 * angle) as u8;
                rgba[offset + 1] = (122.0 + 42.0 * angle) as u8;
                rgba[offset + 2] = (180.0 + 35.0 * angle) as u8;
                rgba[offset + 3] = 255;
            }
        }
    }

    Ok(Icon::from_rgba(rgba, size as u32, size as u32)?)
}

fn is_exit_arrow(x: usize, y: usize) -> bool {
    let shaft = (12..=23).contains(&x) && (14..=17).contains(&y);
    let head = (18..=24).contains(&x) && {
        let tip_distance = 24usize.saturating_sub(x);
        y.abs_diff(16) <= tip_distance + 1
    };

    shaft || head
}
