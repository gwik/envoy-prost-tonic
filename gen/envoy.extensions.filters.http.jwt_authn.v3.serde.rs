// @generated
impl serde::Serialize for FilterStateRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.requires.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.FilterStateRule", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.requires.is_empty() {
            struct_ser.serialize_field("requires", &self.requires)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterStateRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "requires",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Requires,
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
                            "name" => Ok(GeneratedField::Name),
                            "requires" => Ok(GeneratedField::Requires),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterStateRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.FilterStateRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterStateRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut requires__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Requires => {
                            if requires__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requires"));
                            }
                            requires__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(FilterStateRule {
                    name: name__.unwrap_or_default(),
                    requires: requires__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.FilterStateRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwksAsyncFetch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fast_listener {
            len += 1;
        }
        if self.failed_refetch_duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwksAsyncFetch", len)?;
        if self.fast_listener {
            struct_ser.serialize_field("fastListener", &self.fast_listener)?;
        }
        if let Some(v) = self.failed_refetch_duration.as_ref() {
            struct_ser.serialize_field("failedRefetchDuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwksAsyncFetch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fast_listener",
            "fastListener",
            "failed_refetch_duration",
            "failedRefetchDuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FastListener,
            FailedRefetchDuration,
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
                            "fastListener" | "fast_listener" => Ok(GeneratedField::FastListener),
                            "failedRefetchDuration" | "failed_refetch_duration" => Ok(GeneratedField::FailedRefetchDuration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwksAsyncFetch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwksAsyncFetch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwksAsyncFetch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fast_listener__ = None;
                let mut failed_refetch_duration__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FastListener => {
                            if fast_listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fastListener"));
                            }
                            fast_listener__ = Some(map.next_value()?);
                        }
                        GeneratedField::FailedRefetchDuration => {
                            if failed_refetch_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedRefetchDuration"));
                            }
                            failed_refetch_duration__ = map.next_value()?;
                        }
                    }
                }
                Ok(JwksAsyncFetch {
                    fast_listener: fast_listener__.unwrap_or_default(),
                    failed_refetch_duration: failed_refetch_duration__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwksAsyncFetch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtAuthentication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.providers.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        if self.filter_state_rules.is_some() {
            len += 1;
        }
        if self.bypass_cors_preflight {
            len += 1;
        }
        if !self.requirement_map.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtAuthentication", len)?;
        if !self.providers.is_empty() {
            struct_ser.serialize_field("providers", &self.providers)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if let Some(v) = self.filter_state_rules.as_ref() {
            struct_ser.serialize_field("filterStateRules", v)?;
        }
        if self.bypass_cors_preflight {
            struct_ser.serialize_field("bypassCorsPreflight", &self.bypass_cors_preflight)?;
        }
        if !self.requirement_map.is_empty() {
            struct_ser.serialize_field("requirementMap", &self.requirement_map)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtAuthentication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "providers",
            "rules",
            "filter_state_rules",
            "filterStateRules",
            "bypass_cors_preflight",
            "bypassCorsPreflight",
            "requirement_map",
            "requirementMap",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Providers,
            Rules,
            FilterStateRules,
            BypassCorsPreflight,
            RequirementMap,
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
                            "providers" => Ok(GeneratedField::Providers),
                            "rules" => Ok(GeneratedField::Rules),
                            "filterStateRules" | "filter_state_rules" => Ok(GeneratedField::FilterStateRules),
                            "bypassCorsPreflight" | "bypass_cors_preflight" => Ok(GeneratedField::BypassCorsPreflight),
                            "requirementMap" | "requirement_map" => Ok(GeneratedField::RequirementMap),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtAuthentication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwtAuthentication")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtAuthentication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut providers__ = None;
                let mut rules__ = None;
                let mut filter_state_rules__ = None;
                let mut bypass_cors_preflight__ = None;
                let mut requirement_map__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Providers => {
                            if providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providers"));
                            }
                            providers__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map.next_value()?);
                        }
                        GeneratedField::FilterStateRules => {
                            if filter_state_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterStateRules"));
                            }
                            filter_state_rules__ = map.next_value()?;
                        }
                        GeneratedField::BypassCorsPreflight => {
                            if bypass_cors_preflight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bypassCorsPreflight"));
                            }
                            bypass_cors_preflight__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequirementMap => {
                            if requirement_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirementMap"));
                            }
                            requirement_map__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(JwtAuthentication {
                    providers: providers__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    filter_state_rules: filter_state_rules__,
                    bypass_cors_preflight: bypass_cors_preflight__.unwrap_or_default(),
                    requirement_map: requirement_map__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtAuthentication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtCacheConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.jwt_cache_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtCacheConfig", len)?;
        if self.jwt_cache_size != 0 {
            struct_ser.serialize_field("jwtCacheSize", &self.jwt_cache_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtCacheConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "jwt_cache_size",
            "jwtCacheSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JwtCacheSize,
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
                            "jwtCacheSize" | "jwt_cache_size" => Ok(GeneratedField::JwtCacheSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtCacheConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwtCacheConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtCacheConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut jwt_cache_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::JwtCacheSize => {
                            if jwt_cache_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwtCacheSize"));
                            }
                            jwt_cache_size__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(JwtCacheConfig {
                    jwt_cache_size: jwt_cache_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtCacheConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtClaimToHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header_name.is_empty() {
            len += 1;
        }
        if !self.claim_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtClaimToHeader", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("headerName", &self.header_name)?;
        }
        if !self.claim_name.is_empty() {
            struct_ser.serialize_field("claimName", &self.claim_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtClaimToHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
            "claim_name",
            "claimName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderName,
            ClaimName,
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
                            "headerName" | "header_name" => Ok(GeneratedField::HeaderName),
                            "claimName" | "claim_name" => Ok(GeneratedField::ClaimName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtClaimToHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwtClaimToHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtClaimToHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                let mut claim_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClaimName => {
                            if claim_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimName"));
                            }
                            claim_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(JwtClaimToHeader {
                    header_name: header_name__.unwrap_or_default(),
                    claim_name: claim_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtClaimToHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.value_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtHeader", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value_prefix.is_empty() {
            struct_ser.serialize_field("valuePrefix", &self.value_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value_prefix",
            "valuePrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ValuePrefix,
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
                            "name" => Ok(GeneratedField::Name),
                            "valuePrefix" | "value_prefix" => Ok(GeneratedField::ValuePrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwtHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValuePrefix => {
                            if value_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePrefix"));
                            }
                            value_prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(JwtHeader {
                    name: name__.unwrap_or_default(),
                    value_prefix: value_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.audiences.is_empty() {
            len += 1;
        }
        if self.forward {
            len += 1;
        }
        if !self.from_headers.is_empty() {
            len += 1;
        }
        if !self.from_params.is_empty() {
            len += 1;
        }
        if !self.from_cookies.is_empty() {
            len += 1;
        }
        if !self.forward_payload_header.is_empty() {
            len += 1;
        }
        if self.pad_forward_payload_header {
            len += 1;
        }
        if !self.payload_in_metadata.is_empty() {
            len += 1;
        }
        if !self.header_in_metadata.is_empty() {
            len += 1;
        }
        if !self.failed_status_in_metadata.is_empty() {
            len += 1;
        }
        if self.clock_skew_seconds != 0 {
            len += 1;
        }
        if self.jwt_cache_config.is_some() {
            len += 1;
        }
        if !self.claim_to_headers.is_empty() {
            len += 1;
        }
        if self.jwks_source_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtProvider", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.audiences.is_empty() {
            struct_ser.serialize_field("audiences", &self.audiences)?;
        }
        if self.forward {
            struct_ser.serialize_field("forward", &self.forward)?;
        }
        if !self.from_headers.is_empty() {
            struct_ser.serialize_field("fromHeaders", &self.from_headers)?;
        }
        if !self.from_params.is_empty() {
            struct_ser.serialize_field("fromParams", &self.from_params)?;
        }
        if !self.from_cookies.is_empty() {
            struct_ser.serialize_field("fromCookies", &self.from_cookies)?;
        }
        if !self.forward_payload_header.is_empty() {
            struct_ser.serialize_field("forwardPayloadHeader", &self.forward_payload_header)?;
        }
        if self.pad_forward_payload_header {
            struct_ser.serialize_field("padForwardPayloadHeader", &self.pad_forward_payload_header)?;
        }
        if !self.payload_in_metadata.is_empty() {
            struct_ser.serialize_field("payloadInMetadata", &self.payload_in_metadata)?;
        }
        if !self.header_in_metadata.is_empty() {
            struct_ser.serialize_field("headerInMetadata", &self.header_in_metadata)?;
        }
        if !self.failed_status_in_metadata.is_empty() {
            struct_ser.serialize_field("failedStatusInMetadata", &self.failed_status_in_metadata)?;
        }
        if self.clock_skew_seconds != 0 {
            struct_ser.serialize_field("clockSkewSeconds", &self.clock_skew_seconds)?;
        }
        if let Some(v) = self.jwt_cache_config.as_ref() {
            struct_ser.serialize_field("jwtCacheConfig", v)?;
        }
        if !self.claim_to_headers.is_empty() {
            struct_ser.serialize_field("claimToHeaders", &self.claim_to_headers)?;
        }
        if let Some(v) = self.jwks_source_specifier.as_ref() {
            match v {
                jwt_provider::JwksSourceSpecifier::RemoteJwks(v) => {
                    struct_ser.serialize_field("remoteJwks", v)?;
                }
                jwt_provider::JwksSourceSpecifier::LocalJwks(v) => {
                    struct_ser.serialize_field("localJwks", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "audiences",
            "forward",
            "from_headers",
            "fromHeaders",
            "from_params",
            "fromParams",
            "from_cookies",
            "fromCookies",
            "forward_payload_header",
            "forwardPayloadHeader",
            "pad_forward_payload_header",
            "padForwardPayloadHeader",
            "payload_in_metadata",
            "payloadInMetadata",
            "header_in_metadata",
            "headerInMetadata",
            "failed_status_in_metadata",
            "failedStatusInMetadata",
            "clock_skew_seconds",
            "clockSkewSeconds",
            "jwt_cache_config",
            "jwtCacheConfig",
            "claim_to_headers",
            "claimToHeaders",
            "remote_jwks",
            "remoteJwks",
            "local_jwks",
            "localJwks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            Audiences,
            Forward,
            FromHeaders,
            FromParams,
            FromCookies,
            ForwardPayloadHeader,
            PadForwardPayloadHeader,
            PayloadInMetadata,
            HeaderInMetadata,
            FailedStatusInMetadata,
            ClockSkewSeconds,
            JwtCacheConfig,
            ClaimToHeaders,
            RemoteJwks,
            LocalJwks,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "audiences" => Ok(GeneratedField::Audiences),
                            "forward" => Ok(GeneratedField::Forward),
                            "fromHeaders" | "from_headers" => Ok(GeneratedField::FromHeaders),
                            "fromParams" | "from_params" => Ok(GeneratedField::FromParams),
                            "fromCookies" | "from_cookies" => Ok(GeneratedField::FromCookies),
                            "forwardPayloadHeader" | "forward_payload_header" => Ok(GeneratedField::ForwardPayloadHeader),
                            "padForwardPayloadHeader" | "pad_forward_payload_header" => Ok(GeneratedField::PadForwardPayloadHeader),
                            "payloadInMetadata" | "payload_in_metadata" => Ok(GeneratedField::PayloadInMetadata),
                            "headerInMetadata" | "header_in_metadata" => Ok(GeneratedField::HeaderInMetadata),
                            "failedStatusInMetadata" | "failed_status_in_metadata" => Ok(GeneratedField::FailedStatusInMetadata),
                            "clockSkewSeconds" | "clock_skew_seconds" => Ok(GeneratedField::ClockSkewSeconds),
                            "jwtCacheConfig" | "jwt_cache_config" => Ok(GeneratedField::JwtCacheConfig),
                            "claimToHeaders" | "claim_to_headers" => Ok(GeneratedField::ClaimToHeaders),
                            "remoteJwks" | "remote_jwks" => Ok(GeneratedField::RemoteJwks),
                            "localJwks" | "local_jwks" => Ok(GeneratedField::LocalJwks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwtProvider")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut audiences__ = None;
                let mut forward__ = None;
                let mut from_headers__ = None;
                let mut from_params__ = None;
                let mut from_cookies__ = None;
                let mut forward_payload_header__ = None;
                let mut pad_forward_payload_header__ = None;
                let mut payload_in_metadata__ = None;
                let mut header_in_metadata__ = None;
                let mut failed_status_in_metadata__ = None;
                let mut clock_skew_seconds__ = None;
                let mut jwt_cache_config__ = None;
                let mut claim_to_headers__ = None;
                let mut jwks_source_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::Audiences => {
                            if audiences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audiences"));
                            }
                            audiences__ = Some(map.next_value()?);
                        }
                        GeneratedField::Forward => {
                            if forward__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forward"));
                            }
                            forward__ = Some(map.next_value()?);
                        }
                        GeneratedField::FromHeaders => {
                            if from_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromHeaders"));
                            }
                            from_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::FromParams => {
                            if from_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromParams"));
                            }
                            from_params__ = Some(map.next_value()?);
                        }
                        GeneratedField::FromCookies => {
                            if from_cookies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromCookies"));
                            }
                            from_cookies__ = Some(map.next_value()?);
                        }
                        GeneratedField::ForwardPayloadHeader => {
                            if forward_payload_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardPayloadHeader"));
                            }
                            forward_payload_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::PadForwardPayloadHeader => {
                            if pad_forward_payload_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("padForwardPayloadHeader"));
                            }
                            pad_forward_payload_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::PayloadInMetadata => {
                            if payload_in_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadInMetadata"));
                            }
                            payload_in_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeaderInMetadata => {
                            if header_in_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerInMetadata"));
                            }
                            header_in_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::FailedStatusInMetadata => {
                            if failed_status_in_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedStatusInMetadata"));
                            }
                            failed_status_in_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClockSkewSeconds => {
                            if clock_skew_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clockSkewSeconds"));
                            }
                            clock_skew_seconds__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::JwtCacheConfig => {
                            if jwt_cache_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwtCacheConfig"));
                            }
                            jwt_cache_config__ = map.next_value()?;
                        }
                        GeneratedField::ClaimToHeaders => {
                            if claim_to_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimToHeaders"));
                            }
                            claim_to_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemoteJwks => {
                            if jwks_source_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteJwks"));
                            }
                            jwks_source_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(jwt_provider::JwksSourceSpecifier::RemoteJwks)
;
                        }
                        GeneratedField::LocalJwks => {
                            if jwks_source_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localJwks"));
                            }
                            jwks_source_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(jwt_provider::JwksSourceSpecifier::LocalJwks)
;
                        }
                    }
                }
                Ok(JwtProvider {
                    issuer: issuer__.unwrap_or_default(),
                    audiences: audiences__.unwrap_or_default(),
                    forward: forward__.unwrap_or_default(),
                    from_headers: from_headers__.unwrap_or_default(),
                    from_params: from_params__.unwrap_or_default(),
                    from_cookies: from_cookies__.unwrap_or_default(),
                    forward_payload_header: forward_payload_header__.unwrap_or_default(),
                    pad_forward_payload_header: pad_forward_payload_header__.unwrap_or_default(),
                    payload_in_metadata: payload_in_metadata__.unwrap_or_default(),
                    header_in_metadata: header_in_metadata__.unwrap_or_default(),
                    failed_status_in_metadata: failed_status_in_metadata__.unwrap_or_default(),
                    clock_skew_seconds: clock_skew_seconds__.unwrap_or_default(),
                    jwt_cache_config: jwt_cache_config__,
                    claim_to_headers: claim_to_headers__.unwrap_or_default(),
                    jwks_source_specifier: jwks_source_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtRequirement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.requires_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtRequirement", len)?;
        if let Some(v) = self.requires_type.as_ref() {
            match v {
                jwt_requirement::RequiresType::ProviderName(v) => {
                    struct_ser.serialize_field("providerName", v)?;
                }
                jwt_requirement::RequiresType::ProviderAndAudiences(v) => {
                    struct_ser.serialize_field("providerAndAudiences", v)?;
                }
                jwt_requirement::RequiresType::RequiresAny(v) => {
                    struct_ser.serialize_field("requiresAny", v)?;
                }
                jwt_requirement::RequiresType::RequiresAll(v) => {
                    struct_ser.serialize_field("requiresAll", v)?;
                }
                jwt_requirement::RequiresType::AllowMissingOrFailed(v) => {
                    struct_ser.serialize_field("allowMissingOrFailed", v)?;
                }
                jwt_requirement::RequiresType::AllowMissing(v) => {
                    struct_ser.serialize_field("allowMissing", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtRequirement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_name",
            "providerName",
            "provider_and_audiences",
            "providerAndAudiences",
            "requires_any",
            "requiresAny",
            "requires_all",
            "requiresAll",
            "allow_missing_or_failed",
            "allowMissingOrFailed",
            "allow_missing",
            "allowMissing",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderName,
            ProviderAndAudiences,
            RequiresAny,
            RequiresAll,
            AllowMissingOrFailed,
            AllowMissing,
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
                            "providerName" | "provider_name" => Ok(GeneratedField::ProviderName),
                            "providerAndAudiences" | "provider_and_audiences" => Ok(GeneratedField::ProviderAndAudiences),
                            "requiresAny" | "requires_any" => Ok(GeneratedField::RequiresAny),
                            "requiresAll" | "requires_all" => Ok(GeneratedField::RequiresAll),
                            "allowMissingOrFailed" | "allow_missing_or_failed" => Ok(GeneratedField::AllowMissingOrFailed),
                            "allowMissing" | "allow_missing" => Ok(GeneratedField::AllowMissing),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtRequirement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwtRequirement")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtRequirement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requires_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProviderName => {
                            if requires_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerName"));
                            }
                            requires_type__ = map.next_value::<::std::option::Option<_>>()?.map(jwt_requirement::RequiresType::ProviderName);
                        }
                        GeneratedField::ProviderAndAudiences => {
                            if requires_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerAndAudiences"));
                            }
                            requires_type__ = map.next_value::<::std::option::Option<_>>()?.map(jwt_requirement::RequiresType::ProviderAndAudiences)
;
                        }
                        GeneratedField::RequiresAny => {
                            if requires_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiresAny"));
                            }
                            requires_type__ = map.next_value::<::std::option::Option<_>>()?.map(jwt_requirement::RequiresType::RequiresAny)
;
                        }
                        GeneratedField::RequiresAll => {
                            if requires_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiresAll"));
                            }
                            requires_type__ = map.next_value::<::std::option::Option<_>>()?.map(jwt_requirement::RequiresType::RequiresAll)
;
                        }
                        GeneratedField::AllowMissingOrFailed => {
                            if requires_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowMissingOrFailed"));
                            }
                            requires_type__ = map.next_value::<::std::option::Option<_>>()?.map(jwt_requirement::RequiresType::AllowMissingOrFailed)
;
                        }
                        GeneratedField::AllowMissing => {
                            if requires_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowMissing"));
                            }
                            requires_type__ = map.next_value::<::std::option::Option<_>>()?.map(jwt_requirement::RequiresType::AllowMissing)
;
                        }
                    }
                }
                Ok(JwtRequirement {
                    requires_type: requires_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtRequirement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtRequirementAndList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requirements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtRequirementAndList", len)?;
        if !self.requirements.is_empty() {
            struct_ser.serialize_field("requirements", &self.requirements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtRequirementAndList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requirements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requirements,
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
                            "requirements" => Ok(GeneratedField::Requirements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtRequirementAndList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwtRequirementAndList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtRequirementAndList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requirements__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Requirements => {
                            if requirements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirements"));
                            }
                            requirements__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(JwtRequirementAndList {
                    requirements: requirements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtRequirementAndList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtRequirementOrList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requirements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtRequirementOrList", len)?;
        if !self.requirements.is_empty() {
            struct_ser.serialize_field("requirements", &self.requirements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtRequirementOrList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requirements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requirements,
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
                            "requirements" => Ok(GeneratedField::Requirements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtRequirementOrList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.JwtRequirementOrList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtRequirementOrList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requirements__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Requirements => {
                            if requirements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirements"));
                            }
                            requirements__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(JwtRequirementOrList {
                    requirements: requirements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.JwtRequirementOrList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PerRouteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.requirement_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.PerRouteConfig", len)?;
        if let Some(v) = self.requirement_specifier.as_ref() {
            match v {
                per_route_config::RequirementSpecifier::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                per_route_config::RequirementSpecifier::RequirementName(v) => {
                    struct_ser.serialize_field("requirementName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PerRouteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "requirement_name",
            "requirementName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
            RequirementName,
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
                            "disabled" => Ok(GeneratedField::Disabled),
                            "requirementName" | "requirement_name" => Ok(GeneratedField::RequirementName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PerRouteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.PerRouteConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PerRouteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requirement_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Disabled => {
                            if requirement_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            requirement_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(per_route_config::RequirementSpecifier::Disabled);
                        }
                        GeneratedField::RequirementName => {
                            if requirement_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirementName"));
                            }
                            requirement_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(per_route_config::RequirementSpecifier::RequirementName);
                        }
                    }
                }
                Ok(PerRouteConfig {
                    requirement_specifier: requirement_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.PerRouteConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderWithAudiences {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider_name.is_empty() {
            len += 1;
        }
        if !self.audiences.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.ProviderWithAudiences", len)?;
        if !self.provider_name.is_empty() {
            struct_ser.serialize_field("providerName", &self.provider_name)?;
        }
        if !self.audiences.is_empty() {
            struct_ser.serialize_field("audiences", &self.audiences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProviderWithAudiences {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_name",
            "providerName",
            "audiences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderName,
            Audiences,
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
                            "providerName" | "provider_name" => Ok(GeneratedField::ProviderName),
                            "audiences" => Ok(GeneratedField::Audiences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderWithAudiences;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.ProviderWithAudiences")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProviderWithAudiences, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_name__ = None;
                let mut audiences__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProviderName => {
                            if provider_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerName"));
                            }
                            provider_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Audiences => {
                            if audiences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audiences"));
                            }
                            audiences__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ProviderWithAudiences {
                    provider_name: provider_name__.unwrap_or_default(),
                    audiences: audiences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.ProviderWithAudiences", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoteJwks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http_uri.is_some() {
            len += 1;
        }
        if self.cache_duration.is_some() {
            len += 1;
        }
        if self.async_fetch.is_some() {
            len += 1;
        }
        if self.retry_policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.RemoteJwks", len)?;
        if let Some(v) = self.http_uri.as_ref() {
            struct_ser.serialize_field("httpUri", v)?;
        }
        if let Some(v) = self.cache_duration.as_ref() {
            struct_ser.serialize_field("cacheDuration", v)?;
        }
        if let Some(v) = self.async_fetch.as_ref() {
            struct_ser.serialize_field("asyncFetch", v)?;
        }
        if let Some(v) = self.retry_policy.as_ref() {
            struct_ser.serialize_field("retryPolicy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoteJwks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_uri",
            "httpUri",
            "cache_duration",
            "cacheDuration",
            "async_fetch",
            "asyncFetch",
            "retry_policy",
            "retryPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpUri,
            CacheDuration,
            AsyncFetch,
            RetryPolicy,
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
                            "httpUri" | "http_uri" => Ok(GeneratedField::HttpUri),
                            "cacheDuration" | "cache_duration" => Ok(GeneratedField::CacheDuration),
                            "asyncFetch" | "async_fetch" => Ok(GeneratedField::AsyncFetch),
                            "retryPolicy" | "retry_policy" => Ok(GeneratedField::RetryPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoteJwks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.RemoteJwks")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RemoteJwks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_uri__ = None;
                let mut cache_duration__ = None;
                let mut async_fetch__ = None;
                let mut retry_policy__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpUri => {
                            if http_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpUri"));
                            }
                            http_uri__ = map.next_value()?;
                        }
                        GeneratedField::CacheDuration => {
                            if cache_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheDuration"));
                            }
                            cache_duration__ = map.next_value()?;
                        }
                        GeneratedField::AsyncFetch => {
                            if async_fetch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asyncFetch"));
                            }
                            async_fetch__ = map.next_value()?;
                        }
                        GeneratedField::RetryPolicy => {
                            if retry_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicy"));
                            }
                            retry_policy__ = map.next_value()?;
                        }
                    }
                }
                Ok(RemoteJwks {
                    http_uri: http_uri__,
                    cache_duration: cache_duration__,
                    async_fetch: async_fetch__,
                    retry_policy: retry_policy__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.RemoteJwks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequirementRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#match.is_some() {
            len += 1;
        }
        if self.requirement_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.jwt_authn.v3.RequirementRule", len)?;
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if let Some(v) = self.requirement_type.as_ref() {
            match v {
                requirement_rule::RequirementType::Requires(v) => {
                    struct_ser.serialize_field("requires", v)?;
                }
                requirement_rule::RequirementType::RequirementName(v) => {
                    struct_ser.serialize_field("requirementName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequirementRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "match",
            "requires",
            "requirement_name",
            "requirementName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Match,
            Requires,
            RequirementName,
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
                            "match" => Ok(GeneratedField::Match),
                            "requires" => Ok(GeneratedField::Requires),
                            "requirementName" | "requirement_name" => Ok(GeneratedField::RequirementName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequirementRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.jwt_authn.v3.RequirementRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RequirementRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#match__ = None;
                let mut requirement_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map.next_value()?;
                        }
                        GeneratedField::Requires => {
                            if requirement_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requires"));
                            }
                            requirement_type__ = map.next_value::<::std::option::Option<_>>()?.map(requirement_rule::RequirementType::Requires)
;
                        }
                        GeneratedField::RequirementName => {
                            if requirement_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirementName"));
                            }
                            requirement_type__ = map.next_value::<::std::option::Option<_>>()?.map(requirement_rule::RequirementType::RequirementName);
                        }
                    }
                }
                Ok(RequirementRule {
                    r#match: r#match__,
                    requirement_type: requirement_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.jwt_authn.v3.RequirementRule", FIELDS, GeneratedVisitor)
    }
}
