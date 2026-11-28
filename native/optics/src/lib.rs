//! Stateless optical field used by the native hosts. Each surface decides when to repaint.
//! Coordinates are local to the visible field, including on scrollable devices.
#![cfg_attr(target_os = "android", no_std)]

pub const SETTLE_MS: u16 = 1450;

/// Pack one local interaction. An all-zero value represents an absent impulse.
pub const fn ripple(x: u16, y: u16, age_ms: u16, strength: u8) -> u64 {
    (x as u64) | ((y as u64) << 16) | ((age_ms as u64) << 32) | ((strength as u64) << 48)
}

fn noise(x: u32, y: u32) -> u32 {
    let mut h = x.wrapping_mul(0x9e37_79b1) ^ y.wrapping_mul(0x85eb_ca77);
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb_352d);
    h ^ (h >> 15)
}

// The maximum is smooth at the center, so neighbouring glyphs share a soft pool of light.
fn lobe(value: i32, radius: i32) -> i32 {
    let linear = (radius - value.abs()).max(0) * 255 / radius;
    linear * linear * (765 - 2 * linear) / 65025
}

fn wave(x: i32, y: i32, width: i32, height: i32, event: u64) -> i32 {
    let age = (event >> 32) as u16;
    let power = (event >> 48) as u8;
    if power == 0 || age >= SETTLE_MS {
        return 0;
    }
    let position = |coord: u16, span: i32| ((u64::from(coord) * span as u64 + 32768) >> 16) as i32;
    let dx = (x - position(event as u16, width)).clamp(-16384, 16384);
    let dy = (y - position((event >> 16) as u16, height)).clamp(-16384, 16384);
    let distance = ((dx * dx + dy * dy) as u32).isqrt() as i32;
    let front = i32::from(age) / 4;
    let ring = lobe(distance - front, 52) - lobe(distance - front + 48, 38) / 2
        + lobe(distance - front + 92, 48) / 4;
    let remaining = i32::from(SETTLE_MS - age);
    ring * i32::from(power) / 255 * remaining / i32::from(SETTLE_MS)
}

/// Return an ASCII glyph in bits 24..32 and its sRGB colour in bits 0..24.
/// Zero represents empty space. Profile order is Full, Professional, Focus, Eco, Classic.
pub fn sample_cell(
    col: u32,
    row: u32,
    cols: u32,
    rows: u32,
    profile: u32,
    newest: u64,
    previous: u64,
) -> u32 {
    if col >= cols || row >= rows || cols > 8192 || rows > 8192 || profile >= 4 {
        return 0;
    }
    let seed = noise(col, row);
    let x = col as i32 * 16 + 8;
    let y = row as i32 * 20 + 10;
    let width = cols as i32 * 16;
    let height = rows as i32 * 20;
    let u = x * 1024 / width;
    let v = y * 1024 / height;
    let blue = lobe(u - 740 + (v - 520) / 3, 430) * lobe(v - 380, 680) / 255;
    let violet = lobe(u - 255 - (v - 300) / 2, 460) * lobe(v - 650, 800) / 255;
    let impulse = if profile < 2 {
        wave(x, y, width, height, newest) + wave(x, y, width, height, previous) / 2
    } else {
        0
    };
    let density = [82, 102, 146, 211][profile as usize];
    if (seed & 255) as i32 + impulse / 2 < density {
        return 0;
    }
    let gain = [100, 73, 58, 46][profile as usize];
    let grain = (seed >> 24) as i32 / 19;
    let light = |base: i32, lift: i32| {
        (base + (lift + grain + impulse / 2) * gain / 100).clamp(8, 196) as u32
    };
    let red = light(16, violet / 6 + blue / 24);
    let green = light(22, blue / 5 + violet / 16);
    let b = light(31, blue / 4 + violet / 5);
    let index = ((seed >> 9).wrapping_add((impulse.max(0) / 45) as u32)) % 36;
    let glyph = if index < 10 {
        b'0' as u32 + index
    } else {
        b'A' as u32 + index - 10
    };
    (glyph << 24) | (red << 16) | (green << 8) | b
}

#[no_mangle]
pub extern "C" fn inta_optics_cell(
    x: u32,
    y: u32,
    cols: u32,
    rows: u32,
    profile: u32,
    first: u64,
    second: u64,
) -> u32 {
    sample_cell(x, y, cols, rows, profile, first, second)
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_inta_mobile_OpticalField_cell(
    _: *mut core::ffi::c_void,
    _: *mut core::ffi::c_void,
    col: i32,
    row: i32,
    cols: i32,
    rows: i32,
    profile: i32,
    first: i64,
    second: i64,
) -> i32 {
    sample_cell(
        col as u32,
        row as u32,
        cols as u32,
        rows as u32,
        profile as u32,
        first as u64,
        second as u64,
    ) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_is_deterministic_and_sleep_is_exact() {
        let active = ripple(32768, 32768, 300, 255);
        let settled = ripple(32768, 32768, SETTLE_MS, 255);
        let mut changed = 0;
        for x in 0..48 {
            for y in 0..15 {
                let cell = |profile, event| sample_cell(x, y, 48, 15, profile, event, 0);
                assert_eq!(cell(0, 0), cell(0, settled));
                assert_eq!(cell(2, 0), cell(2, active));
                assert_eq!(cell(3, 0), cell(3, active));
                assert_eq!(cell(4, active), 0);
                let glyph = (cell(0, 0) >> 24) as u8;
                assert!(glyph == 0 || glyph.is_ascii_alphanumeric());
                changed += u32::from(cell(0, 0) != cell(0, active));
            }
        }
        assert!(changed > 30 && changed < 480);
        assert_eq!(sample_cell(0, 0, 0, 0, 0, 0, 0), 0);
        assert_eq!(sample_cell(0, 0, u32::MAX, 5, 0, 0, 0), 0);
    }
}
