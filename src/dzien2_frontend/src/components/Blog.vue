<template>
    <div>
        <h2 class="text-blue-600">Wpisy na bloga:</h2>
        <div class="w-100 flex flex-row-reverse">
            <button @click="pobierzWpisy" class="w-15 bg-blue-600 text-white rounded-lg px-4 py-1">Refresh</button>
        </div>
        <div class="grid mx-6 gap-4 my-4">
            <div v-for="wpis in wpisy" class="drop-shadow-xl bg-stone-300">
                <p>{{ wpis }}</p>
            </div>
        </div>
        <div class="flex justify-center flex-row gap-4">
            <input type="text" v-model="nowyBlog" class="bg-stone-300 p-1 border-1 border-stone-600" />
            <button @click="dodajWpis" class="w-15 bg-blue-600 text-white rounded-lg px-4 py-1">Dodaj</button>
        </div>
    </div>
</template>



<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';
export default {
    data() {
        return {
            wpisy: [],
            nowyBlog: "",
        }
    },
    methods: {
        async dodajWpis() {
            await dzien2_backend.dodaj_wpis(this.nowyBlog);
            this.wpisy = await dzien2_backend.odczytaj_wpisy();
        },
        async pobierzWpisy() {
            this.wpisy = await dzien2_backend.odczytaj_wpisy();
        }
    },
    async mounted() {
        this.pobierzWpisy();
    }
}
</script>