use core::time::Duration;

use chip8::{Ch8Image, FrameBuffer, ManagedInterpreter, Nibble};

////////////////////////////////////////////////////////////////////////////////

fn check_display(fb: &FrameBuffer, expected_raw: &str) {
    const DISPLAY_ON: &str = "▓";
    const DISPLAY_OFF: &str = " ";

    let actual_lines = fb
        .iter_rows()
        .map(|row| {
            row.iter()
                .map(|v| if *v { DISPLAY_ON } else { DISPLAY_OFF })
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    let actual = actual_lines.join("\n");

    let expected_lines = expected_raw
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.replace('#', DISPLAY_ON).replace('.', DISPLAY_OFF))
        .collect::<Vec<_>>();
    let expected = expected_lines.join("\n");

    if actual != expected {
        panic!("Wrong display content. Expected:\n\n{expected}\n\nGot:\n\n{actual}");
    }
}

fn test_by_instruction_count(image: &[u8], instruction_count: usize, expected_display: &str) {
    let mut inter = ManagedInterpreter::new(Ch8Image::new(image).unwrap(), rand::random);
    for _ in 0..instruction_count {
        inter.simulate_one_instruction().unwrap();
    }
    check_display(inter.frame_buffer(), expected_display);
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_chip8_logo() {
    test_by_instruction_count(
        include_bytes!("../images/tests/1-chip8-logo.ch8"),
        100,
        "
            ................................................................
            ............#####.#....................#..........##............
            ..............#.....##.#...##..###...###.#..#..##..#............
            ..............#...#.#.#.#.#..#.#..#.#..#.#..#.#.................
            ..............#...#.#...#.####.#..#.#..#.#..#..#................
            ..............#...#.#...#.#....#..#.#..#.#..#...#...............
            ..............#...#.#...#..###.#..#..###..###.##................
            ................................................................
            ................................................................
            ...........#####...##.......##..#####...........#######.........
            ..........#######.###......###.#######.........###...###........
            .........###...##.###......###.###..###.......###.....##........
            ........###.......###..........###...##.......###.....##........
            ........###..#.#..###.......##.###...##.......###.....##........
            ........###.......######...###.###...##........###...##.........
            ........###.#...#.#######..###.###...##.####....######..........
            ........###..###..###..###.###.###..###.####...###..###.........
            ........###.......###...##.###.#######........###....###........
            ........###.......###...##.###.######........###......##........
            ........###.......###...##.###.###...........###......##........
            ........###.......###...##.###.###.#.#...###.###......##........
            .........###...##.###...##.###.###.###...#.#.####....###........
            ..........#######.###...##.###.###...#...#.#..#########.........
            ...........#####..###...##.###.###...#.#.###...#######..........
            ................................................................
            ................................................................
            .............###..##...##.#.......##......#.#....##.............
            ..............#..#..#.#...###....#...#..#...###.#..#............
            ..............#..####..#..#.......#..#..#.#.#...####............
            ..............#..#......#.#........#.#..#.#.#...#...............
            ..............#...###.##...##....##...###.#..##..###............
            ................................................................
        ",
    );
}

#[test]
fn test_ibm_logo() {
    test_by_instruction_count(
        include_bytes!("../images/tests/2-ibm-logo.ch8"),
        100,
        "
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ............########.#########...#####.........#####..#.#.......
            ......................................................#.#.......
            ............########.###########.######.......######...#........
            ................................................................
            ..............####.....###...###...#####.....#####....#.#.......
            ......................................................###.......
            ..............####.....#######.....#######.#######......#.......
            ........................................................#.......
            ..............####.....#######.....###.#######.###..............
            .......................................................#........
            ..............####.....###...###...###..#####..###..............
            ......................................................###.......
            ............########.###########.#####...###...#####..#.#.......
            ......................................................#.#.......
            ............########.#########...#####....#....#####..###.......
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
        ",
    );
}

#[test]
fn test_corax() {
    test_by_instruction_count(
        include_bytes!("../images/tests/3-corax+.ch8"),
        500,
        "
            ................................................................
            ..###.#.#.........###.#.#.........###.#.#.........###.###.......
            ...##..#...#.#......#..#...#.#....###.###..#.#....#...##...#.#..
            ....#.#.#..##.....##..#.#..##.....#.#...#..##.....##....#..##...
            ..###.#.#..#......###.#.#..#......###...#..#......#...##...#....
            ................................................................
            ..#.#.#.#.........###.###.........###.###.........###.###.......
            ..###..#...#.#....#.#.##...#.#....###.##...#.#....#....##..#.#..
            ....#.#.#..##.....#.#.#....##.....#.#...#..##.....##....#..##...
            ....#.#.#..#......###.###..#......###.##...#......#...###..#....
            ................................................................
            ..###.#.#.........###.###.........###.###.........###.###.......
            ..##...#...#.#....###.#.#..#.#....###...#..#.#....#...##...#.#..
            ....#.#.#..##.....#.#.#.#..##.....#.#..#...##.....##..#....##...
            ..##..#.#..#......###.###..#......###..#...#......#...###..#....
            ................................................................
            ..###.#.#.........###.##..........###..##.............#.#.......
            ....#..#...#.#....###..#...#.#....###.#....#.#....#.#..#...#.#..
            ...#..#.#..##.....#.#..#...##.....#.#.###..##.....#.#.#.#..##...
            ...#..#.#..#......###.###..#......###.###..#.......#..#.#..#....
            ................................................................
            ..###.#.#.........###.###.........###.###.......................
            ..###..#...#.#....###...#..#.#....###.##...#.#..................
            ....#.#.#..##.....#.#.##...##.....#.#.#....##...................
            ..##..#.#..#......###.###..#......###.###..#....................
            ................................................................
            ..##..#.#.........###.###.........###..##.............#.#...###.
            ...#...#...#.#....###..##..#.#....#...#....#.#....#.#.###...#.#.
            ...#..#.#..##.....#.#...#..##.....##..###..##.....#.#...#...#.#.
            ..###.#.#..#......###.###..#......#...###..#.......#....#.#.###.
            ................................................................
            ................................................................
        ",
    );
}

#[test]
fn test_flags() {
    test_by_instruction_count(
        include_bytes!("../images/tests/4-flags.ch8"),
        1000,
        "
            #.#..#..##..##..#.#...##....................###.................
            ###.#.#.#.#.#.#.#.#....#...#.#.#.#.#.#........#..#.#.#.#.#.#....
            #.#.###.##..##...#.....#...##..##..##.......##...##..##..##.....
            #.#.#.#.#...#....#....###..#...#...#........###..#...#...#......
            ................................................................
            ###...................#.#...................###.................
            .##..#.#.#.#.#.#......###..#.#.#.#.#.#.#.#..##...#.#.#.#.#.#.#.#
            ..#..##..##..##.........#..##..##..##..##.....#..##..##..##..##.
            ###..#...#...#..........#..#...#...#...#....##...#...#...#...#..
            ................................................................
            ###...................###...................###.................
            #....#.#.#.#.#.#........#..#.#.#.#.#.#.#.#..##...#.#.#.#.#.#....
            ###..##..##..##.........#..##..##..##..##...#....##..##..##.....
            ###..#...#...#..........#..#...#...#...#....###..#...#...#......
            ................................................................
            ................................................................
            ###..#..##..##..#.#...#.#...................###.................
            #...#.#.#.#.#.#.#.#...###..#.#.#.#.#.#.#.#..##...#.#.#.#.#.#.#.#
            #...###.##..##...#......#..##..##..##..##.....#..##..##..##..##.
            ###.#.#.#.#.#.#..#......#..#...#...#...#....##...#...#...#...#..
            ................................................................
            ###...................###...................###.................
            #....#.#.#.#.#.#........#..#.#.#.#.#.#.#.#..##...#.#.#.#.#.#....
            ###..##..##..##.........#..##..##..##..##...#....##..##..##.....
            ###..#...#...#..........#..#...#...#...#....###..#...#...#......
            ................................................................
            ................................................................
            ###.###.#.#.###.##....###.###.........................#.#...###.
            #.#..#..###.##..#.#...#...##...#.#.#.#............#.#.###...#.#.
            #.#..#..#.#.#...##....##..#....##..##.............#.#...#...#.#.
            ###..#..#.#.###.#.#...#...###..#...#...............#....#.#.###.
            ................................................................
        ",
    );
}

#[test]
fn test_quirks() {
    let mut inter = ManagedInterpreter::new(
        Ch8Image::new(include_bytes!("../images/tests/5-quirks.ch8")).unwrap(),
        rand::random,
    );

    inter.set_key_down(Nibble::try_from(1).unwrap(), true);
    inter.simulate_duration(Duration::from_secs(1)).unwrap();
    inter.set_key_down(Nibble::try_from(1).unwrap(), false);
    inter.simulate_duration(Duration::from_secs(5)).unwrap();

    check_display(
        inter.frame_buffer(),
        "
            ................................................................
            .#.#.###.....##..###..##.###.###............###.##..............
            .#.#.#.......#.#.##..##..##...#.............#.#.#.#........#.#..
            .#.#.##......##..#.....#.#....#.............#.#.#.#........##...
            ..#..#.......#.#.###.##..###..#.............###.#.#........#....
            ................................................................
            .###.###.###.###.##..#.#....................###.##..............
            .###.##..###.#.#.#.#.#.#....................#.#.#.#........#.#..
            .#.#.#...#.#.#.#.##...#.....................#.#.#.#........##...
            .#.#.###.#.#.###.#.#..#.....................###.#.#........#....
            ................................................................
            .##..###..##.##......#.#..#..###.###........###.##..............
            .#.#..#..##..#.#.....#.#.#.#..#...#.........#.#.#.#........#.#..
            .#.#..#....#.##......###.###..#...#.........#.#.#.#........##...
            .##..###.##..#....#..###.#.#.###..#.........###.#.#........#....
            ................................................................
            .###.#...###.##..##..###.##...##............###.##..............
            .#...#....#..#.#.#.#..#..#.#.#..............#.#.#.#........#.#..
            .#...#....#..##..##...#..#.#.#.#............#.#.#.#........##...
            .###.###.###.#...#...###.#.#..##............###.#.#........#....
            ................................................................
            ..##.#.#.###.###.###.###.##...##............###.###.###.........
            .##..###..#..#....#...#..#.#.#..............#.#.#...#......#.#..
            ...#.#.#..#..##...#...#..#.#.#.#............#.#.##..##.....##...
            .##..#.#.###.#....#..###.#.#..##............###.#...#......#....
            ................................................................
            ..##.#.#.###.##..###.##...##................###.###.###.........
            ...#.#.#.###.#.#..#..#.#.#..................#.#.#...#......#.#..
            ...#.#.#.#.#.##...#..#.#.#.#................#.#.##..##.....##...
            .##...##.#.#.#...###.#.#..##................###.#...#......#....
            ................................................................
            ................................................................
        ",
    );
}

#[test]
fn test_keypad() {
    let mut inter = ManagedInterpreter::new(
        Ch8Image::new(include_bytes!("../images/tests/6-keypad.ch8")).unwrap(),
        rand::random,
    );

    inter.set_key_down(Nibble::try_from(3).unwrap(), true);
    inter.simulate_duration(Duration::from_secs(1)).unwrap();
    inter.set_key_down(Nibble::try_from(3).unwrap(), false);
    inter.simulate_duration(Duration::from_secs(1)).unwrap();

    inter.set_key_down(Nibble::try_from(0).unwrap(), true);
    inter.simulate_duration(Duration::from_secs(1)).unwrap();
    inter.set_key_down(Nibble::try_from(0).unwrap(), false);
    inter.simulate_duration(Duration::from_secs(1)).unwrap();

    check_display(
        inter.frame_buffer(),
        "
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ..............................#.#...............................
            ..............................##................................
            ..............................#.................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            .................#..#...#........##.###.###.##..................
            ................#.#.#...#.......#...#.#.#.#.#.#.................
            ................###.#...#.......#.#.#.#.#.#.#.#.................
            ................#.#.###.###......##.###.###.##..................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
            ................................................................
        ",
    );
}
