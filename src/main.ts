import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// DOM Elements
let pickColorBtn: HTMLButtonElement | null;
let colorPreview: HTMLDivElement | null;
let colorCodeDisplay: HTMLInputElement | null;
let copyButton: HTMLButtonElement | null;
let paletteColors: HTMLDivElement | null;

const PALETTE_STORAGE_KEY = 'color-palette';

// --- Core Functions ---

async function pickColor() {
  try {
    await invoke('enter_picker_mode');
  } catch (error) {
    console.error("Error entering picker mode:", error);
  }
}

async function copyColor() {
  if (colorCodeDisplay?.value) {
    try {
      await invoke('copy_to_clipboard', { text: colorCodeDisplay.value });
      // Optionally, provide user feedback (e.g., "Copied!")
      if (copyButton) {
        const originalText = copyButton.innerText;
        copyButton.innerText = 'Copied!';
        setTimeout(() => {
          if (copyButton) {
            copyButton.innerText = originalText;
          }
        }, 1500);
      }
    } catch (error) {
      console.error("Error copying to clipboard:", error);
    }
  }
}

function updateColorDisplay(hexColor: string) {
  if (colorPreview) {
    colorPreview.style.backgroundColor = hexColor;
  }
  if (colorCodeDisplay) {
    colorCodeDisplay.value = hexColor;
  }
}

// --- Palette Functions ---

function addToPalette(hexColor: string) {
  if (!paletteColors) return;

  // Avoid adding duplicates
  const existingColors = getPalette();
  if (existingColors.includes(hexColor)) return;

  const newColor = createPaletteColorElement(hexColor);
  paletteColors.appendChild(newColor);

  // Persist
  savePalette([...existingColors, hexColor]);
}

function createPaletteColorElement(hexColor: string): HTMLDivElement {
  const colorElement = document.createElement('div');
  colorElement.classList.add('palette-color');
  colorElement.style.backgroundColor = hexColor;
  colorElement.dataset.color = hexColor;

  colorElement.addEventListener('click', () => {
    updateColorDisplay(hexColor);
  });

  return colorElement;
}


function getPalette(): string[] {
  return JSON.parse(localStorage.getItem(PALETTE_STORAGE_KEY) || '[]');
}

function savePalette(colors: string[]) {
  localStorage.setItem(PALETTE_STORAGE_KEY, JSON.stringify(colors));
}

function loadPalette() {
  const colors = getPalette();
  colors.forEach(color => {
    if (paletteColors) {
      const colorElement = createPaletteColorElement(color);
      paletteColors.appendChild(colorElement);
    }
  });
}


// --- Event Listener Setup ---

window.addEventListener("DOMContentLoaded", () => {
  // Query for elements
  pickColorBtn = document.querySelector("#pick-color-btn");
  colorPreview = document.querySelector("#color-preview");
  colorCodeDisplay = document.querySelector("#color-code-display");
  copyButton = document.querySelector("#copy-button");
  paletteColors = document.querySelector("#palette-colors");

  // Set initial color
  updateColorDisplay("#000000");

  // Load saved palette
  loadPalette();

  // Attach event listeners
  pickColorBtn?.addEventListener("click", pickColor);
  copyButton?.addEventListener("click", copyColor);

  // Listen for the result from the backend
  listen<string>('color-picked', (event) => {
      const hexColor = event.payload;
      updateColorDisplay(hexColor);
      addToPalette(hexColor);
  });
  
  listen('color-pick-failed', () => {
      console.error("The backend failed to pick a color.");
      // Maybe show a user notification here
  });
});