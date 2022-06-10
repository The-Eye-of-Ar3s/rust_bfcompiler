use std::env;
//use std::fmt;
use std::process;
use std::fs;
use std::path::Path;
use regex::Regex;


fn main() {
    let filename = arg_collector();
    let filecontents = file_reader(filename);
    let cleancode = clean(filecontents);
    let optcode = optimizer(cleancode);
    let output = compiler(optcode.0, optcode.1);
    fs::write("out.c", output).expect("Unable to write!");
}

fn arg_collector() -> String{
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("NO FILE SPECIFIED!");
        process::exit(1);
    }
    let arg = args.remove(1);
    return arg;
}

fn file_reader(filename: String) -> String{
    if Path::new(&filename).is_file() {
        let raw = fs::read_to_string(filename).expect("Failed to read File!");
        return raw;
    } else {
        eprintln!("Specified File was not found!");
        process::exit(1);
    }
}

fn clean(raw: String) -> String{
    let expr = Regex::new(r"[^<|>|\+|\-|,|\.|\[|\]]").unwrap();
    let result = expr.replace_all(&raw, "").to_string();
    return [result, "!".to_string()].join("").to_string();
}

fn compiler(keyvec: Vec<String>, countvec: Vec<usize>) -> String{
    let mut outcode = vec!("#include <stdio.h>".to_string(),"int main() {".to_string(),"int array[30000] = {0};".to_string(),"int idx = 0;".to_string());
    for index in 0..keyvec.len() {
        if &keyvec[index] == "." {
            outcode.push("putchar(array[idx]);".to_string());
        } else if &keyvec[index] == "," {
            outcode.push("array[idx] = getchar();".to_string());
        } else if &keyvec[index] == "[" {
            outcode.push("while (array[idx]) {".to_string());
        } else if &keyvec[index] == "]" {
            outcode.push("}".to_string());
        } else if &keyvec[index] == ">" {
            outcode.push(format!("idx+={};", countvec[index]));
        } else if &keyvec[index] == "<" {
            outcode.push(format!("idx-={};", countvec[index]));
        } else if &keyvec[index] == "+" {
            outcode.push(format!("array[idx]+={};", countvec[index]));
        } else if &keyvec[index] == "-" {
            outcode.push(format!("array[idx]-={};", countvec[index]));
        }
    }
    outcode.push("}".to_string());
    return outcode.join("\n");
}

fn optimizer(bfcode: String) -> (Vec<String>, Vec<usize>){
    let mut keyvec: Vec<String> = Vec::new();
    let mut countvec: Vec<usize> = Vec::new();
    let cmdvec: Vec<char> = bfcode.chars().collect();
    let mut cmd;
    let mut prev: String = "".to_string();
    let mut tmp;
    let mut cp: &str;
    for cmdchar in cmdvec {
        cmd = cmdchar.to_string();
        if prev != cmd || ".[]".to_string().contains(&cmd) {
            cp = &cmd;
            prev = cp.to_string();
            keyvec.push(cp.to_string());
            countvec.push(1);
        } else if "+-><,".to_string().contains(&cmd) {
            tmp = countvec.pop().unwrap();
            tmp += 1;
            countvec.push(tmp as usize);
        }
    }
    return (keyvec, countvec);
}