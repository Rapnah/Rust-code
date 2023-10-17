mod ColorHex;
mod MatrixTrans;

use std :: fmt;

// #[derive(Debug)]
// struct MinMax(i64,i64);

// impl fmt :: Display for MinMax
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
//     {
//         write!(f, "{}, {}",self.0,self.1)
//     }
// }

#[derive(Debug)]
struct Point2D
{
    real: f64,
    imag: f64,
}

impl fmt :: Display for Point2D
{
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f,"{} + {}i",self.real,self.imag)
    }
}
// impl  fmt :: Complex for Point2D
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
//     {
//         write!(f,"real: {}, img: {}",self.real,self.imag)
//     }    
// }
// impl fmt :: Binary for Point2D
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
//     {
//         let val1 = self.0;
//         let val2 = self.1;
//         fmt::Binary::fmt(&val1,f);
//         fmt::Binary::fmt(&val2,f)
//     }    
// }
fn main() 
{
    // let minmax: MinMax = MinMax(0,300);
    // println!("Struct Number");
    // println!("Display: {}",minmax);
    // println!("Debug: {:?}",minmax);
    let point: Point2D = Point2D{real: 4.5, imag: 9.9};
    //let point2 =Point2D{4.5 + 9.9i};
    println!("Struct 2Dpoint");
    println!("Display: {}",point);
    println!("Debug: {:?}",point);
    //println!("Point 2D in Binary: {:b}",point)
}