<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";
    import * as drawb from "./drawb";
    import Konva from "konva";

    let { tabId, game, can_move } = $props();
    let { finalWidth } = $state(200);
    let { canvasWidth } = $state(200);
    let { board_x } = $state(0);
    let { board_y } = $state(0);
    let stage;

    let boardElement;
    let resizeObserver;

    // 绘制函数
    function draw() {
        let layer = new Konva.Layer();
        stage.destroyChildren();
        stage.draw();

        const boardDiv = document.getElementById("board");
        const boardWidth = boardDiv.clientWidth - 1;
        const boardHeight = boardDiv.clientHeight - 1;

        // 修改 stage 宽高
        stage.width(boardWidth);
        stage.height(boardHeight);

        let game_board = game.board;
        let cols_len = game_board.cols_len; //多少列
        let rows_len = game_board.rows_len; //多少行
        let board_cols_len = cols_len; //绘制多少列 （加上坐标）
        let board_rows_len = rows_len; //绘制多少行 （加上坐标）
        //左上右下
        let have_coord = [false, false, false, false]; //是否绘制坐标
        // 如果有坐标，则长宽 +1
        if (game.coord_mode[0] !== "0") {
            board_rows_len += 1;
            have_coord[0] = true;
        }
        if (game.coord_mode[1] !== "0") {
            board_cols_len += 1;
            have_coord[1] = true;
        }
        if (game.coord_mode[2] !== "0") {
            board_rows_len += 1;
            have_coord[2] = true;
        }
        if (game.coord_mode[3] !== "0") {
            board_cols_len += 1;
            have_coord[3] = true;
        }
        //坐标缩进
        //左右边起点缩进格数，左右边终点缩进格数，上下边起点缩进格数，上下边终点缩进格数。
        let coord_indent = [
            parseInt(game.coord_mode[4]),
            parseInt(game.coord_mode[5]),
            parseInt(game.coord_mode[6]),
            parseInt(game.coord_mode[7]),
        ];
        // 计算每个单元格的宽度和高度
        const cellWidth = boardWidth / board_cols_len;
        const cellHeight = boardHeight / board_rows_len;
        // 每个格子的最终宽度取最小
        finalWidth = Math.floor(Math.min(cellWidth, cellHeight) * 0.99);

        if (have_coord[0]) {
            board_x = finalWidth;
        } else {
            board_x = 0;
        }
        if (have_coord[1]) {
            board_y = finalWidth;
        } else {
            board_y = 0;
        }

        // 绘制格子
        drawb.drawSide(layer, board_x, board_y, cols_len, rows_len, finalWidth);

        // 绘制坐标
        drawb.drawCoord(
            layer,
            game.coord_mode,
            have_coord,
            coord_indent,
            cols_len,
            rows_len,
            finalWidth,
        );

        drawb.drawSignBefore(
            layer,
            game.sign,
            board_x,
            board_y,
            rows_len,
            cols_len,
            finalWidth,
        );

        drawb.drawSignAfter(
            layer,
            game.sign,
            board_x,
            board_y,
            rows_len,
            cols_len,
            finalWidth,
        );

        stage.add(layer);
        // //绘制sign （在绘制棋子前）
        // ctx.save();
        // for (const item of game.sign) {
        //     let [k, ..._] = Object.keys(item);
        //     let v = item[k];
        //     if (k === "AroundSign") {
        //         ctx.fillStyle = convertColorFormat(v.bg_color);
        //         for (const index of v.index) {
        //             let [x, y] = toIndex(index, rows_len, cols_len);
        //             ctx.fillRect(
        //                 x * finalWidth + board_x,
        //                 y * finalWidth + board_y,
        //                 finalWidth,
        //                 finalWidth,
        //             );
        //             ctx.strokeStyle = convertColorFormat(v.ed_color);
        //             ctx.lineWidth = v.size;
        //             ctx.strokeRect(
        //                 x * finalWidth + board_x,
        //                 y * finalWidth + board_y,
        //                 finalWidth,
        //                 finalWidth,
        //             );
        //         }
        //     } else if (k === "BadgeSign") {
        //         // after
        //     } else if (k === "CacheSign") {
        //     } else if (k === "ColorSign") {
        //         ctx.fillStyle = convertColorFormat(v.color);
        //         for (const index of v.indexes) {
        //             let [x, y] = toIndex(index, rows_len, cols_len);
        //             ctx.fillRect(
        //                 x * finalWidth + board_x,
        //                 y * finalWidth + board_y,
        //                 finalWidth,
        //                 finalWidth,
        //             );
        //         }
        //     } else if (k === "FigureSign") {
        //     } else if (k === "GroundSign") {
        //     } else if (k === "LineSign") {
        //     } else if (k === "PathSign") {
        //     } else if (k === "TextSign") {
        //     } else if (k === "TitleSign") {
        //         // after
        //     }
        // }
        // ctx.stroke();
        // ctx.save();
        // //绘制棋子
        // let pieces = game_board.pieces;
        // for (let i = 0; i < game_board.cols_len; i++) {
        //     for (let j = 0; j < game_board.rows_len; j++) {
        //         let piece_mode = parseInt(pieces[i][j]);
        //         if (piece_mode === 1) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "black";
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.fill();
        //         } else if (piece_mode === 2) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "white";
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.fill();
        //             ctx.beginPath();
        //             ctx.fillStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.stroke();
        //         } else if (piece_mode === 3) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "red";
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.fill();
        //             ctx.beginPath();
        //             ctx.fillStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.stroke();
        //         } else if (piece_mode === 4) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "blue";
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.fill();
        //             ctx.beginPath();
        //             ctx.fillStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.stroke();
        //         } else if (piece_mode === 5) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "yellow";
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.fill();
        //             ctx.beginPath();
        //             ctx.fillStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.stroke();
        //         } else if (piece_mode === 6) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "green";
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.fill();
        //             ctx.beginPath();
        //             ctx.fillStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.stroke();
        //         } else if (piece_mode === 7) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "gray";
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.fill();
        //             ctx.beginPath();
        //             ctx.fillStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.arc(
        //                 board_x + j * finalWidth + finalWidth / 2,
        //                 board_y + i * finalWidth + finalWidth / 2,
        //                 (finalWidth / 2) * 0.85,
        //                 0,
        //                 2 * Math.PI,
        //             );
        //             ctx.stroke();
        //         } else if (piece_mode === 8) {
        //             // 太极
        //             // 保存当前绘图状态
        //             ctx.save();
        //             // 获取太极图案的中心坐标和半径
        //             const centerX = board_x + j * finalWidth + finalWidth / 2;
        //             const centerY = board_y + i * finalWidth + finalWidth / 2;
        //             const radius = (finalWidth / 2) * 0.85;
        //             // 移动到太极图案中心
        //             ctx.translate(centerX, centerY);
        //             // 绘制外圆
        //             ctx.beginPath();
        //             ctx.arc(0, 0, radius, 0, Math.PI * 2);
        //             ctx.fillStyle = "white";
        //             ctx.fill();
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.stroke();
        //             // 绘制S形曲线
        //             ctx.beginPath();
        //             ctx.arc(0, 0, radius, Math.PI * 0.5, Math.PI * 1.5, false);
        //             ctx.arc(
        //                 0,
        //                 -radius * 0.5,
        //                 radius * 0.5,
        //                 Math.PI * 1.5,
        //                 Math.PI * 0.5,
        //                 true,
        //             );
        //             ctx.arc(
        //                 0,
        //                 radius * 0.5,
        //                 radius * 0.5,
        //                 Math.PI * 1.5,
        //                 Math.PI * 0.5,
        //                 false,
        //             );
        //             ctx.closePath();
        //             ctx.fillStyle = "black";
        //             ctx.fill();
        //             // 绘制上半圆内的小圆
        //             ctx.beginPath();
        //             ctx.arc(0, -radius * 0.5, radius * 0.15, 0, Math.PI * 2);
        //             ctx.fillStyle = "white";
        //             ctx.fill();
        //             // 绘制下半圆内的小圆
        //             ctx.beginPath();
        //             ctx.arc(0, radius * 0.5, radius * 0.15, 0, Math.PI * 2);
        //             ctx.fillStyle = "black";
        //             ctx.fill();
        //             // 恢复绘图状态
        //             ctx.restore();
        //         } else if (piece_mode === 9) {
        //             ctx.save();
        //             //绘制围墙
        //             ctx.beginPath();
        //             ctx.fillStyle = "orange";
        //             ctx.fillRect(
        //                 board_x + j * finalWidth,
        //                 board_y + i * finalWidth,
        //                 finalWidth,
        //                 finalWidth,
        //             );
        //             ctx.beginPath();
        //             ctx.fillStyle = "red";
        //             ctx.fillRect(
        //                 board_x + j * finalWidth,
        //                 board_y + i * finalWidth,
        //                 finalWidth * 0.65,
        //                 finalWidth * 0.25,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x + j * finalWidth,
        //                 board_y + i * finalWidth,
        //                 finalWidth * 0.65,
        //                 finalWidth * 0.25,
        //             );
        //             ctx.beginPath();
        //             ctx.fillStyle = "red";
        //             ctx.fillRect(
        //                 board_x + j * finalWidth + finalWidth * 0.35,
        //                 board_y + i * finalWidth + finalWidth * 0.25,
        //                 finalWidth * 0.65,
        //                 finalWidth * 0.25,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x + j * finalWidth + finalWidth * 0.35,
        //                 board_y + i * finalWidth + finalWidth * 0.25,
        //                 finalWidth * 0.65,
        //                 finalWidth * 0.25,
        //             );
        //             ctx.beginPath();
        //             ctx.fillStyle = "red";
        //             ctx.fillRect(
        //                 board_x + j * finalWidth,
        //                 board_y + i * finalWidth + finalWidth * 0.5,
        //                 finalWidth * 0.65,
        //                 finalWidth * 0.25,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x + j * finalWidth,
        //                 board_y + i * finalWidth + finalWidth * 0.5,
        //                 finalWidth * 0.65,
        //                 finalWidth * 0.25,
        //             );
        //             ctx.beginPath();
        //             ctx.fillStyle = "red";
        //             ctx.fillRect(
        //                 board_x + j * finalWidth + finalWidth * 0.35,
        //                 board_y + i * finalWidth + finalWidth * 0.75,
        //                 finalWidth * 0.65,
        //                 finalWidth * 0.25,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x + j * finalWidth + finalWidth * 0.35,
        //                 board_y + i * finalWidth + finalWidth * 0.75,
        //                 finalWidth * 0.65,
        //                 finalWidth * 0.25,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 4;
        //             ctx.strokeRect(
        //                 board_x + j * finalWidth,
        //                 board_y + i * finalWidth,
        //                 finalWidth,
        //                 finalWidth,
        //             );
        //             ctx.restore();
        //         } else if (piece_mode === 11) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "black";
        //             ctx.fillRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //         } else if (piece_mode === 12) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "white";
        //             ctx.fillRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //         } else if (piece_mode === 13) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "red";
        //             ctx.fillRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //         } else if (piece_mode === 14) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "blue";
        //             ctx.fillRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //         } else if (piece_mode === 15) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "yellow";
        //             ctx.fillRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //         } else if (piece_mode === 16) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "green";
        //             ctx.fillRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //         } else if (piece_mode === 17) {
        //             ctx.beginPath();
        //             ctx.fillStyle = "gray";
        //             ctx.fillRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //             ctx.strokeStyle = "black";
        //             ctx.lineWidth = 2;
        //             ctx.strokeRect(
        //                 board_x +
        //                     j * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 board_y +
        //                     i * finalWidth +
        //                     finalWidth / 2 -
        //                     finalWidth * 0.8 * 0.5,
        //                 finalWidth * 0.8,
        //                 finalWidth * 0.8,
        //             );
        //         }
        //     }
        // }
        // ctx.stroke();
        // //绘制sign （在绘制棋子后i）
        // ctx.save();
        // for (const item of game.sign) {
        //     let [k, ..._] = Object.keys(item);
        //     let v = item[k];
        //     if (k === "AroundSign") {
        //         // before
        //     } else if (k === "BadgeSign") {
        //         let [x, y] = toIndex(v.index, rows_len, cols_len);
        //         let [c_x, c_y] = toIndex(v.position, 3, 3); // 角标
        //         ctx.beginPath();
        //         ctx.fillStyle = convertColorFormat(v.bg_color);
        //         ctx.arc(
        //             board_x + x * finalWidth + finalWidth * 0.25 * (c_x + 1),
        //             board_y + y * finalWidth + finalWidth * 0.25 * (c_y + 1),
        //             (finalWidth / 6) * 0.85,
        //             0,
        //             2 * Math.PI,
        //         );
        //         ctx.fill();
        //         ctx.font = `12px 微软雅黑`;
        //         let t_x =
        //             board_x + x * finalWidth + finalWidth * 0.25 * (c_x + 1);
        //         let t_y =
        //             board_y + y * finalWidth + finalWidth * 0.25 * (c_y + 1);
        //         ctx.fillStyle = convertColorFormat(v.te_color);
        //         ctx.fillText(v.value, t_x, t_y);
        //     } else if (k === "CacheSign") {
        //     } else if (k === "ColorSign") {
        //         // before
        //     } else if (k === "FigureSign") {
        //     } else if (k === "GroundSign") {
        //     } else if (k === "LineSign") {
        //     } else if (k === "PathSign") {
        //     } else if (k === "TextSign") {
        //     } else if (k === "TitleSign") {
        //         ctx.font = `${Math.round(v.size * 40)}px 微软雅黑`;
        //         let t_x = finalWidth * cols_len * v.position_x + board_x;
        //         let t_y = finalWidth * rows_len * v.position_y + board_y;
        //         ctx.fillStyle = convertColorFormat(v.color);
        //         ctx.fillText(v.title, t_x, t_y);
        //     }
        // }
        // ctx.stroke();
    }
    let isMounted = false;

    onMount(() => {
        stage = new Konva.Stage({
            container: "board",
            width: 500,
            height: 500,
        });

        stage.on("click", (e) => {
            const pointerPos = stage.getPointerPosition();
            console.log("click", pointerPos);
            const x = Math.floor((pointerPos.x - board_x) / finalWidth);
            const y = Math.floor((pointerPos.y - board_y) / finalWidth);
            if (
                x >= 0 &&
                y >= 0 &&
                x < game.board.cols_len &&
                y < game.board.rows_len
            ) {
                //x y反过来
                invoke("request_move_later", { tabId: tabId, x: y, y: x });
            }
        });

        // 创建ResizeObserver实例
        resizeObserver = new ResizeObserver((entries) => {
            for (let entry of entries) {
                // 获取元素的实际尺寸
                const rect = entry.contentRect;
                let width = rect.width;
                let height = rect.height;

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
        let n = JSON.parse(JSON.stringify(game));
        if (curr === n) return;
        curr = n;
        draw();
    });
</script>

<div class="board" id="board" bind:this={boardElement}></div>

<style>
    .board {
        width: 100%;
        height: 100%;
    }
</style>
