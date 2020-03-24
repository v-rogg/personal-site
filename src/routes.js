import CV from './Profile/CV.svelte';

const routes = [
    {
        name: '/',
        redirectTo: 'lebenslauf'
    },
    {
        name: '/lebenslauf',
        component: CV,
        lang: {en: 'cv'}
    }
]

export { routes }