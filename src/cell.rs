pub mod sudoku_cell{
    use std::cmp::{Ordering};

    #[derive(Clone, Copy)]
    pub struct Pos{
        pub x:i32,
        pub y:i32
    }

    //Ableについてのメンバ変数
    pub struct Able{
        pub able:i32,
        pub rest:i32
    }

    //Ableについての継承
    impl PartialEq for Able {
        fn eq(&self, abl:&Able) -> bool {
                return self.able == abl.able;
            }
    }

    //Ableについての継承
    impl PartialOrd for Able {
        fn partial_cmp(&self, abl: &Able) -> Option<std::cmp::Ordering> {
            if self.able == abl.able{
                Some(Ordering::Equal)
            }
            else {
                None
            }
        }
    }

    //Ableについてのメソッド
    impl Able {
        pub fn addrest_i32(&mut self, num:i32){
            match self.rest {
                0 => { self.rest = num},
                1 => { self.rest += if self.rest != (num){ num } else{ 0 } },
                2 => { self.rest += if self.rest != (num){ num } else{ 0 } },
                3 => { self.rest += if self.rest <= (num){ num } else{ 0 }},
                4 => { self.rest += if self.rest != (num){ num} else{ 0 }},
                5 => { self.rest += if 2 == (num){ num } else{ 0 }},
                6 => { self.rest += if 1 == (num){ num } else{ 0 }},
                7 => { self.rest += 0}
                _ => {self.rest += 0}
            }
            //return self.clone();
        }
    }

    //Cellについてのメンバ変数
    pub struct Cell{
        pub pos:Pos,
        pub able:Vec<Able>,
        pub cnt : u32
    }

    //Cellについてのメンバ関数
    impl Cell {
        pub fn add_able(& mut self, abl:Able){

            if self.able.contains(&abl){
                for a in 0..self.able.len() {
                    if self.able[a].able == abl.able{
                        self.able[a].addrest_i32(abl.rest);
                        return;
                    }
                }
            }
            self.able.push(abl);
        }
    }
}