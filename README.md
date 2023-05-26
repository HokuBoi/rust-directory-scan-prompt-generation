# Rust Directory Scan Prompt Generation

This Rust project is designed to scan a local directory and its subdirectories, generate a prompt based on the names of the directories, and send it to the OpenAI GPT-3.5 Turbo model for a generated response. It's a fun way to ask an AI what it thinks a project could be about based on the structure of its directories.

## Usage

```shell
cargo run <path_to_directory>
```

## Environment Variable

In order to use this script, you need to have your OpenAI API key ready and set it as an environment variable named `OPENAI_API_KEY`. 

In Unix-like systems, you can do it like this:

```shell
export OPENAI_API_KEY=your_api_key_here
```

## Program Flow

1. The program first checks if a directory path is provided and if the `OPENAI_API_KEY` environment variable is set.

2. The program will then recursively scan the given directory and all of its subdirectories and record their names.

3. A prompt is then generated in the form: "The project with directories named {} is about:", where {} is replaced with the list of directory names.

4. The generated prompt is then passed as a chat message to the GPT-3.5 Turbo model via the OpenAI API, along with a few other default chat messages to set the behavior of the assistant.

5. The AI's response to the prompt is printed to the console.

## Dependencies

- `reqwest` for making HTTP requests to the OpenAI API
- `serde` for serializing the API request payload into JSON
- `tokio` as the async runtime for Rust
- `std::env` and `std::fs` for environment variable handling and file system operations

## Note

Please make sure to replace "gpt-3.5-turbo" in the Prompt structure with the desired AI model from OpenAI.

---

The actual capabilities of the program may vary depending on the API key's permissions and the AI model chosen. Always refer to the latest OpenAI documentation for the most accurate information.

Please remember to handle your API keys carefully and not to expose them in your code or any public repositories.
