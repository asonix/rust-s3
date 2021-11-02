use crate::{bucket::Bucket, command::Command, request_trait::Request};

pub trait Client<'a> {
    type Request: Request + 'a;

    fn request(&'a self, bucket: &'a Bucket, path: &'a str, command: Command<'a>) -> Self::Request;
}

#[cfg(feature = "with-attohttpc")]
pub use self::attohttpc::AttoHttpClient;

#[cfg(feature = "with-attohttpc")]
mod attohttpc {
    use super::Client;
    use crate::blocking::AttoRequest;
    use crate::{bucket::Bucket, command::Command};
    use chrono::Utc;

    #[derive(Clone)]
    pub struct AttoHttpClient;

    impl<'a> Client<'a> for AttoHttpClient {
        type Request = AttoRequest<'a>;

        fn request(
            &'a self,
            bucket: &'a Bucket,
            path: &'a str,
            command: Command<'a>,
        ) -> Self::Request {
            AttoRequest {
                bucket,
                path,
                command,
                datetime: Utc::now(),
                sync: true,
            }
        }
    }
}

#[cfg(feature = "with-surf")]
mod surf {
    use super::Client;
    use crate::surf_request::SurfRequest;
    use crate::{bucket::Bucket, command::Command};
    use chrono::Utc;
    use std::borrow::Cow;

    impl<'a> Client<'a> for surf::Client {
        type Request = SurfRequest<'a>;

        fn request(
            &'a self,
            bucket: &'a Bucket,
            path: &'a str,
            command: Command<'a>,
        ) -> Self::Request {
            SurfRequest {
                client: Cow::Borrowed(self),
                bucket,
                path,
                command,
                datetime: Utc::now(),
                sync: false,
            }
        }
    }
}

#[cfg(feature = "with-reqwest")]
mod reqwest {
    use super::Client;
    use crate::request::Reqwest;
    use crate::{bucket::Bucket, command::Command};
    use chrono::Utc;
    use std::borrow::Cow;

    impl<'a> Client<'a> for reqwest::Client {
        type Request = Reqwest<'a>;

        fn request(
            &'a self,
            bucket: &'a Bucket,
            path: &'a str,
            command: Command<'a>,
        ) -> Self::Request {
            Reqwest {
                client: Cow::Borrowed(self),
                bucket,
                path,
                command,
                datetime: Utc::now(),
                sync: false,
            }
        }
    }
}
