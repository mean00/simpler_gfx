use clap::Parser;
use image::GenericImageView;
use std::fs::File;
use std::io::Write;
use heatshrink_byte::Config as hsConfig;

#[derive(Parser, Debug)]
#[clap(author="mean00",version="0.1",about="bitmap generator",long_about = None)]
struct Config {
    /// input image
    #[clap(short, long)]
    image: String,
    /// output rs file
    #[clap(short, long)]
    output: String,
    #[clap(default_value_t=false,short, long, action)]
    shrink: bool,        
}

fn sq(u : u8) -> usize
{
    let u: usize = u as usize;
    return u*u;
}
/**
 * 
 */
fn save_bitmap( name : &String, source: &String, width : usize, height : usize, data : &[u8],  shrinked : bool)
{
    let output_size = data.len();

    let mut ofile = File::create(name).expect("unable to create file");   
    write!(ofile,"// Bitmap image converter  \n").expect("oops");
    write!(ofile,"// https://github.com/mean00/simpler_gfx \n").expect("oops");
    write!(ofile,"// from {} \n",source).expect("oops");
    write!(ofile,"pub const WIDTH : usize = {};\n",width).expect("oops");
    write!(ofile,"pub const HEIGHT : usize = {};\n",height).expect("oops");
    if shrinked
    {
        write!(ofile,"pub const BITMAP_HS : [u8;{}] = [\n",output_size).expect("oops");
    }else
    {
        write!(ofile,"pub const BITMAP : [u8;{}] = [\n",output_size).expect("oops");
    }
    
    for i in 0..output_size
    {
        if  (i&15)==0 && i>0 
        {
            write!(ofile,"\n").expect("oops");
        }
        write!(ofile," 0x{:02X}, ",data[i] ).expect("oops");
    }
    write!(ofile,"\n];\n").expect("oops");
    drop(ofile);
}
/**
 * 
 */
fn main() {
    let args=Config::parse();

    let img = image::open(args.image.clone()).unwrap();

    // The dimensions method returns the images width and height.
    let (width, height) =  img.dimensions();

    println!("{} {}x{} => {}", args.image,width,height,args.output);

    let output_size = ((width*height)/8) as usize;
    let mut buffer =Vec::<u8>::with_capacity(output_size);
    unsafe{
    buffer.set_len(output_size);
    };
    // The color method returns the image's `ColorType`.
    println!("Colorspace {:?}", img.color());

    let mid: usize = 128;
    let mut dex=0;
    for y in 0..height
    {               
        for column in 0..(width/8)
        {
            let mut byte : u8 = 0;             
            for col in 0..8
            {                
                    let pixel = img.get_pixel(column*8+col, y);
                    let val: usize = sq(pixel.0[0])+sq(pixel.0[1])+sq(pixel.0[2]);
                    if val>mid
                    {
                        //print!("*");
                        byte |= 1<<col;
                    }else
                    {
                        //print!(".");
                        byte &= !(1<<col);
                    }
            }
            buffer[dex]=byte;
            dex=dex+1;            
        }
    }
    // compress if need be
    
    if args.shrink
    {
        println!("Compressing...");
        let cfg = hsConfig::new( 7, 4).unwrap();
        let mut output : [u8;128*64]=[0;128*64]; // WAY too big
        match heatshrink_byte::encode(&buffer, 
            &mut output, 
            &cfg)  
            {
                Err(_x) => {panic!("Shrinking eror!");},
                Ok(x) => {
                                        println!(" {} => {} bytes",output_size,x.len());                                                                                            
                                        save_bitmap( &args.output, &args.image, width as usize, height as usize , x, true);
                                },
            };
    }else
    {    
        save_bitmap( &args.output, &args.image, width as usize, height as usize , &buffer, false);
    }
    println!("All done");
}
