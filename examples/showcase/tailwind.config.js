module.exports = {
  mode: "jit",
  purge: [
    "./src/**/*.rs", "./index.html", 
  ],
  darkMode: "media", // or 'media' or 'class'
  theme: {
    // fontFamily: {
    //   display: ['Inter', 'system-ui', 'sans-serif'],
    //   body: ['Inter', 'system-ui', 'sans-serif'],
    // },
    // colors: {
    //   transparent: 'transparent',
    //   current: 'currentColor',
    //   black: '#000000',
    //   white: '#ffffff',
    //   red: {
    //     50: '#febac4',
    //     100: '#fd98a7',
    //     200: '#fd758a',
    //     300: '#fc536c',
    //     400: '#fc304f',
    //     500: '#fb0e31',
    //     600: '#e00324',
    //     700: '#bb031e',
    //     800: '#950218',
    //     900: '#700212',
    //   },
    // },
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}