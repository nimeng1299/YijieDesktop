<script>
    import { Position } from "@tauri-apps/api/dpi";
    import Game from "./pages/game.svelte";
    import Login from "./pages/login.svelte";
    import RoomList from "./pages/roomlist.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let menuVisible = $state(false);
    let menuPosition = $state([0, 0]);

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

    listen("change_mode", (event) => {
        const mode = event.payload;
        modes = mode;
    });

    listen("change_account", (event) => {
        const account_ = event.payload;
        account = account_;
    });

    listen("change_to_hall", (event) => {
        const roomList = event.payload;
        console.log("change_to_hall", roomList);
        roomdata = roomList;
    });

    listen("change_to_room", (event) => {
        const room_ = event.payload;
        console.log("change_to_room", room_);
        room = room_;
    });

    listen("update_game", (event) => {
        const game_ = event.payload;
        console.log("update_game", game_);
        game = game_;
    });

    listen("dispatch_custom_bottom", (event) => {
        const buttons_ = event.payload;
        console.log("dispatch_custom_bottom", buttons_);
        if (buttons_[0] !== "-1") {
            buttons = buttons_;
        }
    });

    listen("refresh_countdown", (event) => {
        const countdown_ = event.payload;
        console.log("refresh_countdown", countdown_);
        countdown = countdown_;
    });

    listen("you_can_move", (event) => {
        console.log("you_can_move");
        can_move = true;
    });

    listen("you_not_move", (event) => {
        console.log("you_not_move");
        can_move = false;
    });

    // 右键菜单
    function show_menu(e) {
        e.preventDefault(); // 阻止默认右键菜单
        menuPosition = [e.clientX, e.clientY];
        menuVisible = true;
    }

    // 刷新数据
    function refresh_data() {
        invoke("refresh_data", {}).then((datas) => {
            roomdata = datas["roomdata"];
            account = datas["account"];
            room = datas["room"];
            game = datas["game"];
            countdown = datas["countdown"];
            buttons = datas["buttons"];
            can_move = datas["can_move"];

            modes = datas["mode"];
        });
    }
</script>

<div
    class="p-4"
    oncontextmenu={show_menu}
    onclick={() => {
        menuVisible = false;
    }}
>
    <Login {modes} />
    <RoomList datas={roomdata} {account} {modes} />
    <Game {room} {game} {buttons} {countdown} {can_move} {modes} />

    {#if menuVisible}
        <ul
            class="menu bg-base-200 rounded-box w-56"
            style:left={menuPosition[0] + "px"}
            style:top={menuPosition[1] + "px"}
            style:Position="absolute"
        >
            <li
                onclick={() => {
                    refresh_data();
                    menuVisible = false;
                }}
            >
                <a>刷新</a>
            </li>
        </ul>
    {/if}
</div>

<style>
    /* Add any additional styles here */
</style>
