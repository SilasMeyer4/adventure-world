<template>
    <div>
        {{ selectedFile?.name }}
    </div>
    <div>
       {{ fileContent }}
    </div>
    <button @click="openFileChooser">Parse Json</button>
    <button @click="test">ButtonTest</button>


    <input
      ref="fileInput"
      type="file"
      style="display: none"
      @change="onFileSelected"
    />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import * as Parser from '../parser.ts';
import { invoke } from '@tauri-apps/api/core';


const fileInput = ref<HTMLInputElement | null>(null)
const selectedFile = ref<File | null>(null)
const fileContent = ref<string | null>(null)

function openFileChooser() {
  fileInput.value?.click()
}

function onFileSelected(event: Event) {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    selectedFile.value = input.files[0]
    readFile(input.files[0])
  }
}

function readFile(file: File) {
  const reader = new FileReader()
  reader.onload = () => {
    fileContent.value = reader.result as string
    const parsedData = Parser.jsonToObject(fileContent.value);

    console.log(parsedData.monster[1].name) // Example usage: Accessing the first monster's name
  }
  reader.readAsText(file) // You can change this to readAsDataURL for images, etc.
}

const test = async () => {
    const monster = {
  name: "Chimeric Fox",
  source: "NRH-AVitW",
  page: 8,
  size: ["T"],
  type: "beast",
  alignment: ["U"],
  ac: [13],
  hp: { average: 2, formula: "1d4" },
  speed: { walk: 30, burrow: 5 },
  str_: 2,
  dex: 16,
  con: 11,
  int_: 3,
  wis: 12,
  cha: 6,
  passive: 13,
  cr: "0",
  skill: { perception: "+3", stealth: "+5" },
  trait_: [{ name: "Chimeric Creation", entries: ["..."] }],
  action: [{ name: "Bite", entries: ["..."] }],
  traitTags: ["Keen Senses"],
  senseTags: ["D"],
  damageTags: ["P"],
  miscTags: ["MW"],
  hasToken: true,
};
    await invoke("insert_monster", { monster });
}

</script>