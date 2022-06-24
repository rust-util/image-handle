
use image::{GenericImageView,imageops};
use clap::Parser;
use colored::*;

fn lattice(jiu_gong_grid:JiuGongGrid){
    //获取图片的宽高
    let img_res = image::open(&jiu_gong_grid.image_path);
    match img_res {
        Ok(mut img) => {
            //如果输入的图片是正确的，则开始处理，
            println!("{}","开始处理".green().bold());
            //1、首先获取尺寸
            let (width,height) = &img.dimensions();
            //计算分成9个可以的尺寸
            let width_1 = width/3;
            let height_1 = height/3;
            let mut suffix = "";
            //获取图片的后缀
            for (index,cha ) in jiu_gong_grid.image_path.char_indices(){
                if cha == '.' {
                    suffix = &jiu_gong_grid.image_path[index .. jiu_gong_grid.image_path.len()];
                }
            }
            //开始进行裁剪图片
            let mut count = 1;
            for w in 0..3 {
                for h in 0..3 {
                    let i = imageops::crop(&mut img, h*width_1, w*height_1, width_1, height_1);
                    i.to_image().save(format!("{}{}{}",jiu_gong_grid.output_path,count,suffix)).unwrap();
                    count=count+1;
                }
            }
            println!("{}","处理完成".green().bold());
            
        },
        Err(err) => {
            println!("{:?}",err.to_string());
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "若梦",about="控制台图片处理工具\r\n目前可以把单张图片分割成9宫格\r\n后续会继续加功能！！")]
struct SubCommand{
    #[clap(subcommand)]
    image_handle:ImageHandle
}

#[derive(Parser, Debug)]
enum ImageHandle {
    #[clap(about="把单张图片处理成九宫格")]
    GridHandleSingle(JiuGongGrid),
}

#[derive(Parser, Debug)]
struct JiuGongGrid{
    #[clap(short, long, value_parser)]
    image_path:String,
    #[clap(short, long, value_parser)]
    output_path:String
}

fn handle(sub_command:SubCommand){
    match sub_command.image_handle {
        ImageHandle::GridHandleSingle(jiu_gong_grid) => {
            lattice(jiu_gong_grid);
        },
        _ => {}
    }
}

fn main() {

    let sub_command = SubCommand::parse();
    handle(sub_command);
}
