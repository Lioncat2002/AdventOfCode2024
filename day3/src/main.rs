use std::fs::read_to_string;

use pest::Parser;
use pest_derive::Parser;
use regex::Regex;


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MulParser;

fn part1(src:&str){
    let mut sum=0;
    let ast=MulParser::parse(Rule::part1_program, src).unwrap().next().unwrap();
    for record in ast.into_inner(){
        match record.as_rule(){
            Rule::call => {
                //println!("{:?}\n",record);
                let mut prod=1;
                for field in record.into_inner(){
                    prod*=field.as_str().parse::<i32>().unwrap();
                }
                sum+=prod;
            },
            _=>{}
        }
    }
    println!("part1 output: {sum}");
}

fn part2(src:&str){
    
    let mut sum=0;
    let mut execute=true;
    let re = Regex::new(r#"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))"#).unwrap();
    for cap in re.captures_iter(src){
        if &cap[1] == "do()" {
            execute=true;
        }else if &cap[1]=="don't()"{
            execute=false;
        } else if cap[1].starts_with("mul") {
            println!(
                "Matched: {} with numbers {} and {}",
                &cap[1],
                &cap[2],
                &cap[3]
            );
            if execute{
                sum+=&cap[2].parse::<i32>().unwrap()*&cap[3].parse::<i32>().unwrap();
            }
            
        }
    }
    println!("part2 output: {sum}");
}


fn main() {
    let input=read_to_string("input.txt").expect("file not found");
    part1(&input);
    part2(&input);
}
