/*
 * fast_easy_cat is not a feature-complete port of easy_cat.py and is not
 * intended to be. The only syntax that works:
 * 
 *    fast_easy_cat DIRNAME
 *
 * on a directory with a bunch of
 *    DIRNAME/*/$filename.out
 *
 * produces catted ./$filename.out
 */*/

extern crate glob;

use glob::glob;

// struct Cli {
//     path: std::path::PathBuf,
// }

fn mapped(pb: &std::path::PathBuf) -> String {
    // takes a PathBuf and removes the first two directories
    let result = pb.file_name();
    match result {
        None => "".to_string(),
        Some(filename) => match filename.to_str() {
            None => "".to_string(),
            Some(filenamestr) => filenamestr.to_string()
        }
    }
}

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn alter_contents(contents: & String, prefix: & String, printed_desc: & mut bool) -> String {
    let lines = contents.lines();
    // println!("{:?}", lines);
    let mut newlines: std::vec::Vec< String > = std::vec::Vec::new();
    let mut descidx: usize = 0;
    let mut desc = "";
    for line in lines {
        // println!("{}", line);
        if line.find("description") != None {
            let objs: std::vec::Vec< &str > =
                line.split_ascii_whitespace().collect();
                // println!("{:?}", objs);
            for ii in 0..objs.len() {
                // println!("{}", objs[ii]);
                if objs[ii] == "description" {
                    descidx = ii;
                }
            }
            if *printed_desc != true {
                newlines.push(line.to_string());
                *printed_desc = true;
            }
            continue;
        } else if line.find("SCORE:") != None {
            let objs: std::vec::Vec< &str > =
                line.split_ascii_whitespace().collect();
            desc = objs[descidx];
        }

        let modded = format!("{}_{}", desc, prefix.as_str());
        let newline = match desc {
            "" => line.to_string(),
            _  => line.replace(desc, modded.as_str()),
        };
        //let newline = tmp.as_str();
        newlines.push(newline);
        // replace description with description_prefix
    }

    return newlines.join("\n");
}

fn filter_for_scorelines(contents: & String) -> String {
    let lines = contents.lines();
    // println!("{:?}", lines);
    let mut newlines: std::vec::Vec< String > = std::vec::Vec::new();
    for line in lines {
        // println!("{}", line);
        if line.find("SCORE:") != None {
            newlines.push(line.to_string());
        }
    }

    return newlines.join("\n");
}

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1).expect("no path given");
    // let args = Cli {
    //     path: std::path::PathBuf::from(path),
    // };
    
    let mut file_map = std::collections::HashMap::new();
    // std::collections::HashMap<std::path::PathBuf, std::vec::Vec<std::path::PathBuf>>;
    for entry in glob(format!("./{}/*/*.out", path).as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(outfile) => {
                // OK, we want to achieve a few things. First, we want to
                // create lists of output files that we are mapping to
                // a common destination: a map of <fn> : [<fn>].
                // OK, that's it actually.
                let ofc = outfile.clone();
                let ofc2 = outfile.clone();
                let mapped_of = mapped(&ofc);
                // let v = match file_map.entry(mapped_of) {
                //     Vacant(entry) => entry.insert(std::vec::Vec::new()),
                //     Occupied(entry) => entry.into_mut(),
                // };
                file_map.entry(mapped_of)
                    .and_modify(|e: &mut Vec<std::path::PathBuf>| { e.push(ofc2) } )
                    .or_insert(vec![ofc]);//.push(ofc2);
                
                // println!("{:?}", outfile.display())
            },
            Err(e) => println!("{:?}", e),
        }
    }


    // iterate over our hashmap.
    for (k, v) in file_map {
        // print contents of each of v to file k.
        // one modification: dirname of v gets appended into description.
        let mut output_file = File::create(&k)?;
        let mut scorefile = File::create(k.replace(".out", ".sc"))?;

        let mut printed_desc = false;

        let mut pb = pbr::ProgressBar::new(v.len() as u64);
        for file_path in v {
            let file = File::open(&file_path)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;

            // modify contents.
            let str_path = file_path.to_string_lossy().to_string();
            let path_components: std::vec::Vec< &str > =
                str_path.split('/').collect();
            contents = alter_contents(& contents,
                &path_components[1].to_string(),
                & mut printed_desc);

            let only_scorelines = filter_for_scorelines(&contents);
            // println!("{:?}", only_scorelines);
                //file_path.strip_prefix(path)?.ancestors().next());
            output_file.write_all(format!("{}\n", contents).as_str().as_bytes())?;
            scorefile.write_all(format!("{}\n", only_scorelines).as_str().as_bytes())?;
            pb.inc();

        }

        // println!("concatenate {:?} into {}", v, k);
    }


    // println!("Hello, world!");
    // println!("{:?}", args.path.display());
    Ok(())
}
