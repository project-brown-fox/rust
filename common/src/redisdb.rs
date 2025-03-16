use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

use crate::error::BrownFoxError;

pub struct RedisDB {
    con: redis::Connection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjWithId<Object: DbObject> {
    pub id: i32,

    #[serde(flatten)]
    pub object: Object,
}

impl<Object: DbObject> ObjWithId<Object> {
    fn new(id: i32, object: Object) -> Self {
        Self { id, object }
    }
}

pub trait DbObject: Serialize {
    fn get_key_prefix() -> String;
}

impl RedisDB {
    pub fn new() -> Result<Self, BrownFoxError> {
        Ok(Self {
            con: redis::Client::open("redis://127.0.0.1")
                .map_err(|e| BrownFoxError::RedisError(e))?
                .get_connection()
                .map_err(|e| BrownFoxError::RedisError(e))?,
        })
    }

    pub fn put<'a, Object: DbObject>(
        &mut self,
        obj: Object,
    ) -> Result<ObjWithId<Object>, BrownFoxError> {
        let obj_with_id = ObjWithId::new(
            self.con
                .incr(Object::get_key_prefix(), 1)
                .map_err(|e| BrownFoxError::RedisError(e))?,
            obj,
        );

        self.con
            .set::<String, String, ()>(
                format!("{}::{}", Object::get_key_prefix(), obj_with_id.id),
                serde_json::to_string(&obj_with_id).map_err(|e| BrownFoxError::JsonError(e))?,
            )
            .map_err(|e| BrownFoxError::RedisError(e))?;

        Ok(obj_with_id)
    }

    pub fn get<Object: DbObject + for<'de> Deserialize<'de>>(
        &mut self,
        id: i32,
    ) -> Result<ObjWithId<Object>, BrownFoxError> {
        let o = self
            .con
            .get::<String, String>(format!("{}::{}", Object::get_key_prefix(), id))
            .map_err(|e| BrownFoxError::RedisError(e))?;

        Ok(serde_json::from_str::<ObjWithId<Object>>(&o)
            .map_err(|e| BrownFoxError::JsonError(e))?)
    }

    pub fn delete<Object: DbObject + for<'de> Deserialize<'de>>(
        &mut self,
        id: i32,
    ) -> Result<(), BrownFoxError> {
        let _: () = self
            .con
            .del(format!("{}::{}", Object::get_key_prefix(), id))
            .map_err(|e| BrownFoxError::RedisError(e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::{DbObject, RedisDB};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestObject {
        data: String,
    }

    impl DbObject for TestObject {
        fn get_key_prefix() -> String {
            "test".to_string()
        }
    }
    #[test]
    fn test_put() {
        let mut db = RedisDB::new().unwrap();
        let id = db
            .put(TestObject {
                data: "some data".to_string(),
            })
            .unwrap()
            .id;

        let obj = db.get::<TestObject>(id).unwrap();
        assert_eq!(obj.object.data, "some data");
        assert!(db.delete::<TestObject>(obj.id).is_ok());
    }
}
