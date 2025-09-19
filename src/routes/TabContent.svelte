<script>
    import Game from "./pages/game.svelte";
    import Login from "./pages/login.svelte";
    import RoomList from "./pages/roomlist.svelte";
    import { listen } from "@tauri-apps/api/event";

    let { tabId } = $props();
    let modes = $state("login");
    let roomdata = $state("");
    let account = $state({
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
    let countdown = $state([0, 0]);

    let buttons = $state([]);
    let can_move = $state(false);

    listen("change_account", (event) => {
        const [tabId_, account_] = event.payload;
        if (tabId_ === tabId) {
            account = account_;
        }
    });

    listen("change_to_hall", (event) => {
        const [tabId_, roomList] = event.payload;
        console.log("change_to_hall", tabId_, roomList);
        // 更新对应tab的mode和数据
        if (tabId === tabId_) {
            modes = "roomlist";
            roomdata = roomList;
        }
    });

    listen("change_to_room", (event) => {
        const [tabId_, room_] = event.payload;
        console.log("change_to_room", tabId_, room_);
        // 更新对应tab的mode和数据
        if (tabId === tabId_) {
            modes = "game";
            room = room_;
        }
    });

    listen("update_game", (event) => {
        const [tabId_, game_] = event.payload;
        console.log("update_game", tabId_, game_);
        // 更新对应tab的mode和数据
        if (tabId === tabId_) {
            game = game_;
        }
    });

    listen("dispatch_custom_bottom", (event) => {
        const [tabId_, buttons_] = event.payload;
        console.log("dispatch_custom_bottom", tabId_, buttons_);
        // 更新对应tab的mode和数据
        if (tabId === tabId_ && buttons_[0] !== "-1") {
            buttons = buttons_;
        }
    });

    listen("refresh_countdown", (event) => {
        const [tabId_, countdown_] = event.payload;
        console.log("refresh_countdown", tabId_, countdown_);
        // 更新对应tab的mode和数据
        if (tabId === tabId_) {
            countdown = countdown_;
        }
    });

    listen("you_can_move", (event) => {
        console.log("you_can_move");
        const tabId_ = event.payload;
        // 更新对应tab的mode和数据
        if (tabId === tabId_) {
            can_move = true;
        }
    });

    listen("you_not_move", (event) => {
        console.log("you_not_move");
        const tabId_ = event.payload;
        // 更新对应tab的mode和数据
        if (tabId === tabId_) {
            can_move = false;
        }
    });
</script>

<div class="p-4">
    {#if modes === "login"}
        <Login {tabId} />
    {:else if modes === "roomlist"}
        <RoomList tab_id={tabId} datas={roomdata} {account} />
    {:else if modes === "game"}
        <Game {tabId} {room} {game} {buttons} {countdown} {can_move} />
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
