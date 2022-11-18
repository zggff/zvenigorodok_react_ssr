const plugin = require('tailwindcss/plugin')

module.exports = {
    content: ['./src/**/*.{js,jsx,ts,tsx}', './index.html'],
    fontFamily: {
        sans: ['Graphik', 'sans-serif'],
        serif: ['Merriweather', 'serif'],
    },
    theme: {
        extend: {
            height: {
                160: '40rem',
                120: '30rem',
                100: '25rem',
            },
        },
    },
    plugins: [
        plugin(function({ addBase }) {
            addBase({
                html: { fontSize: '20px' },
            })
        }),
    ],
}
