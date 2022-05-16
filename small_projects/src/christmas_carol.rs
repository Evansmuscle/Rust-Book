fn numeriser(i: usize) -> String {
    if i == 1 {
        return String::from("first");
    }

    if i == 2 {
        return String::from("second");
    }

    if i == 3 {
        return String::from("third");
    }

    let numerised_value = format!("{}th", i);
    return numerised_value;
}

pub fn christmas_carol() {
    let repeated_lines = [
        "Two candy canes", "Three boughs of holly", "Four colored lights", "A shining star", "Little silver bells", "Candles a-glowing", "Gold and silver tinsel", "A guardian angel", "Some mistletoe", "Gifts for one and all", "All their good wishes"
    ];
    
    for idx in 0..repeated_lines.len() {
        println!("On the {} day of Christmas", numeriser(idx + 1));
        println!("My good friends brought to me");
        if idx != 0 {
            for rev_idx in (0..=idx).rev() {
                let rev_idx_cut: isize = rev_idx as isize - 1;
                
                if rev_idx_cut < 0 {
                    continue;
                }
                
                if rev_idx_cut == 0 {
                    println!("{}", repeated_lines[0]);
                    continue;
                }
                
                println!("{}", repeated_lines[rev_idx_cut as usize]);
            }
        }
        println!("And a song for the christmas tree\n");
    }
}