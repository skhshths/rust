
                            let inputted_ints: Vec<i32> = inputted.iter().map(|x| x.parse::<i32>().unwrap()).collect();
                            for index in 0..function.len() {
                                let item: &str = function[index];
                                if args.contains(&item) {
                                    println!("item: {item}");