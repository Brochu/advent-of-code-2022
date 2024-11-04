package main
import "core:fmt"
import "core:strings"

d2run :: proc (p1, p2: ^strings.Builder) {
    when 0 == 1 { input :: #load("../data/day2.example") }
    else { input :: #load("../data/day2.input") }

    games := strings.split(string(input), "\r\n");
    games = games[0:len(games)-1];

    part1(games, p1);
    part2(games, p2);

    delete(games);
}

@(private="file")
part1 :: proc (games: []string, out: ^strings.Builder) {
    total := 0;
    for l in games {
        a: u8 = l[0];
        b: u8 = l[2];
        val := shape_score(b) + win_score(l);
        //fmt.printfln(" - '%c' vs. '%c' >> %v", a, b, val);
        total += val;
    }
    //fmt.println();

    strings.write_int(out, total);
}

@(private="file")
part2 :: proc (games: []string, out: ^strings.Builder) {
    total := 0;
    for l in games {
        a: u8 = l[0];
        b: u8 = l[2];
        val := shape_score2(l) + win_score2(b);
        //fmt.printfln(" - '%c' vs. '%c' >> %v", a, b, val);
        total += val;
    }
    //fmt.println();

    strings.write_int(out, total);
}

shape_score :: proc (chosen: u8) -> int {
    switch chosen {
        case 'X': return 1;
        case 'Y': return 2;
        case 'Z': return 3;
    }

    return -1;
}
shape_score2 :: proc (game: string) -> int {
    if      game == "B X" || game == "A Y" || game == "C Z" { return 1; }
    else if game == "C X" || game == "B Y" || game == "A Z" { return 2; }
    else if game == "A X" || game == "C Y" || game == "B Z" { return 3; }
    else { return -1; }
}

win_score :: proc (game: string) -> int {
    if      game == "B X" || game == "C Y" || game == "A Z" { return 0; }
    else if game == "A X" || game == "B Y" || game == "C Z" { return 3; }
    else if game == "C X" || game == "A Y" || game == "B Z" { return 6; }
    else { return -1; }
}

win_score2 :: proc (chosen: u8) -> int {
    switch chosen {
        case 'X': return 0;
        case 'Y': return 3;
        case 'Z': return 6;
    }

    return -1;
}
