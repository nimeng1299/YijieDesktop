use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board{
    pieces: Vec<Vec<i32>>,
    rows_len: usize,
    cols_len: usize,
}

impl Board{
    pub fn to_str(str: String) -> Self{
        let mut pieces = Vec::new();
        let rows:Vec<&str> = str.split("r").collect();
        let rows_len = rows.len();
        let mut cols_len = 0;
        for row in rows{
            let cols:Vec<&str> = row.split("c").collect();
            cols_len = cols.len();
            let mut r_pieces = Vec::new();
            for col in cols{
                    let piece = col.parse::<i32>().unwrap_or(0);
                    r_pieces.push(piece);
            }
            pieces.push(r_pieces);
        }
        Self{
            pieces,
            rows_len,
            cols_len,
        }
    }
}