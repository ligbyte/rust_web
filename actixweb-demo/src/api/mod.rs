mod user;
pub use user::greety;


mod query;
pub use query::get_user_name;

mod json;
pub use json::login;

mod url_encoded_form;
pub use url_encoded_form::index;

mod file_other;
pub use file_other::upload_mix;

mod multi_extract;
pub use multi_extract::file_multi_extract;

mod error;
pub use error::error_handle;