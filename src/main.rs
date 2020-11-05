extern crate linear_reg;

pub fn main() {
    println!("\n\nLinear Regression Program\n\n");

    let mut model = linear_reg::regression::linear_regression::LinearRegression::new();
    let x_values = vec![1f32, 2f32, 3f32, 4f32, 5f32];
    let y_values = vec![1f32, 3f32, 2f32, 3f32, 5f32];
    model.fit(&x_values, &y_values);

    println!("Coefficient: {}", model.coefficient.unwrap());
    println!("Intercept: {}", model.intercept.unwrap());
    println!("Accuracy: {}", model.evaluate(&x_values, &y_values));

    /*
     * Should output:
     *  Coefficient: 0.8
     *  Intercept: 0.39999986
     *  Accuracy: 0.69282037
     * */
}
