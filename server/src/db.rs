use client::proto::{NetworkInfo, NetworkPropValue};
use sqlx::pool::PoolConnection;
use sqlx::{Postgres, Row};
use std::collections::{BTreeMap, HashMap};

pub async fn get_networks(conn: &mut PoolConnection<Postgres>) -> Vec<NetworkInfo> {
    let sql = "SELECT nmae FROM networks";
    let res: Vec<NetworkInfo> = match sqlx::query(sql).fetch_all(conn).await {
        Ok(x) => x,
        Err(e) => {
            panic!("sql has error {}", e);
        }
    };
    // TODO: get
    res
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
