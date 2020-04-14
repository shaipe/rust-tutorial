
// #[macro_use]
extern crate serde_json;
use serde_json::Value;

fn main(){
    let js_str = "{\"component_access_token\":\"32_-x8YDnSGPmlUESM_3TywoUdy3CPSW07_MEdt1SuS8DMpMDvdHxCbkwwP1D4IVezUbbqzvdS5o6_RHNq9ar_lIISxaE-N8O1iCwPWdpx3a-EDNSjRjcBt0BjxyOpys_1FyzXCckzkE0CZsM6CZUHhAJAQLY\",\"expires_in\":7200}";
    // 字符串不能直接使用json!的方式进行序列化
    match serde_json::from_str(js_str) {
        Ok(v) =>{
            let v: Value = v;
            println!("{:?}", v["component_access_token"]);
        },
        Err(e) => {println!("{:?}", e);}
    };
    
}