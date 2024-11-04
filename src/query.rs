use queryst::parse;
use serde_json::value::Value;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PageParams {
    pub offset: i64,
    pub limit: i64,
}

/// JSON-API Query parameters
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Query {
    pub _type: String,
    pub include: Option<Vec<String>>,
    pub fields: Option<BTreeMap<String, Vec<String>>>,
    pub page: Option<PageParams>,
    pub sort: Option<Vec<String>>,
    pub filter: Option<Value>,
}

//
// Helper functions to break down the cyclomatic complexity of parameter parsing
//

fn ok_params_include(o: &Value) -> Option<Vec<String>> {
    match o.pointer("/include") {
        None => None,
        Some(inc) => match inc.as_str() {
            None => None,
            Some(include_str) => {
                let arr: Vec<String> = include_str.split(',').map(|s| s.to_string()).collect();
                Some(arr)
            }
        },
    }
}

fn ok_params_fields(o: &Value) -> BTreeMap<String, Vec<String>> {
    let mut fields = BTreeMap::<String, Vec<String>>::new();

    if let Some(x) = o.pointer("/fields") {
        if x.is_object() {
            if let Some(obj) = x.as_object() {
                for (key, value) in obj.iter() {
                    let arr: Vec<String> = match value.as_str() {
                        Some(string) => string.split(',').map(|s| s.to_string()).collect(),
                        None => Vec::<String>::new(),
                    };
                    fields.insert(key.to_string(), arr);
                }
            }
        } else {
            warn!("Query::from_params : No fields found in {:?}", x);
        }
    }

    fields
}

fn ok_params_sort(o: &Value) -> Option<Vec<String>> {
    match o.pointer("/sort") {
        None => None,
        Some(sort) => match sort.as_str() {
            None => None,
            Some(sort_str) => {
                let arr: Vec<String> = sort_str.split(',').map(|s| s.to_string()).collect();
                Some(arr)
            }
        },
    }
}

fn ok_params_filter(o: &Value) -> Option<Value> {
    match o.pointer("/filter") {
        None => None,
        Some(x) => {
            let parsed_json: Value = serde_json::from_str(x.as_str()?).ok()?;

            if parsed_json.is_object() {
                Some(parsed_json.clone())
            } else {
                warn!("Query::from_params : No filter found in {:?}", x);
                None
            }
        }
    }
}

fn ok_params_page(o: &Value) -> PageParams {
    PageParams {
        offset: match o.pointer("/page/offset") {
            None => {
                warn!(
                    "Query::from_params : No page/offset found in {:?}, setting \
                                   default 0",
                    o
                );
                0
            }
            Some(num) => {
                if num.is_string() {
                    match num.as_str().map(str::parse::<i64>) {
                        Some(y) => y.unwrap_or(0),
                        None => {
                            warn!(
                                "Query::from_params : page/offset found in {:?}, \
                                               not able not able to parse it - setting default 0",
                                o
                            );
                            0
                        }
                    }
                } else {
                    warn!(
                        "Query::from_params : page/offset found in {:?}, but it is \
                                       not an expected type - setting default 0",
                        o
                    );
                    0
                }
            }
        },
        limit: match o.pointer("/page/limit") {
            None => {
                warn!(
                    "Query::from_params : No page/limit found in {:?}, setting \
                                   default 0",
                    o
                );
                0
            }
            Some(num) => {
                if num.is_string() {
                    match num.as_str().map(str::parse::<i64>) {
                        Some(y) => y.unwrap_or(0),
                        None => {
                            warn!(
                                "Query::from_params : page/limit found in {:?}, \
                                               not able not able to parse it - setting default 0",
                                o
                            );
                            0
                        }
                    }
                } else {
                    warn!(
                        "Query::from_params : page/limit found in {:?}, but it is \
                                       not an expected type - setting default 0",
                        o
                    );
                    0
                }
            }
        },
    }
}

fn ok_params(o: Value) -> Query {
    Query {
        _type: "none".into(),
        include: ok_params_include(&o),
        fields: Some(ok_params_fields(&o)),
        page: Some(ok_params_page(&o)),
        sort: ok_params_sort(&o),
        filter: ok_params_filter(&o),
    }
}

/// JSON-API Query parameters
impl Query {
    ///
    /// Takes a query parameter string and returns a Query
    ///
    /// ```
    /// use jsonapi::query::Query;
    /// let query = Query::from_params("include=author&fields[articles]=title,\
    ///                                 body&fields[people]=name&page[offset]=3&page[limit]=1");
    /// match query.include {
    ///     None => assert!(false),
    ///     Some(include) => {
    ///         assert_eq!(include.len(), 1);
    ///         assert_eq!(include[0], "author");
    ///     }
    /// }
    ///
    /// ```
    pub fn from_params(params: &str) -> Self {
        match parse(params) {
            Ok(o) => ok_params(o),
            Err(err) => {
                warn!("Query::from_params : Can't parse : {:?}", err);
                Query {
                    _type: "none".into(),
                    ..Default::default()
                }
            }
        }
    }

    ///
    /// Builds a query parameter string from a Query
    ///
    /// ```
    /// use jsonapi::query::{Query, PageParams};
    /// let query = Query {
    ///   _type: "post".into(),
    ///   include: Some(vec!["author".into()]),
    ///   fields: None,
    ///   page: Some(PageParams {
    ///     limit: 5,
    ///     offset: 10,
    ///   }),
    ///   sort: None,
    ///   filter: None,
    /// };
    ///
    /// let query_string = query.to_params();
    /// assert_eq!(query_string, "include=author&page[limit]=5&page[offset]=10");
    ///
    /// ```
    pub fn to_params(&self) -> String {
        let mut params = Vec::<String>::new();

        if let Some(ref include) = self.include {
            params.push(format!("include={}", include.join(",")));
        }

        // Examples from json-api.org,
        // fields[articles]=title,body,author&fields[people]=name
        // fields[articles]=title,body&fields[people]=name

        if let Some(ref fields) = self.fields {
            for (name, val) in fields.iter() {
                params.push(format!("fields[{}]={}", name, val.join(",")));
            }
        }

        if let Some(ref sort) = self.sort {
            params.push(format!("sort={}", sort.join(",")))
        }

        if let Some(ref filter) = self.filter {
            params.push(format!("filter={}", filter))
        }

        if let Some(ref page) = self.page {
            params.push(page.to_params());
        }

        params.join("&")
    }
}

impl PageParams {
    pub fn to_params(&self) -> String {
        format!("page[limit]={}&page[offset]={}", self.limit, self.offset)
    }
}
