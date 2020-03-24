import ProfileLayout from './ProfileLayout.svelte';

const routes = [
    {
        name: '/',
        redirectTo: 'lebenslauf'
    },
    {
        name: '/lebenslauf',
        component: ProfileLayout,
        lang: {en: 'cv'}
    }
]

export { routes }