<template>
    <v-col cols="3">
        <v-card class="playlist">
            <img src="../assets/logo.png" />
            <p>{{ name }}</p>
                <v-card-actions class="playlist-buttons">
                    <v-hover v-slot="{on}">
                        <v-btn rounded :color="on ? 'red': ''">
                            <v-icon  size=20>fa-solid fa-trash</v-icon>
                        </v-btn>
                    </v-hover>
                    <div class="pbutton"><v-icon class="btn" size=20 icon="fa-solid fa-arrow-down" /></div>
                    <div class="pbutton"><v-icon class="btn" size=20 icon="fa-solid fa-pen-to-square" /></div>
                </v-card-actions>
        </v-card>
    </v-col>
</template>

<script>
export default {
    name: "PlaylistFrame",
    props: {
        name: {
            type: String,
            required: true
        },
        amount: Number
    }
}
</script>
