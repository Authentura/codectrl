/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,ts,svelte,md,elm}'],
	theme: {
		extend: {
			colors: {
				primaryLight: '#1b1b1b',
				primaryDark: '#ededed',
				secondaryLight: '#414141',
				secondaryDark: '#bbbbbb',
				authenturaRed: '#c7235d',
				backgroundLight: '#ffffff',
				backgroundDark: '#11191f'
			},
			fontFamily: {
				body: ['Red Hat Display', 'sans-serif']
			}
		}
	},
	variants: {
		extend: {
			backgroundOpacity: ['active']
		}
	},
	plugins: []
};
