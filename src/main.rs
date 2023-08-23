use std::fs::File;
use std::io::Read;

fn main() {
    let f = File::open(r"C:\Users\grego\rust-projects\advent-day-one\src\calories.txt");
    match f{
        Ok(mut file) =>{
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).unwrap();
            let numbers = file_content.split("\n");
            let mut current = 0;
            let mut top_three = vec![0,0,0];
            for number in numbers {
                if number != "" && number != "\t" {
                    current += number.parse::<i32>().unwrap();
                }else{
                    check_top_3(current, &mut top_three);
                    current = 0;
                }
            }
            let mut sum_top_3 = 0;
            for i in (0..3){
                sum_top_3 += top_three[i];
            }

            println!("{}", sum_top_3);
        },
        Err(e) => println!("error: {}", e),
    }
}


fn check_top_3(contender: i32, top_three: &mut Vec<i32>){
    if contender > top_three[0] {
        top_three[2] = top_three[1];
        top_three[1] = top_three[0];
        top_three[0] = contender;
    }else{
        if contender > top_three[1] {
            top_three[2] = top_three[1];
            top_three[1] = contender;
        }else{
            if contender > top_three[2]{
                top_three[2] = contender;
            }
        }
    }
}