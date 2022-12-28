use std::fs::read_to_string;

#[derive(Debug)]
struct Instruction {
    n: i32,
    from: usize,
    to: usize,
}

trait Process {
    fn process_instruction(&mut self, instruction: Instruction);
}

trait Parse {
    fn parse_instructions(self) -> Vec<Instruction>;
}

impl Parse for Vec<&str> {
    fn parse_instructions(self) -> Vec<Instruction> {
        self.iter()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|l| l.to_string().parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .map(|i| Instruction {
                n: i[0].clamp(0, 100),
                from: (i[1] - 1) as usize,
                to: (i[2] - 1) as usize,
            })
            .collect::<Vec<Instruction>>()
    }
}

impl Process for Vec<Vec<char>> {
    fn process_instruction(&mut self, instruction: Instruction) {
        let mut group = Vec::new();
        for _ in 0..instruction.n {
            let c = self[instruction.from].pop();

            if let Some(c) = c {
                group.push(c);
            }
        }
        group.reverse();
        for c in group{
            self[instruction.to].push(c);
        }
    }
}

fn parse_init(init: Vec<&str>) -> Vec<Vec<char>> {
    let mut clean = init
        .iter()
        .map(|line| {
            line.char_indices().filter_map(|(index, item)| match index {
                0 => None,
                _ => match (index - 1) % 4 {
                    0 => Some(item),
                    _ => None,
                },
            })
        })
        .rev();

    let num_cols = clean
        .by_ref()
        .next()
        .unwrap()
        .last()
        .unwrap()
        .to_string()
        .parse::<i32>()
        .unwrap();

    let mut stacks = (0..num_cols).map(|_| Vec::new()).collect::<Vec<Vec<_>>>();

    for line in clean {
        for (index, item) in line.enumerate() {
            if !item.is_whitespace() {
                stacks[index].push(item);
            }
        }
    }

    stacks
}

fn main() {
    let file_string = read_to_string("./input/day5.txt").expect("Could not find input file");

    let mut file_lines = file_string.lines();

    let init = file_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let instructions = file_lines.collect::<Vec<_>>().parse_instructions();

    let mut stacks = parse_init(init);

    for i in instructions {
        stacks.process_instruction(i);
    }

    let result = stacks
        .iter()
        .map(|col| match col.last() {
            Some(c) => c.to_string(),
            _ => "".to_string(),
        })
        .collect::<String>();

    println!("{}", result)
}
