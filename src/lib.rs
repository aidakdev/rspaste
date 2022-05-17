pub mod api;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::api;

    #[test]
    fn get() {
        let req = api::get("...");
        match req {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{}", e)
        }
    }

    #[test]
    fn post() {
        let req = api::post("...");
        match req {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{}", e)
        }
    }

    #[test]
    fn delete() {
        let req = api::delete("...", "...");
        match req {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{}", e)
        }
    }
}