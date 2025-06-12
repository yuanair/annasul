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
const name = 'annasul';
const badges = [
  {
    href: 'https://github.com/yuanair/annasul/releases',
    alt: 'Downloads',
    src: 'https://img.shields.io/github/downloads/yuanair/annasul/total?style=flat-square'
  },
  {
    href: 'https://github.com/yuanair/annasul/releases',
    alt: 'Releases',
    src: 'https://img.shields.io/github/v/release/yuanair/annasul?style=flat-square'
  },
  {
    href: 'https://www.gnu.org/licenses/agpl-3.0',
    alt: 'AGPL-3.0-only',
    src: 'https://img.shields.io/badge/License-AGPL--3.0--only-blue.svg',
  },
]
const projects = [
  {name: 'annasul', description: 'User-friendly yet personalized tool library', docs: 'https://docs.rs/annasul'},
  {name: 'abuild', description: 'A build tool', docs: 'https://docs.rs/abuild'},
];
</script>

<template>
  <h1>{{ name }}</h1>
  <hr>
  <a v-for="badge in badges" :href="badge.href" class="badge">
    <img :alt="badge.alt" :src="badge.src">
  </a>

  <div class="warning">
    This project is licensed under <strong>AGPL-3.0-only</strong>.<br/>Any derivative work must be <strong>open
    source</strong> under the same license.
  </div>

  <h2>Introduction</h2>
  <hr>
  <p>User-friendly yet personalized tool library</p>

  <h2>License</h2>
  <hr>
  <a class="img" href="https://www.gnu.org/licenses/agpl-3.0">
    <img alt="AGPL v3 only" src="https://www.gnu.org/graphics/agplv3-88x31.png">
  </a>

  <h2>Projects</h2>
  <hr>
  <div class="parent">
    <div v-for="project in projects" class="child">
      <div class="child-content">
        <h3 v-text="project.name"></h3>
        <p v-text="project.description"></p>
      </div>
      <div class="child-button">
        <a :href="project.docs" class="link-button">Docs</a>
      </div>
    </div>
  </div>
</template>

<style scoped>
.parent {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.child {
  display: grid;
  grid-template-columns: 1fr auto;
  align-items: center;
  background-color: #2a4d3b;
  border: 1px solid #8dffc4;
  padding: 10px;
  margin: 10px;
  transition: all 0.3s cubic-bezier(0.68, -0.6, 0.32, 1.6);
}

.child:hover {
  transform: scale(1.2);
  box-shadow: 0 1em 2em rgba(0, 0, 0, 0.3);
}

.warning {
  background-color: #5d4b14;
  border: 1px solid #ffda66;
  padding: 10px;
  margin: 10px;
  transition: all 0.3s cubic-bezier(0.68, -0.6, 0.32, 1.6);
}

.warning:hover {
  transform: scale(1.2);
  box-shadow: 0 1em 2em rgba(0, 0, 0, 0.3);
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
  transform: scale(1.2);
  box-shadow: 0 1em 2em rgba(0, 0, 0, 0.3);
}

.badge {
  display: inline-block;
  transition: all 0.3s cubic-bezier(0.68, -0.6, 0.32, 1.6);
}

.badge:hover {
  transform: scale(1.2);
  box-shadow: 0 1em 2em rgba(0, 0, 0, 0.3);
}


.img {
  display: inline-block;
  transition: all 0.3s cubic-bezier(0.68, -0.6, 0.32, 1.6);
}

.img:hover {
  transform: scale(1.2);
}
</style>
