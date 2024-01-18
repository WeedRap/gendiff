use std::collections::HashSet;

#[derive(Debug)]
pub enum DiffType<'a> {
    Deleted {
        key: &'a str,
        value: &'a serde_json::Value,
    },
    Added {
        key: &'a str,
        value: &'a serde_json::Value,
    },
    Nested {
        key: &'a str,
        children: Vec<DiffNode<'a>>,
    },
    Changed {
        key: &'a str,
        value1: &'a serde_json::Value,
        value2: &'a serde_json::Value,
    },
    Unchanged {
        key: &'a str,
        value: &'a serde_json::Value,
    },
}

#[derive(Debug)]
pub struct DiffNode<'a> {
    pub node_type: DiffType<'a>,
}

pub fn build_diff<'a>(
    data1: &'a serde_json::Value,
    data2: &'a serde_json::Value,
) -> Vec<DiffNode<'a>> {
    let mut result = Vec::new();
    let keys1: HashSet<_> = data1.as_object().unwrap().keys().collect();
    let keys2: HashSet<_> = data2.as_object().unwrap().keys().collect();
    let keys: HashSet<_> = keys1.union(&keys2).collect();
    let mut sorted_keys: Vec<_> = keys.into_iter().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        let value1 = data1.get(key);
        let value2 = data2.get(key);

        match (value1, value2) {
            (None, Some(v2)) => result.push(DiffNode {
                node_type: DiffType::Added { key, value: v2 },
            }),
            (Some(v1), None) => result.push(DiffNode {
                node_type: DiffType::Deleted { key, value: v1 },
            }),
            (Some(v1), Some(v2)) if v1 != v2 => {
                if let (Some(_obj1), Some(_obj2)) = (v1.as_object(), v2.as_object()) {
                    result.push(DiffNode {
                        node_type: DiffType::Nested {
                            key,
                            children: build_diff(&v1, &v2),
                        },
                    });
                } else {
                    result.push(DiffNode {
                        node_type: DiffType::Changed {
                            key,
                            value1: v1,
                            value2: v2,
                        },
                    });
                }
            }
            (Some(v1), Some(_)) => result.push(DiffNode {
                node_type: DiffType::Unchanged { key, value: v1 },
            }),
            _ => unreachable!(),
        }
    }

    result
}
