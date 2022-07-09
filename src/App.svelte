<script lang="ts">
  import logo from "./assets/svelte.png";
  import Counter from "./lib/Counter.svelte";

  import { invoke } from "@tauri-apps/api/tauri";
  import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
  } from "@tauri-apps/api/notification";
  import { relaunch } from "@tauri-apps/api/process";

  import { onMount } from "svelte";
  import Dir from "./components/Dir.svelte";

  let permissionGranted;
  onMount(async () => {
    permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
  });
  // const invoke = window.__TAURI__.invoke
  function handleClick() {
    console.log("Clicked");
    invoke("my_custom_command");
    invoke("my_cmd_b", {
      number: 10,
    }).then((res) => {
      console.log("res", res);
    });
  }
  function handleSendNotification() {
    console.log("permisssionGranted", permissionGranted);
    sendNotification({ title: "TAURI", body: "Tauri is awesome!" });
  }
  async function handleRelaunch() {
    await relaunch();
  }
</script>

<main>
  <button class="btn" on:click={handleClick}>Click me</button>
  <button class="btn" on:click={handleSendNotification}>notification</button>
  <button class="btn" on:click={handleRelaunch}>Relaunch</button>
  <hr class="border border-b-2 border-gray-700" />
  <Dir />
</main>

<style>
</style>
