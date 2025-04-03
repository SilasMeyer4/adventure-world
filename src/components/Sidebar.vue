<template>
    <div class="sidebar" :style="{width: sidebarWidth}">
        <h1 @click="toggleSidebar">
            <span v-if="collapsed">
                <div>A</div>
                <div>V</div>

            </span>
            <span v-else>Adventure World</span>
        </h1>

        <div >
                <div class="sidebar-element" @click="currentMenu = NavbarMenus.HOME" :class="{ active: currentMenu === NavbarMenus.HOME }">
                    <i class="fa-solid fa-map"></i> <!--map icon-->
                <span v-if="!collapsed">Home</span>
            </div>

            <div class="sidebar-element" @click="currentMenu = NavbarMenus.CHARACTERS" :class="{ active: currentMenu === NavbarMenus.CHARACTERS }">
                <i class="fa-solid fa-user-large"></i> <!--user icon-->
                <span v-if="!collapsed">Characters</span>
                
            </div>

            <div class="sidebar-element" @click="currentMenu = NavbarMenus.LOOT" :class="{ active: currentMenu === NavbarMenus.LOOT }">
                <i class="fa-solid fa-gem"></i> <!--gem icon-->
                <Span v-if="!collapsed">Loot</Span>
                
            </div>

            <div class="sidebar-element" @click="currentMenu = NavbarMenus.SETTINGS" :class="{ active: currentMenu === NavbarMenus.SETTINGS }">
                
                <i class="fa-solid fa-gear"></i> <!--settings icon-->
                <span v-if="!collapsed">Settings</span>
                
            </div>
        </div>

    

        <span class="collapsed-icon" :class="{'rotate-180': collapsed}" @click="toggleSidebar">
            <i class="fa-solid fa-arrow-left"></i>
        </span>
    </div>
</template>


<script setup lang="ts">
import { collapsed, toggleSidebar, sidebarWidth, NavbarMenus, currentMenu } from './SidebarState';

</script>


<style>
:root {
    --sidebar-bg-color: #333;
    --sidebar-item-hover: #555;
    --sidebar-item-active: #444;
}
</style>

<style scoped>
.sidebar {
    color: black;
    background-color: var(--sidebar-bg-color);
    float: left;
    position: fixed;
    z-index: 1;;
    top: 0;
    left: 0;
    bottom: 0;
    padding: 0.5em;

    transition: 0.3s ease;

    display: flex;
    flex-direction: column;
    align-items: flex-start;
    box-shadow: 2px 0 10px rgba(0, 0, 0, 0.2);
}

/* Title at the top */
.sidebar h1 {
    font-size: 1.5rem;
    font-weight: bold;
    margin: 0;
    color: var(--sidebar-text-color);
    cursor: pointer;
    padding: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.3s;
}

.sidebar-element {
    display: flex;
    align-items: center;
    padding: 20px;
    font-size: 1.1rem;
    font-weight: bold;
    gap: 15px;
    cursor: pointer;
    transition: background-color 0.3s, color 0.3s;
    width: 100%;
}

/* Collapsed sidebar icon rotation */
.collapsed-icon {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    cursor: pointer;
    padding: 0.75em;
    color: var(--sidebar-text-color-inactive);
    font-size: 1.5rem;
    transition: color 0.3s;
}

.sidebar-element span {
    transition: opacity 0.3s ease;
}

/* Active Menu Item */
.sidebar-element.active {
    background-color: var(--sidebar-item-active);
    color: var(--sidebar-text-color);
}

.collapsed .collapsed-icon {
    left: 50%;
}

.collapsed-icon:hover {
    color: var(--sidebar-text-color);
}

.sidebar-element:hover {
    background-color: var(--sidebar-item-hover);
}

/* Icon size */
.sidebar-element i {
    font-size: 1.5rem;
    width: 30px;
    text-align: center;
    color: var(--sidebar-text-color);
}

/* Hide text when collapsed */
.collapsed .sidebar-element {
    justify-content: center;
    text-align: center;
}

.collapsed .sidebar-element i {
    margin-right: 0;
}

/* Hide text when collapsed */
.collapsed .sidebar-element {
    justify-content: center;
}

.collapsed .sidebar-element i {
    margin-right: 0;
}

/* Hide the text but keep space for the icon */
.collapsed .sidebar-element {
    width: 50px; /* Same width as icons */
    text-align: center;
}

.collapsed .sidebar-element i + span {
    display: none; /* Hide text */
}

/* When collapsed, hide text */
.collapsed .sidebar-element span {
    opacity: 0;
}

.rotate-180 {
    transform: rotate(180deg);
    transition: 0.2s linear;
}
</style>