fn get_rgb(c:char) -> (u8, u8, u8){
    match c {
        'R'=> (255,0,0),
        'G'=> (0,255,0),
        'B'=> (0,0,255),
        _ => (0,0,0),
    }

}

fn main() {
   // We are going to accept a letter like RGB
   // and we should return tuple (255,0,0)
   // RED tuple (255,0,0)
   // GREEN tuple (0,255,0)
   // BLUE tuple (0,0,255)

   // write function which accepts char 'R' G B
   // ans return above specified specified tuple
    // let res = get_rgb('R');
    // println!("{:?}", res);
    // let res = get_rgb('G');
    // println!("{:?}", res);
    // let res = get_rgb('B');
    // println!("{:?}", res);
    
    let letters = ['R', 'G', 'B'];

    for idx in 0..letters.len(){
        let res = get_rgb(letters[idx]);
        println!("{:?}", res);
    }
}