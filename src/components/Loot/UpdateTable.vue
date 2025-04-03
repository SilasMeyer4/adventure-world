<template>
    <h2>Update Table</h2>
    <button>Entity Registry</button>
    <button>Table Registry</button>
    <div class="v-select-container vSelectParent">
        <v-select
        v-model="selectedTable"
        :options="options"
        label="name"
        placeholder="Select or type table"
        class="custom-v-select"
        :clearable="false"
            :close-on-select="true"
        />
        <span class="selectTableButtons">
          <button class="" >Select</button>  
          <button v-if="selectedTable !== 'TableRegistry' && selectedTable !== 'EntityRegistry'" class="delete">Delete</button>
          <button v-if="selectedTable !== 'TableRegistry'" :disabled="editedRows.length === 0" class='update'>Update</button>
        
    </span>
    </div>

  <pre :style="{color: queryResultColor}">{{ queryResults }}</pre>
  
      <div v-if="selectedTable">
        <h3>{{ selectedTableKind }}: {{ selectedTable }}</h3>
 <!-- Display selected table name -->
    <table v-if="tableData.length >= 0">
      <thead>
        <tr>
          <!-- Dynamically create headers -->
          <th v-for="header in tableHeaders" :key="header">{{ header }}</th>
        </tr>
      </thead>
      <tbody>
        <!-- Dynamically create rows -->
        

      </tbody>
    </table>
    <p v-else>No data available</p> <!-- Message if no data -->
  </div>
  
  </template>


<script setup lang="ts">
import { defineComponent, ref, onMounted, watch, computed } from 'vue';
import type { PropType } from 'vue';
import "vue3-select"

    const queryResults = ref<string>(''); 
    const queryResultColor = ref<string>('green');
    const selectedTable = ref<string>('');
    const options = ref<string[]>([]);
    const tableHeaders = ref<string[]>([]);
    const tableData = ref<any[]>([]);
    let selectedTableKind = ref<string>('');

    const newRowData = ref<string[]>([]);

    const isInFocus = ref<{ [key: string]: boolean }>({});
    const inputRefs = ref<{ [key: string]: HTMLInputElement | null }>({});
    const editedRows = ref<{ rowIndex: number; original: any[]; updated: any[] }[]>([]);

  
    

</script>

<style scoped>
.editableInput {
  width: 100%;
  border: 1px solid #ccc;
  padding: 5px;
  font-size: 14px;
}

.editableInput:focus {
  border-color: blue; /* Change border color when focused */
  background-color: rgb(41, 41, 41);
}

td div {
  padding: 5px;
  cursor: pointer;
}

td  {
  text-align: center;
  vertical-align: center;
}

td div:hover {
  background-color: gray; /* Highlight when hovering */
}

.edited-cell {
  background-color: rgba(88, 0, 28, 0.3);
}

.update {
  color: teal;
}

.update:disabled {
  background-color: rgb(59, 59, 59);
  color: gray;
}

.update-table-top {
  display: flex;
  justify-content: center;
  gap: 100px;
}

</style>

