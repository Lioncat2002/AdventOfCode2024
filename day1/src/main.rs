use std::{fs::read_to_string, iter::zip};

fn part1(list1:&[i32],list2:&[i32]){
    let mut sum=0;

    for (val1,val2) in zip(list1,list2){
        sum+=(val1-val2).abs();
    }

    println!("Total Distance: {sum}");
}

fn part2(list1:&[i32],list2:&[i32]){
    let mut sim=0;
    for num in list1{
        let count=list2.iter().filter(|num2| num==*num2).count();
        sim+=(count as i32)*num;
    }
    println!("Similarity Score: {sim}");
}

fn main() {
    let input = read_to_string("input.txt").expect("failed to read input file");

    let (mut list1,mut list2):(Vec<i32>,Vec<i32>) = input.lines().map(|line|{
        let mut nums=line.split_ascii_whitespace().map(|n| n.parse::<i32>().unwrap());
        (nums.next().unwrap(),nums.next().unwrap())
    }).unzip();
    

    list1.sort_unstable();
    list2.sort_unstable();

    part1(&list1, &list2);
    part2(&list1, &list2);
}
