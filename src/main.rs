#[macro_use] extern crate nickel;
extern crate toml;
extern crate rustc_serialize;
extern crate postgres;

use nickel::{Nickel, HttpRouter, Response, MediaType, MiddlewareResult, JsonBody};
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rustc_serialize::json;
use postgres::{Connection, SslMode};
use postgres::types::ToSql;

#[derive(RustcDecodable, RustcEncodable)]
struct AnkenCond {
    cond1: String,
    cond2: String,
    cond3: String,
}

#[derive(RustcDecodable, RustcEncodable)]
struct AnkenRet {
    name: String,
    latest_ver: String,
    customer: String,
    owner: String,
    status: String,
}

//https://auth0.com/blog/2015/11/30/build-an-api-in-rust-with-jwt-authentication-using-nickelrs/
fn main() {
    reg_anken();

    let root_dir = parse_toml("server", "root_dir");
    assert!(env::set_current_dir(&root_dir).is_ok());

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", middleware! { |_, res|
        return route(res, "index");
    });
    router.get("/anken", middleware! { |_, res|
        return route(res, "anken");
    });
    router.get("/anken/detail", middleware! { |_, res|
        return route(res, "anken");
    });
    router.get("/anken/ver", middleware! { |_, res|
        return route(res, "anken");
    });
    router.post("/anken/api/search", middleware! { |req, mut res|
        let conds = req.json_as::<AnkenCond>().unwrap();
        res.set(MediaType::Json);
        let ret = [AnkenRet { name: "aaa".to_string(), latest_ver: "1".to_string(), customer: "bbb".to_string(), owner: "hhh".to_string(), status: "?".to_string() }, AnkenRet{ name: "ccc".to_string(), latest_ver: "2".to_string(), customer: "ddd".to_string(), owner: "iii".to_string(), status: "!".to_string() }];
        json::encode(&ret).unwrap()
    });
    router.get("/hello/:name", middleware! { |req|
        format!("Hello, {:?}", req.param("name").unwrap())
    });

    server.utilize(router);
    server.listen("127.0.0.1:8080");
}

fn route<'a, D>(res: Response<'a, D>, page: &str) -> MiddlewareResult<'a, D> {
    let mut data = HashMap::new();
    data.insert("page", page);
    return res.render("html/__template__.html", &data);
}

fn parse_toml(section: &str, key: &str) -> String {
    let mut file = File::open("mulberry.settings.toml").unwrap();
    let mut text = String::new();
    assert!(file.read_to_string(&mut text).is_ok());
    let mut parser = toml::Parser::new(&text);
    match parser.parse() {
        Some(value) => value.get(section).unwrap().lookup(key).unwrap().as_str().unwrap().to_string(),
        None => panic!("Not found: section or key.")
    }
}

struct PgValue {
    key: String,
    value: String,
}

fn conn_pg() -> Connection {
    Connection::connect("postgres://user:password@127.0.0.1:5432/database", &SslMode::None).unwrap()
}

fn reg_pg(table: &str, params : &[PgValue]) {
    assert!(0 < params.len());

    let mut keys = String::new();
    let mut mappers = String::new();
    let mut values = Vec::new();
    for (i, param) in params.iter().enumerate() {
        if i == 0 {
            keys = format!("{}", param.key);
            mappers = format!("${}", i);
        } else {
            keys = format!("{}, {}", keys, param.key);
            mappers = format!("{}, ${}", mappers, i);
        }
        values.push(format!("{}", param.value).as_str());
    }
    assert!(0 < values.len());
    //TODO cannnot get now
//    keys = format!("{}, {}", keys, "created_at");
//    mappers = format!("{}, ${}", mappers, values.len());
//    values.push("current_timestamp");
    keys = format!("{}, {}", keys, "created_by");
    mappers = format!("{}, ${}", mappers, values.len());
    values.push(&"Who?"); //TODO get from session

    let sql = format!("insert into {} ({}) values ({})", table, keys, mappers).as_ref();
    conn_pg().execute(sql, values.as_slice()).unwrap();
}

fn reg_anken() {
    let params = [
        PgValue{key: "name".to_string(), value: "test".to_string()},
        PgValue{key: "customer_id".to_string(), value: "00000001".to_string()},
        PgValue{key: "user_id".to_string(), value: "000001".to_string()},
        PgValue{key: "note".to_string(), value: "-".to_string()},
    ];
    reg_pg(&"t_anken", &params);
}

fn edit_anken() {
}

fn get_anken() {
}

fn reg_anken_ver() {
}

fn edit_anken_ver() {
}

fn get_anken_ver() {
}

fn reg_anken_ver_phase() {
}

fn edit_anken_ver_phase() {
}

fn get_anken_ver_phase() {
}

fn reg_anken_ver_phase_func() {
}

fn edit_anken_ver_phase_func() {
}

fn get_anken_ver_phase_func() {
}

//[dependencies]
//redis = "*"
//
//extern crate redis;
//use redis::Commands;
//
//    conn_redis();
//
//fn conn_redis() {
//    let redis_host = format!("redis://{}/", parse_toml("server", "redis_host"));
//    let client = redis::Client::open(redis_host.as_ref()).unwrap();
//    let conn = client.get_connection().unwrap();
//    let _ : () = conn.hset_multiple("anken:00001", &[("id", "1"), ("name", "test1"), ("latest_ver", "1"), ("customer", "xxx"), ("owner", "yyy"), ("status", "?")]).unwrap();
//}
