use utils::stats::covariance;
use utils::stats::variance;
use utils::stats::mean;

// Linear Regression implementation and properties
pub struct LinearRegression {
    pub coefficient: Option<f32>,
    pub intercept: Option<f32>
}

impl LinearRegression {
    pub fn new() -> LinearRegression {
        LinearRegression { coefficient: None, intercept: None }
    }

    pub fn fit(&mut self, x_vals: &Vec<f32>, y_vals: &Vec<f32>) {
        let b = covariance(x_vals, y_vals) / variance(x_vals);
        let a = mean(y_vals) - b * mean(x_vals);
        self.intercept = Some(a);
        self.coefficient = Some(b);
    }

    pub fn predict(&self, x: f32) -> f32 {
        if self.coefficient.is_none() || self.intercept.is_none() {
            panic!("fit(..) must be called first!");
        }

        let a = self.intercept.unwrap();
        let b = self.coefficient.unwrap();
        a + b * x
    }

    pub fn predict_list(&self, x_vals: &Vec<f32>) -> Vec<f32> {
        let mut predictions = Vec::new();
        for i in 0..x_vals.len() {
            predictions.push(self.predict(x_vals[i]));
        } predictions
    }

    fn root_mean_squared_error(&self, actual: &Vec<f32>, predicted: &Vec<f32>) -> f32 {
        let mut sum_error = 0f32;
        let length = actual.len();

        for i in 0..length {
            sum_error += f32::powf(predicted[i] - actual[i], 2f32);
        }

        let mean_error = sum_error / length as f32;
        mean_error.sqrt()
    }

    pub fn evaluate(&self, x_test: &Vec<f32>, y_test: &Vec<f32>) -> f32 {
        if self.coefficient.is_none() || self.intercept.is_none() {
            panic!("fit(..) must be called first!");
        }

        let y_predicted = self.predict_list(x_test);
        self.root_mean_squared_error(y_test, &y_predicted)
    }
}

