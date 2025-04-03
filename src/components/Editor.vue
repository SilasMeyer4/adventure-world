<template>
    <editor-content :editor="editor" />
</template>

<script setup>


import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Table from '@tiptap/extension-table';
import TableRow from '@tiptap/extension-table-row';
import TableCell from '@tiptap/extension-table-cell';
import CodeBlockLowlight from '@tiptap/extension-code-block-lowlight';
import TableHeader from '@tiptap/extension-table-header'
import {all, createLowlight } from 'lowlight';
const lowlight = createLowlight(all)


      const editor = useEditor({
        content: "<p>I'm running Tiptap with Vue.js. 🎉</p>",
        extensions: [
          StarterKit, 
          Table,
          TableRow, 
          TableHeader,
          TableCell,   
          CodeBlockLowlight.configure({
          lowlight,  // Configure lowlight for syntax highlighting
          }),
              ],
      });


</script>