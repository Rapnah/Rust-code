use std:: fmt::{self,Formatter,Display};

#[derive(Debug)]
struct Matrix (f32, f32, f32, f32);

impl Display for Matrix
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result 
    {
        writeln!(f,"({} {})",self.0,self.1)?;
        writeln!(f,"({} {})",self.2,self.3)    
    }
}
// fn transpose(&mut fn(f32, f32, f32, f32)) -> Matrix {Matrix}: Iterator`
// {
//     Matrix.rev()
// }
fn main()
{
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    //println!("Transpose:\n{}", transpose(matrix));
}