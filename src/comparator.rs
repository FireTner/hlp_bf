pub fn comparator(back: u8, side: u8, mode: bool) -> u8 {
    if back < side {
        return 0;
    } else if mode {
        return back - side;
    } else {
        return back;
    }
}