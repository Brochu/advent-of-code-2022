package main
import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:slice"

d1run :: proc (p1, p2: ^strings.Builder) {
    when 0 == 1 { input :: #load("../data/day1.example") }
    else { input :: #load("../data/day1.input") }
    elves := strings.split(string(input), "\r\n\r\n");
    packs := make([]int, len(elves));

    //fmt.printfln("elves count: %d", len(elves));
    for e, i in elves {
        //fmt.println("GROUP:");
        cals := strings.split(e, "\r\n");
        for c in cals {
            packs[i] += strconv.atoi(c);
        }
        //fmt.printfln("\ntotal: %d\n--------------------", packs[i]);
    }

    part1(packs, p1);
    part2(packs, p2);

    delete(packs);
    delete(elves);
}

@(private="file")
part1 :: proc (packs : []int, out: ^strings.Builder) {
    slice.reverse_sort(packs);
    strings.write_int(out, packs[0]);
}

@(private="file")
part2 :: proc (packs : []int, out: ^strings.Builder) {
    strings.write_int(out, packs[0] + packs[1] + packs[2]);
}
