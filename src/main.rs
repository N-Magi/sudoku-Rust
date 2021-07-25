use std::cmp::{Ordering};
use std::{error, usize};
use std::fs::File;

type Sudoku = Vec<Vec<i32>>;

#[derive(Clone, Copy)]
struct Pos{
    x:i32,
    y:i32
}

//列挙子
enum Restrict {
    Row = 1,
    Column = 2,
    Cell = 4
}

//Ableについてのメンバ変数
struct Able{
    able:i32,
    rest:i32
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
    fn addrest_i32(&mut self, num:i32){
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
struct Cell{
    pos:Pos,
    able:Vec<Able>,
    cnt : u32
}

//Cellについてのメンバ関数
impl Cell {
    fn addable(& mut self, abl:Able){

        if self.able.contains(&abl){
            //let mut idx = -1;
            for a in 0..self.able.len() {
                if self.able[a].able == abl.able{

                    self.able[a].addrest_i32(abl.rest);

                    //idx = a as i32;
                    return;
                }
            }
        }
        self.able.push(abl);
    }
    fn solve(&mut self) {
        self.able.retain(|x|x.rest >= 7);
    }
}

//csvファイルを読み込む
fn readproblem(file_path:String) -> Result<Sudoku, Box<dyn error::Error>>{
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut res:Sudoku = Vec::new();
    for result in rdr.deserialize() {
       res.push(result?)
    }
    Ok(res)
}

//可能性について演算する
fn getable(data:Vec<i32>) -> Vec<i32>{
    let mut res : Vec<i32> = Vec::new();

    for itr in 1..=9{
        if !data.contains(&itr){
            res.push(itr);
        }
    }
    return  res;
}

//数独出力
fn print(res : Vec<Vec<i32>>) {
    for ity in 0..9 {
        for itx in 0..9{
            print!("{} ",res[ity][itx]);
        }
        println!();
    }
}

fn main() -> Result<(),Box<dyn error::Error>>{
    let mut res = readproblem("./test.csv".to_string())?;
    println!("Start Solving SUDOKU shown blew");
    print(res.clone());
    let mut old_res:Vec<(Sudoku,Pos,i32)> = Vec::new();
    let mut iitr = 0;
    loop {

        //print(res.clone());

        let mut cells : Vec<Cell> = Vec::new();
        for y in 0..=8 {
            for x in 0..=8 {
                let val = res[y][x];
                if val != 0{
                    continue;
                }

                //位置構造体の初期化
                let pos = Pos{
                    x:x as i32,
                    y:y as i32
                };
                //マス情報の初期化
                let mut cell:Cell = Cell{
                    pos:pos,
                    able:Vec::new(),
                    cnt:0
                };
                //横の列について走査
                for rowabl in getable(res[y].clone()) {
                    cell.addable(
                        Able{
                            able:rowabl,
                            rest:Restrict::Row as i32
                        }
                    )
                }
                //縦の列について走査
                let mut clm:Vec<i32> = Vec::new();
                for itr_i32 in 0..=8 {
                    clm.push(res[itr_i32][x])
                }
                for clmabl in getable(clm.clone()) {
                    cell.addable(
                        Able{
                            able:clmabl,
                            rest:Restrict::Column as i32
                        }
                    )  
                }
                //3x3単位方のマスについての走査
                let ncell_x = pos.x / 3;
                let ncell_y = pos.y / 3;

                let mut ncell:Vec<i32> = Vec::new();
                for ncell_itry in 0..=2 {
                    for ncell_itrx in 0..=2{
                        let cell_x = ncell_x * 3 + ncell_itrx;
                        let cell_y = ncell_y * 3 + ncell_itry;
                        if pos.x == cell_x && pos.y == cell_y {
                            //continue;
                        }
                        ncell.push(res[cell_y as usize][cell_x as usize]);
                    }
                }
                for clmabl in getable(ncell.clone()) {
                    cell.addable(
                        Able{
                            able:clmabl,
                            rest:Restrict::Cell as i32
                        }
                    )  
                }
                
                cell.solve(); //可能性のないものを排除
                cell.cnt = cell.able.len() as u32;
                cells.push(cell);
            }
        }

        iitr += 1;

        if cells.len() < 1 {break;}//終了条件

        cells.sort_by(|a,b| a.cnt.cmp(&b.cnt));
        
        if cells[0].cnt == 1{
            res[cells[0].pos.y as usize][cells[0].pos.x as usize] = cells[0].able[0].able;
        }

        if cells[0].cnt <= 0 {
            println!("Contradiction Detected!! Rollback to previous point");
            let (res2,p,num) = old_res[0].clone();
            res = res2;
            res[p.y as usize][p.x as usize] = num;
            old_res.remove(0);
            continue;
        }

        if cells[0].cnt >= 2{
            old_res.push((res.clone() ,cells[0].pos ,cells[0].able[1].able));//操作前、操作位置、与なかった操作
            res[cells[0].pos.y as usize][cells[0].pos.x as usize] = cells[0].able[0].able;
        }

        println!("{}: {},{} : Count:{}, Num[0]:{}",iitr,cells[0].pos.x,cells[0].pos.y,cells[0].able.len(),cells[0].able[0].able);
        
    }
    println!("Solving is Finished at {} process",iitr);
    print(res);
    let mut str: String = String::new();
    std::io::stdin().read_line(&mut str);
    Ok(())
}