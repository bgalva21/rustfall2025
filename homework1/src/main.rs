
fn fahrenheit_to_celsius(f: f64) -> f64{
   let celsius = (f - 32) / (9/5);
   return celsius;
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    let fahrenheit = (c * (9/5)) + 32;
    return fahrenheit;
}




fn main(){

   let c =  fahrenheit_to_celsius(100);
    let f = celsius_to_fahrenheit(200);
}