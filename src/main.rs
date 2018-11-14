extern crate rand;
extern crate image;

use rand::Rng;

use std::fs::File;
use std::path::Path;
use std::io::Write;

struct Board {
    width: u32,
    height: u32,
    buff: Vec<u8>,
}

struct BoardParameter {
    k1: f32,
    k2: f32,
    g: u8,
}

impl Board {
    fn new(width: u32, height: u32) -> Board{
        Board {
            width: width,
            height: height,
            buff: vec![0; (width * height) as usize]
        }
    }

    /// Board initializer
    fn seed(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0 .. self.buff.iter().count() {
            self.buff[i] = rng.gen();
        }
    }

    /// getter for value of Board at (x, y)
    fn value(&self, x: u32, y: u32) -> u8{
        self.buff[(y * self.width + x) as usize]
    }

    /// setter for value of Board at (x, y)
    fn set_value(&mut self, x: u32, y: u32, value: u8) {
        self.buff[(y * self.width + x) as usize] = value;
    }

    /// copy board from ther to self
    fn copy_buff(&mut self, other: &Board) {
        for (i, v) in other.buff.iter().enumerate() {
            self.buff[i] = *v;
        }
    }

    /// 画像を保存する関数
    fn image(&self, filename: &str){
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Luma([self.value(x, y)]);
        }
        let ref mut fout = File::create(&Path::new(filename)).unwrap();
        let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
    }

    /// 指定座標の周辺状況を取得する関数
    fn neighborhood(&self, x: u32, y: u32) -> [u8; 8] {
        // 以下トーラス上で考えるものとする
        let x1: u32 = (x + self.width - 1) % self.width; // 左
        let x2: u32 = (x + 1) % self.width; // 右
        let y1: u32 = (y + self.height - 1) % self.height; // 上
        let y2: u32 = (y + 1) % self.height; // 下

        [
            self.value(x1, y1), self.value(x, y1), self.value(x2, y1),
            self.value(x1, y),                     self.value(x2, y),
            self.value(x1, y2), self.value(x, y2), self.value(x2, y2)
        ]
    }

    /// 指定座標の周囲の1~254の範囲内の数値の数を数える
    fn count_infected(&self, x: u32, y: u32) -> u8 {
        let mut count: u8 = 0;
        for value in &self.neighborhood(x, y) {
            if (*value > 0) & (*value < 255) {
                count += 1;
            }
        }
        count
    }

    fn count_illed(&self, x: u32, y: u32) -> u8 {
        let mut count: u8 = 0;
        for value in &self.neighborhood(x, y) {
            if *value == 255 {
                count += 1;
            }
        }
        count
    }

    fn sum(&self, x: u32, y: u32) -> u16{
        self.neighborhood(x, y).iter().fold(0u16, |sum, v| sum + *v as u16) + self.value(x, y) as u16
    }

    fn step(&mut self, params: &BoardParameter) {
        let mut next_board = Board::new(self.width, self.height);
        let mut value: u8;
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                value = self.value(x, y);
                if value == 255 {
                    next_board.set_value(x, y, 0);
                } else {
                    let c_infected = self.count_infected(x, y) as f32;
                    let c_illed = self.count_illed(x, y) as f32;
                    let mut next_value: u16;
                    if value == 0 {
                        let n1 = (c_infected / params.k1).floor() as u16;
                        let n2 = (c_illed / params.k2).floor() as u16;
                        next_value = n1 + n2;
                    } else {
                        let sum = self.sum(x, y) as f32;
                        next_value = (sum / c_infected).floor() as u16 + params.g as u16;
                    }
                    if next_value > 255 {
                        next_value = 255;
                    }
                    next_board.set_value(x, y, next_value as u8);
                }
            }
        }
        self.copy_buff(&next_board);
    }
}

fn main() {
    if std::env::args().len() != 8 { //プログラム名を入れて8
        writeln!(std::io::stderr(), "Error!:number of the argument for this program is not 7");
        writeln!(std::io::stderr(), "Useage:");
        writeln!(std::io::stderr(), "\tgotta dir k1 k2 g t w h\n");
        writeln!(std::io::stderr(), "* all argument without dir is integer of 16bit");
        writeln!(std::io::stderr(), "* dir is target-directory for generateing images. if dir does not exist, this program occured error");
        writeln!(std::io::stderr(), "* k1, k2, g are parameters for simulation");
        writeln!(std::io::stderr(), "* w is width of image");
        writeln!(std::io::stderr(), "* h is height of image");
        writeln!(std::io::stderr(), "* t is time of simulation");
        std::process::exit(1);
    }
    let args: Vec<String> = std::env::args()
                            .skip(1)
                            .collect();
    let dname: &String = &args[0];
    let uargs: Vec<u16>  =  args
                            .iter()
                            .skip(1)
                            .map( |s|
                                s.parse::<u16>().unwrap()
                            )
                            .collect();
    let k1: &u16 = &uargs[0];
    let k2: &u16 = &uargs[1];
    let g: &u16 = &uargs[2];
    let t: &u16 = &uargs[3];
    let w: &u16 = &uargs[4];
    let h: &u16 = &uargs[5];
    let mut b = Board::new(*w as u32, *h as u32);
    let params = BoardParameter{k1: *k1 as f32, k2: *k2 as f32, g: *g as u8};
    b.seed();
    for i in 0 .. *t {
        let s = format!("{}/img-{:04}.png", dname , i);
        b.image(&s);
        b.step(&params);
    }
}
