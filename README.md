# Color Picker

Color Picker is a lightweight desktop tool that enables users to effortlessly capture any color from their screen, view its hexadecimal code, copy it to the clipboard, and save it to a personal color palette for future use. Built with Tauri, Rust, and TypeScript, it offers a seamless and efficient color management experience.

## Features

-   **Screen Color Picking:** Easily pick any color from your screen.
-   **Hex Code Display:** Displays the picked color's hexadecimal code.
-   **Copy to Clipboard:** One-click copy of the color's hex code to your clipboard.
-   **Color Palette:** Save your favorite picked colors to a dynamic palette.

## Technologies Used

-   **Tauri:** For building cross-platform desktop applications using web technologies.
-   **Rust:** For the powerful and efficient backend logic, including screen capturing and clipboard interaction.
-   **TypeScript:** For the type-safe and interactive frontend.
-   **Vite:** As the frontend build tool.

## Installation

To get this project up and running on your local machine, follow these steps:

### Prerequisites

-   [Node.js](https://nodejs.org/en/download/) (LTS version recommended)
-   [Rust](https://www.rust-lang.org/tools/install) (with `rustup` installed)
-   [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for your operating system.

### Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/color-picker.git
    cd color-picker
    ```

2.  **Install frontend dependencies:**
    ```bash
    npm install
    ```

3.  **Run the application in development mode:**
    ```bash
    npm run tauri dev
    ```
    This will open the application window and enable hot-reloading for frontend changes.

4.  **Build the application for production:**
    ```bash
    npm run tauri build
    ```
    This will generate an executable installer for your operating system in the `src-tauri/target/release` directory.

## Usage

1.  Launch the application.
2.  Click the "Pick Color" button.
3.  Move your mouse cursor over any color on your screen. The application will automatically pick the color at the cursor's position.
4.  The picked color will be displayed in the circular preview and its hex code will appear in the input field.
5.  Click the "Copy" button to copy the hex code to your clipboard.
6.  The picked color will also be added to the "Palette" section. Click on any color in the palette to re-display its hex code and preview.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.
