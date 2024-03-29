use crate::item::{Item, PriceInfo};
use rusqlite::{params, Connection, Result};
use std::path::Path;

fn create_db(db_file: &str) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_file)?;
    let _ = conn.execute(
        "create table Item (
        id        INTEGER PRIMARY KEY AUTOINCREMENT,
        name      VARCHAR NOT NULL,
        class     CHAR(16) NOT NULL,
        typo      CHAR(32) NOT NULL,
        ware      CHAR(16) NOT NULL,
        quality   CHAR(16) NOT NULL,
        rarity    CHAR(16) NOT NULL,
        stat_trak INTEGER NOT NULL
    )",
        [],
    );
    let _ = conn.execute(
        "create table PriceInfo (
        id      INTEGER PRIMARY KEY AUTOINCREMENT,
        item_id INTEGER NOT NULL,
        date    CHAR(16) NOT NULL,
        price   NUMERIC(10,1) NOT NULL
    )",
        [],
    );
    Ok(())
}

pub struct DbHelper {
    conn: Connection,
}

#[allow(unused)]
impl DbHelper {
    pub fn new(db_file: &str) -> DbHelper {
        if !Path::new(db_file).exists() {
            create_db(db_file);
        }
        let conn = Connection::open(db_file).unwrap();
        DbHelper { conn }
    }

    fn find_item_id(&self, item: &Item) -> Result<u32> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id from Item \
            where name = ?1 \
            and class = ?2 \
            and typo = ?3 \
            and ware = ?4 \
            and quality = ?5 \
            and rarity = ?6 \
            and stat_trak = ?7",
            )
            .unwrap();
        stmt.query_row(
            params![
                item.name,
                item.class,
                item.typo,
                item.ware,
                item.quality,
                item.rarity,
                item.stat_trak
            ],
            |row| row.get(0),
        )
    }

    pub fn find_item(&self, item: Item) -> Result<Item> {
        return self.find_item_by_fields(
            item.name,
            item.class,
            item.typo,
            item.ware,
            item.stat_trak,
        );
    }

    pub fn find_item_by_fields(
        &self,
        name: String,
        class: String,
        typo: String,
        ware: String,
        stat_trak: bool,
    ) -> Result<Item> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT * from Item \
            where name = ?1 \
            and class = ?2 \
            and typo = ?3 \
            and ware = ?4 \
            and stat_trak = ?5",
            )
            .unwrap();
        stmt.query_row(params![name, class, typo, ware, stat_trak], |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
                typo: row.get(3)?,
                ware: row.get(4)?,
                quality: row.get(5)?,
                rarity: row.get(6)?,
                stat_trak: row.get(7)?,
            })
        })
    }

    pub fn get_item_id(&self, item: &Item) -> Option<u32> {
        match self.find_item_id(item) {
            Ok(id) => Some(id),
            _ => {
                self.add_item(item);
                match self.find_item_id(item) {
                    Ok(_id) => Some(_id),
                    _ => None,
                }
            }
        }
    }

    pub fn add_item(&self, item: &Item) {
        self.conn.execute(
            "INSERT INTO Item \
            (name, class, typo, ware, quality, rarity, stat_trak) \
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                item.name,
                item.class,
                item.typo,
                item.ware,
                item.quality,
                item.rarity,
                item.stat_trak
            ],
        );
    }

    pub fn find_price_info_id(&self, price_info: &PriceInfo) -> Result<usize> {
        let mut stmt = self
            .conn
            .prepare("SELECT id from PriceInfo where item_id = ?1 and date = ?2")
            .unwrap();
        stmt.query_row(params![price_info.item_id, price_info.date], |row| {
            row.get(0)
        })
    }

    pub fn add_price_info(&self, price_info: &PriceInfo) {
        match self.find_price_info_id(price_info) {
            Ok(id) => {
                self.conn.execute(
                    "update PriceInfo set price = ?1 where id = ?2",
                    params![price_info.price, id],
                );
            }
            _ => {
                self.conn.execute(
                    "INSERT INTO PriceInfo (item_id, date, price) VALUES(?1, ?2, ?3)",
                    params![price_info.item_id, price_info.date, price_info.price,],
                );
            }
        }
    }

    pub fn find_items_by_name(&self, name: String) -> Result<Vec<Item>> {
        let mut stmt = self.conn.prepare("SELECT * from Item where name like ?1")?;
        let mut items: Vec<Item> = Vec::new();
        let item_iter = stmt.query_map(params![format!("%{}%", name)], |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
                typo: row.get(3)?,
                ware: row.get(4)?,
                quality: row.get(5)?,
                rarity: row.get(6)?,
                stat_trak: row.get(7)?,
            })
        })?;
        for _item in item_iter {
            if let Ok(item) = _item {
                items.push(item);
            }
        }
        Ok(items)
    }

    pub fn find_price_by_item_id(&self, item_id: u32) -> Result<Vec<PriceInfo>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * from PriceInfo where item_id = ?1")?;
        let mut price_infos: Vec<PriceInfo> = Vec::new();
        let price_info_iter = stmt.query_map(params![item_id], |row| {
            Ok(PriceInfo {
                id: row.get(0)?,
                item_id: row.get(1)?,
                date: row.get(2)?,
                price: row.get(3)?,
            })
        })?;
        for _price_info in price_info_iter {
            if let Ok(price_info) = _price_info {
                price_infos.push(price_info);
            }
        }
        Ok(price_infos)
    }

    pub fn find_lastest_price_by_item_id(&self, item_id: u32) -> Result<f32> {
        let mut stmt = self
            .conn
            .prepare("SELECT price from PriceInfo where item_id = ?1 order by id desc limit 1")?;
        stmt.query_row(params![item_id], |row| row.get(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_db() {
        assert!(create_db("test.db").is_ok());
    }

    fn test_item() -> Item {
        Item {
            id: 0,
            name: "蝴蝶刀（★） | 人工染色 (崭新出厂)".to_string(),
            class: "刀".to_string(),
            typo: "蝴蝶刀".to_string(),
            ware: "久经沙场".to_string(),
            quality: "★".to_string(),
            rarity: "隐秘".to_string(),
            stat_trak: false,
        }
    }

    #[test]
    fn test_add_item() {
        let item = test_item();
        let db_helper = DbHelper::new("test.db");
        db_helper.add_item(&item);
        assert!(true)
    }

    #[test]
    fn test_get_item_id() {
        let item = test_item();
        let db_helper = DbHelper::new("test.db");
        assert_eq!(Some(1), db_helper.get_item_id(&item));
    }
}
