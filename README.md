# Tauti - Vigenère Cipher Tool

Tauti is a lightweight desktop application built with Tauri, designed to easily encode and decode text using the Vigenère Cipher.
Features:

- User-friendly UI: Simple and intuitive design, making the encoding and decoding process seamless.
- Encode: Input your plain text and key to get Vigenère encoded text.
- Decode: Input your cipher text and key to retrieve the original text.
- Real-time Preview: See the encoded or decoded text change in real-time as you type.

## Table of Contents:

- Installation
- Usage
- Contributing
- License

## Installation:
Prerequisites:

- Ensure you have the latest version of Node.js and Rust installed.
- Install Tauri CLI globally: npm install -g tauri

1. Clone the repository:

```bash
git clone https://github.com/yourusername/tauti.git
```

2. Navigate to the project directory:

```bash
cd tauti
```

3. Install the dependencies:

```bash
npm install
```

4. Build the Tauri app:

```bash
tauri build
```

5. Navigate to the src-tauri/target/release directory to find the built application. Launch it!

## Usage:

- Open Tauti.
- Select either "Encode" or "Decode" based on your need.
- Enter your text in the respective text box.
- Enter the key for the Vigenère Cipher.
- View the result in real-time.

## Contributing:

We welcome contributions to Tauti!

- Fork the repository.
- Create your feature branch (git checkout -b feature/yourFeatureName).
- Commit your changes (git commit -am 'Add some feature').
- Push to the branch (git push origin feature/yourFeatureName).
- Create a new pull request.

## License:

This project is licensed under the MIT License. See the [LICENSE](https://github.com/nwrenger/tauti/blob/main/LICENSE) file for more details.