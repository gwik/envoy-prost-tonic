// @generated
impl serde::Serialize for CorsPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allow_origin.is_empty() {
            len += 1;
        }
        if !self.allow_origin_regex.is_empty() {
            len += 1;
        }
        if !self.allow_origin_string_match.is_empty() {
            len += 1;
        }
        if !self.allow_methods.is_empty() {
            len += 1;
        }
        if !self.allow_headers.is_empty() {
            len += 1;
        }
        if !self.expose_headers.is_empty() {
            len += 1;
        }
        if !self.max_age.is_empty() {
            len += 1;
        }
        if self.allow_credentials.is_some() {
            len += 1;
        }
        if self.shadow_enabled.is_some() {
            len += 1;
        }
        if self.enabled_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.CorsPolicy", len)?;
        if !self.allow_origin.is_empty() {
            struct_ser.serialize_field("allowOrigin", &self.allow_origin)?;
        }
        if !self.allow_origin_regex.is_empty() {
            struct_ser.serialize_field("allowOriginRegex", &self.allow_origin_regex)?;
        }
        if !self.allow_origin_string_match.is_empty() {
            struct_ser.serialize_field("allowOriginStringMatch", &self.allow_origin_string_match)?;
        }
        if !self.allow_methods.is_empty() {
            struct_ser.serialize_field("allowMethods", &self.allow_methods)?;
        }
        if !self.allow_headers.is_empty() {
            struct_ser.serialize_field("allowHeaders", &self.allow_headers)?;
        }
        if !self.expose_headers.is_empty() {
            struct_ser.serialize_field("exposeHeaders", &self.expose_headers)?;
        }
        if !self.max_age.is_empty() {
            struct_ser.serialize_field("maxAge", &self.max_age)?;
        }
        if let Some(v) = self.allow_credentials.as_ref() {
            struct_ser.serialize_field("allowCredentials", v)?;
        }
        if let Some(v) = self.shadow_enabled.as_ref() {
            struct_ser.serialize_field("shadowEnabled", v)?;
        }
        if let Some(v) = self.enabled_specifier.as_ref() {
            match v {
                cors_policy::EnabledSpecifier::Enabled(v) => {
                    struct_ser.serialize_field("enabled", v)?;
                }
                cors_policy::EnabledSpecifier::FilterEnabled(v) => {
                    struct_ser.serialize_field("filterEnabled", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CorsPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allow_origin",
            "allowOrigin",
            "allow_origin_regex",
            "allowOriginRegex",
            "allow_origin_string_match",
            "allowOriginStringMatch",
            "allow_methods",
            "allowMethods",
            "allow_headers",
            "allowHeaders",
            "expose_headers",
            "exposeHeaders",
            "max_age",
            "maxAge",
            "allow_credentials",
            "allowCredentials",
            "shadow_enabled",
            "shadowEnabled",
            "enabled",
            "filter_enabled",
            "filterEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowOrigin,
            AllowOriginRegex,
            AllowOriginStringMatch,
            AllowMethods,
            AllowHeaders,
            ExposeHeaders,
            MaxAge,
            AllowCredentials,
            ShadowEnabled,
            Enabled,
            FilterEnabled,
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
                            "allowOrigin" | "allow_origin" => Ok(GeneratedField::AllowOrigin),
                            "allowOriginRegex" | "allow_origin_regex" => Ok(GeneratedField::AllowOriginRegex),
                            "allowOriginStringMatch" | "allow_origin_string_match" => Ok(GeneratedField::AllowOriginStringMatch),
                            "allowMethods" | "allow_methods" => Ok(GeneratedField::AllowMethods),
                            "allowHeaders" | "allow_headers" => Ok(GeneratedField::AllowHeaders),
                            "exposeHeaders" | "expose_headers" => Ok(GeneratedField::ExposeHeaders),
                            "maxAge" | "max_age" => Ok(GeneratedField::MaxAge),
                            "allowCredentials" | "allow_credentials" => Ok(GeneratedField::AllowCredentials),
                            "shadowEnabled" | "shadow_enabled" => Ok(GeneratedField::ShadowEnabled),
                            "enabled" => Ok(GeneratedField::Enabled),
                            "filterEnabled" | "filter_enabled" => Ok(GeneratedField::FilterEnabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorsPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.CorsPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CorsPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allow_origin__ = None;
                let mut allow_origin_regex__ = None;
                let mut allow_origin_string_match__ = None;
                let mut allow_methods__ = None;
                let mut allow_headers__ = None;
                let mut expose_headers__ = None;
                let mut max_age__ = None;
                let mut allow_credentials__ = None;
                let mut shadow_enabled__ = None;
                let mut enabled_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowOrigin => {
                            if allow_origin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowOrigin"));
                            }
                            allow_origin__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowOriginRegex => {
                            if allow_origin_regex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowOriginRegex"));
                            }
                            allow_origin_regex__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowOriginStringMatch => {
                            if allow_origin_string_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowOriginStringMatch"));
                            }
                            allow_origin_string_match__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowMethods => {
                            if allow_methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowMethods"));
                            }
                            allow_methods__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowHeaders => {
                            if allow_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowHeaders"));
                            }
                            allow_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExposeHeaders => {
                            if expose_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exposeHeaders"));
                            }
                            expose_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxAge => {
                            if max_age__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAge"));
                            }
                            max_age__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowCredentials => {
                            if allow_credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowCredentials"));
                            }
                            allow_credentials__ = map.next_value()?;
                        }
                        GeneratedField::ShadowEnabled => {
                            if shadow_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shadowEnabled"));
                            }
                            shadow_enabled__ = map.next_value()?;
                        }
                        GeneratedField::Enabled => {
                            if enabled_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(cors_policy::EnabledSpecifier::Enabled)
;
                        }
                        GeneratedField::FilterEnabled => {
                            if enabled_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabled"));
                            }
                            enabled_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(cors_policy::EnabledSpecifier::FilterEnabled)
;
                        }
                    }
                }
                Ok(CorsPolicy {
                    allow_origin: allow_origin__.unwrap_or_default(),
                    allow_origin_regex: allow_origin_regex__.unwrap_or_default(),
                    allow_origin_string_match: allow_origin_string_match__.unwrap_or_default(),
                    allow_methods: allow_methods__.unwrap_or_default(),
                    allow_headers: allow_headers__.unwrap_or_default(),
                    expose_headers: expose_headers__.unwrap_or_default(),
                    max_age: max_age__.unwrap_or_default(),
                    allow_credentials: allow_credentials__,
                    shadow_enabled: shadow_enabled__,
                    enabled_specifier: enabled_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.CorsPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Decorator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operation.is_empty() {
            len += 1;
        }
        if self.propagate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.Decorator", len)?;
        if !self.operation.is_empty() {
            struct_ser.serialize_field("operation", &self.operation)?;
        }
        if let Some(v) = self.propagate.as_ref() {
            struct_ser.serialize_field("propagate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decorator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation",
            "propagate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
            Propagate,
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
                            "operation" => Ok(GeneratedField::Operation),
                            "propagate" => Ok(GeneratedField::Propagate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Decorator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.Decorator")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Decorator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation__ = None;
                let mut propagate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map.next_value()?);
                        }
                        GeneratedField::Propagate => {
                            if propagate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("propagate"));
                            }
                            propagate__ = map.next_value()?;
                        }
                    }
                }
                Ok(Decorator {
                    operation: operation__.unwrap_or_default(),
                    propagate: propagate__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.Decorator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectResponseAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.body.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.DirectResponseAction", len)?;
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.body.as_ref() {
            struct_ser.serialize_field("body", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectResponseAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "body",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Body,
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
                            "status" => Ok(GeneratedField::Status),
                            "body" => Ok(GeneratedField::Body),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectResponseAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.DirectResponseAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DirectResponseAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut body__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = map.next_value()?;
                        }
                    }
                }
                Ok(DirectResponseAction {
                    status: status__.unwrap_or_default(),
                    body: body__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.DirectResponseAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.FilterAction", len)?;
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Action,
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
                            "action" => Ok(GeneratedField::Action),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.FilterAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map.next_value()?;
                        }
                    }
                }
                Ok(FilterAction {
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.FilterAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderMatcher {
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
        if self.invert_match {
            len += 1;
        }
        if self.header_match_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.HeaderMatcher", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.invert_match {
            struct_ser.serialize_field("invertMatch", &self.invert_match)?;
        }
        if let Some(v) = self.header_match_specifier.as_ref() {
            match v {
                header_matcher::HeaderMatchSpecifier::ExactMatch(v) => {
                    struct_ser.serialize_field("exactMatch", v)?;
                }
                header_matcher::HeaderMatchSpecifier::RegexMatch(v) => {
                    struct_ser.serialize_field("regexMatch", v)?;
                }
                header_matcher::HeaderMatchSpecifier::SafeRegexMatch(v) => {
                    struct_ser.serialize_field("safeRegexMatch", v)?;
                }
                header_matcher::HeaderMatchSpecifier::RangeMatch(v) => {
                    struct_ser.serialize_field("rangeMatch", v)?;
                }
                header_matcher::HeaderMatchSpecifier::PresentMatch(v) => {
                    struct_ser.serialize_field("presentMatch", v)?;
                }
                header_matcher::HeaderMatchSpecifier::PrefixMatch(v) => {
                    struct_ser.serialize_field("prefixMatch", v)?;
                }
                header_matcher::HeaderMatchSpecifier::SuffixMatch(v) => {
                    struct_ser.serialize_field("suffixMatch", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "invert_match",
            "invertMatch",
            "exact_match",
            "exactMatch",
            "regex_match",
            "regexMatch",
            "safe_regex_match",
            "safeRegexMatch",
            "range_match",
            "rangeMatch",
            "present_match",
            "presentMatch",
            "prefix_match",
            "prefixMatch",
            "suffix_match",
            "suffixMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            InvertMatch,
            ExactMatch,
            RegexMatch,
            SafeRegexMatch,
            RangeMatch,
            PresentMatch,
            PrefixMatch,
            SuffixMatch,
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
                            "invertMatch" | "invert_match" => Ok(GeneratedField::InvertMatch),
                            "exactMatch" | "exact_match" => Ok(GeneratedField::ExactMatch),
                            "regexMatch" | "regex_match" => Ok(GeneratedField::RegexMatch),
                            "safeRegexMatch" | "safe_regex_match" => Ok(GeneratedField::SafeRegexMatch),
                            "rangeMatch" | "range_match" => Ok(GeneratedField::RangeMatch),
                            "presentMatch" | "present_match" => Ok(GeneratedField::PresentMatch),
                            "prefixMatch" | "prefix_match" => Ok(GeneratedField::PrefixMatch),
                            "suffixMatch" | "suffix_match" => Ok(GeneratedField::SuffixMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.HeaderMatcher")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut invert_match__ = None;
                let mut header_match_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::InvertMatch => {
                            if invert_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invertMatch"));
                            }
                            invert_match__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExactMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exactMatch"));
                            }
                            header_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::ExactMatch);
                        }
                        GeneratedField::RegexMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexMatch"));
                            }
                            header_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::RegexMatch);
                        }
                        GeneratedField::SafeRegexMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("safeRegexMatch"));
                            }
                            header_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::SafeRegexMatch)
;
                        }
                        GeneratedField::RangeMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rangeMatch"));
                            }
                            header_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::RangeMatch)
;
                        }
                        GeneratedField::PresentMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentMatch"));
                            }
                            header_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::PresentMatch);
                        }
                        GeneratedField::PrefixMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixMatch"));
                            }
                            header_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::PrefixMatch);
                        }
                        GeneratedField::SuffixMatch => {
                            if header_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffixMatch"));
                            }
                            header_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(header_matcher::HeaderMatchSpecifier::SuffixMatch);
                        }
                    }
                }
                Ok(HeaderMatcher {
                    name: name__.unwrap_or_default(),
                    invert_match: invert_match__.unwrap_or_default(),
                    header_match_specifier: header_match_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.HeaderMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HedgePolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.initial_requests.is_some() {
            len += 1;
        }
        if self.additional_request_chance.is_some() {
            len += 1;
        }
        if self.hedge_on_per_try_timeout {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.HedgePolicy", len)?;
        if let Some(v) = self.initial_requests.as_ref() {
            struct_ser.serialize_field("initialRequests", v)?;
        }
        if let Some(v) = self.additional_request_chance.as_ref() {
            struct_ser.serialize_field("additionalRequestChance", v)?;
        }
        if self.hedge_on_per_try_timeout {
            struct_ser.serialize_field("hedgeOnPerTryTimeout", &self.hedge_on_per_try_timeout)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HedgePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "initial_requests",
            "initialRequests",
            "additional_request_chance",
            "additionalRequestChance",
            "hedge_on_per_try_timeout",
            "hedgeOnPerTryTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InitialRequests,
            AdditionalRequestChance,
            HedgeOnPerTryTimeout,
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
                            "initialRequests" | "initial_requests" => Ok(GeneratedField::InitialRequests),
                            "additionalRequestChance" | "additional_request_chance" => Ok(GeneratedField::AdditionalRequestChance),
                            "hedgeOnPerTryTimeout" | "hedge_on_per_try_timeout" => Ok(GeneratedField::HedgeOnPerTryTimeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HedgePolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.HedgePolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HedgePolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut initial_requests__ = None;
                let mut additional_request_chance__ = None;
                let mut hedge_on_per_try_timeout__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InitialRequests => {
                            if initial_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialRequests"));
                            }
                            initial_requests__ = map.next_value()?;
                        }
                        GeneratedField::AdditionalRequestChance => {
                            if additional_request_chance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalRequestChance"));
                            }
                            additional_request_chance__ = map.next_value()?;
                        }
                        GeneratedField::HedgeOnPerTryTimeout => {
                            if hedge_on_per_try_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hedgeOnPerTryTimeout"));
                            }
                            hedge_on_per_try_timeout__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HedgePolicy {
                    initial_requests: initial_requests__,
                    additional_request_chance: additional_request_chance__,
                    hedge_on_per_try_timeout: hedge_on_per_try_timeout__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.HedgePolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParameterMatcher {
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
        if !self.value.is_empty() {
            len += 1;
        }
        if self.regex.is_some() {
            len += 1;
        }
        if self.query_parameter_match_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.QueryParameterMatcher", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if let Some(v) = self.regex.as_ref() {
            struct_ser.serialize_field("regex", v)?;
        }
        if let Some(v) = self.query_parameter_match_specifier.as_ref() {
            match v {
                query_parameter_matcher::QueryParameterMatchSpecifier::StringMatch(v) => {
                    struct_ser.serialize_field("stringMatch", v)?;
                }
                query_parameter_matcher::QueryParameterMatchSpecifier::PresentMatch(v) => {
                    struct_ser.serialize_field("presentMatch", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParameterMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
            "regex",
            "string_match",
            "stringMatch",
            "present_match",
            "presentMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
            Regex,
            StringMatch,
            PresentMatch,
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
                            "value" => Ok(GeneratedField::Value),
                            "regex" => Ok(GeneratedField::Regex),
                            "stringMatch" | "string_match" => Ok(GeneratedField::StringMatch),
                            "presentMatch" | "present_match" => Ok(GeneratedField::PresentMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParameterMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.QueryParameterMatcher")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParameterMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                let mut regex__ = None;
                let mut query_parameter_match_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Regex => {
                            if regex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            regex__ = map.next_value()?;
                        }
                        GeneratedField::StringMatch => {
                            if query_parameter_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringMatch"));
                            }
                            query_parameter_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(query_parameter_matcher::QueryParameterMatchSpecifier::StringMatch)
;
                        }
                        GeneratedField::PresentMatch => {
                            if query_parameter_match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentMatch"));
                            }
                            query_parameter_match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(query_parameter_matcher::QueryParameterMatchSpecifier::PresentMatch);
                        }
                    }
                }
                Ok(QueryParameterMatcher {
                    name: name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    regex: regex__,
                    query_parameter_match_specifier: query_parameter_match_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.QueryParameterMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stage.is_some() {
            len += 1;
        }
        if !self.disable_key.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RateLimit", len)?;
        if let Some(v) = self.stage.as_ref() {
            struct_ser.serialize_field("stage", v)?;
        }
        if !self.disable_key.is_empty() {
            struct_ser.serialize_field("disableKey", &self.disable_key)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stage",
            "disable_key",
            "disableKey",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stage,
            DisableKey,
            Actions,
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
                            "stage" => Ok(GeneratedField::Stage),
                            "disableKey" | "disable_key" => Ok(GeneratedField::DisableKey),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RateLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stage__ = None;
                let mut disable_key__ = None;
                let mut actions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Stage => {
                            if stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            stage__ = map.next_value()?;
                        }
                        GeneratedField::DisableKey => {
                            if disable_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableKey"));
                            }
                            disable_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RateLimit {
                    stage: stage__,
                    disable_key: disable_key__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RateLimit.Action", len)?;
        if let Some(v) = self.action_specifier.as_ref() {
            match v {
                rate_limit::action::ActionSpecifier::SourceCluster(v) => {
                    struct_ser.serialize_field("sourceCluster", v)?;
                }
                rate_limit::action::ActionSpecifier::DestinationCluster(v) => {
                    struct_ser.serialize_field("destinationCluster", v)?;
                }
                rate_limit::action::ActionSpecifier::RequestHeaders(v) => {
                    struct_ser.serialize_field("requestHeaders", v)?;
                }
                rate_limit::action::ActionSpecifier::RemoteAddress(v) => {
                    struct_ser.serialize_field("remoteAddress", v)?;
                }
                rate_limit::action::ActionSpecifier::GenericKey(v) => {
                    struct_ser.serialize_field("genericKey", v)?;
                }
                rate_limit::action::ActionSpecifier::HeaderValueMatch(v) => {
                    struct_ser.serialize_field("headerValueMatch", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_cluster",
            "sourceCluster",
            "destination_cluster",
            "destinationCluster",
            "request_headers",
            "requestHeaders",
            "remote_address",
            "remoteAddress",
            "generic_key",
            "genericKey",
            "header_value_match",
            "headerValueMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceCluster,
            DestinationCluster,
            RequestHeaders,
            RemoteAddress,
            GenericKey,
            HeaderValueMatch,
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
                            "sourceCluster" | "source_cluster" => Ok(GeneratedField::SourceCluster),
                            "destinationCluster" | "destination_cluster" => Ok(GeneratedField::DestinationCluster),
                            "requestHeaders" | "request_headers" => Ok(GeneratedField::RequestHeaders),
                            "remoteAddress" | "remote_address" => Ok(GeneratedField::RemoteAddress),
                            "genericKey" | "generic_key" => Ok(GeneratedField::GenericKey),
                            "headerValueMatch" | "header_value_match" => Ok(GeneratedField::HeaderValueMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RateLimit.Action")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit::Action, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceCluster => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceCluster"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::SourceCluster)
;
                        }
                        GeneratedField::DestinationCluster => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationCluster"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::DestinationCluster)
;
                        }
                        GeneratedField::RequestHeaders => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaders"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::RequestHeaders)
;
                        }
                        GeneratedField::RemoteAddress => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteAddress"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::RemoteAddress)
;
                        }
                        GeneratedField::GenericKey => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genericKey"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::GenericKey)
;
                        }
                        GeneratedField::HeaderValueMatch => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerValueMatch"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit::action::ActionSpecifier::HeaderValueMatch)
;
                        }
                    }
                }
                Ok(rate_limit::Action {
                    action_specifier: action_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RateLimit.Action", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::DestinationCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.route.RateLimit.Action.DestinationCluster", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::DestinationCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::DestinationCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RateLimit.Action.DestinationCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit::action::DestinationCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit::action::DestinationCluster {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RateLimit.Action.DestinationCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::GenericKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RateLimit.Action.GenericKey", len)?;
        if !self.descriptor_value.is_empty() {
            struct_ser.serialize_field("descriptorValue", &self.descriptor_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::GenericKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_value",
            "descriptorValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorValue,
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
                            "descriptorValue" | "descriptor_value" => Ok(GeneratedField::DescriptorValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::GenericKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RateLimit.Action.GenericKey")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit::action::GenericKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DescriptorValue => {
                            if descriptor_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorValue"));
                            }
                            descriptor_value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::GenericKey {
                    descriptor_value: descriptor_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RateLimit.Action.GenericKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::HeaderValueMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_value.is_empty() {
            len += 1;
        }
        if self.expect_match.is_some() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RateLimit.Action.HeaderValueMatch", len)?;
        if !self.descriptor_value.is_empty() {
            struct_ser.serialize_field("descriptorValue", &self.descriptor_value)?;
        }
        if let Some(v) = self.expect_match.as_ref() {
            struct_ser.serialize_field("expectMatch", v)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::HeaderValueMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_value",
            "descriptorValue",
            "expect_match",
            "expectMatch",
            "headers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorValue,
            ExpectMatch,
            Headers,
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
                            "descriptorValue" | "descriptor_value" => Ok(GeneratedField::DescriptorValue),
                            "expectMatch" | "expect_match" => Ok(GeneratedField::ExpectMatch),
                            "headers" => Ok(GeneratedField::Headers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::HeaderValueMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RateLimit.Action.HeaderValueMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit::action::HeaderValueMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_value__ = None;
                let mut expect_match__ = None;
                let mut headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DescriptorValue => {
                            if descriptor_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorValue"));
                            }
                            descriptor_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExpectMatch => {
                            if expect_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expectMatch"));
                            }
                            expect_match__ = map.next_value()?;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::HeaderValueMatch {
                    descriptor_value: descriptor_value__.unwrap_or_default(),
                    expect_match: expect_match__,
                    headers: headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RateLimit.Action.HeaderValueMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::RemoteAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.route.RateLimit.Action.RemoteAddress", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::RemoteAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::RemoteAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RateLimit.Action.RemoteAddress")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit::action::RemoteAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit::action::RemoteAddress {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RateLimit.Action.RemoteAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::RequestHeaders {
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
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RateLimit.Action.RequestHeaders", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("headerName", &self.header_name)?;
        }
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptorKey", &self.descriptor_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::RequestHeaders {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
            "descriptor_key",
            "descriptorKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderName,
            DescriptorKey,
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
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::RequestHeaders;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RateLimit.Action.RequestHeaders")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit::action::RequestHeaders, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                let mut descriptor_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(rate_limit::action::RequestHeaders {
                    header_name: header_name__.unwrap_or_default(),
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RateLimit.Action.RequestHeaders", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::action::SourceCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.route.RateLimit.Action.SourceCluster", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::action::SourceCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::action::SourceCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RateLimit.Action.SourceCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit::action::SourceCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit::action::SourceCluster {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RateLimit.Action.SourceCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RedirectAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_redirect.is_empty() {
            len += 1;
        }
        if self.port_redirect != 0 {
            len += 1;
        }
        if self.response_code != 0 {
            len += 1;
        }
        if self.strip_query {
            len += 1;
        }
        if self.scheme_rewrite_specifier.is_some() {
            len += 1;
        }
        if self.path_rewrite_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RedirectAction", len)?;
        if !self.host_redirect.is_empty() {
            struct_ser.serialize_field("hostRedirect", &self.host_redirect)?;
        }
        if self.port_redirect != 0 {
            struct_ser.serialize_field("portRedirect", &self.port_redirect)?;
        }
        if self.response_code != 0 {
            let v = redirect_action::RedirectResponseCode::from_i32(self.response_code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.response_code)))?;
            struct_ser.serialize_field("responseCode", &v)?;
        }
        if self.strip_query {
            struct_ser.serialize_field("stripQuery", &self.strip_query)?;
        }
        if let Some(v) = self.scheme_rewrite_specifier.as_ref() {
            match v {
                redirect_action::SchemeRewriteSpecifier::HttpsRedirect(v) => {
                    struct_ser.serialize_field("httpsRedirect", v)?;
                }
                redirect_action::SchemeRewriteSpecifier::SchemeRedirect(v) => {
                    struct_ser.serialize_field("schemeRedirect", v)?;
                }
            }
        }
        if let Some(v) = self.path_rewrite_specifier.as_ref() {
            match v {
                redirect_action::PathRewriteSpecifier::PathRedirect(v) => {
                    struct_ser.serialize_field("pathRedirect", v)?;
                }
                redirect_action::PathRewriteSpecifier::PrefixRewrite(v) => {
                    struct_ser.serialize_field("prefixRewrite", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedirectAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_redirect",
            "hostRedirect",
            "port_redirect",
            "portRedirect",
            "response_code",
            "responseCode",
            "strip_query",
            "stripQuery",
            "https_redirect",
            "httpsRedirect",
            "scheme_redirect",
            "schemeRedirect",
            "path_redirect",
            "pathRedirect",
            "prefix_rewrite",
            "prefixRewrite",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostRedirect,
            PortRedirect,
            ResponseCode,
            StripQuery,
            HttpsRedirect,
            SchemeRedirect,
            PathRedirect,
            PrefixRewrite,
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
                            "hostRedirect" | "host_redirect" => Ok(GeneratedField::HostRedirect),
                            "portRedirect" | "port_redirect" => Ok(GeneratedField::PortRedirect),
                            "responseCode" | "response_code" => Ok(GeneratedField::ResponseCode),
                            "stripQuery" | "strip_query" => Ok(GeneratedField::StripQuery),
                            "httpsRedirect" | "https_redirect" => Ok(GeneratedField::HttpsRedirect),
                            "schemeRedirect" | "scheme_redirect" => Ok(GeneratedField::SchemeRedirect),
                            "pathRedirect" | "path_redirect" => Ok(GeneratedField::PathRedirect),
                            "prefixRewrite" | "prefix_rewrite" => Ok(GeneratedField::PrefixRewrite),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedirectAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RedirectAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RedirectAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_redirect__ = None;
                let mut port_redirect__ = None;
                let mut response_code__ = None;
                let mut strip_query__ = None;
                let mut scheme_rewrite_specifier__ = None;
                let mut path_rewrite_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostRedirect => {
                            if host_redirect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRedirect"));
                            }
                            host_redirect__ = Some(map.next_value()?);
                        }
                        GeneratedField::PortRedirect => {
                            if port_redirect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portRedirect"));
                            }
                            port_redirect__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResponseCode => {
                            if response_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseCode"));
                            }
                            response_code__ = Some(map.next_value::<redirect_action::RedirectResponseCode>()? as i32);
                        }
                        GeneratedField::StripQuery => {
                            if strip_query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripQuery"));
                            }
                            strip_query__ = Some(map.next_value()?);
                        }
                        GeneratedField::HttpsRedirect => {
                            if scheme_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpsRedirect"));
                            }
                            scheme_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(redirect_action::SchemeRewriteSpecifier::HttpsRedirect);
                        }
                        GeneratedField::SchemeRedirect => {
                            if scheme_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemeRedirect"));
                            }
                            scheme_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(redirect_action::SchemeRewriteSpecifier::SchemeRedirect);
                        }
                        GeneratedField::PathRedirect => {
                            if path_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathRedirect"));
                            }
                            path_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(redirect_action::PathRewriteSpecifier::PathRedirect);
                        }
                        GeneratedField::PrefixRewrite => {
                            if path_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixRewrite"));
                            }
                            path_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(redirect_action::PathRewriteSpecifier::PrefixRewrite);
                        }
                    }
                }
                Ok(RedirectAction {
                    host_redirect: host_redirect__.unwrap_or_default(),
                    port_redirect: port_redirect__.unwrap_or_default(),
                    response_code: response_code__.unwrap_or_default(),
                    strip_query: strip_query__.unwrap_or_default(),
                    scheme_rewrite_specifier: scheme_rewrite_specifier__,
                    path_rewrite_specifier: path_rewrite_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RedirectAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for redirect_action::RedirectResponseCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::MovedPermanently => "MOVED_PERMANENTLY",
            Self::Found => "FOUND",
            Self::SeeOther => "SEE_OTHER",
            Self::TemporaryRedirect => "TEMPORARY_REDIRECT",
            Self::PermanentRedirect => "PERMANENT_REDIRECT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for redirect_action::RedirectResponseCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MOVED_PERMANENTLY",
            "FOUND",
            "SEE_OTHER",
            "TEMPORARY_REDIRECT",
            "PERMANENT_REDIRECT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = redirect_action::RedirectResponseCode;

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
                    .and_then(redirect_action::RedirectResponseCode::from_i32)
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
                    .and_then(redirect_action::RedirectResponseCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MOVED_PERMANENTLY" => Ok(redirect_action::RedirectResponseCode::MovedPermanently),
                    "FOUND" => Ok(redirect_action::RedirectResponseCode::Found),
                    "SEE_OTHER" => Ok(redirect_action::RedirectResponseCode::SeeOther),
                    "TEMPORARY_REDIRECT" => Ok(redirect_action::RedirectResponseCode::TemporaryRedirect),
                    "PERMANENT_REDIRECT" => Ok(redirect_action::RedirectResponseCode::PermanentRedirect),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RetryPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.retry_on.is_empty() {
            len += 1;
        }
        if self.num_retries.is_some() {
            len += 1;
        }
        if self.per_try_timeout.is_some() {
            len += 1;
        }
        if self.retry_priority.is_some() {
            len += 1;
        }
        if !self.retry_host_predicate.is_empty() {
            len += 1;
        }
        if self.host_selection_retry_max_attempts != 0 {
            len += 1;
        }
        if !self.retriable_status_codes.is_empty() {
            len += 1;
        }
        if self.retry_back_off.is_some() {
            len += 1;
        }
        if !self.retriable_headers.is_empty() {
            len += 1;
        }
        if !self.retriable_request_headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RetryPolicy", len)?;
        if !self.retry_on.is_empty() {
            struct_ser.serialize_field("retryOn", &self.retry_on)?;
        }
        if let Some(v) = self.num_retries.as_ref() {
            struct_ser.serialize_field("numRetries", v)?;
        }
        if let Some(v) = self.per_try_timeout.as_ref() {
            struct_ser.serialize_field("perTryTimeout", v)?;
        }
        if let Some(v) = self.retry_priority.as_ref() {
            struct_ser.serialize_field("retryPriority", v)?;
        }
        if !self.retry_host_predicate.is_empty() {
            struct_ser.serialize_field("retryHostPredicate", &self.retry_host_predicate)?;
        }
        if self.host_selection_retry_max_attempts != 0 {
            struct_ser.serialize_field("hostSelectionRetryMaxAttempts", ToString::to_string(&self.host_selection_retry_max_attempts).as_str())?;
        }
        if !self.retriable_status_codes.is_empty() {
            struct_ser.serialize_field("retriableStatusCodes", &self.retriable_status_codes)?;
        }
        if let Some(v) = self.retry_back_off.as_ref() {
            struct_ser.serialize_field("retryBackOff", v)?;
        }
        if !self.retriable_headers.is_empty() {
            struct_ser.serialize_field("retriableHeaders", &self.retriable_headers)?;
        }
        if !self.retriable_request_headers.is_empty() {
            struct_ser.serialize_field("retriableRequestHeaders", &self.retriable_request_headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "retry_on",
            "retryOn",
            "num_retries",
            "numRetries",
            "per_try_timeout",
            "perTryTimeout",
            "retry_priority",
            "retryPriority",
            "retry_host_predicate",
            "retryHostPredicate",
            "host_selection_retry_max_attempts",
            "hostSelectionRetryMaxAttempts",
            "retriable_status_codes",
            "retriableStatusCodes",
            "retry_back_off",
            "retryBackOff",
            "retriable_headers",
            "retriableHeaders",
            "retriable_request_headers",
            "retriableRequestHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RetryOn,
            NumRetries,
            PerTryTimeout,
            RetryPriority,
            RetryHostPredicate,
            HostSelectionRetryMaxAttempts,
            RetriableStatusCodes,
            RetryBackOff,
            RetriableHeaders,
            RetriableRequestHeaders,
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
                            "retryOn" | "retry_on" => Ok(GeneratedField::RetryOn),
                            "numRetries" | "num_retries" => Ok(GeneratedField::NumRetries),
                            "perTryTimeout" | "per_try_timeout" => Ok(GeneratedField::PerTryTimeout),
                            "retryPriority" | "retry_priority" => Ok(GeneratedField::RetryPriority),
                            "retryHostPredicate" | "retry_host_predicate" => Ok(GeneratedField::RetryHostPredicate),
                            "hostSelectionRetryMaxAttempts" | "host_selection_retry_max_attempts" => Ok(GeneratedField::HostSelectionRetryMaxAttempts),
                            "retriableStatusCodes" | "retriable_status_codes" => Ok(GeneratedField::RetriableStatusCodes),
                            "retryBackOff" | "retry_back_off" => Ok(GeneratedField::RetryBackOff),
                            "retriableHeaders" | "retriable_headers" => Ok(GeneratedField::RetriableHeaders),
                            "retriableRequestHeaders" | "retriable_request_headers" => Ok(GeneratedField::RetriableRequestHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RetryPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RetryPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut retry_on__ = None;
                let mut num_retries__ = None;
                let mut per_try_timeout__ = None;
                let mut retry_priority__ = None;
                let mut retry_host_predicate__ = None;
                let mut host_selection_retry_max_attempts__ = None;
                let mut retriable_status_codes__ = None;
                let mut retry_back_off__ = None;
                let mut retriable_headers__ = None;
                let mut retriable_request_headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RetryOn => {
                            if retry_on__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryOn"));
                            }
                            retry_on__ = Some(map.next_value()?);
                        }
                        GeneratedField::NumRetries => {
                            if num_retries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numRetries"));
                            }
                            num_retries__ = map.next_value()?;
                        }
                        GeneratedField::PerTryTimeout => {
                            if per_try_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perTryTimeout"));
                            }
                            per_try_timeout__ = map.next_value()?;
                        }
                        GeneratedField::RetryPriority => {
                            if retry_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPriority"));
                            }
                            retry_priority__ = map.next_value()?;
                        }
                        GeneratedField::RetryHostPredicate => {
                            if retry_host_predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryHostPredicate"));
                            }
                            retry_host_predicate__ = Some(map.next_value()?);
                        }
                        GeneratedField::HostSelectionRetryMaxAttempts => {
                            if host_selection_retry_max_attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostSelectionRetryMaxAttempts"));
                            }
                            host_selection_retry_max_attempts__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RetriableStatusCodes => {
                            if retriable_status_codes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retriableStatusCodes"));
                            }
                            retriable_status_codes__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::RetryBackOff => {
                            if retry_back_off__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryBackOff"));
                            }
                            retry_back_off__ = map.next_value()?;
                        }
                        GeneratedField::RetriableHeaders => {
                            if retriable_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retriableHeaders"));
                            }
                            retriable_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetriableRequestHeaders => {
                            if retriable_request_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retriableRequestHeaders"));
                            }
                            retriable_request_headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RetryPolicy {
                    retry_on: retry_on__.unwrap_or_default(),
                    num_retries: num_retries__,
                    per_try_timeout: per_try_timeout__,
                    retry_priority: retry_priority__,
                    retry_host_predicate: retry_host_predicate__.unwrap_or_default(),
                    host_selection_retry_max_attempts: host_selection_retry_max_attempts__.unwrap_or_default(),
                    retriable_status_codes: retriable_status_codes__.unwrap_or_default(),
                    retry_back_off: retry_back_off__,
                    retriable_headers: retriable_headers__.unwrap_or_default(),
                    retriable_request_headers: retriable_request_headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RetryPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::RetryBackOff {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_interval.is_some() {
            len += 1;
        }
        if self.max_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RetryPolicy.RetryBackOff", len)?;
        if let Some(v) = self.base_interval.as_ref() {
            struct_ser.serialize_field("baseInterval", v)?;
        }
        if let Some(v) = self.max_interval.as_ref() {
            struct_ser.serialize_field("maxInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::RetryBackOff {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_interval",
            "baseInterval",
            "max_interval",
            "maxInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseInterval,
            MaxInterval,
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
                            "baseInterval" | "base_interval" => Ok(GeneratedField::BaseInterval),
                            "maxInterval" | "max_interval" => Ok(GeneratedField::MaxInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = retry_policy::RetryBackOff;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RetryPolicy.RetryBackOff")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<retry_policy::RetryBackOff, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_interval__ = None;
                let mut max_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BaseInterval => {
                            if base_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseInterval"));
                            }
                            base_interval__ = map.next_value()?;
                        }
                        GeneratedField::MaxInterval => {
                            if max_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInterval"));
                            }
                            max_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(retry_policy::RetryBackOff {
                    base_interval: base_interval__,
                    max_interval: max_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RetryPolicy.RetryBackOff", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::RetryHostPredicate {
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
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RetryPolicy.RetryHostPredicate", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                retry_policy::retry_host_predicate::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                retry_policy::retry_host_predicate::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::RetryHostPredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Config,
            TypedConfig,
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
                            "config" => Ok(GeneratedField::Config),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = retry_policy::RetryHostPredicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RetryPolicy.RetryHostPredicate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<retry_policy::RetryHostPredicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(retry_policy::retry_host_predicate::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(retry_policy::retry_host_predicate::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(retry_policy::RetryHostPredicate {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RetryPolicy.RetryHostPredicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for retry_policy::RetryPriority {
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
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RetryPolicy.RetryPriority", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                retry_policy::retry_priority::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                retry_policy::retry_priority::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for retry_policy::RetryPriority {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Config,
            TypedConfig,
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
                            "config" => Ok(GeneratedField::Config),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = retry_policy::RetryPriority;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RetryPolicy.RetryPriority")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<retry_policy::RetryPriority, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(retry_policy::retry_priority::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(retry_policy::retry_priority::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(retry_policy::RetryPriority {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RetryPolicy.RetryPriority", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Route {
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
        if self.r#match.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.decorator.is_some() {
            len += 1;
        }
        if !self.per_filter_config.is_empty() {
            len += 1;
        }
        if !self.typed_per_filter_config.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_remove.is_empty() {
            len += 1;
        }
        if self.tracing.is_some() {
            len += 1;
        }
        if self.per_request_buffer_limit_bytes.is_some() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.Route", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.decorator.as_ref() {
            struct_ser.serialize_field("decorator", v)?;
        }
        if !self.per_filter_config.is_empty() {
            struct_ser.serialize_field("perFilterConfig", &self.per_filter_config)?;
        }
        if !self.typed_per_filter_config.is_empty() {
            struct_ser.serialize_field("typedPerFilterConfig", &self.typed_per_filter_config)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("requestHeadersToAdd", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("requestHeadersToRemove", &self.request_headers_to_remove)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        if !self.response_headers_to_remove.is_empty() {
            struct_ser.serialize_field("responseHeadersToRemove", &self.response_headers_to_remove)?;
        }
        if let Some(v) = self.tracing.as_ref() {
            struct_ser.serialize_field("tracing", v)?;
        }
        if let Some(v) = self.per_request_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("perRequestBufferLimitBytes", v)?;
        }
        if let Some(v) = self.action.as_ref() {
            match v {
                route::Action::Route(v) => {
                    struct_ser.serialize_field("route", v)?;
                }
                route::Action::Redirect(v) => {
                    struct_ser.serialize_field("redirect", v)?;
                }
                route::Action::DirectResponse(v) => {
                    struct_ser.serialize_field("directResponse", v)?;
                }
                route::Action::FilterAction(v) => {
                    struct_ser.serialize_field("filterAction", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Route {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "match",
            "metadata",
            "decorator",
            "per_filter_config",
            "perFilterConfig",
            "typed_per_filter_config",
            "typedPerFilterConfig",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "response_headers_to_remove",
            "responseHeadersToRemove",
            "tracing",
            "per_request_buffer_limit_bytes",
            "perRequestBufferLimitBytes",
            "route",
            "redirect",
            "direct_response",
            "directResponse",
            "filter_action",
            "filterAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Match,
            Metadata,
            Decorator,
            PerFilterConfig,
            TypedPerFilterConfig,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            ResponseHeadersToAdd,
            ResponseHeadersToRemove,
            Tracing,
            PerRequestBufferLimitBytes,
            Route,
            Redirect,
            DirectResponse,
            FilterAction,
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
                            "match" => Ok(GeneratedField::Match),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "decorator" => Ok(GeneratedField::Decorator),
                            "perFilterConfig" | "per_filter_config" => Ok(GeneratedField::PerFilterConfig),
                            "typedPerFilterConfig" | "typed_per_filter_config" => Ok(GeneratedField::TypedPerFilterConfig),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "responseHeadersToRemove" | "response_headers_to_remove" => Ok(GeneratedField::ResponseHeadersToRemove),
                            "tracing" => Ok(GeneratedField::Tracing),
                            "perRequestBufferLimitBytes" | "per_request_buffer_limit_bytes" => Ok(GeneratedField::PerRequestBufferLimitBytes),
                            "route" => Ok(GeneratedField::Route),
                            "redirect" => Ok(GeneratedField::Redirect),
                            "directResponse" | "direct_response" => Ok(GeneratedField::DirectResponse),
                            "filterAction" | "filter_action" => Ok(GeneratedField::FilterAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Route;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.Route")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Route, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#match__ = None;
                let mut metadata__ = None;
                let mut decorator__ = None;
                let mut per_filter_config__ = None;
                let mut typed_per_filter_config__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut response_headers_to_add__ = None;
                let mut response_headers_to_remove__ = None;
                let mut tracing__ = None;
                let mut per_request_buffer_limit_bytes__ = None;
                let mut action__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::Decorator => {
                            if decorator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decorator"));
                            }
                            decorator__ = map.next_value()?;
                        }
                        GeneratedField::PerFilterConfig => {
                            if per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perFilterConfig"));
                            }
                            per_filter_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::TypedPerFilterConfig => {
                            if typed_per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedPerFilterConfig"));
                            }
                            typed_per_filter_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToRemove => {
                            if response_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToRemove"));
                            }
                            response_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::Tracing => {
                            if tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracing"));
                            }
                            tracing__ = map.next_value()?;
                        }
                        GeneratedField::PerRequestBufferLimitBytes => {
                            if per_request_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perRequestBufferLimitBytes"));
                            }
                            per_request_buffer_limit_bytes__ = map.next_value()?;
                        }
                        GeneratedField::Route => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            action__ = map.next_value::<::std::option::Option<_>>()?.map(route::Action::Route)
;
                        }
                        GeneratedField::Redirect => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirect"));
                            }
                            action__ = map.next_value::<::std::option::Option<_>>()?.map(route::Action::Redirect)
;
                        }
                        GeneratedField::DirectResponse => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directResponse"));
                            }
                            action__ = map.next_value::<::std::option::Option<_>>()?.map(route::Action::DirectResponse)
;
                        }
                        GeneratedField::FilterAction => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterAction"));
                            }
                            action__ = map.next_value::<::std::option::Option<_>>()?.map(route::Action::FilterAction)
;
                        }
                    }
                }
                Ok(Route {
                    name: name__.unwrap_or_default(),
                    r#match: r#match__,
                    metadata: metadata__,
                    decorator: decorator__,
                    per_filter_config: per_filter_config__.unwrap_or_default(),
                    typed_per_filter_config: typed_per_filter_config__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    response_headers_to_remove: response_headers_to_remove__.unwrap_or_default(),
                    tracing: tracing__,
                    per_request_buffer_limit_bytes: per_request_buffer_limit_bytes__,
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.Route", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cluster_not_found_response_code != 0 {
            len += 1;
        }
        if self.metadata_match.is_some() {
            len += 1;
        }
        if !self.prefix_rewrite.is_empty() {
            len += 1;
        }
        if self.regex_rewrite.is_some() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.retry_policy.is_some() {
            len += 1;
        }
        if self.retry_policy_typed_config.is_some() {
            len += 1;
        }
        if self.request_mirror_policy.is_some() {
            len += 1;
        }
        if !self.request_mirror_policies.is_empty() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if !self.rate_limits.is_empty() {
            len += 1;
        }
        if self.include_vh_rate_limits.is_some() {
            len += 1;
        }
        if !self.hash_policy.is_empty() {
            len += 1;
        }
        if self.cors.is_some() {
            len += 1;
        }
        if self.max_grpc_timeout.is_some() {
            len += 1;
        }
        if self.grpc_timeout_offset.is_some() {
            len += 1;
        }
        if !self.upgrade_configs.is_empty() {
            len += 1;
        }
        if self.internal_redirect_action != 0 {
            len += 1;
        }
        if self.max_internal_redirects.is_some() {
            len += 1;
        }
        if self.hedge_policy.is_some() {
            len += 1;
        }
        if self.cluster_specifier.is_some() {
            len += 1;
        }
        if self.host_rewrite_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction", len)?;
        if self.cluster_not_found_response_code != 0 {
            let v = route_action::ClusterNotFoundResponseCode::from_i32(self.cluster_not_found_response_code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.cluster_not_found_response_code)))?;
            struct_ser.serialize_field("clusterNotFoundResponseCode", &v)?;
        }
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadataMatch", v)?;
        }
        if !self.prefix_rewrite.is_empty() {
            struct_ser.serialize_field("prefixRewrite", &self.prefix_rewrite)?;
        }
        if let Some(v) = self.regex_rewrite.as_ref() {
            struct_ser.serialize_field("regexRewrite", v)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idleTimeout", v)?;
        }
        if let Some(v) = self.retry_policy.as_ref() {
            struct_ser.serialize_field("retryPolicy", v)?;
        }
        if let Some(v) = self.retry_policy_typed_config.as_ref() {
            struct_ser.serialize_field("retryPolicyTypedConfig", v)?;
        }
        if let Some(v) = self.request_mirror_policy.as_ref() {
            struct_ser.serialize_field("requestMirrorPolicy", v)?;
        }
        if !self.request_mirror_policies.is_empty() {
            struct_ser.serialize_field("requestMirrorPolicies", &self.request_mirror_policies)?;
        }
        if self.priority != 0 {
            let v = super::core::RoutingPriority::from_i32(self.priority)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.priority)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        if !self.rate_limits.is_empty() {
            struct_ser.serialize_field("rateLimits", &self.rate_limits)?;
        }
        if let Some(v) = self.include_vh_rate_limits.as_ref() {
            struct_ser.serialize_field("includeVhRateLimits", v)?;
        }
        if !self.hash_policy.is_empty() {
            struct_ser.serialize_field("hashPolicy", &self.hash_policy)?;
        }
        if let Some(v) = self.cors.as_ref() {
            struct_ser.serialize_field("cors", v)?;
        }
        if let Some(v) = self.max_grpc_timeout.as_ref() {
            struct_ser.serialize_field("maxGrpcTimeout", v)?;
        }
        if let Some(v) = self.grpc_timeout_offset.as_ref() {
            struct_ser.serialize_field("grpcTimeoutOffset", v)?;
        }
        if !self.upgrade_configs.is_empty() {
            struct_ser.serialize_field("upgradeConfigs", &self.upgrade_configs)?;
        }
        if self.internal_redirect_action != 0 {
            let v = route_action::InternalRedirectAction::from_i32(self.internal_redirect_action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.internal_redirect_action)))?;
            struct_ser.serialize_field("internalRedirectAction", &v)?;
        }
        if let Some(v) = self.max_internal_redirects.as_ref() {
            struct_ser.serialize_field("maxInternalRedirects", v)?;
        }
        if let Some(v) = self.hedge_policy.as_ref() {
            struct_ser.serialize_field("hedgePolicy", v)?;
        }
        if let Some(v) = self.cluster_specifier.as_ref() {
            match v {
                route_action::ClusterSpecifier::Cluster(v) => {
                    struct_ser.serialize_field("cluster", v)?;
                }
                route_action::ClusterSpecifier::ClusterHeader(v) => {
                    struct_ser.serialize_field("clusterHeader", v)?;
                }
                route_action::ClusterSpecifier::WeightedClusters(v) => {
                    struct_ser.serialize_field("weightedClusters", v)?;
                }
            }
        }
        if let Some(v) = self.host_rewrite_specifier.as_ref() {
            match v {
                route_action::HostRewriteSpecifier::HostRewrite(v) => {
                    struct_ser.serialize_field("hostRewrite", v)?;
                }
                route_action::HostRewriteSpecifier::AutoHostRewrite(v) => {
                    struct_ser.serialize_field("autoHostRewrite", v)?;
                }
                route_action::HostRewriteSpecifier::AutoHostRewriteHeader(v) => {
                    struct_ser.serialize_field("autoHostRewriteHeader", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_not_found_response_code",
            "clusterNotFoundResponseCode",
            "metadata_match",
            "metadataMatch",
            "prefix_rewrite",
            "prefixRewrite",
            "regex_rewrite",
            "regexRewrite",
            "timeout",
            "idle_timeout",
            "idleTimeout",
            "retry_policy",
            "retryPolicy",
            "retry_policy_typed_config",
            "retryPolicyTypedConfig",
            "request_mirror_policy",
            "requestMirrorPolicy",
            "request_mirror_policies",
            "requestMirrorPolicies",
            "priority",
            "rate_limits",
            "rateLimits",
            "include_vh_rate_limits",
            "includeVhRateLimits",
            "hash_policy",
            "hashPolicy",
            "cors",
            "max_grpc_timeout",
            "maxGrpcTimeout",
            "grpc_timeout_offset",
            "grpcTimeoutOffset",
            "upgrade_configs",
            "upgradeConfigs",
            "internal_redirect_action",
            "internalRedirectAction",
            "max_internal_redirects",
            "maxInternalRedirects",
            "hedge_policy",
            "hedgePolicy",
            "cluster",
            "cluster_header",
            "clusterHeader",
            "weighted_clusters",
            "weightedClusters",
            "host_rewrite",
            "hostRewrite",
            "auto_host_rewrite",
            "autoHostRewrite",
            "auto_host_rewrite_header",
            "autoHostRewriteHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterNotFoundResponseCode,
            MetadataMatch,
            PrefixRewrite,
            RegexRewrite,
            Timeout,
            IdleTimeout,
            RetryPolicy,
            RetryPolicyTypedConfig,
            RequestMirrorPolicy,
            RequestMirrorPolicies,
            Priority,
            RateLimits,
            IncludeVhRateLimits,
            HashPolicy,
            Cors,
            MaxGrpcTimeout,
            GrpcTimeoutOffset,
            UpgradeConfigs,
            InternalRedirectAction,
            MaxInternalRedirects,
            HedgePolicy,
            Cluster,
            ClusterHeader,
            WeightedClusters,
            HostRewrite,
            AutoHostRewrite,
            AutoHostRewriteHeader,
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
                            "clusterNotFoundResponseCode" | "cluster_not_found_response_code" => Ok(GeneratedField::ClusterNotFoundResponseCode),
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            "prefixRewrite" | "prefix_rewrite" => Ok(GeneratedField::PrefixRewrite),
                            "regexRewrite" | "regex_rewrite" => Ok(GeneratedField::RegexRewrite),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "retryPolicy" | "retry_policy" => Ok(GeneratedField::RetryPolicy),
                            "retryPolicyTypedConfig" | "retry_policy_typed_config" => Ok(GeneratedField::RetryPolicyTypedConfig),
                            "requestMirrorPolicy" | "request_mirror_policy" => Ok(GeneratedField::RequestMirrorPolicy),
                            "requestMirrorPolicies" | "request_mirror_policies" => Ok(GeneratedField::RequestMirrorPolicies),
                            "priority" => Ok(GeneratedField::Priority),
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            "includeVhRateLimits" | "include_vh_rate_limits" => Ok(GeneratedField::IncludeVhRateLimits),
                            "hashPolicy" | "hash_policy" => Ok(GeneratedField::HashPolicy),
                            "cors" => Ok(GeneratedField::Cors),
                            "maxGrpcTimeout" | "max_grpc_timeout" => Ok(GeneratedField::MaxGrpcTimeout),
                            "grpcTimeoutOffset" | "grpc_timeout_offset" => Ok(GeneratedField::GrpcTimeoutOffset),
                            "upgradeConfigs" | "upgrade_configs" => Ok(GeneratedField::UpgradeConfigs),
                            "internalRedirectAction" | "internal_redirect_action" => Ok(GeneratedField::InternalRedirectAction),
                            "maxInternalRedirects" | "max_internal_redirects" => Ok(GeneratedField::MaxInternalRedirects),
                            "hedgePolicy" | "hedge_policy" => Ok(GeneratedField::HedgePolicy),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "clusterHeader" | "cluster_header" => Ok(GeneratedField::ClusterHeader),
                            "weightedClusters" | "weighted_clusters" => Ok(GeneratedField::WeightedClusters),
                            "hostRewrite" | "host_rewrite" => Ok(GeneratedField::HostRewrite),
                            "autoHostRewrite" | "auto_host_rewrite" => Ok(GeneratedField::AutoHostRewrite),
                            "autoHostRewriteHeader" | "auto_host_rewrite_header" => Ok(GeneratedField::AutoHostRewriteHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_not_found_response_code__ = None;
                let mut metadata_match__ = None;
                let mut prefix_rewrite__ = None;
                let mut regex_rewrite__ = None;
                let mut timeout__ = None;
                let mut idle_timeout__ = None;
                let mut retry_policy__ = None;
                let mut retry_policy_typed_config__ = None;
                let mut request_mirror_policy__ = None;
                let mut request_mirror_policies__ = None;
                let mut priority__ = None;
                let mut rate_limits__ = None;
                let mut include_vh_rate_limits__ = None;
                let mut hash_policy__ = None;
                let mut cors__ = None;
                let mut max_grpc_timeout__ = None;
                let mut grpc_timeout_offset__ = None;
                let mut upgrade_configs__ = None;
                let mut internal_redirect_action__ = None;
                let mut max_internal_redirects__ = None;
                let mut hedge_policy__ = None;
                let mut cluster_specifier__ = None;
                let mut host_rewrite_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterNotFoundResponseCode => {
                            if cluster_not_found_response_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterNotFoundResponseCode"));
                            }
                            cluster_not_found_response_code__ = Some(map.next_value::<route_action::ClusterNotFoundResponseCode>()? as i32);
                        }
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map.next_value()?;
                        }
                        GeneratedField::PrefixRewrite => {
                            if prefix_rewrite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixRewrite"));
                            }
                            prefix_rewrite__ = Some(map.next_value()?);
                        }
                        GeneratedField::RegexRewrite => {
                            if regex_rewrite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexRewrite"));
                            }
                            regex_rewrite__ = map.next_value()?;
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map.next_value()?;
                        }
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::RetryPolicy => {
                            if retry_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicy"));
                            }
                            retry_policy__ = map.next_value()?;
                        }
                        GeneratedField::RetryPolicyTypedConfig => {
                            if retry_policy_typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicyTypedConfig"));
                            }
                            retry_policy_typed_config__ = map.next_value()?;
                        }
                        GeneratedField::RequestMirrorPolicy => {
                            if request_mirror_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMirrorPolicy"));
                            }
                            request_mirror_policy__ = map.next_value()?;
                        }
                        GeneratedField::RequestMirrorPolicies => {
                            if request_mirror_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMirrorPolicies"));
                            }
                            request_mirror_policies__ = Some(map.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = Some(map.next_value::<super::core::RoutingPriority>()? as i32);
                        }
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map.next_value()?);
                        }
                        GeneratedField::IncludeVhRateLimits => {
                            if include_vh_rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeVhRateLimits"));
                            }
                            include_vh_rate_limits__ = map.next_value()?;
                        }
                        GeneratedField::HashPolicy => {
                            if hash_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashPolicy"));
                            }
                            hash_policy__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cors => {
                            if cors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cors"));
                            }
                            cors__ = map.next_value()?;
                        }
                        GeneratedField::MaxGrpcTimeout => {
                            if max_grpc_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGrpcTimeout"));
                            }
                            max_grpc_timeout__ = map.next_value()?;
                        }
                        GeneratedField::GrpcTimeoutOffset => {
                            if grpc_timeout_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcTimeoutOffset"));
                            }
                            grpc_timeout_offset__ = map.next_value()?;
                        }
                        GeneratedField::UpgradeConfigs => {
                            if upgrade_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeConfigs"));
                            }
                            upgrade_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::InternalRedirectAction => {
                            if internal_redirect_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalRedirectAction"));
                            }
                            internal_redirect_action__ = Some(map.next_value::<route_action::InternalRedirectAction>()? as i32);
                        }
                        GeneratedField::MaxInternalRedirects => {
                            if max_internal_redirects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInternalRedirects"));
                            }
                            max_internal_redirects__ = map.next_value()?;
                        }
                        GeneratedField::HedgePolicy => {
                            if hedge_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hedgePolicy"));
                            }
                            hedge_policy__ = map.next_value()?;
                        }
                        GeneratedField::Cluster => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::Cluster);
                        }
                        GeneratedField::ClusterHeader => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterHeader"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::ClusterHeader);
                        }
                        GeneratedField::WeightedClusters => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weightedClusters"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::WeightedClusters)
;
                        }
                        GeneratedField::HostRewrite => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRewrite"));
                            }
                            host_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::HostRewriteSpecifier::HostRewrite);
                        }
                        GeneratedField::AutoHostRewrite => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoHostRewrite"));
                            }
                            host_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::HostRewriteSpecifier::AutoHostRewrite)
;
                        }
                        GeneratedField::AutoHostRewriteHeader => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoHostRewriteHeader"));
                            }
                            host_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::HostRewriteSpecifier::AutoHostRewriteHeader);
                        }
                    }
                }
                Ok(RouteAction {
                    cluster_not_found_response_code: cluster_not_found_response_code__.unwrap_or_default(),
                    metadata_match: metadata_match__,
                    prefix_rewrite: prefix_rewrite__.unwrap_or_default(),
                    regex_rewrite: regex_rewrite__,
                    timeout: timeout__,
                    idle_timeout: idle_timeout__,
                    retry_policy: retry_policy__,
                    retry_policy_typed_config: retry_policy_typed_config__,
                    request_mirror_policy: request_mirror_policy__,
                    request_mirror_policies: request_mirror_policies__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    rate_limits: rate_limits__.unwrap_or_default(),
                    include_vh_rate_limits: include_vh_rate_limits__,
                    hash_policy: hash_policy__.unwrap_or_default(),
                    cors: cors__,
                    max_grpc_timeout: max_grpc_timeout__,
                    grpc_timeout_offset: grpc_timeout_offset__,
                    upgrade_configs: upgrade_configs__.unwrap_or_default(),
                    internal_redirect_action: internal_redirect_action__.unwrap_or_default(),
                    max_internal_redirects: max_internal_redirects__,
                    hedge_policy: hedge_policy__,
                    cluster_specifier: cluster_specifier__,
                    host_rewrite_specifier: host_rewrite_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::ClusterNotFoundResponseCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ServiceUnavailable => "SERVICE_UNAVAILABLE",
            Self::NotFound => "NOT_FOUND",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for route_action::ClusterNotFoundResponseCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SERVICE_UNAVAILABLE",
            "NOT_FOUND",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::ClusterNotFoundResponseCode;

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
                    .and_then(route_action::ClusterNotFoundResponseCode::from_i32)
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
                    .and_then(route_action::ClusterNotFoundResponseCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SERVICE_UNAVAILABLE" => Ok(route_action::ClusterNotFoundResponseCode::ServiceUnavailable),
                    "NOT_FOUND" => Ok(route_action::ClusterNotFoundResponseCode::NotFound),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::HashPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.terminal {
            len += 1;
        }
        if self.policy_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction.HashPolicy", len)?;
        if self.terminal {
            struct_ser.serialize_field("terminal", &self.terminal)?;
        }
        if let Some(v) = self.policy_specifier.as_ref() {
            match v {
                route_action::hash_policy::PolicySpecifier::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                route_action::hash_policy::PolicySpecifier::Cookie(v) => {
                    struct_ser.serialize_field("cookie", v)?;
                }
                route_action::hash_policy::PolicySpecifier::ConnectionProperties(v) => {
                    struct_ser.serialize_field("connectionProperties", v)?;
                }
                route_action::hash_policy::PolicySpecifier::QueryParameter(v) => {
                    struct_ser.serialize_field("queryParameter", v)?;
                }
                route_action::hash_policy::PolicySpecifier::FilterState(v) => {
                    struct_ser.serialize_field("filterState", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::HashPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "terminal",
            "header",
            "cookie",
            "connection_properties",
            "connectionProperties",
            "query_parameter",
            "queryParameter",
            "filter_state",
            "filterState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Terminal,
            Header,
            Cookie,
            ConnectionProperties,
            QueryParameter,
            FilterState,
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
                            "terminal" => Ok(GeneratedField::Terminal),
                            "header" => Ok(GeneratedField::Header),
                            "cookie" => Ok(GeneratedField::Cookie),
                            "connectionProperties" | "connection_properties" => Ok(GeneratedField::ConnectionProperties),
                            "queryParameter" | "query_parameter" => Ok(GeneratedField::QueryParameter),
                            "filterState" | "filter_state" => Ok(GeneratedField::FilterState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::HashPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction.HashPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::HashPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut terminal__ = None;
                let mut policy_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Terminal => {
                            if terminal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terminal"));
                            }
                            terminal__ = Some(map.next_value()?);
                        }
                        GeneratedField::Header => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            policy_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::Header)
;
                        }
                        GeneratedField::Cookie => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cookie"));
                            }
                            policy_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::Cookie)
;
                        }
                        GeneratedField::ConnectionProperties => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionProperties"));
                            }
                            policy_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::ConnectionProperties)
;
                        }
                        GeneratedField::QueryParameter => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParameter"));
                            }
                            policy_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::QueryParameter)
;
                        }
                        GeneratedField::FilterState => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterState"));
                            }
                            policy_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::hash_policy::PolicySpecifier::FilterState)
;
                        }
                    }
                }
                Ok(route_action::HashPolicy {
                    terminal: terminal__.unwrap_or_default(),
                    policy_specifier: policy_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction.HashPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::ConnectionProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_ip {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.ConnectionProperties", len)?;
        if self.source_ip {
            struct_ser.serialize_field("sourceIp", &self.source_ip)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::ConnectionProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_ip",
            "sourceIp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceIp,
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
                            "sourceIp" | "source_ip" => Ok(GeneratedField::SourceIp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::ConnectionProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction.HashPolicy.ConnectionProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::hash_policy::ConnectionProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_ip__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceIp => {
                            if source_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIp"));
                            }
                            source_ip__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::ConnectionProperties {
                    source_ip: source_ip__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.ConnectionProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::Cookie {
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
        if self.ttl.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.Cookie", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.ttl.as_ref() {
            struct_ser.serialize_field("ttl", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::Cookie {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "ttl",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Ttl,
            Path,
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
                            "ttl" => Ok(GeneratedField::Ttl),
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::Cookie;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction.HashPolicy.Cookie")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::hash_policy::Cookie, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut ttl__ = None;
                let mut path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ttl => {
                            if ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttl"));
                            }
                            ttl__ = map.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::Cookie {
                    name: name__.unwrap_or_default(),
                    ttl: ttl__,
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.Cookie", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::FilterState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.FilterState", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::FilterState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::FilterState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction.HashPolicy.FilterState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::hash_policy::FilterState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::FilterState {
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.FilterState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::Header {
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
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.Header", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("headerName", &self.header_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::Header {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderName,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::Header;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction.HashPolicy.Header")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::hash_policy::Header, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::Header {
                    header_name: header_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.Header", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::hash_policy::QueryParameter {
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
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.QueryParameter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::hash_policy::QueryParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::hash_policy::QueryParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction.HashPolicy.QueryParameter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::hash_policy::QueryParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(route_action::hash_policy::QueryParameter {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction.HashPolicy.QueryParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::InternalRedirectAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::PassThroughInternalRedirect => "PASS_THROUGH_INTERNAL_REDIRECT",
            Self::HandleInternalRedirect => "HANDLE_INTERNAL_REDIRECT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for route_action::InternalRedirectAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PASS_THROUGH_INTERNAL_REDIRECT",
            "HANDLE_INTERNAL_REDIRECT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::InternalRedirectAction;

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
                    .and_then(route_action::InternalRedirectAction::from_i32)
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
                    .and_then(route_action::InternalRedirectAction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PASS_THROUGH_INTERNAL_REDIRECT" => Ok(route_action::InternalRedirectAction::PassThroughInternalRedirect),
                    "HANDLE_INTERNAL_REDIRECT" => Ok(route_action::InternalRedirectAction::HandleInternalRedirect),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::RequestMirrorPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster.is_empty() {
            len += 1;
        }
        if !self.runtime_key.is_empty() {
            len += 1;
        }
        if self.runtime_fraction.is_some() {
            len += 1;
        }
        if self.trace_sampled.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction.RequestMirrorPolicy", len)?;
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if !self.runtime_key.is_empty() {
            struct_ser.serialize_field("runtimeKey", &self.runtime_key)?;
        }
        if let Some(v) = self.runtime_fraction.as_ref() {
            struct_ser.serialize_field("runtimeFraction", v)?;
        }
        if let Some(v) = self.trace_sampled.as_ref() {
            struct_ser.serialize_field("traceSampled", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::RequestMirrorPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "runtime_key",
            "runtimeKey",
            "runtime_fraction",
            "runtimeFraction",
            "trace_sampled",
            "traceSampled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            RuntimeKey,
            RuntimeFraction,
            TraceSampled,
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
                            "cluster" => Ok(GeneratedField::Cluster),
                            "runtimeKey" | "runtime_key" => Ok(GeneratedField::RuntimeKey),
                            "runtimeFraction" | "runtime_fraction" => Ok(GeneratedField::RuntimeFraction),
                            "traceSampled" | "trace_sampled" => Ok(GeneratedField::TraceSampled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::RequestMirrorPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction.RequestMirrorPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::RequestMirrorPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut runtime_key__ = None;
                let mut runtime_fraction__ = None;
                let mut trace_sampled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::RuntimeKey => {
                            if runtime_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKey"));
                            }
                            runtime_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::RuntimeFraction => {
                            if runtime_fraction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeFraction"));
                            }
                            runtime_fraction__ = map.next_value()?;
                        }
                        GeneratedField::TraceSampled => {
                            if trace_sampled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceSampled"));
                            }
                            trace_sampled__ = map.next_value()?;
                        }
                    }
                }
                Ok(route_action::RequestMirrorPolicy {
                    cluster: cluster__.unwrap_or_default(),
                    runtime_key: runtime_key__.unwrap_or_default(),
                    runtime_fraction: runtime_fraction__,
                    trace_sampled: trace_sampled__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction.RequestMirrorPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::UpgradeConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.upgrade_type.is_empty() {
            len += 1;
        }
        if self.enabled.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteAction.UpgradeConfig", len)?;
        if !self.upgrade_type.is_empty() {
            struct_ser.serialize_field("upgradeType", &self.upgrade_type)?;
        }
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::UpgradeConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upgrade_type",
            "upgradeType",
            "enabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpgradeType,
            Enabled,
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
                            "upgradeType" | "upgrade_type" => Ok(GeneratedField::UpgradeType),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::UpgradeConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteAction.UpgradeConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::UpgradeConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upgrade_type__ = None;
                let mut enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UpgradeType => {
                            if upgrade_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeType"));
                            }
                            upgrade_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map.next_value()?;
                        }
                    }
                }
                Ok(route_action::UpgradeConfig {
                    upgrade_type: upgrade_type__.unwrap_or_default(),
                    enabled: enabled__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteAction.UpgradeConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.case_sensitive.is_some() {
            len += 1;
        }
        if self.runtime_fraction.is_some() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.query_parameters.is_empty() {
            len += 1;
        }
        if self.grpc.is_some() {
            len += 1;
        }
        if self.tls_context.is_some() {
            len += 1;
        }
        if self.path_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteMatch", len)?;
        if let Some(v) = self.case_sensitive.as_ref() {
            struct_ser.serialize_field("caseSensitive", v)?;
        }
        if let Some(v) = self.runtime_fraction.as_ref() {
            struct_ser.serialize_field("runtimeFraction", v)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.query_parameters.is_empty() {
            struct_ser.serialize_field("queryParameters", &self.query_parameters)?;
        }
        if let Some(v) = self.grpc.as_ref() {
            struct_ser.serialize_field("grpc", v)?;
        }
        if let Some(v) = self.tls_context.as_ref() {
            struct_ser.serialize_field("tlsContext", v)?;
        }
        if let Some(v) = self.path_specifier.as_ref() {
            match v {
                route_match::PathSpecifier::Prefix(v) => {
                    struct_ser.serialize_field("prefix", v)?;
                }
                route_match::PathSpecifier::Path(v) => {
                    struct_ser.serialize_field("path", v)?;
                }
                route_match::PathSpecifier::Regex(v) => {
                    struct_ser.serialize_field("regex", v)?;
                }
                route_match::PathSpecifier::SafeRegex(v) => {
                    struct_ser.serialize_field("safeRegex", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "case_sensitive",
            "caseSensitive",
            "runtime_fraction",
            "runtimeFraction",
            "headers",
            "query_parameters",
            "queryParameters",
            "grpc",
            "tls_context",
            "tlsContext",
            "prefix",
            "path",
            "regex",
            "safe_regex",
            "safeRegex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CaseSensitive,
            RuntimeFraction,
            Headers,
            QueryParameters,
            Grpc,
            TlsContext,
            Prefix,
            Path,
            Regex,
            SafeRegex,
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
                            "caseSensitive" | "case_sensitive" => Ok(GeneratedField::CaseSensitive),
                            "runtimeFraction" | "runtime_fraction" => Ok(GeneratedField::RuntimeFraction),
                            "headers" => Ok(GeneratedField::Headers),
                            "queryParameters" | "query_parameters" => Ok(GeneratedField::QueryParameters),
                            "grpc" => Ok(GeneratedField::Grpc),
                            "tlsContext" | "tls_context" => Ok(GeneratedField::TlsContext),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "path" => Ok(GeneratedField::Path),
                            "regex" => Ok(GeneratedField::Regex),
                            "safeRegex" | "safe_regex" => Ok(GeneratedField::SafeRegex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut case_sensitive__ = None;
                let mut runtime_fraction__ = None;
                let mut headers__ = None;
                let mut query_parameters__ = None;
                let mut grpc__ = None;
                let mut tls_context__ = None;
                let mut path_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CaseSensitive => {
                            if case_sensitive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caseSensitive"));
                            }
                            case_sensitive__ = map.next_value()?;
                        }
                        GeneratedField::RuntimeFraction => {
                            if runtime_fraction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeFraction"));
                            }
                            runtime_fraction__ = map.next_value()?;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::QueryParameters => {
                            if query_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParameters"));
                            }
                            query_parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Grpc => {
                            if grpc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpc"));
                            }
                            grpc__ = map.next_value()?;
                        }
                        GeneratedField::TlsContext => {
                            if tls_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsContext"));
                            }
                            tls_context__ = map.next_value()?;
                        }
                        GeneratedField::Prefix => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            path_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::Prefix);
                        }
                        GeneratedField::Path => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::Path);
                        }
                        GeneratedField::Regex => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            path_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::Regex);
                        }
                        GeneratedField::SafeRegex => {
                            if path_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("safeRegex"));
                            }
                            path_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_match::PathSpecifier::SafeRegex)
;
                        }
                    }
                }
                Ok(RouteMatch {
                    case_sensitive: case_sensitive__,
                    runtime_fraction: runtime_fraction__,
                    headers: headers__.unwrap_or_default(),
                    query_parameters: query_parameters__.unwrap_or_default(),
                    grpc: grpc__,
                    tls_context: tls_context__,
                    path_specifier: path_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_match::GrpcRouteMatchOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteMatch.GrpcRouteMatchOptions", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_match::GrpcRouteMatchOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_match::GrpcRouteMatchOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteMatch.GrpcRouteMatchOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_match::GrpcRouteMatchOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(route_match::GrpcRouteMatchOptions {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteMatch.GrpcRouteMatchOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_match::TlsContextMatchOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.presented.is_some() {
            len += 1;
        }
        if self.validated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.RouteMatch.TlsContextMatchOptions", len)?;
        if let Some(v) = self.presented.as_ref() {
            struct_ser.serialize_field("presented", v)?;
        }
        if let Some(v) = self.validated.as_ref() {
            struct_ser.serialize_field("validated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_match::TlsContextMatchOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "presented",
            "validated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Presented,
            Validated,
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
                            "presented" => Ok(GeneratedField::Presented),
                            "validated" => Ok(GeneratedField::Validated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_match::TlsContextMatchOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.RouteMatch.TlsContextMatchOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_match::TlsContextMatchOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut presented__ = None;
                let mut validated__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Presented => {
                            if presented__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presented"));
                            }
                            presented__ = map.next_value()?;
                        }
                        GeneratedField::Validated => {
                            if validated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validated"));
                            }
                            validated__ = map.next_value()?;
                        }
                    }
                }
                Ok(route_match::TlsContextMatchOptions {
                    presented: presented__,
                    validated: validated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.RouteMatch.TlsContextMatchOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tracing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.client_sampling.is_some() {
            len += 1;
        }
        if self.random_sampling.is_some() {
            len += 1;
        }
        if self.overall_sampling.is_some() {
            len += 1;
        }
        if !self.custom_tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.Tracing", len)?;
        if let Some(v) = self.client_sampling.as_ref() {
            struct_ser.serialize_field("clientSampling", v)?;
        }
        if let Some(v) = self.random_sampling.as_ref() {
            struct_ser.serialize_field("randomSampling", v)?;
        }
        if let Some(v) = self.overall_sampling.as_ref() {
            struct_ser.serialize_field("overallSampling", v)?;
        }
        if !self.custom_tags.is_empty() {
            struct_ser.serialize_field("customTags", &self.custom_tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tracing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_sampling",
            "clientSampling",
            "random_sampling",
            "randomSampling",
            "overall_sampling",
            "overallSampling",
            "custom_tags",
            "customTags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientSampling,
            RandomSampling,
            OverallSampling,
            CustomTags,
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
                            "clientSampling" | "client_sampling" => Ok(GeneratedField::ClientSampling),
                            "randomSampling" | "random_sampling" => Ok(GeneratedField::RandomSampling),
                            "overallSampling" | "overall_sampling" => Ok(GeneratedField::OverallSampling),
                            "customTags" | "custom_tags" => Ok(GeneratedField::CustomTags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tracing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.Tracing")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Tracing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_sampling__ = None;
                let mut random_sampling__ = None;
                let mut overall_sampling__ = None;
                let mut custom_tags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientSampling => {
                            if client_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSampling"));
                            }
                            client_sampling__ = map.next_value()?;
                        }
                        GeneratedField::RandomSampling => {
                            if random_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("randomSampling"));
                            }
                            random_sampling__ = map.next_value()?;
                        }
                        GeneratedField::OverallSampling => {
                            if overall_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overallSampling"));
                            }
                            overall_sampling__ = map.next_value()?;
                        }
                        GeneratedField::CustomTags => {
                            if custom_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customTags"));
                            }
                            custom_tags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Tracing {
                    client_sampling: client_sampling__,
                    random_sampling: random_sampling__,
                    overall_sampling: overall_sampling__,
                    custom_tags: custom_tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.Tracing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pattern.is_empty() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.VirtualCluster", len)?;
        if !self.pattern.is_empty() {
            struct_ser.serialize_field("pattern", &self.pattern)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.method != 0 {
            let v = super::core::RequestMethod::from_i32(self.method)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pattern",
            "headers",
            "name",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pattern,
            Headers,
            Name,
            Method,
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
                            "pattern" => Ok(GeneratedField::Pattern),
                            "headers" => Ok(GeneratedField::Headers),
                            "name" => Ok(GeneratedField::Name),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.VirtualCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pattern__ = None;
                let mut headers__ = None;
                let mut name__ = None;
                let mut method__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pattern => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            pattern__ = Some(map.next_value()?);
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map.next_value::<super::core::RequestMethod>()? as i32);
                        }
                    }
                }
                Ok(VirtualCluster {
                    pattern: pattern__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.VirtualCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VirtualHost {
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
        if !self.domains.is_empty() {
            len += 1;
        }
        if !self.routes.is_empty() {
            len += 1;
        }
        if self.require_tls != 0 {
            len += 1;
        }
        if !self.virtual_clusters.is_empty() {
            len += 1;
        }
        if !self.rate_limits.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_remove.is_empty() {
            len += 1;
        }
        if self.cors.is_some() {
            len += 1;
        }
        if !self.per_filter_config.is_empty() {
            len += 1;
        }
        if !self.typed_per_filter_config.is_empty() {
            len += 1;
        }
        if self.include_request_attempt_count {
            len += 1;
        }
        if self.include_attempt_count_in_response {
            len += 1;
        }
        if self.retry_policy.is_some() {
            len += 1;
        }
        if self.retry_policy_typed_config.is_some() {
            len += 1;
        }
        if self.hedge_policy.is_some() {
            len += 1;
        }
        if self.per_request_buffer_limit_bytes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.VirtualHost", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.domains.is_empty() {
            struct_ser.serialize_field("domains", &self.domains)?;
        }
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
        }
        if self.require_tls != 0 {
            let v = virtual_host::TlsRequirementType::from_i32(self.require_tls)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.require_tls)))?;
            struct_ser.serialize_field("requireTls", &v)?;
        }
        if !self.virtual_clusters.is_empty() {
            struct_ser.serialize_field("virtualClusters", &self.virtual_clusters)?;
        }
        if !self.rate_limits.is_empty() {
            struct_ser.serialize_field("rateLimits", &self.rate_limits)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("requestHeadersToAdd", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("requestHeadersToRemove", &self.request_headers_to_remove)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        if !self.response_headers_to_remove.is_empty() {
            struct_ser.serialize_field("responseHeadersToRemove", &self.response_headers_to_remove)?;
        }
        if let Some(v) = self.cors.as_ref() {
            struct_ser.serialize_field("cors", v)?;
        }
        if !self.per_filter_config.is_empty() {
            struct_ser.serialize_field("perFilterConfig", &self.per_filter_config)?;
        }
        if !self.typed_per_filter_config.is_empty() {
            struct_ser.serialize_field("typedPerFilterConfig", &self.typed_per_filter_config)?;
        }
        if self.include_request_attempt_count {
            struct_ser.serialize_field("includeRequestAttemptCount", &self.include_request_attempt_count)?;
        }
        if self.include_attempt_count_in_response {
            struct_ser.serialize_field("includeAttemptCountInResponse", &self.include_attempt_count_in_response)?;
        }
        if let Some(v) = self.retry_policy.as_ref() {
            struct_ser.serialize_field("retryPolicy", v)?;
        }
        if let Some(v) = self.retry_policy_typed_config.as_ref() {
            struct_ser.serialize_field("retryPolicyTypedConfig", v)?;
        }
        if let Some(v) = self.hedge_policy.as_ref() {
            struct_ser.serialize_field("hedgePolicy", v)?;
        }
        if let Some(v) = self.per_request_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("perRequestBufferLimitBytes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VirtualHost {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "domains",
            "routes",
            "require_tls",
            "requireTls",
            "virtual_clusters",
            "virtualClusters",
            "rate_limits",
            "rateLimits",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "response_headers_to_remove",
            "responseHeadersToRemove",
            "cors",
            "per_filter_config",
            "perFilterConfig",
            "typed_per_filter_config",
            "typedPerFilterConfig",
            "include_request_attempt_count",
            "includeRequestAttemptCount",
            "include_attempt_count_in_response",
            "includeAttemptCountInResponse",
            "retry_policy",
            "retryPolicy",
            "retry_policy_typed_config",
            "retryPolicyTypedConfig",
            "hedge_policy",
            "hedgePolicy",
            "per_request_buffer_limit_bytes",
            "perRequestBufferLimitBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Domains,
            Routes,
            RequireTls,
            VirtualClusters,
            RateLimits,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            ResponseHeadersToAdd,
            ResponseHeadersToRemove,
            Cors,
            PerFilterConfig,
            TypedPerFilterConfig,
            IncludeRequestAttemptCount,
            IncludeAttemptCountInResponse,
            RetryPolicy,
            RetryPolicyTypedConfig,
            HedgePolicy,
            PerRequestBufferLimitBytes,
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
                            "domains" => Ok(GeneratedField::Domains),
                            "routes" => Ok(GeneratedField::Routes),
                            "requireTls" | "require_tls" => Ok(GeneratedField::RequireTls),
                            "virtualClusters" | "virtual_clusters" => Ok(GeneratedField::VirtualClusters),
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "responseHeadersToRemove" | "response_headers_to_remove" => Ok(GeneratedField::ResponseHeadersToRemove),
                            "cors" => Ok(GeneratedField::Cors),
                            "perFilterConfig" | "per_filter_config" => Ok(GeneratedField::PerFilterConfig),
                            "typedPerFilterConfig" | "typed_per_filter_config" => Ok(GeneratedField::TypedPerFilterConfig),
                            "includeRequestAttemptCount" | "include_request_attempt_count" => Ok(GeneratedField::IncludeRequestAttemptCount),
                            "includeAttemptCountInResponse" | "include_attempt_count_in_response" => Ok(GeneratedField::IncludeAttemptCountInResponse),
                            "retryPolicy" | "retry_policy" => Ok(GeneratedField::RetryPolicy),
                            "retryPolicyTypedConfig" | "retry_policy_typed_config" => Ok(GeneratedField::RetryPolicyTypedConfig),
                            "hedgePolicy" | "hedge_policy" => Ok(GeneratedField::HedgePolicy),
                            "perRequestBufferLimitBytes" | "per_request_buffer_limit_bytes" => Ok(GeneratedField::PerRequestBufferLimitBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VirtualHost;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.VirtualHost")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VirtualHost, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut domains__ = None;
                let mut routes__ = None;
                let mut require_tls__ = None;
                let mut virtual_clusters__ = None;
                let mut rate_limits__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut response_headers_to_add__ = None;
                let mut response_headers_to_remove__ = None;
                let mut cors__ = None;
                let mut per_filter_config__ = None;
                let mut typed_per_filter_config__ = None;
                let mut include_request_attempt_count__ = None;
                let mut include_attempt_count_in_response__ = None;
                let mut retry_policy__ = None;
                let mut retry_policy_typed_config__ = None;
                let mut hedge_policy__ = None;
                let mut per_request_buffer_limit_bytes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Domains => {
                            if domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domains"));
                            }
                            domains__ = Some(map.next_value()?);
                        }
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequireTls => {
                            if require_tls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requireTls"));
                            }
                            require_tls__ = Some(map.next_value::<virtual_host::TlsRequirementType>()? as i32);
                        }
                        GeneratedField::VirtualClusters => {
                            if virtual_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualClusters"));
                            }
                            virtual_clusters__ = Some(map.next_value()?);
                        }
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToRemove => {
                            if response_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToRemove"));
                            }
                            response_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cors => {
                            if cors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cors"));
                            }
                            cors__ = map.next_value()?;
                        }
                        GeneratedField::PerFilterConfig => {
                            if per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perFilterConfig"));
                            }
                            per_filter_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::TypedPerFilterConfig => {
                            if typed_per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedPerFilterConfig"));
                            }
                            typed_per_filter_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::IncludeRequestAttemptCount => {
                            if include_request_attempt_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeRequestAttemptCount"));
                            }
                            include_request_attempt_count__ = Some(map.next_value()?);
                        }
                        GeneratedField::IncludeAttemptCountInResponse => {
                            if include_attempt_count_in_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeAttemptCountInResponse"));
                            }
                            include_attempt_count_in_response__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetryPolicy => {
                            if retry_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicy"));
                            }
                            retry_policy__ = map.next_value()?;
                        }
                        GeneratedField::RetryPolicyTypedConfig => {
                            if retry_policy_typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicyTypedConfig"));
                            }
                            retry_policy_typed_config__ = map.next_value()?;
                        }
                        GeneratedField::HedgePolicy => {
                            if hedge_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hedgePolicy"));
                            }
                            hedge_policy__ = map.next_value()?;
                        }
                        GeneratedField::PerRequestBufferLimitBytes => {
                            if per_request_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perRequestBufferLimitBytes"));
                            }
                            per_request_buffer_limit_bytes__ = map.next_value()?;
                        }
                    }
                }
                Ok(VirtualHost {
                    name: name__.unwrap_or_default(),
                    domains: domains__.unwrap_or_default(),
                    routes: routes__.unwrap_or_default(),
                    require_tls: require_tls__.unwrap_or_default(),
                    virtual_clusters: virtual_clusters__.unwrap_or_default(),
                    rate_limits: rate_limits__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    response_headers_to_remove: response_headers_to_remove__.unwrap_or_default(),
                    cors: cors__,
                    per_filter_config: per_filter_config__.unwrap_or_default(),
                    typed_per_filter_config: typed_per_filter_config__.unwrap_or_default(),
                    include_request_attempt_count: include_request_attempt_count__.unwrap_or_default(),
                    include_attempt_count_in_response: include_attempt_count_in_response__.unwrap_or_default(),
                    retry_policy: retry_policy__,
                    retry_policy_typed_config: retry_policy_typed_config__,
                    hedge_policy: hedge_policy__,
                    per_request_buffer_limit_bytes: per_request_buffer_limit_bytes__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.VirtualHost", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for virtual_host::TlsRequirementType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::ExternalOnly => "EXTERNAL_ONLY",
            Self::All => "ALL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for virtual_host::TlsRequirementType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "EXTERNAL_ONLY",
            "ALL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = virtual_host::TlsRequirementType;

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
                    .and_then(virtual_host::TlsRequirementType::from_i32)
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
                    .and_then(virtual_host::TlsRequirementType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NONE" => Ok(virtual_host::TlsRequirementType::None),
                    "EXTERNAL_ONLY" => Ok(virtual_host::TlsRequirementType::ExternalOnly),
                    "ALL" => Ok(virtual_host::TlsRequirementType::All),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WeightedCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.clusters.is_empty() {
            len += 1;
        }
        if self.total_weight.is_some() {
            len += 1;
        }
        if !self.runtime_key_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.WeightedCluster", len)?;
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        if let Some(v) = self.total_weight.as_ref() {
            struct_ser.serialize_field("totalWeight", v)?;
        }
        if !self.runtime_key_prefix.is_empty() {
            struct_ser.serialize_field("runtimeKeyPrefix", &self.runtime_key_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WeightedCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "clusters",
            "total_weight",
            "totalWeight",
            "runtime_key_prefix",
            "runtimeKeyPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Clusters,
            TotalWeight,
            RuntimeKeyPrefix,
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
                            "clusters" => Ok(GeneratedField::Clusters),
                            "totalWeight" | "total_weight" => Ok(GeneratedField::TotalWeight),
                            "runtimeKeyPrefix" | "runtime_key_prefix" => Ok(GeneratedField::RuntimeKeyPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WeightedCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.WeightedCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WeightedCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut clusters__ = None;
                let mut total_weight__ = None;
                let mut runtime_key_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map.next_value()?);
                        }
                        GeneratedField::TotalWeight => {
                            if total_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalWeight"));
                            }
                            total_weight__ = map.next_value()?;
                        }
                        GeneratedField::RuntimeKeyPrefix => {
                            if runtime_key_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKeyPrefix"));
                            }
                            runtime_key_prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WeightedCluster {
                    clusters: clusters__.unwrap_or_default(),
                    total_weight: total_weight__,
                    runtime_key_prefix: runtime_key_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.WeightedCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for weighted_cluster::ClusterWeight {
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
        if self.weight.is_some() {
            len += 1;
        }
        if self.metadata_match.is_some() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.per_filter_config.is_empty() {
            len += 1;
        }
        if !self.typed_per_filter_config.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.route.WeightedCluster.ClusterWeight", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.weight.as_ref() {
            struct_ser.serialize_field("weight", v)?;
        }
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadataMatch", v)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("requestHeadersToAdd", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("requestHeadersToRemove", &self.request_headers_to_remove)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        if !self.response_headers_to_remove.is_empty() {
            struct_ser.serialize_field("responseHeadersToRemove", &self.response_headers_to_remove)?;
        }
        if !self.per_filter_config.is_empty() {
            struct_ser.serialize_field("perFilterConfig", &self.per_filter_config)?;
        }
        if !self.typed_per_filter_config.is_empty() {
            struct_ser.serialize_field("typedPerFilterConfig", &self.typed_per_filter_config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for weighted_cluster::ClusterWeight {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "weight",
            "metadata_match",
            "metadataMatch",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "response_headers_to_remove",
            "responseHeadersToRemove",
            "per_filter_config",
            "perFilterConfig",
            "typed_per_filter_config",
            "typedPerFilterConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Weight,
            MetadataMatch,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            ResponseHeadersToAdd,
            ResponseHeadersToRemove,
            PerFilterConfig,
            TypedPerFilterConfig,
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
                            "weight" => Ok(GeneratedField::Weight),
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "responseHeadersToRemove" | "response_headers_to_remove" => Ok(GeneratedField::ResponseHeadersToRemove),
                            "perFilterConfig" | "per_filter_config" => Ok(GeneratedField::PerFilterConfig),
                            "typedPerFilterConfig" | "typed_per_filter_config" => Ok(GeneratedField::TypedPerFilterConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = weighted_cluster::ClusterWeight;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.route.WeightedCluster.ClusterWeight")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<weighted_cluster::ClusterWeight, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut weight__ = None;
                let mut metadata_match__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut response_headers_to_add__ = None;
                let mut response_headers_to_remove__ = None;
                let mut per_filter_config__ = None;
                let mut typed_per_filter_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = map.next_value()?;
                        }
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map.next_value()?;
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToRemove => {
                            if response_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToRemove"));
                            }
                            response_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::PerFilterConfig => {
                            if per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perFilterConfig"));
                            }
                            per_filter_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::TypedPerFilterConfig => {
                            if typed_per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedPerFilterConfig"));
                            }
                            typed_per_filter_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(weighted_cluster::ClusterWeight {
                    name: name__.unwrap_or_default(),
                    weight: weight__,
                    metadata_match: metadata_match__,
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    response_headers_to_remove: response_headers_to_remove__.unwrap_or_default(),
                    per_filter_config: per_filter_config__.unwrap_or_default(),
                    typed_per_filter_config: typed_per_filter_config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.route.WeightedCluster.ClusterWeight", FIELDS, GeneratedVisitor)
    }
}
