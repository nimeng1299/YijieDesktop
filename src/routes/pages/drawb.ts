import Konva from "konva";
// 计算绘制哪个文字 i：坐标，len：长度，mode：哪种模式
// 1- 6 英文字母正序，英文字母倒序，阿拉伯数字正序，阿拉伯数字倒序，中文数字正序，中文数字倒序
export function getText(i, len, mode) {
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
  return [index % col, Math.floor(index / col)];
}

//转换颜色 argb -> rgba
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

// 绘制棋子
// board_x:       棋盘开始 x 坐标
// board_y:       棋盘开始 y 坐标
// cols_len:      多少列
// rows_len:      多少行
// finalWidth:    每格棋盘的宽度
export function drawSide(
  layer: Konva.Layer,
  board_x,
  board_y,
  cols_len,
  rows_len,
  finalWidth,
) {
  // 画横线
  for (let i = 0; i <= rows_len; i++) {
    const line = new Konva.Line({
      points: [
        board_x,
        board_y + i * finalWidth,
        board_x + cols_len * finalWidth,
        board_y + i * finalWidth,
      ],
      stroke: "black",
      strokeWidth: 2,
    });
    layer.add(line);
  }
  // 画竖线
  for (let i = 0; i <= cols_len; i++) {
    const line = new Konva.Line({
      points: [
        board_x + i * finalWidth,
        board_y,
        board_x + i * finalWidth,
        board_y + rows_len * finalWidth,
      ],
      stroke: "black",
      strokeWidth: 2,
    });
    layer.add(line);
  }
}

// 绘制坐标
// coord_mode:    坐标模式 1-6
// have_coord:    是否绘制坐标，左上右下
// coord_indent:  左右边起点缩进格数，左右边终点缩进格数，上下边起点缩进格数，上下边终点缩进格数。
// cols_len:      多少列
// rows_len:      多少行
// finalWidth:    每格棋盘的宽度
export function drawCoord(
  layer: Konva.Layer,
  coord_mode,
  have_coord,
  coord_indent,
  cols_len,
  rows_len,
  finalWidth,
) {
  // 绘制横向
  let row_count = 0; //计数
  //绘制上方横向坐标
  if (have_coord[1]) {
    let indent = 0; //左边有坐标就缩进
    if (have_coord[0]) {
      indent = 1;
    }
    for (
      let i = coord_indent[2] + indent;
      i < cols_len - coord_indent[3] + indent;
      i++
    ) {
      let complexText = new Konva.Text({
        x: i * finalWidth,
        y: 0,
        text: getText(i - coord_indent[2], cols_len, coord_mode[1]),
        width: finalWidth,
        height: finalWidth,
        align: "center",
        verticalAlign: "middle",
        fontSize: Math.round(finalWidth / 2),
        fontFamily: "微软雅黑",
      });
      layer.add(complexText);
    }
    row_count += 1;
  }
  //绘制下方横向坐标
  if (have_coord[3]) {
    let indent = 0; //左边有坐标就缩进
    if (have_coord[0]) {
      indent = 1;
    }
    for (
      let i = coord_indent[2] + indent;
      i < cols_len - coord_indent[3] + indent;
      i++
    ) {
      let complexText = new Konva.Text({
        x: i * finalWidth,
        y: (row_count + rows_len) * finalWidth,
        text: getText(i - coord_indent[2], cols_len, coord_mode[3]),
        width: finalWidth,
        height: finalWidth,
        align: "center",
        verticalAlign: "middle",
        fontSize: Math.round(finalWidth / 2),
        fontFamily: "微软雅黑",
      });
      layer.add(complexText);
    }
  }
  //绘制纵向
  let col_count = 0; //计数
  //绘制左方纵向坐标
  if (have_coord[0]) {
    let indent = 0; //上边有坐标就缩进
    if (have_coord[1]) {
      indent = 1;
    }
    for (
      let i = coord_indent[0] + indent;
      i < rows_len - coord_indent[1] + indent;
      i++
    ) {
      let complexText = new Konva.Text({
        x: 0,
        y: i * finalWidth,
        text: getText(i - coord_indent[0], rows_len, coord_mode[0]),
        width: finalWidth,
        height: finalWidth,
        align: "center",
        verticalAlign: "middle",
        fontSize: Math.round(finalWidth / 2),
        fontFamily: "微软雅黑",
      });
      layer.add(complexText);
    }
    col_count += 1;
  }
  //绘制右方纵向坐标
  if (have_coord[2]) {
    let indent = 0; //左边有坐标就缩进
    if (have_coord[1]) {
      indent = 1;
    }
    for (
      let i = coord_indent[2] + indent;
      i < cols_len - coord_indent[3] + indent;
      i++
    ) {
      let complexText = new Konva.Text({
        x: (col_count + cols_len) * finalWidth,
        y: i * finalWidth,
        text: getText(i - coord_indent[0], cols_len, coord_mode[3]),
        width: finalWidth,
        height: finalWidth,
        align: "center",
        verticalAlign: "middle",
        fontSize: Math.round(finalWidth / 2),
        fontFamily: "微软雅黑",
      });
      layer.add(complexText);
    }
  }
}

// 绘制 sign (在绘制棋子前面 / 后景)
export function drawSignBefore(
  layer: Konva.Layer,
  sign,
  board_x,
  board_y,
  rows_len,
  cols_len,
  finalWidth,
) {
  for (const item of sign) {
    let [k, ..._] = Object.keys(item);
    let v = item[k];
    if (k === "AroundSign") {
      //
    } else if (k === "BadgeSign") {
    } else if (k === "CacheSign") {
    } else if (k === "ColorSign") {
      for (const index of v.indexes) {
        let [x, y] = toIndex(index, rows_len, cols_len);
        const rect = new Konva.Rect({
          x: board_x + x * finalWidth,
          y: board_y + y * finalWidth,
          width: finalWidth,
          height: finalWidth,
          fill: convertColorFormat(v.color),
        });
        layer.add(rect);
      }
    } else if (k === "FigureSign") {
      //
    } else if (k === "GroundSign") {
      let [sx, sy] = toIndex(v.start, rows_len, cols_len);
      let [ex, ey] = toIndex(v.end, rows_len, cols_len);
      const rect = new Konva.Rect({
        x: board_x + sx * finalWidth,
        y: board_y + sy * finalWidth,
        width: (ex - sx + 1) * finalWidth,
        height: (ey - sy + 1) * finalWidth,
        fill: convertColorFormat(v.bg_color),
        stroke: convertColorFormat(v.ed_color),
        strokeWidth: v.size,
        dash: [10, 5],
      });
      layer.add(rect);
    } else if (k === "LineSign") {
    } else if (k === "PathSign") {
      //
    } else if (k === "TextSign") {
    } else if (k === "TitleSign") {
    }
  }
}

// 绘制 sign (在绘制棋子后面 / 前景)
//
export function drawSignAfter(
  layer: Konva.Layer,
  sign,
  board_x,
  board_y,
  rows_len,
  cols_len,
  finalWidth,
) {
  for (const item of sign) {
    let [k, ..._] = Object.keys(item);
    let v = item[k];
    if (k === "AroundSign") {
    } else if (k === "BadgeSign") {
      let [x, y] = toIndex(v.index, rows_len, cols_len);
      let left = board_x + x * finalWidth;
      let top = board_y + y * finalWidth;
      let right = left + finalWidth;
      let bottom = top + finalWidth;
      let radius = finalWidth / 7; // 圆形标记半径
      let textSize = finalWidth / 4; // 文字大小
      let cX, cY, tX, tY;
      let position = v.position;
      if (position == 0) {
        //左上角
        cX = left + radius;
        cY = top + radius;
      } else if (position == 1) {
        cX = left / 2 + right / 2;
        cY = top + radius;
      } else if (position == 2) {
        cX = right - radius;
        cY = top + radius;
      } else if (position == 3) {
        cX = left + radius;
        cY = top / 2 + bottom / 2;
      } else if (position == 4) {
        cX = left / 2 + right / 2;
        cY = top / 2 + bottom / 2;
      } else if (position == 5) {
        cX = right - radius;
        cY = top / 2 + bottom / 2;
      } else if (position == 6) {
        cX = left + radius;
        cY = bottom - radius;
      } else if (position == 7) {
        cX = left / 2 + right / 2;
        cY = bottom - radius;
      } else if (position == 8) {
        cX = right - radius;
        cY = bottom - radius;
      } else {
        //错误位置默认居中
        cX = left / 2 + right / 2;
        cY = top / 2 + bottom / 2;
      }
      tX = cX;
      tY = cY + textSize / 3;
      const circle = new Konva.Circle({
        x: cX,
        y: cY,
        radius: radius,
        fill: convertColorFormat(v.bg_color),
      });
      layer.add(circle);
      const text = new Konva.Text({
        x: cX - radius,
        y: cY - radius,
        width: radius * 2,
        height: radius * 2,
        text: v.value,
        fontSize: textSize,
        align: "center",
        verticalAlign: "middle",
        fontFamily: "微软雅黑",
      });
      layer.add(text);
    } else if (k === "CacheSign") {
    } else if (k === "ColorSign") {
    } else if (k === "FigureSign") {
    } else if (k === "GroundSign") {
    } else if (k === "LineSign") {
      //
    } else if (k === "PathSign") {
    } else if (k === "TextSign") {
      let [x, y] = toIndex(v.index, rows_len, cols_len);
      const text = new Konva.Text({
        x: board_x + x * finalWidth,
        y: board_y + y * finalWidth,
        width: finalWidth,
        height: finalWidth,
        text: v.value,
        fontSize: (finalWidth * 3) / 5,
        align: "center",
        verticalAlign: "middle",
        fontFamily: "微软雅黑",
      });
      layer.add(text);
    } else if (k === "TitleSign") {
      //
    }
  }
}

// 绘制棋子
export function drawPiece(
  layer: Konva.Layer,
  pieces,
  board_x,
  board_y,
  rows_len,
  cols_len,
  finalWidth,
) {
  for (let i = 0; i < cols_len; i++) {
    for (let j = 0; j < rows_len; j++) {
      let piece_mode = parseInt(pieces[i][j]);
      if (piece_mode === 1) {
        const circle = new Konva.Circle({
          x: board_x + j * finalWidth + finalWidth / 2,
          y: board_y + i * finalWidth + finalWidth / 2,
          radius: (finalWidth / 2) * 0.85,
          fill: "black",
          stroke: "black",
          strokeWidth: 1,
        });
        layer.add(circle);
      } else if (piece_mode === 2) {
        const circle = new Konva.Circle({
          x: board_x + j * finalWidth + finalWidth / 2,
          y: board_y + i * finalWidth + finalWidth / 2,
          radius: (finalWidth / 2) * 0.85,
          fill: "white",
          stroke: "black",
          strokeWidth: 1,
        });
        layer.add(circle);
      } else if (piece_mode === 3) {
        const circle = new Konva.Circle({
          x: board_x + j * finalWidth + finalWidth / 2,
          y: board_y + i * finalWidth + finalWidth / 2,
          radius: (finalWidth / 2) * 0.85,
          fill: "red",
          stroke: "black",
          strokeWidth: 1,
        });
        layer.add(circle);
      } else if (piece_mode === 4) {
        const circle = new Konva.Circle({
          x: board_x + j * finalWidth + finalWidth / 2,
          y: board_y + i * finalWidth + finalWidth / 2,
          radius: (finalWidth / 2) * 0.85,
          fill: "blue",
          stroke: "black",
          strokeWidth: 1,
        });
        layer.add(circle);
      } else if (piece_mode === 5) {
        const circle = new Konva.Circle({
          x: board_x + j * finalWidth + finalWidth / 2,
          y: board_y + i * finalWidth + finalWidth / 2,
          radius: (finalWidth / 2) * 0.85,
          fill: "yellow",
          stroke: "black",
          strokeWidth: 1,
        });
        layer.add(circle);
      } else if (piece_mode === 6) {
        const circle = new Konva.Circle({
          x: board_x + j * finalWidth + finalWidth / 2,
          y: board_y + i * finalWidth + finalWidth / 2,
          radius: (finalWidth / 2) * 0.85,
          fill: "green",
          stroke: "black",
          strokeWidth: 1,
        });
        layer.add(circle);
      } else if (piece_mode === 7) {
        const circle = new Konva.Circle({
          x: board_x + j * finalWidth + finalWidth / 2,
          y: board_y + i * finalWidth + finalWidth / 2,
          radius: (finalWidth / 2) * 0.85,
          fill: "gray",
          stroke: "black",
          strokeWidth: 1,
        });
        layer.add(circle);
      } else if (piece_mode === 8) {
        // 太极
        const shape = new Konva.Shape({
          x: board_x + j * finalWidth + finalWidth / 2,
          y: board_y + i * finalWidth + finalWidth / 2,
          radius: (finalWidth / 2) * 0.85,
          sceneFunc: function (context, shape) {
            // 绘制两个大半圆
            context.beginPath();
            context.arc(0, 0, shape.getAttr("radius"), 0, Math.PI * 2);
            context.closePath();
            context.fillStyle = "white";
            context.fill();
            context.strokeStyle = "black";
            context.lineWidth = 2;
            context.stroke();

            context.beginPath();
            context.arc(
              0,
              0,
              shape.getAttr("radius"),
              (Math.PI / 4) * 7,
              (Math.PI / 4) * 3,
            );
            context.closePath();
            context.fillStyle = "black";
            context.fill();

            // 绘制小半圆
            context.beginPath();
            context.arc(
              +Math.sqrt(
                (shape.getAttr("radius") *
                  0.5 *
                  shape.getAttr("radius") *
                  0.5) /
                  2,
              ) * 0.98,
              -Math.sqrt(
                (shape.getAttr("radius") *
                  0.5 *
                  shape.getAttr("radius") *
                  0.5) /
                  2,
              ) * 0.98,
              shape.getAttr("radius") * 0.5,

              0,
              Math.PI * 2,
            );
            context.closePath();
            context.fillStyle = "white";
            context.fill();

            context.beginPath();
            context.arc(
              -Math.sqrt(
                (shape.getAttr("radius") *
                  0.5 *
                  shape.getAttr("radius") *
                  0.5) /
                  2,
              ) * 0.98,
              +Math.sqrt(
                (shape.getAttr("radius") *
                  0.5 *
                  shape.getAttr("radius") *
                  0.5) /
                  2,
              ) * 0.98,
              shape.getAttr("radius") * 0.5,

              0,
              Math.PI * 2,
            );
            context.closePath();
            context.fillStyle = "black";
            context.fill();

            // 绘制两个中心点
            context.beginPath();
            context.arc(
              +Math.sqrt(
                (shape.getAttr("radius") *
                  0.5 *
                  shape.getAttr("radius") *
                  0.5) /
                  2,
              ) * 0.98,
              -Math.sqrt(
                (shape.getAttr("radius") *
                  0.5 *
                  shape.getAttr("radius") *
                  0.5) /
                  2,
              ) * 0.98,
              shape.getAttr("radius") * 0.15,

              0,
              Math.PI * 2,
            );
            context.closePath();
            context.fillStyle = "black";
            context.fill();

            context.beginPath();
            context.arc(
              -Math.sqrt(
                (shape.getAttr("radius") *
                  0.5 *
                  shape.getAttr("radius") *
                  0.5) /
                  2,
              ) * 0.98,
              +Math.sqrt(
                (shape.getAttr("radius") *
                  0.5 *
                  shape.getAttr("radius") *
                  0.5) /
                  2,
              ) * 0.98,
              shape.getAttr("radius") * 0.15,

              0,
              Math.PI * 2,
            );
            context.closePath();
            context.fillStyle = "white";
            context.fill();
          },
        });
        layer.add(shape);
      } else if (piece_mode === 9) {
        console.log(j, i);

        //绘制围墙
        const shape = new Konva.Shape({
          x: board_x + j * finalWidth,
          y: board_y + i * finalWidth,
          width: finalWidth,
          sceneFunc: function (context, shape) {
            context.beginPath();
            context.fillStyle = "orange";
            context.closePath();
            context.fillRect(
              0,
              0,
              shape.getAttr("width"),
              shape.getAttr("width"),
            );
            context.strokeStyle = "black";
            context.lineWidth = 2;
            context.strokeRect(
              0,
              0,
              shape.getAttr("width"),
              shape.getAttr("width"),
            );

            context.beginPath();
            context.fillStyle = "red";
            context.closePath();
            context.strokeStyle = "black";
            context.lineWidth = 2;
            context.fillRect(
              0,
              0,
              shape.getAttr("width") * 0.65,
              shape.getAttr("width") * 0.25,
            );
            context.strokeRect(
              0,
              0,
              shape.getAttr("width") * 0.65,
              shape.getAttr("width") * 0.25,
            );

            context.beginPath();
            context.fillStyle = "red";
            context.closePath();
            context.strokeStyle = "black";
            context.lineWidth = 2;
            context.fillRect(
              0 + shape.getAttr("width") * 0.35,
              0 + shape.getAttr("width") * 0.25,
              shape.getAttr("width") * 0.65,
              shape.getAttr("width") * 0.25,
            );
            context.strokeRect(
              0 + shape.getAttr("width") * 0.35,
              0 + shape.getAttr("width") * 0.25,
              shape.getAttr("width") * 0.65,
              shape.getAttr("width") * 0.25,
            );

            context.beginPath();
            context.fillStyle = "red";
            context.closePath();
            context.strokeStyle = "black";
            context.lineWidth = 2;
            context.fillRect(
              0,
              0 + shape.getAttr("width") * 0.5,
              shape.getAttr("width") * 0.65,
              shape.getAttr("width") * 0.25,
            );
            context.strokeRect(
              0,
              0 + shape.getAttr("width") * 0.5,
              shape.getAttr("width") * 0.65,
              shape.getAttr("width") * 0.25,
            );

            context.beginPath();
            context.fillStyle = "red";
            context.closePath();
            context.strokeStyle = "black";
            context.lineWidth = 2;
            context.fillRect(
              0 + shape.getAttr("width") * 0.35,
              0 + shape.getAttr("width") * 0.75,
              shape.getAttr("width") * 0.65,
              shape.getAttr("width") * 0.25,
            );
            context.strokeRect(
              0 + shape.getAttr("width") * 0.35,
              0 + shape.getAttr("width") * 0.75,
              shape.getAttr("width") * 0.65,
              shape.getAttr("width") * 0.25,
            );
          },
        });
        layer.add(shape);
      } else if (piece_mode === 11) {
        const rect = new Konva.Rect({
          x: board_x + j * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          y: board_y + i * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          width: finalWidth * 0.8,
          height: finalWidth * 0.8,
          fill: "black",
          stroke: "black",
          strokeWidth: 2,
        });
        layer.add(rect);
      } else if (piece_mode === 12) {
        const rect = new Konva.Rect({
          x: board_x + j * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          y: board_y + i * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          width: finalWidth * 0.8,
          height: finalWidth * 0.8,
          fill: "white",
          stroke: "black",
          strokeWidth: 2,
        });
        layer.add(rect);
      } else if (piece_mode === 13) {
        const rect = new Konva.Rect({
          x: board_x + j * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          y: board_y + i * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          width: finalWidth * 0.8,
          height: finalWidth * 0.8,
          fill: "red",
          stroke: "black",
          strokeWidth: 2,
        });
        layer.add(rect);
      } else if (piece_mode === 14) {
        const rect = new Konva.Rect({
          x: board_x + j * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          y: board_y + i * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          width: finalWidth * 0.8,
          height: finalWidth * 0.8,
          fill: "blue",
          stroke: "black",
          strokeWidth: 2,
        });
        layer.add(rect);
      } else if (piece_mode === 15) {
        const rect = new Konva.Rect({
          x: board_x + j * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          y: board_y + i * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          width: finalWidth * 0.8,
          height: finalWidth * 0.8,
          fill: "yellow",
          stroke: "black",
          strokeWidth: 2,
        });
        layer.add(rect);
      } else if (piece_mode === 16) {
        const rect = new Konva.Rect({
          x: board_x + j * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          y: board_y + i * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          width: finalWidth * 0.8,
          height: finalWidth * 0.8,
          fill: "green",
          stroke: "black",
          strokeWidth: 2,
        });
        layer.add(rect);
      } else if (piece_mode === 17) {
        const rect = new Konva.Rect({
          x: board_x + j * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          y: board_y + i * finalWidth + finalWidth / 2 - finalWidth * 0.8 * 0.5,
          width: finalWidth * 0.8,
          height: finalWidth * 0.8,
          fill: "gray",
          stroke: "black",
          strokeWidth: 2,
        });
        layer.add(rect);
      } else if (piece_mode === 21) {
        let _x = board_x + j * finalWidth;
        let _y = board_y + i * finalWidth;
        const polygon = new Konva.Line({
          points: [
            _x + finalWidth * 0.5,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.8,
            _y + finalWidth * 0.3,
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.2,
            _y + finalWidth * 0.3,
          ],
          fill: "#f6c9a5",
          stroke: "black",
          strokeWidth: 2,

          closed: true,
        });

        layer.add(polygon);
      } else if (piece_mode === 22) {
        let _x = board_x + j * finalWidth;
        let _y = board_y + i * finalWidth;
        const polygon = new Konva.Line({
          points: [
            _x + finalWidth * 0.5,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.8,
            _y + finalWidth * 0.7,
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.2,
            _y + finalWidth * 0.7,
          ],
          fill: "#f6c9a5",
          stroke: "black",
          strokeWidth: 2,

          closed: true,
        });

        layer.add(polygon);
      } else if (piece_mode === 23) {
        let _x = board_x + j * finalWidth;
        let _y = board_y + i * finalWidth;
        const polygon = new Konva.Line({
          points: [
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.5,
            _x + finalWidth * 0.3,
            _y + finalWidth * 0.8,
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.3,
            _y + finalWidth * 0.2,
          ],
          fill: "#f6c9a5",
          stroke: "black",
          strokeWidth: 2,

          closed: true,
        });

        layer.add(polygon);
      } else if (piece_mode === 24) {
        let _x = board_x + j * finalWidth;
        let _y = board_y + i * finalWidth;
        const polygon = new Konva.Line({
          points: [
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.5,
            _x + finalWidth * 0.7,
            _y + finalWidth * 0.8,
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.7,
            _y + finalWidth * 0.2,
          ],
          fill: "#f6c9a5",
          stroke: "black",
          strokeWidth: 2,

          closed: true,
        });

        layer.add(polygon);
      } else if (piece_mode === 31) {
        let _x = board_x + j * finalWidth;
        let _y = board_y + i * finalWidth;
        const polygon = new Konva.Line({
          points: [
            _x + finalWidth * 0.5,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.8,
            _y + finalWidth * 0.3,
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.2,
            _y + finalWidth * 0.3,
          ],
          fill: "#f6c9a5",
          stroke: "#c7edcc",
          strokeWidth: 2,

          closed: true,
        });

        layer.add(polygon);
      } else if (piece_mode === 32) {
        let _x = board_x + j * finalWidth;
        let _y = board_y + i * finalWidth;
        const polygon = new Konva.Line({
          points: [
            _x + finalWidth * 0.5,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.8,
            _y + finalWidth * 0.7,
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.2,
            _y + finalWidth * 0.7,
          ],
          fill: "#f6c9a5",
          stroke: "#c7edcc",
          strokeWidth: 2,

          closed: true,
        });

        layer.add(polygon);
      } else if (piece_mode === 33) {
        let _x = board_x + j * finalWidth;
        let _y = board_y + i * finalWidth;
        const polygon = new Konva.Line({
          points: [
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.5,
            _x + finalWidth * 0.3,
            _y + finalWidth * 0.8,
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.3,
            _y + finalWidth * 0.2,
          ],
          fill: "#f6c9a5",
          stroke: "#c7edcc",
          strokeWidth: 2,

          closed: true,
        });

        layer.add(polygon);
      } else if (piece_mode === 34) {
        let _x = board_x + j * finalWidth;
        let _y = board_y + i * finalWidth;
        const polygon = new Konva.Line({
          points: [
            _x + finalWidth * 0.88,
            _y + finalWidth * 0.5,
            _x + finalWidth * 0.7,
            _y + finalWidth * 0.8,
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.88,
            _x + finalWidth * 0.12,
            _y + finalWidth * 0.12,
            _x + finalWidth * 0.7,
            _y + finalWidth * 0.2,
          ],
          fill: "#f6c9a5",
          stroke: "#c7edcc",
          strokeWidth: 2,

          closed: true,
        });

        layer.add(polygon);
      }
    }
  }
}
