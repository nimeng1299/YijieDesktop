import Konva from "konva";
import type { Ring } from "konva/lib/shapes/Ring";
import { union, type Polygon } from "martinez-polygon-clipping";

function numberToChineseObj(num: number): string {
  const numberMap: { [key: number]: string } = {
    1: "一",
    2: "二",
    3: "三",
    4: "四",
    5: "五",
    6: "六",
    7: "七",
    8: "八",
    9: "九",
    10: "十",
  };

  // 检查数字是否在映射中
  if (numberMap.hasOwnProperty(num)) {
    return numberMap[num];
  } else {
    return ""; // 或返回默认值/抛错
  }
}

// 创建椭圆圆弧
function createEllipseArc(
  left,
  top,
  right,
  bottom,
  startAngle, // 单位：°
  angle, // 扫过多少角度 单位：°
  color = "black",
  strokeWidth = 2,
) {
  const width = right - left;
  const height = bottom - top;
  const centerX = left + width / 2;
  const centerY = top + height / 2;
  const rx = width / 2;
  const ry = height / 2;

  const ellipseArc = new Konva.Shape({
    x: centerX,
    y: centerY,
    stroke: color,
    strokeWidth,
    sceneFunc: (context, shape) => {
      context.beginPath();
      // 绘制椭圆弧
      context.ellipse(
        0,
        0,
        rx,
        ry,
        0,
        (startAngle * Math.PI) / 180,
        (startAngle + angle * Math.PI) / 180,
      );
      context.strokeShape(shape);
    },
  });

  return ellipseArc;
}

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
    return numberToChineseObj(i + 1);
  } else if (mode === "6") {
    return numberToChineseObj(len - i);
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
  // x, y 加 1 为了防止 0 的线画不出来
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
        x: i * finalWidth + 1,
        y: 1,
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
        x: i * finalWidth + 1,
        y: (row_count + rows_len) * finalWidth + 1,
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
        x: 0 + 1,
        y: i * finalWidth + 1,
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
        x: (col_count + cols_len) * finalWidth + 1,
        y: i * finalWidth + 1,
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
      let unionPoly: any = null;
      v.index.map((index) => {
        let [col, row] = toIndex(index, rows_len, cols_len);
        const x = board_x + col * finalWidth;
        const y = board_y + row * finalWidth;
        const square = [
          [
            [x, y],
            [x + finalWidth, y],
            [x + finalWidth, y + finalWidth],
            [x, y + finalWidth],
            [x, y], // 闭合
          ],
        ];
        unionPoly = unionPoly ? union(unionPoly, square) : square;
        const rect = new Konva.Rect({
          x: x,
          y: y,
          width: finalWidth,
          height: finalWidth,
          fill: convertColorFormat(v.bg_color),
        });
        layer.add(rect);
      });
      if (!unionPoly) return;
      const Ring = unionPoly[0];
      Ring.map((outerRing) => {
        const points = outerRing.flat();
        const polygon = new Konva.Line({
          points: points,
          // fill: convertColorFormat(v.bg_color),
          stroke: convertColorFormat(v.ed_color),
          strokeWidth: v.size * 2, // 基础线宽为2，乘以缩放因子
          cornerRadius: finalWidth / 3,
          opacity: 0.7,
          closed: true, // 关键：闭合多边形
        });

        layer.add(polygon);
      });
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
      for (const index of v.tiles) {
        let [sx, sy] = toIndex(index[0], rows_len, cols_len);
        let [ex, ey] = toIndex(index[1], rows_len, cols_len);
        let color = convertColorFormat(v.color);
        // 矩形包围框
        let left = board_x + sx * finalWidth;
        let top = board_y + sy * finalWidth;
        let right = board_x + ex * finalWidth + finalWidth;
        let bottom = board_y + ey * finalWidth + finalWidth;

        let mid_x = (left + right) / 2;
        let mid_y = (top + bottom) / 2;

        let half_x = (right - left) / 2;
        let half_y = (bottom - top) / 2;

        // drawMidSideLine
        if (v.mid_side_line.length === 8) {
          if (v.mid_side_line[0] === "1") {
            const line = new Konva.Line({
              points: [left, top, left, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_side_line[1] === "1") {
            const line = new Konva.Line({
              points: [left, top, mid_x, top],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_side_line[2] === "1") {
            const line = new Konva.Line({
              points: [right, top, mid_x, top],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_side_line[3] === "1") {
            const line = new Konva.Line({
              points: [right, top, right, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_side_line[4] === "1") {
            const line = new Konva.Line({
              points: [right, bottom, mid_x, bottom],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_side_line[5] === "1") {
            const line = new Konva.Line({
              points: [right, bottom, mid_x, bottom],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_side_line[6] === "1") {
            const line = new Konva.Line({
              points: [left, bottom, mid_x, bottom],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_side_line[7] === "1") {
            const line = new Konva.Line({
              points: [left, bottom, left, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
        }

        // drawMidCenLine
        if (v.mid_cen_line.length === 4) {
          if (v.mid_cen_line[0] === "1") {
            const line = new Konva.Line({
              points: [mid_x, top, mid_x, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_cen_line[1] === "1") {
            const line = new Konva.Line({
              points: [right, mid_y, mid_x, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_cen_line[2] === "1") {
            const line = new Konva.Line({
              points: [mid_x, bottom, mid_x, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.mid_cen_line[3] === "1") {
            const line = new Konva.Line({
              points: [left, mid_y, mid_x, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
        }

        // drawCorCenLine
        if (v.cor_cen_line.length === 4) {
          if (v.cor_cen_line[0] === "1") {
            const line = new Konva.Line({
              points: [left, top, mid_x, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_cen_line[1] === "1") {
            const line = new Konva.Line({
              points: [right, top, mid_x, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_cen_line[2] === "1") {
            const line = new Konva.Line({
              points: [right, bottom, mid_x, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_cen_line[3] === "1") {
            const line = new Konva.Line({
              points: [left, bottom, mid_x, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
        }

        // drawAdjMidLine
        if (v.adj_mid_line.length === 4) {
          if (v.adj_mid_line[0] === "1") {
            const line = new Konva.Line({
              points: [left, mid_y, mid_x, top],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.adj_mid_line[1] === "1") {
            const line = new Konva.Line({
              points: [mid_x, top, right, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.adj_mid_line[2] === "1") {
            const line = new Konva.Line({
              points: [right, mid_y, mid_x, bottom],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.adj_mid_line[3] === "1") {
            const line = new Konva.Line({
              points: [mid_x, bottom, left, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
        }

        // drawCorMidLine
        if (v.cor_mid_line.length === 8) {
          if (v.cor_mid_line[0] === "1") {
            const line = new Konva.Line({
              points: [left, top, mid_x, bottom],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_mid_line[1] === "1") {
            const line = new Konva.Line({
              points: [left, top, right, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_mid_line[2] === "1") {
            const line = new Konva.Line({
              points: [right, top, left, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_mid_line[3] === "1") {
            const line = new Konva.Line({
              points: [right, top, mid_x, bottom],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_mid_line[4] === "1") {
            const line = new Konva.Line({
              points: [right, bottom, mid_x, top],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_mid_line[5] === "1") {
            const line = new Konva.Line({
              points: [right, bottom, left, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_mid_line[6] === "1") {
            const line = new Konva.Line({
              points: [left, bottom, right, mid_y],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
          if (v.cor_mid_line[7] === "1") {
            const line = new Konva.Line({
              points: [left, bottom, mid_x, top],
              stroke: color,
              strokeWidth: 2,
              lineCap: "round",
              lineJoin: "round",
            });
            layer.add(line);
          }
        }

        // drawCompCenArc
        if (v.comp_cen_arc.length === 4) {
          if (v.comp_cen_arc[0] === "1") {
            let arc = createEllipseArc(
              left,
              top,
              right,
              bottom,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.comp_cen_arc[1] === "1") {
            let arc = createEllipseArc(
              left,
              top,
              right,
              bottom,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.comp_cen_arc[2] === "1") {
            let arc = createEllipseArc(left, top, right, bottom, 0, 90, color);
            layer.add(arc);
          }
          if (v.comp_cen_arc[3] === "1") {
            let arc = createEllipseArc(left, top, right, bottom, 90, 90, color);
            layer.add(arc);
          }
        }

        // drawCenExpArc
        if (v.cen_exp_arc.length === 8) {
          if (v.cen_exp_arc[0] === "1") {
            let arc = createEllipseArc(
              left,
              top - half_x,
              right,
              top + half_y,
              90,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cen_exp_arc[1] === "1") {
            let arc = createEllipseArc(
              left - half_x,
              top,
              mid_x,
              bottom,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cen_exp_arc[2] === "1") {
            let arc = createEllipseArc(
              mid_x,
              top,
              right + half_x,
              bottom,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cen_exp_arc[3] === "1") {
            let arc = createEllipseArc(
              left,
              top - half_y,
              right,
              top + half_y,
              0,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cen_exp_arc[4] === "1") {
            let arc = createEllipseArc(
              left,
              mid_y,
              right,
              bottom + half_y,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cen_exp_arc[5] === "1") {
            let arc = createEllipseArc(
              mid_x,
              top,
              right + half_x,
              bottom,
              90,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cen_exp_arc[6] === "1") {
            let arc = createEllipseArc(
              left - half_x,
              top,
              mid_x,
              bottom,
              0,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cen_exp_arc[7] === "1") {
            let arc = createEllipseArc(
              left,
              mid_y,
              right,
              bottom + half_y,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
        }

        // drawCorExpArc
        if (v.cor_exp_arc.length === 4) {
          if (v.cor_exp_arc[0] === "1") {
            let arc = createEllipseArc(
              left - half_x,
              top - half_y,
              mid_x,
              mid_y,
              0,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_exp_arc[1] === "1") {
            let arc = createEllipseArc(
              mid_x,
              top - half_y,
              right + half_x,
              mid_y,
              90,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_exp_arc[2] === "1") {
            let arc = createEllipseArc(
              mid_x,
              mid_y,
              right + half_x,
              bottom + half_y,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_exp_arc[3] === "1") {
            let arc = createEllipseArc(
              left - half_x,
              mid_y,
              mid_x,
              bottom + half_y,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
        }

        // drawCorCenArc
        if (v.cor_cen_arc.length === 4) {
          let width = right - left;
          let height = bottom - top;
          if (v.cor_cen_arc[0] === "1") {
            let arc = createEllipseArc(
              left - width,
              top - height,
              right,
              bottom,
              0,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_cen_arc[1] === "1") {
            let arc = createEllipseArc(
              left,
              top - height,
              right + width,
              bottom,
              90,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_cen_arc[2] === "1") {
            let arc = createEllipseArc(
              left,
              top,
              right + width,
              bottom + height,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_cen_arc[3] === "1") {
            let arc = createEllipseArc(
              left - width,
              top,
              right,
              bottom + height,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
        }

        // drawSideExpArc
        if (v.side_exp_arc.length === 8) {
          let width = right - left;
          let height = bottom - top;
          if (v.side_exp_arc[0] === "1") {
            let arc = createEllipseArc(
              left - half_x,
              top,
              mid_x,
              bottom + height,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.side_exp_arc[1] === "1") {
            let arc = createEllipseArc(
              left,
              top - half_y,
              right + width,
              mid_y,
              90,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.side_exp_arc[2] === "1") {
            let arc = createEllipseArc(
              left - width,
              top - half_y,
              right,
              mid_y,
              0,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.side_exp_arc[3] === "1") {
            let arc = createEllipseArc(
              mid_x,
              top,
              right + half_x,
              bottom + height,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.side_exp_arc[4] === "1") {
            let arc = createEllipseArc(
              mid_x,
              top - height,
              right + half_x,
              bottom,
              90,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.side_exp_arc[5] === "1") {
            let arc = createEllipseArc(
              left - width,
              mid_y,
              right,
              bottom + half_y,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.side_exp_arc[6] === "1") {
            let arc = createEllipseArc(
              left,
              mid_y,
              right + width,
              bottom + half_y,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.side_exp_arc[7] === "1") {
            let arc = createEllipseArc(
              left - half_x,
              top - height,
              mid_x,
              bottom,
              0,
              90,
              color,
            );
            layer.add(arc);
          }
        }

        // drawCorMidArc
        if (v.cor_mid_arc.length === 8) {
          let width = right - left;
          let height = bottom - top;
          if (v.cor_mid_arc[0] === "1") {
            let arc = createEllipseArc(
              left,
              top - height,
              right,
              top + height,
              90,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_mid_arc[1] === "1") {
            let arc = createEllipseArc(
              left - width,
              top,
              right,
              bottom,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_mid_arc[2] === "1") {
            let arc = createEllipseArc(
              left,
              top,
              right + width,
              bottom,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_mid_arc[3] === "1") {
            let arc = createEllipseArc(
              left,
              top - height,
              right,
              top + height,
              0,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_mid_arc[4] === "1") {
            let arc = createEllipseArc(
              left,
              top,
              right,
              bottom + height,
              270,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_mid_arc[5] === "1") {
            let arc = createEllipseArc(
              left,
              top,
              right + width,
              bottom,
              90,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_mid_arc[6] === "1") {
            let arc = createEllipseArc(
              left - width,
              top,
              right,
              bottom,
              0,
              90,
              color,
            );
            layer.add(arc);
          }
          if (v.cor_mid_arc[7] === "1") {
            let arc = createEllipseArc(
              left,
              top,
              right,
              bottom + height,
              180,
              90,
              color,
            );
            layer.add(arc);
          }
        }

        // drawCompPoint
        if (v.comp_point.length === 9) {
          if (v.comp_point[0] === "1") {
            const circle = new Konva.Circle({
              x: left,
              y: top,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[1] === "1") {
            const circle = new Konva.Circle({
              x: mid_x,
              y: top,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[2] === "1") {
            const circle = new Konva.Circle({
              x: right,
              y: top,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[3] === "1") {
            const circle = new Konva.Circle({
              x: right,
              y: mid_y,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[4] === "1") {
            const circle = new Konva.Circle({
              x: right,
              y: bottom,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[5] === "1") {
            const circle = new Konva.Circle({
              x: mid_x,
              y: bottom,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[6] === "1") {
            const circle = new Konva.Circle({
              x: left,
              y: bottom,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[7] === "1") {
            const circle = new Konva.Circle({
              x: left,
              y: mid_y,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[8] === "1") {
            const circle = new Konva.Circle({
              x: mid_x,
              y: mid_y,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
          if (v.comp_point[0] === "1") {
            const circle = new Konva.Circle({
              x: left,
              y: top,
              radius: 2,
              fill: color,
            });
            layer.add(circle);
          }
        }

        // drawCompAngle
        if (v.comp_angle.length === 8) {
          let space = 3;
          let len = space * 2;
          if (v.comp_angle[0] === "1") {
            let startX = left + space;
            let startY = top + space;
            const line1 = new Konva.Line({
              points: [startX, startY, startX, startY + len],
              stroke: color,
              strokeWidth: 1,
              lineCap: "round",
              lineJoin: "round",
            });
            const line2 = new Konva.Line({
              points: [startX, startY, startX + len, startY],
              stroke: color,
              strokeWidth: 1,
              lineJoin: "round",
              lineCap: "round",
            });
            layer.add(line1, line2);
          }
          if (v.comp_angle[1] === "1") {
            let startX = right - space;
            let startY = top + space;
            const line1 = new Konva.Line({
              points: [startX, startY, startX, startY + len],
              stroke: color,
              strokeWidth: 1,
              lineCap: "round",
              lineJoin: "round",
            });
            const line2 = new Konva.Line({
              points: [startX, startY, startX - len, startY],
              stroke: color,
              strokeWidth: 1,
              lineJoin: "round",
              lineCap: "round",
            });
            layer.add(line1, line2);
          }
          if (v.comp_angle[2] === "1") {
            let startX = left - space;
            let startY = bottom - space;
            const line1 = new Konva.Line({
              points: [startX, startY, startX, startY - len],
              stroke: color,
              strokeWidth: 1,
              lineCap: "round",
              lineJoin: "round",
            });
            const line2 = new Konva.Line({
              points: [startX, startY, startX - len, startY],
              stroke: color,
              strokeWidth: 1,
              lineJoin: "round",
              lineCap: "round",
            });
            layer.add(line1, line2);
          }
          if (v.comp_angle[3] === "1") {
            let startX = left + space;
            let startY = bottom - space;
            const line1 = new Konva.Line({
              points: [startX, startY, startX, startY - len],
              stroke: color,
              strokeWidth: 1,
              lineCap: "round",
              lineJoin: "round",
            });
            const line2 = new Konva.Line({
              points: [startX, startY, startX + len, startY],
              stroke: color,
              strokeWidth: 1,
              lineJoin: "round",
              lineCap: "round",
            });
            layer.add(line1, line2);
          }
          if (v.comp_angle[4] === "1") {
            let startX = mid_x - space;
            let startY = mid_y - space;
            const line1 = new Konva.Line({
              points: [startX, startY, startX, startY - len],
              stroke: color,
              strokeWidth: 1,
              lineCap: "round",
              lineJoin: "round",
            });
            const line2 = new Konva.Line({
              points: [startX, startY, startX - len, startY],
              stroke: color,
              strokeWidth: 1,
              lineJoin: "round",
              lineCap: "round",
            });
            layer.add(line1, line2);
          }
          if (v.comp_angle[5] === "1") {
            let startX = mid_x + space;
            let startY = mid_y - space;
            const line1 = new Konva.Line({
              points: [startX, startY, startX, startY - len],
              stroke: color,
              strokeWidth: 1,
              lineCap: "round",
              lineJoin: "round",
            });
            const line2 = new Konva.Line({
              points: [startX, startY, startX + len, startY],
              stroke: color,
              strokeWidth: 1,
              lineJoin: "round",
              lineCap: "round",
            });
            layer.add(line1, line2);
          }
          if (v.comp_angle[6] === "1") {
            let startX = mid_x + space;
            let startY = mid_y + space;
            const line1 = new Konva.Line({
              points: [startX, startY, startX, startY + len],
              stroke: color,
              strokeWidth: 1,
              lineCap: "round",
              lineJoin: "round",
            });
            const line2 = new Konva.Line({
              points: [startX, startY, startX + len, startY],
              stroke: color,
              strokeWidth: 1,
              lineJoin: "round",
              lineCap: "round",
            });
            layer.add(line1, line2);
          }
          if (v.comp_angle[7] === "1") {
            let startX = mid_x - space;
            let startY = mid_y + space;
            const line1 = new Konva.Line({
              points: [startX, startY, startX, startY + len],
              stroke: color,
              strokeWidth: 1,
              lineCap: "round",
              lineJoin: "round",
            });
            const line2 = new Konva.Line({
              points: [startX, startY, startX - len, startY],
              stroke: color,
              strokeWidth: 1,
              lineJoin: "round",
              lineCap: "round",
            });
            layer.add(line1, line2);
          }
        }
      }
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
      let p = [];
      // pos 为 [a , b] a为格子坐标，b为以这个格子为锚点自身的9个位置
      for (const pos of v.node) {
        let [x, y] = toIndex(pos[0], rows_len, cols_len);
        const left = board_x + x * finalWidth;
        const top = board_y + y * finalWidth;
        if (pos[1] === 0) {
          p.push(left, top);
        } else if (pos[1] === 1) {
          p.push(left + finalWidth / 2, top);
        } else if (pos[1] === 2) {
          p.push(left + finalWidth, top);
        } else if (pos[1] === 3) {
          p.push(left, top + finalWidth / 2);
        } else if (pos[1] === 4) {
          p.push(left + finalWidth / 2, top + finalWidth / 2);
        } else if (pos[1] === 5) {
          p.push(left + finalWidth, top + finalWidth / 2);
        } else if (pos[1] === 6) {
          p.push(left, top + finalWidth);
        } else if (pos[1] === 7) {
          p.push(left + finalWidth / 2, top + finalWidth);
        } else if (pos[1] === 8) {
          p.push(left + finalWidth, top + finalWidth);
        }
      }

      const polygon = new Konva.Line({
        points: p,
        fill: convertColorFormat(v.gr_color),
        stroke: convertColorFormat(v.line_color),
        strokeWidth: v.size,
        closed: true,
      });
      if (v.style === 1) {
        polygon.setAttr("dash", [5, 3]);
      }

      layer.add(polygon);
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
      let [start_x, start_y] = toIndex(v.start, rows_len, cols_len);
      let [end_x, end_y] = toIndex(v.end, rows_len, cols_len);
      let color = convertColorFormat(v.color);

      let sx = board_x + start_x * finalWidth + finalWidth / 2;
      let sy = board_x + start_y * finalWidth + finalWidth / 2;
      let ex = board_x + end_x * finalWidth + finalWidth / 2;
      let ey = board_x + end_y * finalWidth + finalWidth / 2;

      const line = new Konva.Line({
        points: [sx, sy, ex, ey],
        stroke: color,
        strokeWidth: 2,
        lineCap: "round",
        lineJoin: "round",
      });
      layer.add(line);

      const drawTriangle = (
        fromX: number,
        fromY: number,
        toX: number,
        toY: number,
        height: number,
        bottom: number,
      ) => {
        const juli = Math.sqrt((toX - fromX) ** 2 + (toY - fromY) ** 2);
        const juliX = toX - fromX;
        const juliY = toY - fromY;

        const dianX = toX - (height / juli) * juliX;
        const dianY = toY - (height / juli) * juliY;

        // 箭头的三个顶点
        const points = [
          toX,
          toY,
          dianX + (bottom / juli) * juliY,
          dianY - (bottom / juli) * juliX,
          dianX - (bottom / juli) * juliY,
          dianY + (bottom / juli) * juliX,
        ];

        const triangle = new Konva.Line({
          points,
          fill: color,
          closed: true,
          stroke: color,
          strokeWidth: 1,
        });

        layer.add(triangle);
      };

      if (v.style === "1") {
        drawTriangle(sx, sy, ex, ey, finalWidth / 4, finalWidth / 6);
      } else if (v.style === "2") {
        drawTriangle(sx, sy, ex, ey, finalWidth / 4, finalWidth / 6);
        drawTriangle(ex, ey, sx, sy, finalWidth / 4, finalWidth / 6);
      }
    } else if (k === "PathSign") {
    } else if (k === "TextSign") {
      let [x, y] = toIndex(v.index, rows_len, cols_len);
      const text = new Konva.Text({
        x: board_x + x * finalWidth,
        y: board_y + y * finalWidth,
        width: finalWidth,
        height: finalWidth,
        fill: convertColorFormat(v.color),
        text: v.value,
        fontSize: (finalWidth * 3) / 5,
        align: "center",
        verticalAlign: "middle",
        fontFamily: "微软雅黑",
      });
      layer.add(text);
    } else if (k === "TitleSign") {
      let targetX = board_x + finalWidth * cols_len * v.position_x - 1; //-1 看起来更能居中
      let targetY = board_y + finalWidth * rows_len * v.position_y;

      if (v.direction === 0) {
        //横向居中
        let text = new Konva.Text({
          x: targetX,
          y: targetY,
          text: v.title,
          height: finalWidth,
          fontSize: finalWidth * v.size,
          align: "center",
          verticalAlign: "middle",
          fontFamily: "微软雅黑",
          fill: convertColorFormat(v.color),
        });
        text = text.setAttr("x", targetX - text.width() / 2);
        text = text.setAttr("y", targetY - text.height() / 2);
        layer.add(text);
      } else if (v.direction === 1) {
        //纵向居中
        const characters = [...v.title];
        //示例文字，用于计算宽高
        let simple = new Konva.Text({
          x: 0,
          y: 0,
          text: "坤",
          height: finalWidth,
          fontSize: finalWidth * v.size,
          align: "center",
          verticalAlign: "middle",
          fontFamily: "微软雅黑",
          fill: convertColorFormat(v.color),
        });
        characters.forEach((char, index) => {
          const charText = new Konva.Text({
            x: targetX - simple.width() / 2,
            y: targetY + index * simple.height() - simple.height() / 2,
            width: simple.width(),
            height: simple.height(),
            text: char,
            fontSize: finalWidth * v.size,
            fontFamily: "微软雅黑",
            verticalAlign: "middle",
            fill: convertColorFormat(v.color),
          });

          layer.add(charText);
        });
      } else if (v.direction === 2) {
        //向右延申
        let text = new Konva.Text({
          x: targetX,
          y: targetY,
          text: v.title,
          height: finalWidth,
          fontSize: finalWidth * v.size,
          align: "center",
          verticalAlign: "middle",
          fontFamily: "微软雅黑",
          fill: convertColorFormat(v.color),
        });
        text = text.setAttr("y", targetY - text.height() / 2);
        layer.add(text);
      } else if (v.direction === 3) {
        //向下延申
        const characters = [...v.title];
        //示例文字，用于计算宽高
        let simple = new Konva.Text({
          x: 0,
          y: 0,
          text: "坤",
          height: finalWidth,
          fontSize: finalWidth * v.size,
          align: "center",
          verticalAlign: "middle",
          fontFamily: "微软雅黑",
          fill: convertColorFormat(v.color),
        });
        characters.forEach((char, index) => {
          const charText = new Konva.Text({
            x: targetX - simple.width() / 2,
            y: targetY + index * simple.height(),
            width: simple.width(),
            height: simple.height(),
            text: char,
            fontSize: finalWidth * v.size,
            fontFamily: "微软雅黑",
            verticalAlign: "middle",
            fill: convertColorFormat(v.color),
          });

          layer.add(charText);
        });
      }
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
  for (let i = 0; i < rows_len; i++) {
    for (let j = 0; j < cols_len; j++) {
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
