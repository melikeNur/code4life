use std::io;
use std::ptr::NonNull;
use std::ptr;
use core::prelude::v1::Option::None;
use std::env;
use std::ptr::null;
use std::clone::Clone;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Player{
 storage_a: i32,
 storage_b: i32,
 storage_c: i32,
 storage_d: i32,
 storage_e: i32,
 target: String,
}
impl Player{
  
    
}
struct Sample{
 sample_id: i32,
 carried_by: i32,
 rank : i32,
 expertise_gain: String,
 health: i32,
 cost_a: i32,
 cost_b: i32,
 cost_c: i32,
 cost_d: i32,
 cost_e: i32,
}
impl Sample{           
    fn clone(&self) -> Sample{      //I wrote this function because I could not directly assign a value that I defined earlier in this language.
   return Sample{sample_id:self.sample_id,carried_by:self.carried_by,rank:self.rank,expertise_gain:self.expertise_gain.to_string(),health:self.health,cost_a:self.cost_a,cost_b:self.cost_b,cost_c:self.cost_c,cost_d:self.cost_d,cost_e:self.cost_e};
     
    }   
}
/**
 * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let project_count = parse_input!(input_line, i32);
    for i in 0..project_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let a = parse_input!(inputs[0], i32);
        let b = parse_input!(inputs[1], i32);
        let c = parse_input!(inputs[2], i32);
        let d = parse_input!(inputs[3], i32);
        let e = parse_input!(inputs[4], i32);
    }
let mut SampleR: bool = false;
let mut desiredSample=Sample{sample_id:0,carried_by:0,rank:0,expertise_gain:"".to_string(),health:0,cost_a:0,cost_b:0,cost_c:0,cost_d:0,cost_e:0}; //A Default Sample
let mut elements :Vec<char>= Vec::new();  // to keep the molecules inside and remove them whenever we want
    // game loop
    loop {
        let mut samples : Vec<Sample>=Vec::new();
        let mut players :Vec<Player>= Vec::new();
   

        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let target = inputs[0].trim().to_string();
            let eta = parse_input!(inputs[1], i32);
            let score = parse_input!(inputs[2], i32);
            let storage_a = parse_input!(inputs[3], i32);
            let storage_b = parse_input!(inputs[4], i32);
            let storage_c = parse_input!(inputs[5], i32);
            let storage_d = parse_input!(inputs[6], i32);
            let storage_e = parse_input!(inputs[7], i32);
            let expertise_a = parse_input!(inputs[8], i32);
            let expertise_b = parse_input!(inputs[9], i32);
            let expertise_c = parse_input!(inputs[10], i32);
            let expertise_d = parse_input!(inputs[11], i32);
            let expertise_e = parse_input!(inputs[12], i32);
            let newPlayer = Player{storage_a, storage_b, storage_c, storage_d, storage_e,target};
            players.push(newPlayer);                     //I did this so that I could access my own player at any time
            
        } 
        
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let available_a = parse_input!(inputs[0], i32);
        let available_b = parse_input!(inputs[1], i32);
        let available_c = parse_input!(inputs[2], i32);
        let available_d = parse_input!(inputs[3], i32);
        let available_e = parse_input!(inputs[4], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let sample_count = parse_input!(input_line, i32);
        for i in 0..sample_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let sample_id = parse_input!(inputs[0], i32);
            let carried_by = parse_input!(inputs[1], i32);
            let rank = parse_input!(inputs[2], i32);
            let expertise_gain = inputs[3].trim().to_string();
            let health = parse_input!(inputs[4], i32);
            let cost_a = parse_input!(inputs[5], i32);
            let cost_b = parse_input!(inputs[6], i32);
            let cost_c = parse_input!(inputs[7], i32);
            let cost_d = parse_input!(inputs[8], i32);
            let cost_e = parse_input!(inputs[9], i32);
            let newSample=Sample{sample_id,carried_by,rank,expertise_gain,health,cost_a,cost_b,cost_c,cost_d,cost_e };
            samples.push(newSample);              //I put every newly created sample with all these features into the samples vector
        }
           let melikeNur = &players[0];
         if (melikeNur.target == "DIAGNOSIS") {
            if (SampleR) {
                println!("GOTO MOLECULES");
            } else {
                if desiredSample.carried_by == 1 {
                    desiredSample = samples[1].clone();
                }else{
                    desiredSample = samples[0].clone();
                SampleR = true;
                
                println!("{} {}","CONNECT " , desiredSample.sample_id.to_string());
                
                for i in 0..desiredSample.cost_a {
                    elements.push('A');
                }
               for i in 0..desiredSample.cost_b {
                    elements.push('B');
                }
               for i in 0..desiredSample.cost_c{
                    elements.push('C');
                }
               for i in 0..desiredSample.cost_d {
                    elements.push('D');
                }
             for i in 0..desiredSample.cost_e {
                    elements.push('E');
                }
            }}
        } else if melikeNur.target == "MOLECULES" {
            if elements.is_empty(){
                println!("GOTO LABORATORY");
            }else{
                println!("{} {}","CONNECT " , elements.remove(elements.len() - 1));
            }
        } else if melikeNur.target == "LABORATORY"{
            if (SampleR) {
                println!("{} {}","CONNECT " , desiredSample.sample_id.to_string());
                SampleR = false;
            } else {
                println!("GOTO DIAGNOSIS");
            }

        } else {
            println!("GOTO DIAGNOSIS");
        }
