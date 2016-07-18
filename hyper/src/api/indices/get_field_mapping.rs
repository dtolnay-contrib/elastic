//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-get-field-mapping.html

//Autogenerated

use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

pub fn get_index_fields<'a>(client: &'a mut Client, req: &'a RequestParams,
                        index: &'a str, fields: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 16 + index.len() + fields.len()
                                  + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_mapping/field/");
    url_fmtd.push_str(fields);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}
pub fn get_index_type_fields<'a>(client: &'a mut Client, req: &'a RequestParams,
                             index: &'a str, _type: &'a str, fields: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + 7 + index.len() +
                                  _type.len() + fields.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_mapping/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/field/");
    url_fmtd.push_str(fields);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}
pub fn get_fields<'a>(client: &'a mut Client, req: &'a RequestParams,
                  fields: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 16 + fields.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_mapping/field/");
    url_fmtd.push_str(fields);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}
pub fn get_type_fields<'a>(client: &'a mut Client, req: &'a RequestParams,
                       _type: &'a str, fields: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 10 + 7 + _type.len() + fields.len()
                                  + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_mapping/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/field/");
    url_fmtd.push_str(fields);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}
