extern crate env_logger;
extern crate jsonapi;

use jsonapi::query::*;

#[test]
fn can_print() {
    let _ = env_logger::try_init();
    let query = Query::from_params(
        "include=author&fields[articles]=title,\
                                    body&fields[people]=name&page[limit]=3&page[offset]=1",
    );
    println!("Query is {:?}", query);

    let pageparams = PageParams {
        limit: 1,
        offset: 1,
    };

    println!("PageParams is {:?}", pageparams);
}

#[test]
fn can_parse() {
    let _ = env_logger::try_init();
    let query = Query::from_params(
        "include=author&fields[articles]=title,\
                                    body&fields[people]=name&filter={}&sort=name&page[offset]=3&page[limit]=1",
    );

    match query.include {
        None => assert!(false),
        Some(include) => {
            assert_eq!(include.len(), 1);
            assert_eq!(include[0], "author");
        }
    }

    match query.page {
        None => assert!(false),
        Some(page) => {
            assert_eq!(page.limit, 1);
            assert_eq!(page.offset, 3);
        }
    }

    match query.fields {
        None => assert!(false),
        Some(fields) => {
            assert_eq!(fields.contains_key("people"), true);
            assert_eq!(fields.contains_key("articles"), true);

            match fields.get("people") {
                None => assert!(false),
                Some(arr) => {
                    assert_eq!(arr.len(), 1);
                    assert_eq!(arr[0], "name");
                }
            }
            match fields.get("articles") {
                None => assert!(false),
                Some(arr) => {
                    assert_eq!(arr.len(), 2);
                    assert_eq!(arr[0], "title");
                    assert_eq!(arr[1], "body");
                }
            }
        }
    }

    match query.sort {
        None => assert!(false),
        Some(sort) => {
            assert_eq!(sort.len(), 1);
            assert_eq!(sort[0], "name");
        }
    }

    assert!(query.filter.is_some());
}

#[test]
fn can_parse_and_provide_defaults_for_partial_fields() {
    let _ = env_logger::try_init();
    let query = Query::from_params("");

    match query.include {
        None => assert!(true),
        Some(_) => assert!(false),
    }

    match query.fields {
        None => assert!(false),
        Some(_) => assert!(true),
    }

    match query.page {
        None => assert!(false),
        Some(page) => {
            assert_eq!(page.limit, 0);
            assert_eq!(page.offset, 0);
        }
    }

    match query.sort {
        None => assert!(true),
        Some(_) => assert!(false),
    }

    match query.filter {
        None => assert!(true),
        Some(_) => assert!(false),
    }
}

#[test]
fn can_parse_and_handle_missing_fields_values() {
    let _ = env_logger::try_init();

    let query = Query::from_params("fields=");
    match query.fields {
        None => assert!(false),
        Some(_) => assert!(true),
    }

    let query = Query::from_params("fields=key");
    match query.fields {
        None => assert!(false),
        Some(_) => assert!(true),
    }

    let query = Query::from_params("fields=[key]");
    match query.fields {
        None => assert!(false),
        Some(_) => assert!(true),
    }

    let query = Query::from_params("fields[key]");
    match query.fields {
        None => assert!(false),
        Some(_) => assert!(true),
    }

    let query = Query::from_params("fields[key]=");
    match query.fields {
        None => assert!(false),
        Some(_) => assert!(true),
    }
}

#[test]
fn can_parse_and_handle_missing_page_values() {
    let _ = env_logger::try_init();

    let query = Query::from_params("page=&");

    match query.page {
        None => assert!(false),
        Some(pageparams) => {
            assert_eq!(pageparams.offset, 0);
            assert_eq!(pageparams.limit, 0);
        }
    }

    let query = Query::from_params("page[offset]=&page[limit]=");

    match query.page {
        None => assert!(false),
        Some(pageparams) => {
            assert_eq!(pageparams.offset, 0);
            assert_eq!(pageparams.limit, 0);
        }
    }

    let query = Query::from_params("page[offset]=/&page[limit]=/");

    match query.page {
        None => assert!(false),
        Some(pageparams) => {
            assert_eq!(pageparams.offset, 0);
            assert_eq!(pageparams.limit, 0);
        }
    }
}

#[test]
fn can_parse_and_use_defaults_for_invalid_values() {
    let _ = env_logger::try_init();
    let query = Query::from_params("page[offset]=x&page[limit]=y");

    match query.include {
        None => assert!(true),
        Some(_) => assert!(false),
    }

    match query.fields {
        None => assert!(false),
        Some(_) => assert!(true),
    }

    match query.page {
        None => assert!(false),
        Some(page) => {
            assert_eq!(page.limit, 0);
            assert_eq!(page.offset, 0);
        }
    }

    match query.sort {
        None => assert!(true),
        Some(_) => assert!(false),
    }

    match query.filter {
        None => assert!(true),
        Some(_) => assert!(false),
    }
}

#[test]
fn can_provide_and_empty_struct() {
    let _ = env_logger::try_init();
    let query = Query::from_params("!");

    match query.include {
        None => assert!(true),
        Some(_) => assert!(false),
    }

    match query.fields {
        None => assert!(false),
        Some(_) => assert!(true),
    }

    match query.page {
        None => assert!(false),
        Some(_) => assert!(true),
    }

    match query.sort {
        None => assert!(true),
        Some(_) => assert!(false),
    }

    match query.filter {
        None => assert!(true),
        Some(_) => assert!(false),
    }
}

#[test]
fn can_generate_string_empty() {
    let _ = env_logger::try_init();
    let query = Query {
        _type: "none".into(),
        include: None,
        fields: None,
        page: None,
        sort: None,
        filter: None,
    };

    let query_string = query.to_params();

    assert_eq!(query_string, "");
}

#[test]
fn can_generate_string_include() {
    let _ = env_logger::try_init();
    let query = Query {
        _type: "none".into(),
        include: Some(vec!["author".into()]),
        fields: None,
        page: None,
        sort: None,
        filter: None,
    };

    let query_string = query.to_params();

    assert_eq!(query_string, "include=author");
}

#[test]
fn can_generate_string_include_multiple() {
    let _ = env_logger::try_init();
    let query = Query {
        _type: "none".into(),
        include: Some(vec!["author".into(), "publisher".into()]),
        fields: None,
        page: None,
        sort: None,
        filter: None,
    };

    let query_string = query.to_params();

    assert_eq!(query_string, "include=author,publisher");
}

#[test]
fn can_generate_string_sort() {
    let _ = env_logger::try_init();
    let query = Query {
        _type: "none".into(),
        include: None,
        fields: None,
        page: None,
        sort: Some(vec!["name".into()]),
        filter: None,
    };

    let query_string = query.to_params();

    assert_eq!(query_string, "sort=name");
}

#[test]
fn can_generate_string_sort_multiple() {
    let _ = env_logger::try_init();
    let query = Query {
        _type: "none".into(),
        include: None,
        fields: None,
        page: None,
        sort: Some(vec!["-name".into(), "created".into()]),
        filter: None,
    };

    let query_string = query.to_params();

    assert_eq!(query_string, "sort=-name,created");
}

#[test]
fn can_generate_string_fields() {
    let _ = env_logger::try_init();
    type VecOfStrings = Vec<String>;
    let mut fields = std::collections::BTreeMap::<String, VecOfStrings>::new();

    fields.insert("user".into(), vec!["name".into()]);

    let query = Query {
        _type: "none".into(),
        include: None,
        fields: Some(fields),
        page: None,
        sort: None,
        filter: None,
    };

    let query_string = query.to_params();

    assert_eq!(query_string, "fields[user]=name");
}

#[test]
fn can_generate_string_fields_multiple_values() {
    let _ = env_logger::try_init();
    type VecOfStrings = Vec<String>;
    let mut fields = std::collections::BTreeMap::<String, VecOfStrings>::new();

    fields.insert("user".into(), vec!["name".into(), "dateofbirth".into()]);

    let query = Query {
        _type: "none".into(),
        include: None,
        fields: Some(fields),
        page: None,
        sort: None,
        filter: None,
    };

    let query_string = query.to_params();

    assert_eq!(query_string, "fields[user]=name,dateofbirth");
}

#[test]
fn can_generate_string_fields_multiple_key_and_values() {
    let _ = env_logger::try_init();
    type VecOfStrings = Vec<String>;
    let mut fields = std::collections::BTreeMap::<String, VecOfStrings>::new();

    fields.insert("item".into(), vec!["title".into(), "description".into()]);
    fields.insert("user".into(), vec!["name".into(), "dateofbirth".into()]);

    let query = Query {
        _type: "none".into(),
        include: None,
        fields: Some(fields),
        page: None,
        sort: None,
        filter: None,
    };

    let query_string = query.to_params();

    // We don't have any guarantees on the order in which fields are output
    //

    assert!(
        query_string.eq("fields[item]=title,description&fields[user]=name,dateofbirth",)
            || query_string.eq("fields[user]=name,dateofbirth&fields[item]=title,description",)
    );
}

#[test]
fn can_generate_string_filter_multiple_key_and_values() {
    let _ = env_logger::try_init();
    type VecOfStrings = Vec<String>;
    let filter = serde_json::Value::Object(serde_json::Map::new());

    let query = Query {
        _type: "none".into(),
        include: None,
        fields: None,
        page: None,
        sort: None,
        filter: Some(filter),
    };

    let query_string = query.to_params();

    // We don't have any guarantees on the order in which fields are output
    //

    assert!(query_string.eq("filter={}"));
}

#[test]
fn can_generate_page_fields() {
    let _ = env_logger::try_init();

    let query = Query {
        _type: "none".into(),
        include: None,
        fields: None,
        page: Some(PageParams {
            limit: 5,
            offset: 10,
        }),
        sort: None,
        filter: None,
    };

    let query_string = query.to_params();

    assert_eq!(query_string, "page[limit]=5&page[offset]=10");
}
