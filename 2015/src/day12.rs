// https://adventofcode.com/2015/day/12

use serde_json::Value;

type RejectFilter = fn(v: &Value) -> bool;

fn json_sum(v: &Value, reject: RejectFilter) -> i64 {
    let json_sum2 = |v| json_sum(v, reject);

    if reject(v) {
        return 0;
    }

    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(vec) => vec.iter().map(json_sum2).sum(),
        Value::Object(map) => map.values().map(json_sum2).sum(),
        _ => 0,
    }
}

fn has_red_val(v: &Value) -> bool {
    v.as_object().map_or(false, |map| {
        map.values()
            .any(|e| e.as_str().map_or(false, |s| s == "red"))
    })
}

pub fn day12(filename: Option<&str>) -> (String, String) {
    let v: Value = serde_json::from_reader(crate::bufread(filename.unwrap_or("input/12"))).unwrap();
    (
        json_sum(&v, |_| false).to_string(),
        json_sum(&v, has_red_val).to_string(),
    )
}
