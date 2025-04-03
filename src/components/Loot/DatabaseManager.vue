<template>
    <h2>Database Manager</h2>
    <div>
     
    </div>
    <pre style="color: green;">{{ queryResults }}</pre>
  </template>
  
  <script setup lang="ts">
  import { defineComponent, ref, computed} from 'vue';
  import type { PropType } from 'vue';

      const queryResults = ref<string>(''); // To display query results
  
      const sheetURL = ref<string>('');
 
  

  </script>
  
  <style scoped>
  /* Add some basic styling */
  #app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    text-align: center;
    color: #2c3e50;
  }
  
  button {
    margin: 10px;
    padding: 10px;
    font-size: 16px;
    cursor: pointer;
  }

  #csvDropArea.dragover button{
    color: #f0f0f0;
  }
  </style>