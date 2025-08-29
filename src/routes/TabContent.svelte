<script>
    import Game from './pages/game.svelte';
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
  let room = $state({
    name: "",
    introduction: "",
    black_player: "",
    white_player: "",
    status: 0,
    spectator: [],
    player_num: 0,
    max_player_num: 0,
    is_forbid_chat: true,
    other: "",
  });
  let game = $state({
    step: 0,
    current_player: 0,
    black_score: -1,
    white_score: -1,
    game_tip: "",
    board: [],
    sign: [],
    coord_mode: "31000000",
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

    listen('change_to_room', (event) => {
    const [tabId_, room_] = event.payload;
    console.log("change_to_room", tabId_, room_);
    // 更新对应tab的mode和数据
    if (tabId === tabId_) {
      modes = "game";
      room = room_;
    }
  });

  listen('update_game', (event) => {
    const [tabId_, game_] = event.payload;
    console.log("update_game", tabId_, game_);
    // 更新对应tab的mode和数据
    if (tabId === tabId_) {
      game = game_;
    }
  });

</script>

<div class="p-4 ">
  {#if modes === 'login'}
    <Login tabId={tabId} />
  {:else if modes === 'roomlist'}
    <RoomList tab_id={tabId} datas={roomdata}  account={account} />
  {:else if modes === 'game'}
    <Game tabId={tabId} room={room} game={game} />
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