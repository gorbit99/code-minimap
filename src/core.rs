use crate::cli::Opt;
use itertools::Itertools;
use std::cmp;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;

pub fn print_minimap(reader: Box<dyn BufRead>, opt: &Opt) -> io::Result<()> {
    let hscale = opt.hscale;
    let vscale = opt.vscale;
    let mut frame = vec![0..=0; 4];
    for chunk in &reader
        .lines()
        .enumerate()
        .map(|(i, line)| (scale(i, vscale), line))
        .group_by(|(i, _)| *i)
        .into_iter()
        .chunks(4)
    {
        for (i, (_, group)) in chunk.enumerate() {
            let (mut beg, mut end) = (0, 0);
            for (_, line) in group {
                let line: String = line?;
                beg = cmp::min(beg, line.find(|c: char| !c.is_whitespace()).unwrap_or(0));
                end = cmp::max(end, line.rfind(|c: char| !c.is_whitespace()).unwrap_or(line.len()));
            }
            frame[i] = beg..=end;
        }
        scale_width(&mut frame, hscale);
        print_miniline(&frame);
    }
    Ok(())
}

fn print_miniline(frame: &Vec<RangeInclusive<usize>>) {
    let end = frame.iter().max_by_key(|range| range.end()).unwrap().end();
    let line: String = (0..=*end)
        .step_by(2)
        .map(|i| BRAILLE_MATRIX[(char_idx(frame, i)) + (char_idx(frame, i + 1) << 4)])
        .collect();
    println!("{}", line);
}

fn char_idx(frame: &Vec<RangeInclusive<usize>>, pos: usize) -> usize {
    frame
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| if x.contains(&pos) { acc + (1 << i) } else { acc })
}

fn scale_width(frame: &mut Vec<RangeInclusive<usize>>, factor: f64) {
    for x in frame.iter_mut() {
        *x = RangeInclusive::new(scale(*x.start(), factor), scale(*x.end(), factor));
    }
}

fn scale(x: usize, factor: f64) -> usize {
    (x as f64 * factor).round() as usize
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const BRAILLE_MATRIX : [char; 256] = [
    '⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇',
    '⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', '⡈', '⡉', '⡊', '⡋', '⡌', '⡍', '⡎', '⡏',
    '⠐', '⠑', '⠒', '⠓', '⠔', '⠕', '⠖', '⠗', '⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗',
    '⠘', '⠙', '⠚', '⠛', '⠜', '⠝', '⠞', '⠟', '⡘', '⡙', '⡚', '⡛', '⡜', '⡝', '⡞', '⡟',
    '⠠', '⠡', '⠢', '⠣', '⠤', '⠥', '⠦', '⠧', '⡠', '⡡', '⡢', '⡣', '⡤', '⡥', '⡦', '⡧',
    '⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯', '⡨', '⡩', '⡪', '⡫', '⡬', '⡭', '⡮', '⡯',
    '⠰', '⠱', '⠲', '⠳', '⠴', '⠵', '⠶', '⠷', '⡰', '⡱', '⡲', '⡳', '⡴', '⡵', '⡶', '⡷',
    '⠸', '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿', '⡸', '⡹', '⡺', '⡻', '⡼', '⡽', '⡾', '⡿',
    '⢀', '⢁', '⢂', '⢃', '⢄', '⢅', '⢆', '⢇', '⣀', '⣁', '⣂', '⣃', '⣄', '⣅', '⣆', '⣇',
    '⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏', '⣈', '⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏',
    '⢐', '⢑', '⢒', '⢓', '⢔', '⢕', '⢖', '⢗', '⣐', '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗',
    '⢘', '⢙', '⢚', '⢛', '⢜', '⢝', '⢞', '⢟', '⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟',
    '⢠', '⢡', '⢢', '⢣', '⢤', '⢥', '⢦', '⢧', '⣠', '⣡', '⣢', '⣣', '⣤', '⣥', '⣦', '⣧',
    '⢨', '⢩', '⢪', '⢫', '⢬', '⢭', '⢮', '⢯', '⣨', '⣩', '⣪', '⣫', '⣬', '⣭', '⣮', '⣯',
    '⢰', '⢱', '⢲', '⢳', '⢴', '⢵', '⢶', '⢷', '⣰', '⣱', '⣲', '⣳', '⣴', '⣵', '⣶', '⣷',
    '⢸', '⢹', '⢺', '⢻', '⢼', '⢽', '⢾', '⢿', '⣸', '⣹', '⣺', '⣻', '⣼', '⣽', '⣾', '⣿',
];
