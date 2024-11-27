/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "index.html",
    "src/**/*.rs",
  ],
  theme: {
    extend: {
      fontFamily: {
        'title': ["Chomsky"],
        'noto': ["Noto Sans Display"],
        'sans': ["PT Sans"],
        'serif': ["PT Serif"],
      }
    },
  },
  plugins: [],
}
