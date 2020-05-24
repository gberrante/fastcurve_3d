
/// Returns the tuple of vector points of the smoothed curve
/// The size of the output vector is proportional to the number of smoothening iterations and the initial points
/// 
/// ## Inputs:
/// - x = vector of points on x axis
/// - y = vector of points on y axis
/// - n = number of smoothening iterations
/// 
/// ## Outputs:
/// - x = augmented vector of smoothed points on x axis
/// - y = augmented vector of smoothed points on y axis
pub fn fast_curve_2d(x : &Vec<f64>, y : &Vec<f64>, n: u8)-> (Vec<f64>,Vec<f64>){
    let (mut xnew, mut ynew) = fast_2d_step(&x,&y);
    for _i in 0..n-1 {
        let (xnew2,ynew2) = fast_2d_step(&xnew,&ynew);
        xnew=xnew2;
        ynew=ynew2;
    }
    (xnew,ynew)
}

/// Returns the tuple of vector points of the smoothed curve
/// The size of the output vector is proportional to the number of smoothening iterations and the initial points
/// ## Inputs:
/// - x = vector of points on x axis
/// - y = vector of points on y axis
/// - z = vector of points on z axis
/// - n = number of smoothening iterations
/// 
/// ## Outputs:
/// - x = augmented vector of smoothed points on x axis
/// - y = augmented vector of smoothed points on y axis
/// - z = augmented vector of smoothed points on z axis
pub fn fast_curve_3d(x : &Vec<f64>, y : &Vec<f64>, z : &Vec<f64>, n: u8)-> (Vec<f64>,Vec<f64>,Vec<f64>){
    let (mut xnew, mut ynew,mut znew) = fast_3d_step(&x,&y,&z);
    for _i in 0..n-1 {
        let (xnew2,ynew2, znew2) = fast_3d_step(&xnew,&ynew,&znew);
        xnew=xnew2;
        ynew=ynew2;
        znew=znew2;
    }
    (xnew,ynew,znew)
}

/// Returns the tuple of vector points of a single smoothening iteration
/// 
/// ## Inputs:
/// - x = vector of points on x axis
/// - y = vector of points on y axis
/// 
/// ## Outputs:
/// - x = augmented vector of smoothed points on x axis
/// - y = augmented vector of smoothed points on y axis
pub fn fast_2d_step(x : &Vec<f64>,y : &Vec<f64>) -> (Vec<f64>,Vec<f64>){
    let mut xnew: Vec<f64> = vec![];
    let mut ynew: Vec<f64> = vec![];

    xnew.push(x[0]);
    ynew.push(y[0]);
    for i in 1..x.len()-1 {
        let a1_x = 0.25*x[i-1]+0.75*x[i]; 
        let a1_y = 0.25*y[i-1]+0.75*y[i]; 
        let c1_x = 0.75*x[i]+0.25*x[i+1];
        let c1_y = 0.75*y[i]+0.25*y[i+1];

        xnew.push(a1_x);
        xnew.push(c1_x);

        ynew.push(a1_y);
        ynew.push(c1_y);
    }
    xnew.push(x[x.len()-1]);
    ynew.push(y[y.len()-1]);
    (xnew,ynew)
}

/// Returns the tuple of vector points of a single smoothening iteration
/// 
/// ## Inputs:
/// - x = vector of points on x axis
/// - y = vector of points on y axis
/// - z = vector of points on z axis
/// 
/// ## Outputs:
/// - x = augmented vector of smoothed points on x axis
/// - y = augmented vector of smoothed points on y axis
/// - z = augmented vector of smoothed points on z axis
pub fn fast_3d_step(x : &Vec<f64>,y : &Vec<f64>,z : &Vec<f64>) -> (Vec<f64>,Vec<f64>,Vec<f64>){
    let mut xnew: Vec<f64> = vec![];
    let mut ynew: Vec<f64> = vec![];
    let mut znew: Vec<f64> = vec![];

    xnew.push(x[0]);
    ynew.push(y[0]);
    znew.push(z[0]);
    for i in 1..x.len()-1 {
        let a1_x = 0.25*x[i-1]+0.75*x[i]; 
        let a1_y = 0.25*y[i-1]+0.75*y[i]; 
        let a1_z = 0.25*z[i-1]+0.75*z[i]; 

        let c1_x = 0.75*x[i]+0.25*x[i+1];
        let c1_y = 0.75*y[i]+0.25*y[i+1];
        let c1_z = 0.75*z[i]+0.25*z[i+1];

        xnew.push(a1_x);
        xnew.push(c1_x);

        ynew.push(a1_y);
        ynew.push(c1_y);

        znew.push(a1_z);
        znew.push(c1_z);
    }
    xnew.push(x[x.len()-1]);
    ynew.push(y[y.len()-1]);
    znew.push(z[z.len()-1]);

    (xnew,ynew,znew)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d() {
        let x = vec![1.0,1.0,4.0,5.0,2.0];
        let y = vec![1.0,2.0,0.5,1.0,2.0];
        let n: u8 = 4;

        let xref=[1.0, 1.0, 1.01171875, 1.03515625, 1.0703125, 1.1171875, 1.17578125, 1.24609375, 1.328125, 1.421875, 1.52734375, 1.64453125, 1.7734375, 
                            1.9140625, 2.06640625, 2.23046875, 2.40625, 2.59375, 2.7734375, 2.9453125, 3.109375, 3.265625, 3.4140625, 3.5546875, 3.6875, 3.8125, 
                            3.9296875, 4.0390625, 4.140625, 4.234375, 4.3203125, 4.3984375, 4.46875, 4.53125, 4.578125, 4.609375, 4.625, 4.625, 4.609375, 4.578125, 
                            4.53125, 4.46875, 4.37890625, 4.26171875, 4.1171875, 3.9453125, 3.7109375, 3.4140625, 2.94921875, 2.0];

        let yref = [1.0, 1.31640625, 1.466796875, 1.556640625, 1.62109375, 1.66015625, 1.685546875, 1.697265625, 1.6953125, 1.6796875, 1.654296875, 1.619140625, 
                            1.57421875, 1.51953125, 1.455078125, 1.380859375, 1.296875, 1.203125, 1.1171875, 1.0390625, 0.96875, 0.90625, 0.8515625, 0.8046875, 0.765625, 
                            0.734375, 0.7109375, 0.6953125, 0.6875, 0.6875, 0.6953125, 0.7109375, 0.734375, 0.765625, 0.798828125, 0.833984375, 0.87109375, 0.91015625, 
                            0.951171875, 0.994140625, 1.0390625, 1.0859375, 1.138671875, 1.197265625, 1.26171875, 1.33203125, 1.419921875, 1.525390625, 1.68359375, 2.0];

        let (xres,yres) = fast_curve_2d(&x, &y, n);

        for i in 0..xref.len() {
            assert_eq!(xres[i],xref[i]);
            assert_eq!(yres[i],yref[i]);
        }
    }

    #[test]
    fn test_3d() {
        let x = vec![1.0,1.0,4.0,5.0,2.0];
        let y = vec![1.0,2.0,0.5,1.0,2.0];
        let z = vec![1.0,2.0,0.8,1.0,1.5];
        let n: u8 = 4;

        let xref=[1.0, 1.0, 1.01171875, 1.03515625, 1.0703125, 1.1171875, 1.17578125, 1.24609375, 1.328125, 1.421875, 1.52734375, 1.64453125, 1.7734375, 
                            1.9140625, 2.06640625, 2.23046875, 2.40625, 2.59375, 2.7734375, 2.9453125, 3.109375, 3.265625, 3.4140625, 3.5546875, 3.6875, 3.8125, 
                            3.9296875, 4.0390625, 4.140625, 4.234375, 4.3203125, 4.3984375, 4.46875, 4.53125, 4.578125, 4.609375, 4.625, 4.625, 4.609375, 4.578125, 
                            4.53125, 4.46875, 4.37890625, 4.26171875, 4.1171875, 3.9453125, 3.7109375, 3.4140625, 2.94921875, 2.0];

        let yref = [1.0, 1.31640625, 1.466796875, 1.556640625, 1.62109375, 1.66015625, 1.685546875, 1.697265625, 1.6953125, 1.6796875, 1.654296875, 1.619140625, 
                            1.57421875, 1.51953125, 1.455078125, 1.380859375, 1.296875, 1.203125, 1.1171875, 1.0390625, 0.96875, 0.90625, 0.8515625, 0.8046875, 0.765625, 
                            0.734375, 0.7109375, 0.6953125, 0.6875, 0.6875, 0.6953125, 0.7109375, 0.734375, 0.765625, 0.798828125, 0.833984375, 0.87109375, 0.91015625, 
                            0.951171875, 0.994140625, 1.0390625, 1.0859375, 1.138671875, 1.197265625, 1.26171875, 1.33203125, 1.419921875, 1.525390625, 1.68359375, 2.0];

        let zref = [1.0, 1.31640625, 1.46796875, 1.56015625, 1.6281249999999998, 1.6718750000000002, 1.7031250000000002, 1.7218750000000003, 1.7281250000000001, 
                            1.7218749999999998, 1.7070312499999998, 1.6835937499999998, 1.6515624999999998, 1.6109375, 1.5617187499999998, 1.50390625, 1.4375, 1.3624999999999998,
                            1.29296875, 1.2289062499999999, 1.1703125, 1.1171875, 1.0695312500000003, 1.02734375, 0.9906250000000001, 0.9593750000000001, 0.9335937500000001, 
                            0.91328125, 0.8984375, 0.8890625000000001, 0.8851562500000001, 0.8867187499999999, 0.8937499999999999, 0.90625, 0.919921875, 0.9347656249999999, 
                            0.9507812499999999, 0.9679687499999998, 0.9863281249999999, 1.005859375, 1.0265625, 1.0484375000000001, 1.0734375000000003, 1.1015625, 1.1328125, 
                            1.1671874999999998, 1.210546875, 1.262890625, 1.341796875, 1.5];

        let (xres,yres,zres) = fast_curve_3d(&x, &y, &z, n);

        for i in 0..xref.len() {
            assert_eq!(xres[i],xref[i]);
            assert_eq!(yres[i],yref[i]);
            assert_eq!(zres[i],zref[i]);
        }
    }
}
