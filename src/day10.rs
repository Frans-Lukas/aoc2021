#[aoc(day10, part1)]
pub fn part1_chars(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut line_stack: Vec<char> = Vec::new();
        sum += syntax_score(line, &mut line_stack);
    }
    sum
}

fn syntax_score(line: &str, stack: &mut Vec<char>) -> i32 {
    for c in line.chars() {
        let mut pc: char = 'n';
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' || c == '>' {
            pc = stack.pop().unwrap();
        }

        if pc != 'n' && !is_valid_pair(pc, c) {
            match c {
                ')' => {
                    return 3;
                }
                ']' => {
                    return 57;
                }
                '}' => {
                    return 1197;
                }
                '>' => {
                    return 25137;
                }
                _ => {}
            }
        }
    }
    0
}

fn is_valid_pair(cs: char, ce: char) -> bool {
    if cs == '(' && ce == ')' {
        return true;
    }
    if cs == '{' && ce == '}' {
        return true;
    }
    if cs == '[' && ce == ']' {
        return true;
    }
    if cs == '<' && ce == '>' {
        return true;
    }

    false
}

#[aoc(day10, part2)]
pub fn part2_chars(input: &str) -> i64 {
    let mut sc;
    let mut scores: Vec<i64> = Vec::new();
    for line in input.lines() {
        let mut line_stack: Vec<char> = Vec::new();
        {
            sc = syntax_score(line, &mut line_stack);
        }
        if sc == 0 {
            let mut score = 0;
            line_stack.reverse();
            for c in line_stack {
                score *= 5;
                match c {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => {}
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}
