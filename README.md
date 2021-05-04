# Perceptron-rust-notebook
A Perceptron to solve linear equations in Rust, using a Jupyter notebook

First install Jupyter Notebook on your device.

* [https://jupyter.org/install](https://jupyter.org/install)

Next, install the evcxr rust kernel for Jupyter

* [https://depth-first.com/articles/2020/09/21/interactive-rust-in-a-repl-and-jupyter-notebook-with-evcxr/](https://depth-first.com/articles/2020/09/21/interactive-rust-in-a-repl-and-jupyter-notebook-with-evcxr/)

Clone this project

Open Jupyter notebooks, navigate to the download directory and open the file **PerceptronRustNotebook.ipynb**

Execute each cell of the notebook in sequence. The first one will take a few minutes to install the imports, and the last will take a few seconds to execute the perceptron.

# Running the code from the command line
The code in the notebook is the source of truth for this project. However, additional files are provided so that you can build and execute the code from the command line. 

You will need to install the Rust compiler and the Cargo tool.

* https://doc.rust-lang.org/cargo/getting-started/installation.html

After cloning the project, cd to the project directory and execute:

* cargo build
* cargo run 1.3 -4.2 8.7

You can replace the parameters with any valid floating point values.

If you want to convert the notebook code to Rust on your own, execute this command:

* jupyter nbconvert --to script PerceptronRustNotebook.ipynb

You will need to edit PerceptronRustNotebook.rs and comment out the last line, which calls the run() function.

