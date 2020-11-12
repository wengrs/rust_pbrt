
pub fn quadratic(a:f64, b:f64, c:f64) -> (bool, f64, f64)
{
    let delta = b*b - 4.*a*c;
    if delta < 0.
    {
        return (false, 0., 0.);
    }
    else
    {
        let t0 = (-b-f64::sqrt(delta))/(2.*a);
        let t1 = (-b+f64::sqrt(delta))/(2.*a);      
        return (true, t0, t1);  
    }
}
