/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html",
  "./src/**/*.{svelte,js,ts,jsx,tsx}",],
  theme: {
    extend: {
        colors: {
            green: "#00bf63",
            cyan: "#0cc0df",
            blue: "#004aad",
            black: "#545454",
            gray: "#d9d9d9",
            red: "#ff3131",
            yellow: "#ffde59"
        },
        fontFamily: {
            pixel: "Pixelify\\ Sans"
        }
    },
  },
  plugins: [],
}

