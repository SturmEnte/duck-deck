<script setup>
import DeckButton from "./components/DeckButton.vue";
</script>

<script>
import { emit } from "@tauri-apps/api/event";

export default {
	methods: {
		async press(button) {
			await emit("button_press", button.srcElement.getAttribute("bid"));
		},
	},
};
</script>

<template>
	<div id="button-container">
		<DeckButton bid="1" @click="press" />
		<DeckButton bid="2" @click="press" />
		<DeckButton bid="3" @click="press" />
		<DeckButton bid="4" @click="press" />
		<DeckButton bid="5" @click="press" />
		<DeckButton bid="6" @click="press" />
		<DeckButton bid="7" @click="press" />
		<DeckButton bid="8" @click="press" />
	</div>
</template>

<style scoped>
#button-container {
	position: absolute;
	top: 0;
	left: 0;
	width: 100vw;
	height: 100vh;
	display: flex;
	flex-direction: row;
	align-content: space-around;
	justify-content: space-around;
	flex-wrap: wrap;
}
</style>
