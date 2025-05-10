# gptut

gptut is a simple command-line interface (CLI) application that allows users to interact with OpenAI models. It provides a user-friendly way to select a model and input text to generate responses.

## Features

- [x] Select a model to interact with.
- [x] (chat) Chatting with selected model
- [ ] tests
- [ ] docs
- [ ] sqlite
- [ ] make
- [ ] pipeline

## Prerequisites

- Rust installed on your machine. _(You can install it from [rust-lang.org](https://www.rust-lang.org/tools/install))_.
- An OpenAI API key. Sign up at [OpenAI](https://platform.openai.com/signup/) to obtain one.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/gptut.git
   ```

2. Navigate to the project directory:

   ```bash
   cd gptut
   ```

3. Set up your [OpenAI API key](https://platform.openai.com/signup/) as an environment variable:

   ```bash
   export OPENAI_API_KEY="your_api_key_here"
   ```

4. Build and run the application using Cargo:

   ```bash
   cargo run
   ```

## Usage

1. After starting the application with the "chat" ommand, you will be prompted to select a model from a list of available models.
2. Choose a model using the arrow keys and press `Enter`.
3. Enter the text for the selected model when prompted.
4. The application currently collects user input but doesn't output the model's response. (You might want to implement that in future versions.)

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Serghei Mateas - [matiaspub@gmail.com](mailto:matiaspub@gmail.com)
