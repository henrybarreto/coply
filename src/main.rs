use std::env::{self, Args};

use coply::{reader::Reader, writer::Writer};

fn remove_first_arg(args: Args) -> Vec<String> {
    args.into_iter()
        .enumerate()
        .filter(|predicate| predicate.0 != 0)
        .map(|(_, arg)| arg)
        .collect()
}

fn main() -> Result<(), std::io::Error> {
    let args = remove_first_arg(env::args());

    let mut reader = Reader::new(&args[0]);
    let mut writer = Writer::new(&args[1]);

    let steps = if reader.iteration.steps % 4 == 0 {
        reader.iteration.steps / 4
    } else {
        (reader.iteration.steps / 4) + 1
    };

    if steps <= 0 {
        for _i in 0..1 {
            let buffer = reader.read();
            writer.write(buffer);
        }
    } else {
        for _i in 0..steps {
            let buffer = reader.read();
            writer.write(buffer);
        }
    }
    Ok(())
}
