use regex::Regex;

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn aarrggbb_to_rrggbbaa(color: String) -> String {
    if color.len() != 9 || !color.starts_with('#') {
        return color; // 格式不对
    }
    let a = &color[1..3];   // aa
    let rgb = &color[3..];  // rrggbb
    format!("#{}{}", rgb, a)// 拼成 #rrggbbaa
}


///
/// 把字符串中 `{{...}}` 替换为 HTML。
/// 如果匹配中的任一属性名出现在 `ignore_keys` 中，则跳过该标记（保持原样）
///
/// 示例:
/// ```
/// let input = "操作 {{失败@color=red}}，请联系{{管理员@click=info}}。";
/// let ignore = vec!["click"];
/// assert_eq!(
///     rich_to_html(input, &ignore),
///     "操作 <span style='color:red'>失败</span>，请联系{{管理员@click=info}}。"
/// );
/// ```
pub fn rich_to_html(input: &str, ignore_keys: &[&str]) -> String {
    let re = Regex::new(r"\{\{\s*(?P<inner>.+?)\s*\}\}").unwrap();

    re.replace_all(input, |caps: &regex::Captures| {
        let inner = caps.name("inner").unwrap().as_str();

        // tokens 用 @ 分割，第一个 token 为显示文本（或 id）
        let mut parts: Vec<&str> = inner.split('@').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            // 空标记则原样返回
            return caps.get(0).unwrap().as_str().to_string();
        }

        let text = parts.remove(0);
        let mut styles: Vec<String> = Vec::new();
        let mut data_attrs: Vec<(String, String)> = Vec::new();

        for token in parts {
            // 解析 key=value 或 flag
            if let Some(eq_pos) = token.find('=') {
                let key = token[..eq_pos].trim();
                let value = token[eq_pos+1..].trim();

                // 如果是被忽略的属性，跳过对它的任何处理（不加入 style/data 等）
                if ignore_keys.iter().any(|k| *k == key) {
                    continue;
                }

                match key {
                    "color" => {
                        styles.push(format!("color:{}", aarrggbb_to_rrggbbaa(escape_html(value))));
                    }
                    "scale" => {
                        styles.push(format!("font-size:{}em", value));
                    }
                    "style" => {
                        match value {
                            "b" | "bold" => styles.push("font-weight:bold".to_string()),
                            "i" | "italic" => styles.push("font-style:italic".to_string()),
                            "u" | "ul" | "underline" => styles.push("text-decoration:underline".to_string()),
                            other => {
                                // 直接把值当作额外 CSS 片段（例如 "font-size:14px"）
                                styles.push(escape_html(other));
                            }
                        }
                    }
                    // 其它属性转换为 data-xxx，便于前端使用；如果也想忽略这些，加入 ignore_keys 即可
                    other => {
                        data_attrs.push((other.to_string(), value.to_string()));
                    }
                }
            } else {
                // 无等号的 flag，如 @b @u 等
                let key = token;
                if ignore_keys.iter().any(|k| *k == key) {
                    continue;
                }
                match key {
                    "b" | "bold" => styles.push("font-weight:bold".to_string()),
                    "i" | "italic" => styles.push("font-style:italic".to_string()),
                    "u" | "underline" => styles.push("text-decoration:underline".to_string()),
                    _ => {
                        // 非识别 flag，视为 data-flag="true"
                        data_attrs.push((key.to_string(), "true".to_string()));
                    }
                }
            }
        }

        // 组合 style 和 data 属性（忽略属性已被跳过）
        let style_attr = if !styles.is_empty() {
            format!(" style='{}'", styles.join(";"))
        } else {
            String::new()
        };

        let data_attr_str = if !data_attrs.is_empty() {
            data_attrs.into_iter()
                .map(|(k, v)| format!(" data-{}='{}'", escape_html(&k), escape_html(&v)))
                .collect::<Vec<_>>()
                .join("")
        } else {
            String::new()
        };

        format!(
            "<span{}{}>{}</span>",
            style_attr,
            data_attr_str,
            escape_html(text)
        )
    })
    .into_owned()
}
