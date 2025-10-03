<script>
    import "../../app.css";
    import { Menu, MenuItem, Submenu } from "@tauri-apps/api/menu";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import GameCard from "../pages/GameCard.svelte";
    import GameBoard from "../pages/GameBoard.svelte";
    import ReolyList from "./ReolyList.svelte";

    let id = $state(-1);
    let reply_data = $state({
        black_player: "黑方",
        white_player: "白方",
        datas: [],
        time: "",
        title: "",
        version: "",
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

    let mainMenu;

    // 创建菜单项的工厂函数
    const createMenuItem = (id, text, action) => {
        return MenuItem.new({ id, text, action });
    };

    // 创建子菜单的工厂函数
    const createSubmenu = (text, items) => {
        return Submenu.new({ text, items });
    };

    const initMenu = async () => {
        const fileItems = await Promise.all([
            createMenuItem("open", "打开", () => {
                console.log("Open clicked");
                open({
                    multiple: false,
                    directory: false,
                }).then((file) => {
                    invoke("reply_open", {
                        id: id,
                        file: file,
                    }).then((res) => {
                        reply_data = res;
                        if (reply_data.datas.length > 0) {
                            game = reply_data.datas[0];
                        }
                        showToast("打开成功", "success");
                    });
                });
            }),
            createMenuItem("save", "保存", () => {
                console.log("Save clicked");

                invoke("reply_save", {
                    id: id,
                });
            }),
            createMenuItem("save_as", "另存为...", () => {
                console.log("Save As clicked");
                save({
                    filters: [
                        {
                            name: "棋谱文件",
                            extensions: ["json"],
                        },
                    ],
                }).then((file) => {
                    invoke("reply_save_as", {
                        id: id,
                        file: file,
                    }).then((res) => {
                        reply_data = res;
                    });
                });
            }),
        ]);

        const editItems = await Promise.all([
            createMenuItem("undo", "撤回", () => {
                console.log("undo clicked");

                invoke("reply_undo", {
                    id: id,
                }).then(() => {
                    showToast("撤回成功", "success");
                });
            }),
        ]);

        const fileSubmenu = await createSubmenu("文件", fileItems);
        const editSubmenu = await createSubmenu("编辑", editItems);

        mainMenu = await Menu.new({
            items: [
                fileSubmenu,
                editSubmenu,
                // await MenuItem.new({
                //     id: "quit",
                //     text: "Quit",
                //     action: () => {
                //         console.log("Quit pressed");
                //     },
                // }),
            ],
        });
        await mainMenu.setAsAppMenu();
    };

    initMenu().catch(console.error);

    invoke("reply_init", {}).then((res) => {
        console.log("id: ", res);
        id = res;
    });

    function showToast(message, type = "success") {
        console.log("toast: ", message, type);
        const toastContainer = document.getElementById("toast-container");
        const toast = document.createElement("div");
        toast.className = `alert alert-${type}`;
        toast.innerHTML = `<span>${message}</span>`;
        toastContainer.appendChild(toast);

        // Auto-remove after 3 seconds
        setTimeout(() => {
            toast.remove();
        }, 3000);
    }
</script>

<div class="game-container">
    <div class="game-left">
        <div class="gamecard">
            <GameCard
                player={reply_data.black_player}
                score={game.black_score}
                side="black"
                countdown={-1}
            />
        </div>
        <div class="gamecard">
            <GameCard
                player={reply_data.white_player}
                score={game.white_score}
                side="white"
                countdown={-1}
            />
        </div>
    </div>
    <div class="game-middle">
        <GameBoard {game} can_move={false} is_reply={true} />
        <div hidden={game.game_tip === ""}>{game.game_tip}</div>
    </div>
    <div class="game-right">
        <ReolyList
            datas={reply_data.datas}
            title={reply_data.title}
            on:chenge-index={(e) => {
                game = reply_data.datas[e.detail.index];
            }}
            on:delete-index={(e) => {
                invoke("reply_delete", {
                    id: id,
                    index: e.detail.index,
                }).then((res) => {
                    reply_data = res;
                    showToast("删除成功", "success");
                });
                if (e.detail.index >= 0) {
                    game = reply_data.datas[e.detail.index - 1];
                } else {
                    game = reply_data.datas[e.detail.index];
                }
            }}
        />
    </div>
    <div id="toast-container" class="toast toast-end"></div>
</div>

<style>
    .game-container {
        display: flex;
        width: 100wh;
        height: 100vh;
    }
    .game-left {
        width: 15%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }
    .gamecard {
        width: 100%; /* 添加宽度以占满 game-left */
    }
    .game-middle {
        width: 50%;
    }
    .game-right {
        width: 35%;
    }
</style>
