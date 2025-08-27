<script>
  import Login from './pages/login.svelte';
  import RoomList from './pages/roomlist.svelte';
  import { listen } from "@tauri-apps/api/event";

  let {tabId} = $props();
  let modes = $state("login");
  let roomdata = $state("");
  let account =$state({
    id_code: "",
    nick_name: "",
    chat_level: 0,
    chat_tip: "",
    other_user_info: "",
  });

  listen("change_account", (event) => {
    const [tabId_, account_] = event.payload;
    if(tabId_ === tabId){
      account = account_;
    }

  });

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
    <RoomList tab_id={tabId} datas={roomdata}  account={account} />
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