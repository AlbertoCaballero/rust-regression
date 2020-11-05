pub fn mean(values: &Vec<f32>) -> f32 {
    if values.len() == 0 { return 0f32; }
    values.iter().sum::<f32>() / (values.len() as f32)
}

pub fn variance(values: &Vec<f32>) -> f32 {
    if values.len() == 0 { return 0f32; }
    let mean = mean(values);
    values.iter()
        .map(|x| f32::powf(x-mean, 2 as f32))
        .sum::<f32>() / values.len() as f32
}

pub fn covariance(x_vals: &Vec<f32>, y_vals: &Vec<f32>) -> f32 {
    if x_vals.len() != y_vals.len() { 
        panic!("x_vals and y_vals have to be of same length!"); 
    }
    
    let length: usize = x_vals.len();
    if length == 0usize { return 0f32; }

    let mut covariance: f32 = 0f32;
    let mean_x = mean(x_vals);
    let mean_y = mean(y_vals);

    for i in 0..length {
        covariance += (x_vals[i] - mean_x) * (y_vals[i] - mean_y)
    } covariance
}

