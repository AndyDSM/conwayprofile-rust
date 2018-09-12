extern crate gif;
extern crate crypto;

use gif::*;
use std::fs::File;
use std::borrow::Cow;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

#[derive(Debug)]
struct ConwayState {
    width: usize,
    height: usize,
    width_depth: usize,
    height_depth: usize,
    width_gap: usize,
    height_gap: usize,
    bits_needed: usize,
    bytes_needed: usize,
    boards: Vec<Vec<u8>>,
}

impl ConwayState {
    fn get_hash(&self, str: &String) -> [u8; 32] {
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(str);

        let mut r = [0u8; 32];

        hasher.result(&mut r[..]);

        r
    }

    fn board_initial(&self, str: &String) -> Vec<u8> {
        let mut hash = self.get_hash(str);

        let mut board_initial: Vec<u8> = vec![0; self.width*self.height];

        for byte in 0..self.bytes_needed {
            for i in 0..8 {
                let pos = byte * 8 + i;
                let bit = hash[byte] & 1;
                hash[byte] >>= 1;
                
                if pos >= self.bits_needed { break; }

                let x = (pos % self.width_depth) + self.width_gap;
                let y = (pos / self.width_depth) + self.height_gap;
                let pos_in_vec1 = x + y * self.width;
                let pos_in_vec2 = (self.width - x - 1) + y * self.width;

                board_initial[pos_in_vec1] = bit as u8;
                board_initial[pos_in_vec2] = bit as u8;
            }
        }

        board_initial
    }

    fn board_square_next(&self, board: &Vec<u8>, x: usize, y: usize, wrap: bool) -> u8 {

        /*let valid_top = y > 0;
        let valid_bottom = y < self.height-1;
        let valid_left = x > 0;
        let valid_right = x < self.width - 1;*/

        let mut neighbours = 0;

        if wrap {
            let top = (y+self.height-1)%self.height;
            let bottom = (y+1)%self.height;
            let left = (x+self.width-1)%self.width;
            let right = (x+1)%self.width;

            for i in vec![(top,left), (top,x), (top,right), (y,left), (y,right), (bottom,left), (bottom,x), (bottom,right)] {
                
                //println!("{0:?}, {1:?}", i, board[i.0 * self.width + i.1]);
                if board[i.0 * self.width + i.1] == 1 { neighbours += 1; };
            }

        }

        let alive = board[x+y*self.width] == 1;

        ((alive && neighbours == 2) || neighbours == 3) as u8

    }

    fn board_next(&self, board: &Vec<u8>, wrap: bool) -> Vec<u8> {

        let mut board_next: Vec<u8> = vec![0; self.width*self.height];

        for x in 0..self.width {
            for y in 0..self.height {
                board_next[x + y * self.width] = self.board_square_next(&board, x, y, true);
            }
        }

        board_next

    }

    fn is_board_empty(&self, board: &Vec<u8>) -> bool {
        for i in board {
            if *i == 1u8 { return false }
        }
        true
    }

    fn all_boards(&self, str: &String, max: usize) -> Vec<Vec<u8>> {
        
        let board_initial = self.board_initial(str);
        let mut r = vec![board_initial];

        for i in 0..max {
            let last = r.len() - 1;
            if self.is_board_empty(&r[last]) { break }
            let mut next_board = self.board_next(&r[last], true);
            r.push(next_board);
        }

        r
    }

    fn new(width: usize, height: usize) -> ConwayState {

        let width_depth = (width - 1) / 4 + 1;
        let height_depth = height - 2 * ((height + 1) / 4);
        let width_gap = (width + 1) / 4;
        let height_gap = (height + 1) / 4;
        let bits_needed = width_depth*height_depth;
        let bytes_needed = (bits_needed - 1) / 8 + 1;

        ConwayState {
            width, height, boards: Vec::new(), width_depth, height_depth, width_gap, height_gap, bits_needed, bytes_needed
        }
    }

    fn gen(&mut self, str: &String, max: usize) -> () {
        self.boards = self.all_boards(str, max);
    }

    fn growed(&self, factor: usize) -> Vec<Vec<u8>> {
        let mut r: Vec<Vec<u8>> = Vec::new();

        for board_base in &self.boards {
            let mut board_new = vec![0u8; self.width*self.height*factor*factor];
            for x in 0..self.width {
                for y in 0..self.height {
                    let pos_old = x + y * self.width;
                    let x_big = x*factor;
                    let y_big = y*factor;
                    let width_new = self.width * factor;
                    for x_ in 0..factor {
                        for y_ in 0..factor {
                            let x_new = x_big + x_;
                            let y_new = y_big + y_;
                            let pos_new = x_new + y_new * width_new;
                            board_new[pos_new] = board_base[pos_old];
                        }
                    }
                }
            }
            r.push(board_new);
        }

        r
        
    }
}

pub fn from_string(str: &String, factor: u16) -> () {

    let mut conway = ConwayState::new(16, 16);

    /*let mut bool_arr: [bool; 48] = [false; 48];

    for i in 0..48 {
        bool_arr[i] = (out_u64 & 1) == 1;
        out_u64 >>= 1;
    }*/


    conway.gen(str, 32);

    let boards = &conway.growed(factor as usize);
    //let mut boards = conway.all_boards(str, 32);

    /*for board in boards {
        for i in 0..64 {
            println!("{:?}",&board[i*16..(i+1)*16]);
        }   
        println!("\n");
    }*/

    let filename = format!("{}.gif", str);

    let mut image = File::create(&filename[..]).unwrap();;
    let mut encoder = Encoder::new(&mut image, factor * (conway.width as u16), factor * (conway.height as u16), &[0xFF, 0xFF, 0xFF, 0, 0, 0]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    for state in boards {
        let mut frame = Frame::default();
        frame.width = factor * (conway.width as u16);
        frame.height = factor * (conway.height as u16);
        frame.buffer = Cow::Borrowed(&*&state[..]);
        encoder.write_frame(&frame).unwrap();
    }

    /*let mut board_initial = conway.board_initial(str);


    println!("\n");

    let mut next_board = conway.board_next(&board_initial, true);

    for i in 0..16 {
        println!("{:?}",&next_board[i*16..(i+1)*16]);
    }*/

    /*for i in 0..48 {

        let pos = i+1;
        let x = (pos % 5) + 3;
        let y = (pos / 5) + 3;
        let pos_in_array1 = x + y * 16;
        let pos_in_array2 = (15 - x) + y * 16;

        let val = out_u64 & 1;
        board_initial[pos_in_array1] = val as u32;
        board_initial[pos_in_array2] = val as u32;
        out_u64 >>= 1;

    }*/

    //println!("{0:?}, {1}", &out[0..6], out_u64);
    //println!("{0:?}, {1:?}", &bool_arr[0..32], &bool_arr[32..48]);

    
}