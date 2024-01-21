/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        archway: '#FF4D00',
        web3: '#007BFF',
        cosmiclatte: '#FFF8E7',
      },
      borderColor: {
        'gradient-glow': 'linear-gradient(to right, #FF4D00, #3F4E67)'
      }
    },
  },
  plugins: [],
}

