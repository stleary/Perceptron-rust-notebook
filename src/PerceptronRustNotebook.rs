extern crate rand;
use rand::Rng;
use std::env;



/// The number of inputs for this Perceptron
const INPUTS: usize = 2;

/// This object will hold the inputs for the Perceptron
struct PerceptronInputs {
    values: [f64; INPUTS],
}

/// A Perceptron is the lowest level unit of an artificial neural network. 
/// The Perceptron coded here has 2 inputs and can be used to solve 2-variable classification problems,
/// in this case to solve a straight line linear equation in the form y = mx + b.
/// Writing an algorithm to do this is trivially easy, but the Perceptron will be trained to solve it,
/// and will then be tested to confirm a correct solution.
/// 
/// The bias property corresponds to the b constant, which in this example is interpreted as the y intercept.
/// The learning_rate property determine how much to change the weights are each training cycle.
/// The weights will be used to determine the importance of each input. 
/// weight[0] is associated with the x variable, and weight[1] with the y variable.
#[derive(Debug)]
struct Perceptron {
    bias: f64,
    learning_rate: f64,
    weights: [f64; INPUTS],
}



impl Perceptron {
    /// This function returns the predicted result. In a neural network, this
    /// would be a sigmoid or some similar differentiable function that returns a value in the range (0,1).
    /// For a standalone Perceptron, it is sufficient to use a simple step function
    /// that returns 0 or 1, by comparing the incoming value+bias to 0.
    /// # Arguments
    ///
    /// * value: sum of the weighted inputs
    /// 
    /// # Returns
    ///
    /// * 0 if the sum is less than 0, otherwise returns 1
    ///
    fn activate(&self, value: f64) -> i32 {
        // s is the sigmoid function, in python, in case you want to try it out
        // return 0 if (1 / (1 + math.exp(-1 * (value+self.bias)))) < .5 else 1

        // simple stepwise will work fine for a simple standalone Perceptron
        // let result = value + self.bias;
        // let separator = 0.0;
        
        // calculate the result based on the sigmoid function
        let result = 1.0 / (1.0 + (-1.0 * (value + self.bias)).exp());
        let separator = 0.5;

        if  result < separator { 
            0
        } else {
            1
        }
    }
    
    /// The perceptron can be tested by providing training data but no expected result.
    /// Each raw input is modified by the weight for that input, then the sum of the inputs
    /// is passed to the activation function. The activation function decides how to classify the value.
    /// # Arguments
    ///
    /// * `perceptron` - the Perceptron to be queried
    /// * `perceptron_inputs` - object that contains an array of values, one for each Perceptron input
    /// 
    /// # Return
    ///
    /// * The result of the activation function
    fn query(&mut self, perceptron_inputs: &PerceptronInputs) -> i32 {
        let mut weighted_sum: f64 = 0.0;
        for i in 0..INPUTS {
            weighted_sum += self.weights[i] * perceptron_inputs.values[i];
        }
        self.activate(weighted_sum)
    }    
    
    
    /// The perceptron can be trained by providing training data and an expected result. If it guesses
    /// wrong, the weight for each input will be adjusted.
    ///
    /// # Arguments
    ///
    /// * `perceptron` - the perceptron to be trained
    /// * `perceptron_inputs` - object that contains an array of values, one for each Perceptron input
    ///
    /// # Returns 
    /// 
    /// * the query result
    fn train(&mut self, perceptron_inputs: &PerceptronInputs, target: i32) -> i32 {
        let result = self.query(perceptron_inputs);
        if target != result {
            // the result was incorrect. Modify the weights according to whether 0 or 1 was expected
            // Change the weights according to the value * learning rate.
            let delta64: f64 = f64::from(target - result);
            for i in 0..INPUTS {
                self.weights[i] += delta64 * perceptron_inputs.values[i] * self.learning_rate;
            }
            self.bias += delta64 * self.learning_rate;
        }
        result
    }
}


/// Returns an initialized Perceptron
/// 
/// # Arguments
///
/// * `bias` - offset for the activation function
/// * `learning_rate`: how much to change the weights when a prediction is incorrect
/// 
/// # Returns
/// 
/// * the initialized Perceptron
fn init_perceptron(bias: f64, learning_rate: f64) -> Perceptron {

    let mut rng = rand::thread_rng();
    let mut perceptron = Perceptron {
        bias: bias,
        learning_rate: learning_rate,
        weights: [0.0; INPUTS],
    };
    // the weights are initialized to a random value in the range [0.01..1)
    for i in 0..INPUTS {
        perceptron.weights[i] = rng.gen_range(0.0..0.99)+0.01; 
    }    
    perceptron
}



/// This function creates a perceptron and solves a linear inequality
/// in the form ax + by + c > 0
///
/// # Arguments
///
/// * `x_coefficient` - the coefffient of x (a)
/// * `y_coefficient` - the coefffient of y (b)
/// * `constant_term` - the constant value (c)
pub fn run(x_coefficient: f64, y_coefficient: f64, constant_term: f64 ) {
    
    let mut rng = rand::thread_rng();
    
    // bias should be initialized to a near-zero value
    let bias: f64 = 0.01;
    // learning rate should be low enough to avoid overcorrecting, but smaller values
    // will take longer to converge
    let learning_rate: f64 = 0.0005;
 
    let mut perceptron = init_perceptron(bias, learning_rate);
    println!("Initialized, before training {:?}", perceptron);
    
    // train the Perceptron with a large number of random x and y variable values
    // we hope that it will eventually determine the a, b, and c values of the inequality
    // ax + by + c > 0
    for _ in 0..10000 {
        // The test range matters. Too small or too large a range 
        // will affect the accuracy of the results
        let x = rng.gen_range(-10.0..10.0);
        let y = rng.gen_range(-10.0..10.0);
        let mut perceptron_inputs = PerceptronInputs { values: [0.0; INPUTS] };
        perceptron_inputs.values[0] = x;
        perceptron_inputs.values[1] = y;
        //  we have to know the intended result in order to perform the training
        let mut target = 0;
        if x_coefficient * x + y_coefficient * y - constant_term > 0.0 {
            target = 1;
        }
        perceptron.train(&perceptron_inputs, target);
    }
    println!("After training {:?}", perceptron);
    
    // normalize the actual and calculated inequalities in y = mx + b format for intuitive display
    let mut actual_y_intercept = constant_term / y_coefficient;
    let actual_slope = -1.0 * x_coefficient / y_coefficient;
    let mut actual_sign = " ";
    if actual_y_intercept > 0.0 {
        actual_sign = "+";
    } else {
        actual_sign = "-";
        actual_y_intercept *= -1.0;
    }
    let mut calculated_y_intercept = -perceptron.bias / perceptron.weights[1];
    let calculated_slope = -perceptron.weights[0] / perceptron.weights[1];
    let mut calculated_sign = " ";
    if calculated_y_intercept > 0.0 {
        calculated_sign = "+";
    } else {
        calculated_sign = "-";
        calculated_y_intercept *= -1.0;
    }
    
    // show the results!
    let mut x_value = x_coefficient;
    let mut y_value = y_coefficient;
    let mut c_term = constant_term;
    let mut xplusy_sign = "+";
    if y_value < 0.0 {
        y_value *= -1.0;
        xplusy_sign = "-";
    }
    let mut yplusc_sign = "+";
    if c_term < 0.0 {
        c_term *= -1.0;
        yplusc_sign = "-";
    }
    println!("Original inequality: {:.2}x {} {:.2}y {} {:2}c > 0", 
        x_value, xplusy_sign, y_value, yplusc_sign, c_term);
    println!("The actual slope/intercept form : y = {:.2}x {} {:.2}", 
        actual_slope, actual_sign, actual_y_intercept);
    println!("Calculated slope/intercept form : y = {:.2}x {} {:.2}", 
        calculated_slope, calculated_sign, calculated_y_intercept);
    
    
}



// In this example, the perceptron solves for the inequality 1.4x -5y - 13 > 0')
//run(1.4, -5.0, 13.0);





