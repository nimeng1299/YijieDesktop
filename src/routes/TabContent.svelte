<script>
  import { createEventDispatcher } from 'svelte';
  import Login from './pages/login.svelte';
  import RoomList from './pages/roomlist.svelte';
  import { listen } from "@tauri-apps/api/event";

  let {tabId} = $props();
  let modes = $state("login");
  let roomdata = $state("");

  // 监听change_to_hall事件
  listen('change_to_hall', (event) => {
    const [tabId_, roomList] = event.payload;
    console.log("change_to_hall", tabId_, roomList);
    // 更新对应tab的mode和数据
    if (tabId === tabId_) {
      modes = "roomlist";
      roomdata = roomList;
    }
  });
</script>

<div class="p-4">
  {#if modes === 'login'}
    <Login tabId={tabId} />
  {:else if modes === 'roomlist'}
    <RoomList data={roomdata} />
  {:else}
    <input 
      type="text" 
      placeholder="Tab {tabId} content" 
      class="input input-bordered w-full max-w-xs"
    />
  {/if}
</div>

<style>
  /* Add any additional styles here */
</style>