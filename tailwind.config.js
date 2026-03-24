/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,svelte,js,ts}"],
  theme: {
    extend: {
      colors: {
        lugos: {
          bg: "#0c0f14",
          surface: "#141a24",
          border: "#243044",
          accent: "#3b82f6",
          muted: "#8b9cb3",
        },
      },
      fontFamily: {
        sans: ['"DM Sans"', "system-ui", "sans-serif"],
      },
    },
  },
  plugins: [],
};
