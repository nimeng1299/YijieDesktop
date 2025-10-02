use anyhow::Result;

pub trait Sign: Sized {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self>;
}
