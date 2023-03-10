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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.FilterStateRule", len)?;
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.FilterStateRule")
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
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.FilterStateRule", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtAuthentication", len)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Providers,
            Rules,
            FilterStateRules,
            BypassCorsPreflight,
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.JwtAuthentication")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JwtAuthentication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut providers__ = None;
                let mut rules__ = None;
                let mut filter_state_rules__ = None;
                let mut bypass_cors_preflight__ = None;
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
                    }
                }
                Ok(JwtAuthentication {
                    providers: providers__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    filter_state_rules: filter_state_rules__,
                    bypass_cors_preflight: bypass_cors_preflight__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtAuthentication", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtHeader", len)?;
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.JwtHeader")
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
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtHeader", FIELDS, GeneratedVisitor)
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
        if !self.forward_payload_header.is_empty() {
            len += 1;
        }
        if !self.payload_in_metadata.is_empty() {
            len += 1;
        }
        if self.jwks_source_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtProvider", len)?;
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
        if !self.forward_payload_header.is_empty() {
            struct_ser.serialize_field("forwardPayloadHeader", &self.forward_payload_header)?;
        }
        if !self.payload_in_metadata.is_empty() {
            struct_ser.serialize_field("payloadInMetadata", &self.payload_in_metadata)?;
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
            "forward_payload_header",
            "forwardPayloadHeader",
            "payload_in_metadata",
            "payloadInMetadata",
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
            ForwardPayloadHeader,
            PayloadInMetadata,
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
                            "forwardPayloadHeader" | "forward_payload_header" => Ok(GeneratedField::ForwardPayloadHeader),
                            "payloadInMetadata" | "payload_in_metadata" => Ok(GeneratedField::PayloadInMetadata),
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.JwtProvider")
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
                let mut forward_payload_header__ = None;
                let mut payload_in_metadata__ = None;
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
                        GeneratedField::ForwardPayloadHeader => {
                            if forward_payload_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardPayloadHeader"));
                            }
                            forward_payload_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::PayloadInMetadata => {
                            if payload_in_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadInMetadata"));
                            }
                            payload_in_metadata__ = Some(map.next_value()?);
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
                    forward_payload_header: forward_payload_header__.unwrap_or_default(),
                    payload_in_metadata: payload_in_metadata__.unwrap_or_default(),
                    jwks_source_specifier: jwks_source_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtProvider", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirement", len)?;
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirement")
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
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirement", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirementAndList", len)?;
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirementAndList")
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
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirementAndList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirementOrList", len)?;
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirementOrList")
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
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.JwtRequirementOrList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.ProviderWithAudiences", len)?;
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.ProviderWithAudiences")
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
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.ProviderWithAudiences", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.RemoteJwks", len)?;
        if let Some(v) = self.http_uri.as_ref() {
            struct_ser.serialize_field("httpUri", v)?;
        }
        if let Some(v) = self.cache_duration.as_ref() {
            struct_ser.serialize_field("cacheDuration", v)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpUri,
            CacheDuration,
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
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.RemoteJwks")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RemoteJwks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_uri__ = None;
                let mut cache_duration__ = None;
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
                    }
                }
                Ok(RemoteJwks {
                    http_uri: http_uri__,
                    cache_duration: cache_duration__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.RemoteJwks", FIELDS, GeneratedVisitor)
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
        if self.requires.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.RequirementRule", len)?;
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if let Some(v) = self.requires.as_ref() {
            struct_ser.serialize_field("requires", v)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Match,
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
                            "match" => Ok(GeneratedField::Match),
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
            type Value = RequirementRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.jwt_authn.v2alpha.RequirementRule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RequirementRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#match__ = None;
                let mut requires__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map.next_value()?;
                        }
                        GeneratedField::Requires => {
                            if requires__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requires"));
                            }
                            requires__ = map.next_value()?;
                        }
                    }
                }
                Ok(RequirementRule {
                    r#match: r#match__,
                    requires: requires__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.jwt_authn.v2alpha.RequirementRule", FIELDS, GeneratedVisitor)
    }
}
