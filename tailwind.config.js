/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "index.html",
    "src/**/*.rs",
  ],
  theme: {
    extend: {
      fontFamily: {
        'newspaper': ["Old Newspaper"],
        'noto': ["Noto Sans Display"],
        'sans': ["PT Sans"],
        'serif': ["PT Serif"],
      }
    },
  },
  plugins: [],
}
