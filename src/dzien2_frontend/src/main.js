import { createPinia } from 'pinia';
import { createApp } from 'vue';
import './index.scss';
import App from './App.vue';
import './index.css';

createApp(App).use(createPinia()).mount('#app');
