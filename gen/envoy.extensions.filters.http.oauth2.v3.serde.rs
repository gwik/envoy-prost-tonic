// @generated
impl serde::Serialize for OAuth2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.oauth2.v3.OAuth2", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OAuth2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OAuth2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.oauth2.v3.OAuth2")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OAuth2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map.next_value()?;
                        }
                    }
                }
                Ok(OAuth2 {
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.oauth2.v3.OAuth2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OAuth2Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.token_endpoint.is_some() {
            len += 1;
        }
        if !self.authorization_endpoint.is_empty() {
            len += 1;
        }
        if self.credentials.is_some() {
            len += 1;
        }
        if !self.redirect_uri.is_empty() {
            len += 1;
        }
        if self.redirect_path_matcher.is_some() {
            len += 1;
        }
        if self.signout_path.is_some() {
            len += 1;
        }
        if self.forward_bearer_token {
            len += 1;
        }
        if !self.pass_through_matcher.is_empty() {
            len += 1;
        }
        if !self.auth_scopes.is_empty() {
            len += 1;
        }
        if !self.resources.is_empty() {
            len += 1;
        }
        if self.auth_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.oauth2.v3.OAuth2Config", len)?;
        if let Some(v) = self.token_endpoint.as_ref() {
            struct_ser.serialize_field("tokenEndpoint", v)?;
        }
        if !self.authorization_endpoint.is_empty() {
            struct_ser.serialize_field("authorizationEndpoint", &self.authorization_endpoint)?;
        }
        if let Some(v) = self.credentials.as_ref() {
            struct_ser.serialize_field("credentials", v)?;
        }
        if !self.redirect_uri.is_empty() {
            struct_ser.serialize_field("redirectUri", &self.redirect_uri)?;
        }
        if let Some(v) = self.redirect_path_matcher.as_ref() {
            struct_ser.serialize_field("redirectPathMatcher", v)?;
        }
        if let Some(v) = self.signout_path.as_ref() {
            struct_ser.serialize_field("signoutPath", v)?;
        }
        if self.forward_bearer_token {
            struct_ser.serialize_field("forwardBearerToken", &self.forward_bearer_token)?;
        }
        if !self.pass_through_matcher.is_empty() {
            struct_ser.serialize_field("passThroughMatcher", &self.pass_through_matcher)?;
        }
        if !self.auth_scopes.is_empty() {
            struct_ser.serialize_field("authScopes", &self.auth_scopes)?;
        }
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        if self.auth_type != 0 {
            let v = o_auth2_config::AuthType::from_i32(self.auth_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.auth_type)))?;
            struct_ser.serialize_field("authType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OAuth2Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token_endpoint",
            "tokenEndpoint",
            "authorization_endpoint",
            "authorizationEndpoint",
            "credentials",
            "redirect_uri",
            "redirectUri",
            "redirect_path_matcher",
            "redirectPathMatcher",
            "signout_path",
            "signoutPath",
            "forward_bearer_token",
            "forwardBearerToken",
            "pass_through_matcher",
            "passThroughMatcher",
            "auth_scopes",
            "authScopes",
            "resources",
            "auth_type",
            "authType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TokenEndpoint,
            AuthorizationEndpoint,
            Credentials,
            RedirectUri,
            RedirectPathMatcher,
            SignoutPath,
            ForwardBearerToken,
            PassThroughMatcher,
            AuthScopes,
            Resources,
            AuthType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tokenEndpoint" | "token_endpoint" => Ok(GeneratedField::TokenEndpoint),
                            "authorizationEndpoint" | "authorization_endpoint" => Ok(GeneratedField::AuthorizationEndpoint),
                            "credentials" => Ok(GeneratedField::Credentials),
                            "redirectUri" | "redirect_uri" => Ok(GeneratedField::RedirectUri),
                            "redirectPathMatcher" | "redirect_path_matcher" => Ok(GeneratedField::RedirectPathMatcher),
                            "signoutPath" | "signout_path" => Ok(GeneratedField::SignoutPath),
                            "forwardBearerToken" | "forward_bearer_token" => Ok(GeneratedField::ForwardBearerToken),
                            "passThroughMatcher" | "pass_through_matcher" => Ok(GeneratedField::PassThroughMatcher),
                            "authScopes" | "auth_scopes" => Ok(GeneratedField::AuthScopes),
                            "resources" => Ok(GeneratedField::Resources),
                            "authType" | "auth_type" => Ok(GeneratedField::AuthType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OAuth2Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.oauth2.v3.OAuth2Config")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OAuth2Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token_endpoint__ = None;
                let mut authorization_endpoint__ = None;
                let mut credentials__ = None;
                let mut redirect_uri__ = None;
                let mut redirect_path_matcher__ = None;
                let mut signout_path__ = None;
                let mut forward_bearer_token__ = None;
                let mut pass_through_matcher__ = None;
                let mut auth_scopes__ = None;
                let mut resources__ = None;
                let mut auth_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TokenEndpoint => {
                            if token_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenEndpoint"));
                            }
                            token_endpoint__ = map.next_value()?;
                        }
                        GeneratedField::AuthorizationEndpoint => {
                            if authorization_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationEndpoint"));
                            }
                            authorization_endpoint__ = Some(map.next_value()?);
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = map.next_value()?;
                        }
                        GeneratedField::RedirectUri => {
                            if redirect_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectUri"));
                            }
                            redirect_uri__ = Some(map.next_value()?);
                        }
                        GeneratedField::RedirectPathMatcher => {
                            if redirect_path_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectPathMatcher"));
                            }
                            redirect_path_matcher__ = map.next_value()?;
                        }
                        GeneratedField::SignoutPath => {
                            if signout_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signoutPath"));
                            }
                            signout_path__ = map.next_value()?;
                        }
                        GeneratedField::ForwardBearerToken => {
                            if forward_bearer_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardBearerToken"));
                            }
                            forward_bearer_token__ = Some(map.next_value()?);
                        }
                        GeneratedField::PassThroughMatcher => {
                            if pass_through_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passThroughMatcher"));
                            }
                            pass_through_matcher__ = Some(map.next_value()?);
                        }
                        GeneratedField::AuthScopes => {
                            if auth_scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authScopes"));
                            }
                            auth_scopes__ = Some(map.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map.next_value()?);
                        }
                        GeneratedField::AuthType => {
                            if auth_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authType"));
                            }
                            auth_type__ = Some(map.next_value::<o_auth2_config::AuthType>()? as i32);
                        }
                    }
                }
                Ok(OAuth2Config {
                    token_endpoint: token_endpoint__,
                    authorization_endpoint: authorization_endpoint__.unwrap_or_default(),
                    credentials: credentials__,
                    redirect_uri: redirect_uri__.unwrap_or_default(),
                    redirect_path_matcher: redirect_path_matcher__,
                    signout_path: signout_path__,
                    forward_bearer_token: forward_bearer_token__.unwrap_or_default(),
                    pass_through_matcher: pass_through_matcher__.unwrap_or_default(),
                    auth_scopes: auth_scopes__.unwrap_or_default(),
                    resources: resources__.unwrap_or_default(),
                    auth_type: auth_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.oauth2.v3.OAuth2Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for o_auth2_config::AuthType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UrlEncodedBody => "URL_ENCODED_BODY",
            Self::BasicAuth => "BASIC_AUTH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for o_auth2_config::AuthType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "URL_ENCODED_BODY",
            "BASIC_AUTH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = o_auth2_config::AuthType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(o_auth2_config::AuthType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(o_auth2_config::AuthType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "URL_ENCODED_BODY" => Ok(o_auth2_config::AuthType::UrlEncodedBody),
                    "BASIC_AUTH" => Ok(o_auth2_config::AuthType::BasicAuth),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OAuth2Credentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.token_secret.is_some() {
            len += 1;
        }
        if self.cookie_names.is_some() {
            len += 1;
        }
        if self.token_formation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.oauth2.v3.OAuth2Credentials", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if let Some(v) = self.token_secret.as_ref() {
            struct_ser.serialize_field("tokenSecret", v)?;
        }
        if let Some(v) = self.cookie_names.as_ref() {
            struct_ser.serialize_field("cookieNames", v)?;
        }
        if let Some(v) = self.token_formation.as_ref() {
            match v {
                o_auth2_credentials::TokenFormation::HmacSecret(v) => {
                    struct_ser.serialize_field("hmacSecret", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OAuth2Credentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "token_secret",
            "tokenSecret",
            "cookie_names",
            "cookieNames",
            "hmac_secret",
            "hmacSecret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            TokenSecret,
            CookieNames,
            HmacSecret,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "tokenSecret" | "token_secret" => Ok(GeneratedField::TokenSecret),
                            "cookieNames" | "cookie_names" => Ok(GeneratedField::CookieNames),
                            "hmacSecret" | "hmac_secret" => Ok(GeneratedField::HmacSecret),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OAuth2Credentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.oauth2.v3.OAuth2Credentials")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OAuth2Credentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut token_secret__ = None;
                let mut cookie_names__ = None;
                let mut token_formation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::TokenSecret => {
                            if token_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenSecret"));
                            }
                            token_secret__ = map.next_value()?;
                        }
                        GeneratedField::CookieNames => {
                            if cookie_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cookieNames"));
                            }
                            cookie_names__ = map.next_value()?;
                        }
                        GeneratedField::HmacSecret => {
                            if token_formation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hmacSecret"));
                            }
                            token_formation__ = map.next_value::<::std::option::Option<_>>()?.map(o_auth2_credentials::TokenFormation::HmacSecret)
;
                        }
                    }
                }
                Ok(OAuth2Credentials {
                    client_id: client_id__.unwrap_or_default(),
                    token_secret: token_secret__,
                    cookie_names: cookie_names__,
                    token_formation: token_formation__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.oauth2.v3.OAuth2Credentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for o_auth2_credentials::CookieNames {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bearer_token.is_empty() {
            len += 1;
        }
        if !self.oauth_hmac.is_empty() {
            len += 1;
        }
        if !self.oauth_expires.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.oauth2.v3.OAuth2Credentials.CookieNames", len)?;
        if !self.bearer_token.is_empty() {
            struct_ser.serialize_field("bearerToken", &self.bearer_token)?;
        }
        if !self.oauth_hmac.is_empty() {
            struct_ser.serialize_field("oauthHmac", &self.oauth_hmac)?;
        }
        if !self.oauth_expires.is_empty() {
            struct_ser.serialize_field("oauthExpires", &self.oauth_expires)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for o_auth2_credentials::CookieNames {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bearer_token",
            "bearerToken",
            "oauth_hmac",
            "oauthHmac",
            "oauth_expires",
            "oauthExpires",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BearerToken,
            OauthHmac,
            OauthExpires,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bearerToken" | "bearer_token" => Ok(GeneratedField::BearerToken),
                            "oauthHmac" | "oauth_hmac" => Ok(GeneratedField::OauthHmac),
                            "oauthExpires" | "oauth_expires" => Ok(GeneratedField::OauthExpires),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = o_auth2_credentials::CookieNames;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.oauth2.v3.OAuth2Credentials.CookieNames")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<o_auth2_credentials::CookieNames, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bearer_token__ = None;
                let mut oauth_hmac__ = None;
                let mut oauth_expires__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BearerToken => {
                            if bearer_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bearerToken"));
                            }
                            bearer_token__ = Some(map.next_value()?);
                        }
                        GeneratedField::OauthHmac => {
                            if oauth_hmac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oauthHmac"));
                            }
                            oauth_hmac__ = Some(map.next_value()?);
                        }
                        GeneratedField::OauthExpires => {
                            if oauth_expires__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oauthExpires"));
                            }
                            oauth_expires__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(o_auth2_credentials::CookieNames {
                    bearer_token: bearer_token__.unwrap_or_default(),
                    oauth_hmac: oauth_hmac__.unwrap_or_default(),
                    oauth_expires: oauth_expires__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.oauth2.v3.OAuth2Credentials.CookieNames", FIELDS, GeneratedVisitor)
    }
}
