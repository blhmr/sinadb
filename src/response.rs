#[derive(Debug)]
pub enum Response {
    Ok(String, String, Option<String>, Option<u64>),
    Err(String),
}

impl From<String> for Response {
    fn from(value: String) -> Self {
        // SET, GET, DEL
        let mut tokens = value.split(" ");

        let method = tokens.next();
        let key = tokens.next();

        if method.is_none() {
            return Response::Err("Missing method".to_string());
        }

        if key.is_none() {
            return Response::Err("Missing key".to_string());
        }

        match method.unwrap() {
            "SET" => {
                let value = tokens.next();
                let ttl = tokens.next();

                if value.is_none() {
                    return Response::Err("Missing value".to_string());
                }

                let mut ttl_uint = 0;

                if let Some(ttl) = ttl {
                    ttl_uint = match ttl.parse::<u64>() {
                        Ok(n) => n,
                        Err(e) => return Response::Err(format!("Could not parse TTL to uint: {}", e)),
                    };
                }
            
                return Response::Ok(
                    method.unwrap().to_string(),
                    key.unwrap().to_string(),
                    Some(value.unwrap().to_string()),
                    match ttl_uint {
                        0 => None,
                        _ => Some(ttl_uint),
                    }
                );
            },
            "GET" | "DEL" => return Response::Ok(
                method.unwrap().to_string(),
                key.unwrap().to_string(),
                None,
                None,
            ),
            _ => return Response::Err(format!("Unkown method `{}`", method.unwrap())),
        }
    }
}