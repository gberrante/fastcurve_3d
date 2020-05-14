
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
    znew.push(y[0]);
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
