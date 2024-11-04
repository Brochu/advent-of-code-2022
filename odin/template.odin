package main
import "core:fmt"
import "core:strings"

d0run :: proc (p1, p2: ^strings.Builder) {
    //when 1 == 1 { input :: #load("../data/day0.example") }
    //else { input :: #load("../data/day0.input") }
    // Parse input

    part1(p1);
    part2(p2);
    
    // Cleanup
}

@(private="file")
part1 :: proc (out: ^strings.Builder) {
    strings.write_f64(out, 0.5, 'f');
}

@(private="file")
part2 :: proc (out: ^strings.Builder) {
    strings.write_f64(out, 0.25, 'f');
}
