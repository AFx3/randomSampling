use::rand::Rng;
use std::io;
fn main() {
    
    let mut sequence: [i32;30] = [0;30]; //= [101, 23, 13, 4, 7, 21, 39, 88, 69, 21, 12, 432, 532, 42, 62, 64, 9 , 2, 1, 0, 664, 342, 98]; // Sequence
    let n = sequence.len(); 
    for i in 0..30 {
        sequence[i] = rand::thread_rng().gen_range(0..=100);
    }
    println!("Welcome to random sampling algorithms simulation");
    println!("Assume having a random sequence of items: {:?}", sequence);
    println!();
    println!("type 1 for Reservoir Sampling");
    println!("type 2 for Random samping with a known sequence length");
    println!("type 3 for Scanning and select: andom samping with a known sequence length - streaming model");
    let mut val = String::new();
    io::stdin()
        .read_line(&mut val)
        .expect("Error in reading from the command line");

    let val:  usize = val.trim().parse().expect("You've not inserted a number");

    if val == 1 {
    // RESERVOIR SAMPLING
    
    const M:usize = 4;  //items to sample in the reservoir
    println!("...RESERVOIR SAMPLING...");
    println!("Form the streaming sequence {:?}", sequence);
    println!("How many items you want to put in the reservoir?");
    let mut valore = String::new();
    io::stdin()
        .read_line(&mut valore)
        .expect("Error in reading from the command line");

    let valore:  usize = valore.trim().parse().expect("You've not inserted a number");
    
    if valore <= sequence.len(){
    println!("I'll sample {} items randomly:", valore);
    println!("{:?}", reservoir_sampling::<i32>(&sequence,valore));
    } else {
        println!("Error, not possible have a reservoir greater than the sequence!");
        panic!();
    }  
    } 

    if val == 2 {
    //RANDOM SAMPLING n known (DISK MODEL)    
        println!("How many items you want to sample?");
        let mut v = String::new();
        io::stdin()
            .read_line(&mut v)
            .expect("Error in reading from the command line");

        let v:  usize = v.trim().parse().expect("You've not inserted a number");
    
        if v <= sequence.len(){
            println!("I'll sample {} items randomly from {:?}", v, sequence);
            println!("{:?}", rand_sampling::<i32>(&mut sequence, v));
        } else {
            println!("ERROR: is not possible to sample more items than the sequence!");
            panic!();
            //println!("{:?}", rand_sampling::<i32>(&mut sequence, 3));

        }
    }
    if val == 3 {

    //SCANNING AND SELECT
        //let x = [70,45,83,1,21]; // Sequence
        //let n = sequence.len(); 
        println!("How many items you want to sample from {:?} ?", sequence);
        let mut vvv = String::new();
        io::stdin()
            .read_line(&mut vvv)
            .expect("Error in reading from the command line");
        let vvv:  usize= vvv.trim().parse().expect("You've not inserted a number");
        //let mm = vvv as f32;
        if vvv <= sequence.len(){


        // vvv Items I want to sample
            let mut s = 0;     //items already sampled
            let mut p: f32 = 0.0;
            println!("I'm going to sample {} items from the sequence", vvv);
   
            for j in 0..n {
                while s<vvv { 
                    break 
                }
                    p= rand::thread_rng().gen_range(0.00..1.00);
                    let f1 = vvv as f32;
                    let f2 = s as f32;
                    let f3 = n as f32;
                    let f4 = j as f32;
                    let probability:f32 = ((f1 - f2) / (f3 - f4)) as f32;

                    //println!("prob: {probability}");
                    if p < probability {
                        println!("The picked item is {:?}", sequence[j]);
                        s+=1;
                    }  
        
            }

        }



    } 
    if val > 3 || val < 1{
        println!("ERROR, don't have more options");
        panic!();
    } 
}


fn reservoir_sampling<I32>(s: &[i32], m: usize) -> Vec<i32> {
    //const M: usize = m;
    let mut reservoir = vec![0;m];
    //let n = s.len();
    for i in 0..m {
        reservoir[i] = s[i]; 
    }
    for j in m..s.len() {
        let h = rand::thread_rng().gen_range(0..=j);
        if h < m {
            reservoir[h] = s[j];
        }
    }
    return reservoir;
}
fn rand_sampling<I32>(x: &mut[i32], samples: usize) -> &[i32]{ //function takes an array source for the sampling and an integer that represents the number of items to sample

    //let x_copy = x;
    let mut pp=0;
    let n=x.len();
    for s in 0..= samples-1 { //iterate untill you get 4 samples
       
        pp = rand::thread_rng().gen_range(0..n-s); //extract a rand number, the position between 0 and n-s (items of the array - sampled items)
        x.swap(pp,n-s-1);
    } 
    let sampled_items = &x[n-samples..n];
    return sampled_items;
}
