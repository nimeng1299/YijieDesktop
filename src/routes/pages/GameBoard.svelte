<script>
    import { invoke } from "@tauri-apps/api/core";

    let { tabId, game, can_move } = $props();
    let { finalWidth } = $state(0);
    let { canvasWidth } = $state(0);
    let { board_x } = $state(0);
    let { board_y } = $state(0);
    let canvas;
    let resizeObserver;

    // 计算绘制哪个文字 i：坐标，len：长度，mode：哪种模式
    // 1- 6 英文字母正序，英文字母倒序，阿拉伯数字正序，阿拉伯数字倒序，中文数字正序，中文数字倒序
    function getText(i, len, mode) {
        if (mode === "1") {
            return String.fromCharCode(64 + i);
        } else if (mode === "2") {
            return String.fromCharCode(64 + len - i);
        } else if (mode === "3") {
            let num = i;
            return num.toString();
        } else if (mode === "4") {
            let num = len - i;
            return num.toString();
        } else if (mode === "5") {
            return String.fromCharCode(19968 + i);
        } else if (mode === "6") {
            return String.fromCharCode(19968 + len - i);
        } else {
            return "";
        }
    }

    //转换回坐标
    function toIndex(index, row, col) {
        return [index % col, Math.floor(index / row)];
    }

    function convertColorFormat(colorStr) {
        // 假设输入格式为 #AARRGGBB
        if (colorStr.length !== 9) {
            throw new Error("Invalid color format");
        }
        const aa = colorStr.substring(1, 3);
        const rr = colorStr.substring(3, 5);
        const gg = colorStr.substring(5, 7);
        const bb = colorStr.substring(7, 9);
        return `#${rr}${gg}${bb}${aa}`;
    }

    // 绘制函数
    function draw() {
        if (!canvas || !game) return;

        const ctx = canvas.getContext("2d");
        if (!ctx) {
            console.error("无法获取Canvas上下文");
            return;
        }

        // 清除画布

        ctx.imageSmoothingEnabled = true;
        // 先获取区域宽高
        const boardDiv = canvas.parentElement;
        const boardWidth = boardDiv.clientWidth;
        const boardHeight = boardDiv.clientHeight;
        canvasWidth = Math.min(boardWidth, boardHeight);

        ctx.clearRect(0, 0, boardWidth, boardHeight);

        let game_board = game.board;
        let cols_len = game_board.cols_len; //多少列
        let rows_len = game_board.rows_len; //多少行
        let board_cols_len = cols_len;
        let board_rows_len = rows_len;
        //左上右下
        let have_coord = [false, false, false, false];
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
        // 最终宽度取最小
        finalWidth = Math.min(cellWidth, cellHeight) - 1;

        //绘制棋盘
        ctx.fillStyle = "black";
        ctx.strokeStyle = "black";

        ctx.save();
        //绘制坐标
        //绘制横向
        let row_count = 0; //计数
        if (have_coord[1]) {
            //绘制上方横向坐标
            ctx.font = `${Math.round(finalWidth / 2)}px 微软雅黑`;
            ctx.textAlign = "center";
            ctx.textBaseline = "middle";
            let indent = 0; //左边有坐标就缩进
            if (have_coord[0]) {
                indent = 1;
            }
            for (
                let i = coord_indent[2] + indent;
                i < cols_len - coord_indent[3] + indent;
                i++
            ) {
                ctx.fillText(
                    getText(i, cols_len, game.coord_mode[1]),
                    i * finalWidth + Math.round(finalWidth / 2),
                    Math.round(finalWidth / 2),
                );
            }
            row_count += 1;
        }
        if (have_coord[3]) {
            //绘制下方横向坐标
            ctx.font = `${Math.round(finalWidth / 2)}px 微软雅黑`;
            ctx.textAlign = "center";
            ctx.textBaseline = "middle";
            let indent = 0; //左边有坐标就缩进
            if (have_coord[0]) {
                indent = 1;
            }
            for (
                let i = coord_indent[2] + indent;
                i < cols_len - coord_indent[3] + indent;
                i++
            ) {
                ctx.fillText(
                    getText(i, cols_len, game.coord_mode[1]),
                    i * finalWidth + Math.round(finalWidth / 2),
                    (row_count + rows_len) * finalWidth +
                        Math.round(finalWidth / 2),
                );
            }
        }
        //绘制纵向
        let col_count = 0; //计数
        if (have_coord[0]) {
            //绘制左方纵向坐标
            ctx.font = `${Math.round(finalWidth / 2)}px 微软雅黑`;
            ctx.textAlign = "center";
            ctx.textBaseline = "middle";
            let indent = 0; //上边有坐标就缩进
            if (have_coord[1]) {
                indent = 1;
            }
            for (
                let i = coord_indent[0] + indent;
                i < rows_len - coord_indent[1] + indent;
                i++
            ) {
                ctx.fillText(
                    getText(i, rows_len, game.coord_mode[0]),
                    Math.round(finalWidth / 2),
                    i * finalWidth + Math.round(finalWidth / 2),
                );
            }
            col_count += 1;
        }
        if (have_coord[2]) {
            //绘制右方纵向坐标
            ctx.font = `${Math.round(finalWidth / 2)}px 微软雅黑`;
            ctx.textAlign = "center";
            ctx.textBaseline = "middle";
            let indent = 0; //左边有坐标就缩进
            if (have_coord[1]) {
                indent = 1;
            }
            for (
                let i = coord_indent[2] + indent;
                i < rows_len - coord_indent[3] + indent;
                i++
            ) {
                ctx.fillText(
                    getText(i, rows_len, game.coord_mode[2]),
                    (col_count + cols_len) * finalWidth +
                        Math.round(finalWidth / 2),
                    i * finalWidth + Math.round(finalWidth / 2),
                );
            }
        }
        ctx.stroke();

        ctx.save();

        //绘制棋盘
        board_x = col_count * finalWidth;
        board_y = row_count * finalWidth;
        //横向
        for (let i = 0; i < rows_len + 1; i++) {
            ctx.moveTo(board_x, board_y + i * finalWidth);
            ctx.lineTo(
                board_x + cols_len * finalWidth,
                board_y + i * finalWidth,
            );
        }
        //纵向
        for (let i = 0; i < cols_len + 1; i++) {
            ctx.moveTo(board_x + i * finalWidth, board_y);
            ctx.lineTo(
                board_x + i * finalWidth,
                board_y + rows_len * finalWidth,
            );
        }
        ctx.stroke();

        //绘制sign （在绘制棋子前）
        ctx.save();

        for (const item of game.sign) {
            let [k, ..._] = Object.keys(item);
            let v = item[k];
            if (k === "AroundSign") {
                ctx.fillStyle = convertColorFormat(v.bg_color);
                for (const index of v.index) {
                    let [x, y] = toIndex(index, rows_len, cols_len);
                    ctx.fillRect(
                        x * finalWidth + board_x,
                        y * finalWidth + board_y,
                        finalWidth,
                        finalWidth,
                    );
                    ctx.strokeStyle = convertColorFormat(v.ed_color);
                    ctx.lineWidth = v.size;
                    ctx.strokeRect(
                        x * finalWidth + board_x,
                        y * finalWidth + board_y,
                        finalWidth,
                        finalWidth,
                    );
                }
            } else if (k === "BadgeSign") {
                // after
            } else if (k === "CacheSign") {
            } else if (k === "ColorSign") {
                ctx.fillStyle = convertColorFormat(v.color);
                for (const index of v.indexes) {
                    let [x, y] = toIndex(index, rows_len, cols_len);
                    ctx.fillRect(
                        x * finalWidth + board_x,
                        y * finalWidth + board_y,
                        finalWidth,
                        finalWidth,
                    );
                }
            } else if (k === "FigureSign") {
            } else if (k === "GroundSign") {
            } else if (k === "LineSign") {
            } else if (k === "PathSign") {
            } else if (k === "TextSign") {
            } else if (k === "TitleSign") {
                // after
            }
        }

        ctx.stroke();

        //绘制棋子
        let pieces = game_board.pieces;
        for (let i = 0; i < game_board.cols_len; i++) {
            for (let j = 0; j < game_board.rows_len; j++) {
                let piece_mode = parseInt(pieces[i][j]);
                if (piece_mode === 1) {
                    ctx.beginPath();
                    ctx.fillStyle = "black";
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.fill();
                } else if (piece_mode === 2) {
                    ctx.beginPath();
                    ctx.fillStyle = "white";
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.fill();
                    ctx.beginPath();
                    ctx.fillStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.stroke();
                } else if (piece_mode === 3) {
                    ctx.beginPath();
                    ctx.fillStyle = "red";
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.fill();
                    ctx.beginPath();
                    ctx.fillStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.stroke();
                } else if (piece_mode === 4) {
                    ctx.beginPath();
                    ctx.fillStyle = "blue";
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.fill();
                    ctx.beginPath();
                    ctx.fillStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.stroke();
                } else if (piece_mode === 5) {
                    ctx.beginPath();
                    ctx.fillStyle = "yellow";
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.fill();
                    ctx.beginPath();
                    ctx.fillStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.stroke();
                } else if (piece_mode === 6) {
                    ctx.beginPath();
                    ctx.fillStyle = "green";
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.fill();
                    ctx.beginPath();
                    ctx.fillStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.stroke();
                } else if (piece_mode === 7) {
                    ctx.beginPath();
                    ctx.fillStyle = "gray";
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.fill();
                    ctx.beginPath();
                    ctx.fillStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.arc(
                        board_x + j * finalWidth + finalWidth / 2,
                        board_y + i * finalWidth + finalWidth / 2,
                        (finalWidth / 2) * 0.85,
                        0,
                        2 * Math.PI,
                    );
                    ctx.stroke();
                } else if (piece_mode === 8) {
                    // 太极
                    // 保存当前绘图状态
                    ctx.save();

                    // 获取太极图案的中心坐标和半径
                    const centerX = board_x + j * finalWidth + finalWidth / 2;
                    const centerY = board_y + i * finalWidth + finalWidth / 2;
                    const radius = (finalWidth / 2) * 0.85;

                    // 移动到太极图案中心
                    ctx.translate(centerX, centerY);

                    // 绘制外圆
                    ctx.beginPath();
                    ctx.arc(0, 0, radius, 0, Math.PI * 2);
                    ctx.fillStyle = "white";
                    ctx.fill();
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.stroke();

                    // 绘制S形曲线
                    ctx.beginPath();
                    ctx.arc(0, 0, radius, Math.PI * 0.5, Math.PI * 1.5, false);
                    ctx.arc(
                        0,
                        -radius * 0.5,
                        radius * 0.5,
                        Math.PI * 1.5,
                        Math.PI * 0.5,
                        true,
                    );
                    ctx.arc(
                        0,
                        radius * 0.5,
                        radius * 0.5,
                        Math.PI * 1.5,
                        Math.PI * 0.5,
                        false,
                    );
                    ctx.closePath();
                    ctx.fillStyle = "black";
                    ctx.fill();

                    // 绘制上半圆内的小圆
                    ctx.beginPath();
                    ctx.arc(0, -radius * 0.5, radius * 0.15, 0, Math.PI * 2);
                    ctx.fillStyle = "white";
                    ctx.fill();

                    // 绘制下半圆内的小圆
                    ctx.beginPath();
                    ctx.arc(0, radius * 0.5, radius * 0.15, 0, Math.PI * 2);
                    ctx.fillStyle = "black";
                    ctx.fill();

                    // 恢复绘图状态
                    ctx.restore();
                } else if (piece_mode === 9) {
                    ctx.save();
                    //绘制围墙
                    ctx.beginPath();
                    ctx.fillStyle = "orange";
                    ctx.fillRect(
                        board_x + j * finalWidth,
                        board_y + i * finalWidth,
                        finalWidth,
                        finalWidth,
                    );

                    ctx.beginPath();
                    ctx.fillStyle = "red";
                    ctx.fillRect(
                        board_x + j * finalWidth,
                        board_y + i * finalWidth,
                        finalWidth * 0.65,
                        finalWidth * 0.25,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x + j * finalWidth,
                        board_y + i * finalWidth,
                        finalWidth * 0.65,
                        finalWidth * 0.25,
                    );

                    ctx.beginPath();
                    ctx.fillStyle = "red";
                    ctx.fillRect(
                        board_x + j * finalWidth + finalWidth * 0.35,
                        board_y + i * finalWidth + finalWidth * 0.25,
                        finalWidth * 0.65,
                        finalWidth * 0.25,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x + j * finalWidth + finalWidth * 0.35,
                        board_y + i * finalWidth + finalWidth * 0.25,
                        finalWidth * 0.65,
                        finalWidth * 0.25,
                    );

                    ctx.beginPath();
                    ctx.fillStyle = "red";
                    ctx.fillRect(
                        board_x + j * finalWidth,
                        board_y + i * finalWidth + finalWidth * 0.5,
                        finalWidth * 0.65,
                        finalWidth * 0.25,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x + j * finalWidth,
                        board_y + i * finalWidth + finalWidth * 0.5,
                        finalWidth * 0.65,
                        finalWidth * 0.25,
                    );

                    ctx.beginPath();
                    ctx.fillStyle = "red";
                    ctx.fillRect(
                        board_x + j * finalWidth + finalWidth * 0.35,
                        board_y + i * finalWidth + finalWidth * 0.75,
                        finalWidth * 0.65,
                        finalWidth * 0.25,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x + j * finalWidth + finalWidth * 0.35,
                        board_y + i * finalWidth + finalWidth * 0.75,
                        finalWidth * 0.65,
                        finalWidth * 0.25,
                    );

                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 4;
                    ctx.strokeRect(
                        board_x + j * finalWidth,
                        board_y + i * finalWidth,
                        finalWidth,
                        finalWidth,
                    );
                    ctx.restore();
                } else if (piece_mode === 11) {
                    ctx.beginPath();
                    ctx.fillStyle = "black";
                    ctx.fillRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                } else if (piece_mode === 12) {
                    ctx.beginPath();
                    ctx.fillStyle = "white";
                    ctx.fillRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                } else if (piece_mode === 13) {
                    ctx.beginPath();
                    ctx.fillStyle = "red";
                    ctx.fillRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                } else if (piece_mode === 14) {
                    ctx.beginPath();
                    ctx.fillStyle = "blue";
                    ctx.fillRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                } else if (piece_mode === 15) {
                    ctx.beginPath();
                    ctx.fillStyle = "yellow";
                    ctx.fillRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                } else if (piece_mode === 16) {
                    ctx.beginPath();
                    ctx.fillStyle = "green";
                    ctx.fillRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                } else if (piece_mode === 17) {
                    ctx.beginPath();
                    ctx.fillStyle = "gray";
                    ctx.fillRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                    ctx.strokeStyle = "black";
                    ctx.lineWidth = 2;
                    ctx.strokeRect(
                        board_x +
                            j * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        board_y +
                            i * finalWidth +
                            finalWidth / 2 -
                            finalWidth * 0.8 * 0.5,
                        finalWidth * 0.8,
                        finalWidth * 0.8,
                    );
                }
            }
        }
        ctx.stroke();

        //绘制sign （在绘制棋子后i）
        ctx.save();

        for (const item of game.sign) {
            let [k, ..._] = Object.keys(item);
            let v = item[k];
            if (k === "AroundSign") {
                // before
            } else if (k === "BadgeSign") {
                let [x, y] = toIndex(v.index, rows_len, cols_len);
                let [c_x, c_y] = toIndex(v.position, 3, 3); // 角标
                ctx.beginPath();
                ctx.fillStyle = convertColorFormat(v.bg_color);
                ctx.arc(
                    board_x + x * finalWidth + finalWidth * 0.25 * (c_x + 1),
                    board_y + y * finalWidth + finalWidth * 0.25 * (c_y + 1),
                    (finalWidth / 6) * 0.85,
                    0,
                    2 * Math.PI,
                );
                ctx.fill();

                ctx.font = `12px 微软雅黑`;
                let t_x =
                    board_x + x * finalWidth + finalWidth * 0.25 * (c_x + 1);
                let t_y =
                    board_y + y * finalWidth + finalWidth * 0.25 * (c_y + 1);
                ctx.fillStyle = convertColorFormat(v.te_color);
                ctx.fillText(v.value, t_x, t_y);
            } else if (k === "CacheSign") {
            } else if (k === "ColorSign") {
                // before
            } else if (k === "FigureSign") {
            } else if (k === "GroundSign") {
            } else if (k === "LineSign") {
            } else if (k === "PathSign") {
            } else if (k === "TextSign") {
            } else if (k === "TitleSign") {
                ctx.font = `${Math.round(v.size * 40)}px 微软雅黑`;
                let t_x = finalWidth * cols_len * v.position_x + board_x;
                let t_y = finalWidth * rows_len * v.position_y + board_y;
                ctx.fillStyle = convertColorFormat(v.color);
                ctx.fillText(v.title, t_x, t_y);
            }
        }

        ctx.stroke();
    }

    // 监听game变量变化并触发绘制
    $effect(() => {
        if (!game) return;
        console.log("draw");
        draw();
    });

    // 监听div尺寸变化并触发绘制
    $effect(() => {
        const boardDiv = canvas?.parentElement;
        if (!boardDiv) return;

        // 创建ResizeObserver
        resizeObserver = new ResizeObserver(() => {
            console.log("Board size changed");
            draw();
        });

        // 开始观察
        resizeObserver.observe(boardDiv);

        // 清理函数
        return () => {
            if (resizeObserver) {
                resizeObserver.disconnect();
            }
        };
    });

    // 点击事件
    function handleClick(e) {
        console.log("can move", can_move);
        if (!can_move) return;
        const rect = canvas?.getBoundingClientRect();
        if (!rect) return;

        const x = Math.floor((e.clientX - rect.left - board_x) / finalWidth);
        const y = Math.floor((e.clientY - rect.top - board_y) / finalWidth);
        console.log(rect, e, x, y, board_x, board_y, finalWidth);
        if (
            x >= 0 &&
            y >= 0 &&
            x < game.board.cols_len &&
            y < game.board.rows_len
        ) {
            console.log(rect, e, x, y, board_x, board_y, finalWidth);
            //x y反过来
            invoke("request_move_later", { tabId: tabId, x: y, y: x });
        }
    }
</script>

<div class="board">
    <canvas
        bind:this={canvas}
        width={canvasWidth}
        height={canvasWidth}
        onclick={handleClick}
    ></canvas>
</div>

<style>
    .board {
        width: 100%;
        height: 100%;
    }
</style>
