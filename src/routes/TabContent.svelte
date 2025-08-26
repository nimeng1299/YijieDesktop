<script>
  import { createEventDispatcher } from 'svelte';
  import Login from './pages/login.svelte';
  import RoomList from './pages/roomlist.svelte';
  
  export let tabId;
  export let instance = {};
  export let mode = 'login'; // 默认为login模式
  
  
  // 使用instance对象来维护独立的状态
  let inputValue = instance.value || '';
  
  function handleInput(event) {
    inputValue = event.target.value;
    // 更新instance对象中的值
    instance.value = inputValue;
  }
</script>

<div class="p-4">
  {#if mode === 'login'}
    <Login />
  {:else if mode === 'roomlist'}
    <RoomList />
  {:else}
    <input 
      type="text" 
      placeholder="Tab {tabId} content" 
      class="input input-bordered w-full max-w-xs"
      value={inputValue}
      on:input={handleInput}
    />
  {/if}
</div>

<style>
  /* Add any additional styles here */
</style>