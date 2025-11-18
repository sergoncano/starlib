pub fn check_map(map: &Vec<&'static str>) {
    let mut map_size_x: usize = 0;
    for _ in map[0].chars() {
        map_size_x += 1;
    }
    for (i, line) in map.iter().enumerate() {
        let mut line_size_x: usize = 0;
        for _char in line.chars() {
            line_size_x += 1;
        }
        assert_eq!(map_size_x, line_size_x, "Map size mismatch! First line contains {map_size_x} chars, while line {i} contains {line_size_x}!");
    }
}
