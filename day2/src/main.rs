use std::str::Lines;
fn is_safe(report: &[i32]) -> bool {
    let mut diff_iter = report.iter().zip(report.iter().skip(1)).map(|(x, y)| x - y);
    let diff = diff_iter.next().unwrap();
    if diff == 0 || diff.abs() > 3 {
        return false;
    }
    let s = diff.signum();
    diff_iter.all(|d| s * d > 0 && d.abs() < 4)
}
fn part1(lines: Lines) {
    let mut safe = vec![];
    for line in lines {
        let readings: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        if is_safe(&readings){
            safe.push(line);
        }
    }
    println!("Safe readings: {}", safe.len());
}

fn part2(lines:Lines){
    let mut count=0;
    for line in lines {
        let readings: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if is_safe(&readings){
            count+=1;
        }else{
            for i in 0..readings.len(){
                let mut modified_readings=Vec::from(&readings[..i]);
                modified_readings.extend_from_slice(&readings[i+1..]);
                if is_safe(&modified_readings){
                    
                    count+=1;
                    break;
                }
            }
        }
    }
    println!("Safe readings: {count}");
}

fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    part1(lines.lines());
    part2(lines.lines());
}
