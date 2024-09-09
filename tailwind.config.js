module.exports = {
  content: [
    './src/**/*.rs', // Watch for class names inside Rust files
    './index.html',
  ],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
}