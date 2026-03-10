#[allow(warnings)]
pub mod Jwt {
    use std::fs;
    use md5;
    use crate::Model::model::Admin;

    pub fn compute_md5<S:Into<String>>(input: S) -> String {
        let input = input.into();
        format!("{:x}", md5::compute(input.as_bytes()))
    }

    // 生成一个 jwt
    pub fn jwt(admin: Admin) -> String {
        let flag = "jwt";
        let jwt = compute_md5(format!("{}-{}-{}", flag, admin.username, admin.password));
        let jwt = format!("{} 2022-1-14", jwt);
        let jwt = compute_md5(jwt);
        jwt
    }

    // 签发 jwt
    pub fn jwtCreate(admin: Admin) -> String {
        let jwt = jwt(admin);

        let path = "src/config/jwt.txt";
        fs::write(path, jwt.clone()).unwrap();
        jwt
    }

    // 通过判断 Jwt 是否存在来判断是否登录
    pub fn isLogin(jwt: String) -> bool {
        let path = "src/config/jwt.txt";

        match fs::read_to_string(path) {
            Ok(_jwt) => {
                let _jwt = _jwt.trim();
                _jwt == jwt
            },
            Err(_) => false
        }
    }

    // 删除 jwt
    pub fn jwtDel() {
        let path = "src/config/jwt.txt";
        fs::write(path, "").unwrap();
    }
}