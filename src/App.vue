<template>
    <v-app>
        <v-navigation-drawer app>
            <v-container>
                <h1>test</h1>
            </v-container>
        </v-navigation-drawer>

        <v-app-bar app>
            <img src="../src-tauri/icons/32x32.png"/>
            <v-app-bar-title>test</v-app-bar-title>
        </v-app-bar>
        <v-main>
            <v-container fluid>
                <v-row >
                    <PlaylistFrame name="Playlist one"/>
                    <PlaylistFrame name="Playlist two"/>
                </v-row>
                <v-btn elevation="2" rounded @click="test">Get settings</v-btn>
            </v-container>
        </v-main>
        <v-footer app>
        </v-footer>
    </v-app>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri"
import PlaylistFrame from "./components/PlaylistFrame.vue"


export default {
    name: 'App',
    components: {
        PlaylistFrame
    },
    data() {
        return {
        }
    },
    methods: {
        test() {
            invoke("my_custom_command")
        }
    }
}
</script>

<style>
@import "./main.css";
#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
    margin-top: 60px;
}
</style>
