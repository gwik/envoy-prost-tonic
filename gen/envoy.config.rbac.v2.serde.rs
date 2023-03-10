// @generated
impl serde::Serialize for Permission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v2.Permission", len)?;
        if let Some(v) = self.rule.as_ref() {
            match v {
                permission::Rule::AndRules(v) => {
                    struct_ser.serialize_field("andRules", v)?;
                }
                permission::Rule::OrRules(v) => {
                    struct_ser.serialize_field("orRules", v)?;
                }
                permission::Rule::Any(v) => {
                    struct_ser.serialize_field("any", v)?;
                }
                permission::Rule::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                permission::Rule::UrlPath(v) => {
                    struct_ser.serialize_field("urlPath", v)?;
                }
                permission::Rule::DestinationIp(v) => {
                    struct_ser.serialize_field("destinationIp", v)?;
                }
                permission::Rule::DestinationPort(v) => {
                    struct_ser.serialize_field("destinationPort", v)?;
                }
                permission::Rule::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                permission::Rule::NotRule(v) => {
                    struct_ser.serialize_field("notRule", v)?;
                }
                permission::Rule::RequestedServerName(v) => {
                    struct_ser.serialize_field("requestedServerName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Permission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "and_rules",
            "andRules",
            "or_rules",
            "orRules",
            "any",
            "header",
            "url_path",
            "urlPath",
            "destination_ip",
            "destinationIp",
            "destination_port",
            "destinationPort",
            "metadata",
            "not_rule",
            "notRule",
            "requested_server_name",
            "requestedServerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AndRules,
            OrRules,
            Any,
            Header,
            UrlPath,
            DestinationIp,
            DestinationPort,
            Metadata,
            NotRule,
            RequestedServerName,
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
                            "andRules" | "and_rules" => Ok(GeneratedField::AndRules),
                            "orRules" | "or_rules" => Ok(GeneratedField::OrRules),
                            "any" => Ok(GeneratedField::Any),
                            "header" => Ok(GeneratedField::Header),
                            "urlPath" | "url_path" => Ok(GeneratedField::UrlPath),
                            "destinationIp" | "destination_ip" => Ok(GeneratedField::DestinationIp),
                            "destinationPort" | "destination_port" => Ok(GeneratedField::DestinationPort),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "notRule" | "not_rule" => Ok(GeneratedField::NotRule),
                            "requestedServerName" | "requested_server_name" => Ok(GeneratedField::RequestedServerName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Permission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v2.Permission")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Permission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AndRules => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andRules"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::AndRules)
;
                        }
                        GeneratedField::OrRules => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orRules"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::OrRules)
;
                        }
                        GeneratedField::Any => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("any"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::Any);
                        }
                        GeneratedField::Header => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::Header)
;
                        }
                        GeneratedField::UrlPath => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlPath"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::UrlPath)
;
                        }
                        GeneratedField::DestinationIp => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationIp"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::DestinationIp)
;
                        }
                        GeneratedField::DestinationPort => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPort"));
                            }
                            rule__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| permission::Rule::DestinationPort(x.0));
                        }
                        GeneratedField::Metadata => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::Metadata)
;
                        }
                        GeneratedField::NotRule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notRule"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::NotRule)
;
                        }
                        GeneratedField::RequestedServerName => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestedServerName"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(permission::Rule::RequestedServerName)
;
                        }
                    }
                }
                Ok(Permission {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v2.Permission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for permission::Set {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v2.Permission.Set", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for permission::Set {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = permission::Set;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v2.Permission.Set")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<permission::Set, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(permission::Set {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v2.Permission.Set", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Policy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permissions.is_empty() {
            len += 1;
        }
        if !self.principals.is_empty() {
            len += 1;
        }
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v2.Policy", len)?;
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if !self.principals.is_empty() {
            struct_ser.serialize_field("principals", &self.principals)?;
        }
        if let Some(v) = self.condition.as_ref() {
            struct_ser.serialize_field("condition", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Policy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permissions",
            "principals",
            "condition",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permissions,
            Principals,
            Condition,
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
                            "permissions" => Ok(GeneratedField::Permissions),
                            "principals" => Ok(GeneratedField::Principals),
                            "condition" => Ok(GeneratedField::Condition),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Policy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v2.Policy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Policy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permissions__ = None;
                let mut principals__ = None;
                let mut condition__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Principals => {
                            if principals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principals"));
                            }
                            principals__ = Some(map.next_value()?);
                        }
                        GeneratedField::Condition => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            condition__ = map.next_value()?;
                        }
                    }
                }
                Ok(Policy {
                    permissions: permissions__.unwrap_or_default(),
                    principals: principals__.unwrap_or_default(),
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v2.Policy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Principal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v2.Principal", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                principal::Identifier::AndIds(v) => {
                    struct_ser.serialize_field("andIds", v)?;
                }
                principal::Identifier::OrIds(v) => {
                    struct_ser.serialize_field("orIds", v)?;
                }
                principal::Identifier::Any(v) => {
                    struct_ser.serialize_field("any", v)?;
                }
                principal::Identifier::Authenticated(v) => {
                    struct_ser.serialize_field("authenticated", v)?;
                }
                principal::Identifier::SourceIp(v) => {
                    struct_ser.serialize_field("sourceIp", v)?;
                }
                principal::Identifier::DirectRemoteIp(v) => {
                    struct_ser.serialize_field("directRemoteIp", v)?;
                }
                principal::Identifier::RemoteIp(v) => {
                    struct_ser.serialize_field("remoteIp", v)?;
                }
                principal::Identifier::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                principal::Identifier::UrlPath(v) => {
                    struct_ser.serialize_field("urlPath", v)?;
                }
                principal::Identifier::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                principal::Identifier::NotId(v) => {
                    struct_ser.serialize_field("notId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Principal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "and_ids",
            "andIds",
            "or_ids",
            "orIds",
            "any",
            "authenticated",
            "source_ip",
            "sourceIp",
            "direct_remote_ip",
            "directRemoteIp",
            "remote_ip",
            "remoteIp",
            "header",
            "url_path",
            "urlPath",
            "metadata",
            "not_id",
            "notId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AndIds,
            OrIds,
            Any,
            Authenticated,
            SourceIp,
            DirectRemoteIp,
            RemoteIp,
            Header,
            UrlPath,
            Metadata,
            NotId,
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
                            "andIds" | "and_ids" => Ok(GeneratedField::AndIds),
                            "orIds" | "or_ids" => Ok(GeneratedField::OrIds),
                            "any" => Ok(GeneratedField::Any),
                            "authenticated" => Ok(GeneratedField::Authenticated),
                            "sourceIp" | "source_ip" => Ok(GeneratedField::SourceIp),
                            "directRemoteIp" | "direct_remote_ip" => Ok(GeneratedField::DirectRemoteIp),
                            "remoteIp" | "remote_ip" => Ok(GeneratedField::RemoteIp),
                            "header" => Ok(GeneratedField::Header),
                            "urlPath" | "url_path" => Ok(GeneratedField::UrlPath),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "notId" | "not_id" => Ok(GeneratedField::NotId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Principal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v2.Principal")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Principal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AndIds => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andIds"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::AndIds)
;
                        }
                        GeneratedField::OrIds => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orIds"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::OrIds)
;
                        }
                        GeneratedField::Any => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("any"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Any);
                        }
                        GeneratedField::Authenticated => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authenticated"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Authenticated)
;
                        }
                        GeneratedField::SourceIp => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIp"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::SourceIp)
;
                        }
                        GeneratedField::DirectRemoteIp => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directRemoteIp"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::DirectRemoteIp)
;
                        }
                        GeneratedField::RemoteIp => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteIp"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::RemoteIp)
;
                        }
                        GeneratedField::Header => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Header)
;
                        }
                        GeneratedField::UrlPath => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlPath"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::UrlPath)
;
                        }
                        GeneratedField::Metadata => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::Metadata)
;
                        }
                        GeneratedField::NotId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notId"));
                            }
                            identifier__ = map.next_value::<::std::option::Option<_>>()?.map(principal::Identifier::NotId)
;
                        }
                    }
                }
                Ok(Principal {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v2.Principal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for principal::Authenticated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v2.Principal.Authenticated", len)?;
        if let Some(v) = self.principal_name.as_ref() {
            struct_ser.serialize_field("principalName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for principal::Authenticated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_name",
            "principalName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalName,
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
                            "principalName" | "principal_name" => Ok(GeneratedField::PrincipalName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = principal::Authenticated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v2.Principal.Authenticated")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<principal::Authenticated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PrincipalName => {
                            if principal_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalName"));
                            }
                            principal_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(principal::Authenticated {
                    principal_name: principal_name__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v2.Principal.Authenticated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for principal::Set {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v2.Principal.Set", len)?;
        if !self.ids.is_empty() {
            struct_ser.serialize_field("ids", &self.ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for principal::Set {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ids,
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
                            "ids" => Ok(GeneratedField::Ids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = principal::Set;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v2.Principal.Set")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<principal::Set, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ids => {
                            if ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            ids__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(principal::Set {
                    ids: ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v2.Principal.Set", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rbac {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action != 0 {
            len += 1;
        }
        if !self.policies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.rbac.v2.RBAC", len)?;
        if self.action != 0 {
            let v = rbac::Action::from_i32(self.action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        if !self.policies.is_empty() {
            struct_ser.serialize_field("policies", &self.policies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rbac {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action",
            "policies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Action,
            Policies,
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
                            "policies" => Ok(GeneratedField::Policies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rbac;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.rbac.v2.RBAC")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Rbac, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                let mut policies__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map.next_value::<rbac::Action>()? as i32);
                        }
                        GeneratedField::Policies => {
                            if policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policies"));
                            }
                            policies__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Rbac {
                    action: action__.unwrap_or_default(),
                    policies: policies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.rbac.v2.RBAC", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rbac::Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Allow => "ALLOW",
            Self::Deny => "DENY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rbac::Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALLOW",
            "DENY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rbac::Action;

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
                    .and_then(rbac::Action::from_i32)
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
                    .and_then(rbac::Action::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ALLOW" => Ok(rbac::Action::Allow),
                    "DENY" => Ok(rbac::Action::Deny),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
