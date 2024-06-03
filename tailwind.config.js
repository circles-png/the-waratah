/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "index.html",
    "src/**/*.rs",
  ],
  theme: {
    extend: {
      fontFamily: {
        'blackletter': ["Unifraktur Maguntia"],
        'sans': ["ui-serif", "serif"],
      }
    },
  },
  plugins: [],
}
