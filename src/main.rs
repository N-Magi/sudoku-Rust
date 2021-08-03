use std::time::Instant;
use std::{error, usize};
use std::fs::File;

mod cell;
type Pos = cell::sudoku_cell::Pos;
type Cell = cell::sudoku_cell::Cell;
type Able = cell::sudoku_cell::Able;
type Sudoku = Vec<Vec<i32>>;

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
// fn getable(data:Vec<i32>, range:Vec<i32>) -> Vec<i32>{
//     let mut res : Vec<i32> = Vec::new();
//     for itr in range{
//         if !data.contains(&itr){
//             res.push(itr);
//         }
//     }
//     return  res;
// }
fn getable(data:Vec<i32>, range:&Vec<i32>) -> Vec<i32>{
    let mut res : Vec<i32> = Vec::new();
    for itr in range{
        if !data.contains(&itr){
            res.push(*itr)
        }
    }
    return res;
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

fn main(){

    let mut res = readproblem("./test.csv".to_string()).expect("can't Read test.csv");

    //let mut res = readproblem("./test.csv".to_string())?;
    println!("Start Solving SUDOKU shown blew");
    let start_time = Instant::now();
    print(res.clone());
    let mut old_res:Vec<(i32,Sudoku,Pos,i32)> = Vec::new();
    let mut iitr = 0;
    loop {
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
                let mut range:Vec<i32> = Vec::new();
                for itr in 1..=9{
                    range.push(itr);
                }
                 //横の列について走査
                range = getable(res[y].clone(), &range);

                //縦の列について走査
                let mut clm:Vec<i32> = Vec::new();
                for itr_i32 in 0..=8 {
                    clm.push(res[itr_i32][x])
                }
                range = getable(clm.clone(), &range);
                
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
                range = getable(ncell.clone(), &range);
                for ran in range{
                    cell.add_able(
                        Able{
                            able:ran,
                            rest:7
                        }
                    )
                }

                cell.cnt = cell.able.len() as u32;
                cells.push(cell);
            }
        }

        iitr += 1;

        if cells.len() < 1 {break;}//終了条件

        cells.sort_by(|a,b| a.cnt.cmp(&b.cnt));
        //rollnack 
        if cells[0].cnt <= 0 {
            let (trouble,res2,p,num) = old_res[0].clone();
            println!("Contradiction Detected!! Rollback to {}",(trouble));
            res = res2;
            res[p.y as usize][p.x as usize] = num;
            old_res.remove(0);
            continue;
        }

        //for rollback Data Generation
        if cells[0].cnt >= 2{
            old_res.push((iitr,res.clone() ,cells[0].pos ,cells[0].able[1].able));//手数,操作前、操作位置、与なかった操作
        }

        res[cells[0].pos.y as usize][cells[0].pos.x as usize] = cells[0].able[0].able;

        println!("{:2}: {},{} : Count:{}, Num[0]:{}",iitr,cells[0].pos.x,cells[0].pos.y,cells[0].able.len(),cells[0].able[0].able);
        
    }
    let time = start_time.elapsed();
    println!("Solving is Finished at {} process at {} mili seconds",iitr, time.as_millis());
    print(res);
    //let mut str: String = String::new();
    //std::io::stdin().read_line(&mut str);
}