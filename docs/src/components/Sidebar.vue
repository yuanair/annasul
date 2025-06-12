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

</script>

<template>
  <div class="sidebar-container">
    <a class="link-button" href="https://github.com/yuanair/annasul">Repository</a>
    <a class="link-button" href="https://yuanair.github.io">Author</a>
  </div>
</template>

<style scoped>
.sidebar-container {
  position: fixed;
  display: flex;
  flex-direction: column;
  user-select: none;
  top: 8px;
  left: 8px;
  width: auto;
  height: auto;
  gap: 8px;
}

.link-button {
  border: 1px solid transparent;
  display: inline-block;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  background-color: rgba(26, 26, 26, 0.5);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.68, -0.6, 0.32, 1.6);
}

.link-button:hover {
  border-color: rgba(99, 107, 255, 0.5);
  transform: scale(1.2) translateX(0.2em);
  box-shadow: 0 1em 2em rgba(0, 0, 0, 0.3);
}
</style>