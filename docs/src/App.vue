<!-- Copyright (c) 2025 air (https://yuanair.github.io). -->
<!-- This program is free software: you can redistribute it and/or modify -->
<!-- it under the terms of the GNU Affero General Public License as published by -->
<!-- the Free Software Foundation, version 3 of the License only. -->
<!-- -->
<!-- This program is distributed in the hope that it will be useful, -->
<!-- but WITHOUT ANY WARRANTY; without even the implied warranty of -->
<!-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the -->
<!-- GNU Affero General Public License for more details. -->
<!-- -->
<!-- You should have received a copy of the GNU Affero General Public License -->
<!-- along with this program. If not, see <https://www.gnu.org/licenses/>. -->

<script lang="ts" setup>
import Homepage from './components/Homepage.vue'
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
  <Sidebar/>
  <Homepage/>
</template>

<style>

* {
  border: none;
  border-radius: 8px;
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  font-family: system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;

  color-scheme: light dark;
  color: rgba(255, 255, 255, 0.86);
  background-color: #242424;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

body {
  margin: 0;
  display: flex;
  place-items: center;
  min-width: 320px;
  min-height: 100vh;
}

h1 {
  font-size: 3.2em;
  line-height: 1.1;
  margin: 0 0 2rem 0;
}

h2 {
  margin: 2rem 0 0 0;
}

#app {
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}

hr {
  border: none;
  border-top: 1px solid #505050;
  margin: 0 0 2rem 0;
}

</style>
