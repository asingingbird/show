use memchr::memchr;
use std::cmp;

/// Count the number of lines in the given buffer.
/// This code is copied directly from [ripgrep](https://github.com/BurntSushi/ripgrep/blob/919c5c72994edb378706594f6268542983eeee6d/src/search_stream.rs#L549) project.
#[inline(never)]
pub fn count_lines(buf: &[u8], eol: u8) -> u64 {
    // This was adapted from code in the memchr crate. The specific benefit
    // here is that we can avoid a branch in the inner loop because all we're
    // doing is counting.

    // The technique to count EOL bytes was adapted from:
    // http://bits.stephan-brumme.com/null.html
    const LO_U64: u64 = 0x0101010101010101;
    const HI_U64: u64 = 0x8080808080808080;

    // use truncation
    const LO_USIZE: usize = LO_U64 as usize;
    const HI_USIZE: usize = HI_U64 as usize;

    #[cfg(target_pointer_width = "32")]
    const USIZE_BYTES: usize = 4;
    #[cfg(target_pointer_width = "64")]
    const USIZE_BYTES: usize = 8;

    fn count_eol(eol: usize) -> u64 {
        // Ideally, this would compile down to a POPCNT instruction, but
        // it looks like you need to set RUSTFLAGS="-C target-cpu=native"
        // (or target-feature=+popcnt) to get that to work. Bummer.
        (eol.wrapping_sub(LO_USIZE) & !eol & HI_USIZE).count_ones() as u64
    }

    #[cfg(target_pointer_width = "32")]
    fn repeat_byte(b: u8) -> usize {
        let mut rep = (b as usize) << 8 | b as usize;
        rep = rep << 16 | rep;
        rep
    }

    #[cfg(target_pointer_width = "64")]
    fn repeat_byte(b: u8) -> usize {
        let mut rep = (b as usize) << 8 | b as usize;
        rep = rep << 16 | rep;
        rep = rep << 32 | rep;
        rep
    }

    fn count_lines_slow(mut buf: &[u8], eol: u8) -> u64 {
        let mut count = 0;
        while let Some(pos) = memchr(eol, buf) {
            count += 1;
            buf = &buf[pos + 1..];
        }
        count
    }

    let len = buf.len();
    let ptr = buf.as_ptr();
    let mut count = 0;

    // Search up to an aligned boundary...
    let align = (ptr as usize) & (USIZE_BYTES - 1);
    let mut i = 0;
    if align > 0 {
        i = cmp::min(USIZE_BYTES - align, len);
        count += count_lines_slow(&buf[..i], eol);
    }

    // ... and search the rest.
    let repeated_eol = repeat_byte(eol);

    if len >= 2 * USIZE_BYTES {
        while i <= len - (2 * USIZE_BYTES) {
            unsafe {
                let u = *(ptr.offset(i as isize) as *const usize);
                let v = *(ptr.offset((i + USIZE_BYTES) as isize) as *const usize);

                count += count_eol(u ^ repeated_eol);
                count += count_eol(v ^ repeated_eol);
            }
            i += USIZE_BYTES * 2;
        }
    }
    count += count_lines_slow(&buf[i..], eol);
    count
}
