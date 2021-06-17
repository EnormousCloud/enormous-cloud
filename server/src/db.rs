use client::proto::{ChainStateUrl, NetworkInfo, NetworkPropValue};
use sqlx::pool::PoolConnection;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::BTreeMap;

fn optional_str(input: &str) -> Option<String> {
    if input.len() > 0 {
        Some(input.to_owned())
    } else {
        None
    }
}

pub async fn get_links(conn: &mut PoolConnection<Postgres>) -> Vec<ChainStateUrl> {
    let sql = "select url, auth_header FROM networks_chainstate";
    let rows: Vec<PgRow> = match sqlx::query(sql).fetch_all(conn).await {
        Ok(x) => x,
        Err(e) => panic!("sql error {}", e),
    };
    let mut res = vec![];
    for row in rows {
        res.push(ChainStateUrl {
            url: row.get("url"),
            auth_header: row.get("auth_header"),
        })
    }
    res
}

pub async fn get_properties_map(
    conn: &mut PoolConnection<Postgres>,
) -> BTreeMap<String, Vec<NetworkPropValue>> {
    let sql = "SELECT name, label, value, link FROM networks_properties ORDER BY sortorder";
    let rows: Vec<PgRow> = match sqlx::query(sql).fetch_all(conn).await {
        Ok(x) => x,
        Err(e) => panic!("sql error {}", e),
    };
    let mut res = BTreeMap::new();
    for row in rows {
        let name: String = row.get("name");
        if let None = &res.get(name.as_str()) {
            let v: Vec<NetworkPropValue> = vec![];
            res.insert(name.clone(), v);
        }
        if let Some(v) = res.get_mut(name.as_str()) {
            v.push(NetworkPropValue {
                label: row.get("label"),
                value: row.get("value"),
                link: optional_str(row.get("link")),
            })
        }
    }
    res
}

pub async fn get_networks(conn: &mut PoolConnection<Postgres>, props: Option<BTreeMap<String, NetworkInfo>>) -> Vec<NetworkInfo> {
    let by_network = get_properties_map(conn).await;
    let sql = "SELECT name, coin, test, link FROM networks_list WHERE active=true";
    let rows: Vec<PgRow> = match sqlx::query(sql).fetch_all(conn).await {
        Ok(x) => x,
        Err(e) => panic!("sql error {}", e),
    };
    let mut res = vec![];
    for row in rows {
        let name: String = row.get("name");
        let mut properties: Vec<NetworkPropValue> = match by_network.get(name.as_str()) {
            Some(x) => x.to_vec(),
            None => vec![],
        };
        if let Some(pmap) = &props { // if props are provided
            if let Some(netinfo) = pmap.get(name.as_str())  { // and we have network listed
                for v in netinfo.properties.iter() {
                    properties.push(v.clone());
                }
            }
        }
        
        res.push(NetworkInfo {
            name: name.clone(),
            link: optional_str(row.get("link")),
            test: Some(row.get("test")),
            coin: optional_str(row.get("coin")),
            price: None,
            properties: properties,
        })
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::{pool::PoolConnection, Postgres};

    async fn get_test_conn() -> Result<PoolConnection<Postgres>, sqlx::Error> {
        let pool =
            sqlx::postgres::PgPool::connect("postgres://postgres:password@localhost/enormous")
                .await?;
        pool.acquire().await
    }

    #[ignore]
    #[async_std::test]
    async fn it_retrieves_links() -> std::io::Result<()> {
        let mut conn = get_test_conn().await.unwrap();
        eprintln!("links: {:?}", get_links(&mut conn).await);
        Ok(())
    }

    #[ignore]
    #[async_std::test]
    async fn it_retrieves_props() -> std::io::Result<()> {
        let mut conn = get_test_conn().await.unwrap();
        eprintln!("networks: {:?}", get_networks(&mut conn, None).await);
        Ok(())
    }
}
