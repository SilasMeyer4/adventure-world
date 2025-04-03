<template>
    <h2>Create Table (PLEASE USE CSV FILES FOR CREATING NEW TABLES AT THE MOMENT)</h2>
    <div>
        <select name="" id="selectCreate" v-model="selectedTableKind">
            <option disabled value="">Select table type</option>
            <option v-for="table in tableOptions" :key="table" :value="table">
                {{ table }} <!--List of all table kinds that can be created-->
            </option>
        </select>
        <input type="text" name="" v-model="tableName" placeholder="Table Name">
      <button>Create</button>
    </div>

    <div>
    </div>
    <pre :style="{color: queryResultColor}">{{ queryResults }}</pre>
  </template>



<script setup lang="ts">
  import { defineComponent, ref, } from 'vue';
  import type { PropType } from 'vue';

  enum TableKind {
    TYPE = 'Type',
    CLASS = 'Class',
    ENTITY = 'Entity',
    POOL = 'Pool',
  }
  
      const queryResults = ref<string>(''); 
      const queryResultColor = ref<string>('green');
      const selectedTableKind = ref<string>('');
      const tableOptions = ref<string[]>([TableKind.ENTITY, TableKind.CLASS, TableKind.TYPE, TableKind.POOL]);
      const tableName = ref<string>('');
      
    

  </script>