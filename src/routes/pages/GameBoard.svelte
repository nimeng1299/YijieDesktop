<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";
    import * as drawb from "./drawb";
    import Konva from "konva";

    let { game, can_move, is_reply } = $props();
    let { finalWidth } = $state(200);
    let { canvasWidth } = $state(200);
    let { board_x } = $state(0);
    let { board_y } = $state(0);
    const assistMap = new Map();
    let stage;

    let boardElement;
    let resizeObserver;

    // 绘制函数
    function draw(isGameChange = false) {
        stage.destroyChildren();
        stage.draw();
        if (
            !(
                typeof game.board === "object" &&
                Object.prototype.toString.call(game.board) === "[object Object]"
            )
        ) {
            console.log(
                "not draw: ",
                Object.prototype.toString.call(game.board),
            );
            return;
        }
        const _game = $state.snapshot(game);
        let layer = new Konva.Layer();

        if (isGameChange) {
            assistMap.clear();
        }

        const boardDiv = document.getElementById("board");
        const boardWidth = boardDiv.clientWidth - 1.5;
        const boardHeight = boardDiv.clientHeight - 1.5;

        // 修改 stage 宽高
        stage.width(boardWidth);
        stage.height(boardHeight);

        let game_board = _game.board;
        let cols_len = game_board.cols_len; //多少列
        let rows_len = game_board.rows_len; //多少行
        let board_cols_len = cols_len; //绘制多少列 （加上坐标）
        let board_rows_len = rows_len; //绘制多少行 （加上坐标）
        //左上右下
        let have_coord = [false, false, false, false]; //是否绘制坐标
        // 如果有坐标，则长宽 +1
        if (_game.coord_mode[0] !== "0") {
            board_cols_len += 1;
            have_coord[0] = true;
        }
        if (_game.coord_mode[1] !== "0") {
            board_rows_len += 1;
            have_coord[1] = true;
        }
        if (_game.coord_mode[2] !== "0") {
            board_cols_len += 1;
            have_coord[2] = true;
        }
        if (_game.coord_mode[3] !== "0") {
            board_rows_len += 1;
            have_coord[3] = true;
        }
        //坐标缩进
        //左右边起点缩进格数，左右边终点缩进格数，上下边起点缩进格数，上下边终点缩进格数。
        let coord_indent = [
            parseInt(_game.coord_mode[4]),
            parseInt(_game.coord_mode[5]),
            parseInt(_game.coord_mode[6]),
            parseInt(_game.coord_mode[7]),
        ];
        // 计算每个单元格的宽度和高度
        const cellWidth = boardWidth / board_cols_len;
        const cellHeight = boardHeight / board_rows_len;
        // 每个格子的最终宽度取最小
        finalWidth = Math.floor(Math.min(cellWidth, cellHeight) * 0.99);

        // 加 1 为了防止 0 的线画不出来
        if (have_coord[0]) {
            board_x = finalWidth + 1;
        } else {
            board_x = 0 + 1;
        }
        if (have_coord[1]) {
            board_y = finalWidth + 1;
        } else {
            board_y = 0 + 1;
        }

        // 绘制格子
        drawb.drawSide(layer, board_x, board_y, cols_len, rows_len, finalWidth);

        // 绘制坐标
        drawb.drawCoord(
            layer,
            _game.coord_mode,
            have_coord,
            coord_indent,
            cols_len,
            rows_len,
            finalWidth,
        );

        const clipGroup = new Konva.Group({
            clip: {
                x: board_x,
                y: board_y,
                width: rows_len * finalWidth,
                height: cols_len * finalWidth,
            },
        });
        layer.add(clipGroup);

        drawb.drawSignBefore(
            clipGroup,
            _game.sign,
            board_x,
            board_y,
            rows_len,
            cols_len,
            finalWidth,
        );

        drawb.drawAassist(
            layer,
            assistMap,
            board_x,
            board_y,
            rows_len,
            cols_len,
            finalWidth,
        );

        drawb.drawPiece(
            layer,
            _game.board.pieces,
            board_x,
            board_y,
            rows_len,
            cols_len,
            finalWidth,
        );

        drawb.drawSignAfter(
            clipGroup,
            _game.sign,
            board_x,
            board_y,
            rows_len,
            cols_len,
            finalWidth,
        );

        stage.add(layer);
    }
    let isMounted = false;

    onMount(() => {
        stage = new Konva.Stage({
            container: "board",
            width: 500,
            height: 500,
        });

        stage.on("click", (e) => {
            if (e.evt.button === 0) {
                // 左键
                if (is_reply) return;
                const pointerPos = stage.getPointerPosition();
                const x = Math.floor((pointerPos.x - board_x) / finalWidth);
                const y = Math.floor((pointerPos.y - board_y) / finalWidth);
                if (
                    x >= 0 &&
                    y >= 0 &&
                    x < game.board.cols_len &&
                    y < game.board.rows_len
                ) {
                    //x y反过来
                    console.log("click", y, x);
                    invoke("request_move_later", { x: y, y: x });
                }
            }
        });

        stage.on("contextmenu", (e) => {
            e.evt.preventDefault(); // 阻止浏览器默认右键菜单
            // 左键
            if (is_reply) return;
            const pointerPos = stage.getPointerPosition();
            const x = Math.floor((pointerPos.x - board_x) / finalWidth);
            const y = Math.floor((pointerPos.y - board_y) / finalWidth);
            if (
                x >= 0 &&
                y >= 0 &&
                x < game.board.cols_len &&
                y < game.board.rows_len
            ) {
                const index = x * game.board.cols_len + y;
                if (assistMap.has(index)) {
                    let old = assistMap.get(index);
                    assistMap.set(index, (old + 1) % 8);
                } else {
                    assistMap.set(index, 0);
                }
                draw();
            }
        });

        // 创建ResizeObserver实例
        resizeObserver = new ResizeObserver((entries) => {
            for (let entry of entries) {
                // 获取元素的实际尺寸
                const rect = entry.contentRect;
                let width = rect.width;
                let height = rect.height;
                if (width < 1 || height < 1) {
                    return;
                }
                console.log("change size draw");
                draw();
            }
        });

        // 开始观察board元素
        if (boardElement) {
            resizeObserver.observe(boardElement);
        }

        isMounted = true;
    });

    onDestroy(() => {
        // 组件销毁时停止观察
        if (resizeObserver) {
            resizeObserver.disconnect();
        }
    });

    let curr;
    // 监听game变量变化并触发绘制
    $effect(() => {
        if (!isMounted) return;
        if (!game) return;
        let n = JSON.stringify($state.snapshot(game));
        if (curr === n) return;
        curr = n;
        draw(true);
    });
</script>

<div class="board" id="board" bind:this={boardElement}></div>

<style>
    .board {
        width: 100%;
        height: 100%;
    }
</style>
