// @generated
impl serde::Serialize for OnDemand {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.odcds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.on_demand.v3.OnDemand", len)?;
        if let Some(v) = self.odcds.as_ref() {
            struct_ser.serialize_field("odcds", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OnDemand {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "odcds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Odcds,
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
                            "odcds" => Ok(GeneratedField::Odcds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OnDemand;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.on_demand.v3.OnDemand")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OnDemand, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut odcds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Odcds => {
                            if odcds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("odcds"));
                            }
                            odcds__ = map.next_value()?;
                        }
                    }
                }
                Ok(OnDemand {
                    odcds: odcds__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.on_demand.v3.OnDemand", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OnDemandCds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if !self.resources_locator.is_empty() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.on_demand.v3.OnDemandCds", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.resources_locator.is_empty() {
            struct_ser.serialize_field("resourcesLocator", &self.resources_locator)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OnDemandCds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "resources_locator",
            "resourcesLocator",
            "timeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            ResourcesLocator,
            Timeout,
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
                            "source" => Ok(GeneratedField::Source),
                            "resourcesLocator" | "resources_locator" => Ok(GeneratedField::ResourcesLocator),
                            "timeout" => Ok(GeneratedField::Timeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OnDemandCds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.on_demand.v3.OnDemandCds")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OnDemandCds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut resources_locator__ = None;
                let mut timeout__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map.next_value()?;
                        }
                        GeneratedField::ResourcesLocator => {
                            if resources_locator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourcesLocator"));
                            }
                            resources_locator__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map.next_value()?;
                        }
                    }
                }
                Ok(OnDemandCds {
                    source: source__,
                    resources_locator: resources_locator__.unwrap_or_default(),
                    timeout: timeout__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.on_demand.v3.OnDemandCds", FIELDS, GeneratedVisitor)
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
        if self.odcds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.on_demand.v3.PerRouteConfig", len)?;
        if let Some(v) = self.odcds.as_ref() {
            struct_ser.serialize_field("odcds", v)?;
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
            "odcds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Odcds,
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
                            "odcds" => Ok(GeneratedField::Odcds),
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
                formatter.write_str("struct envoy.extensions.filters.http.on_demand.v3.PerRouteConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PerRouteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut odcds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Odcds => {
                            if odcds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("odcds"));
                            }
                            odcds__ = map.next_value()?;
                        }
                    }
                }
                Ok(PerRouteConfig {
                    odcds: odcds__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.on_demand.v3.PerRouteConfig", FIELDS, GeneratedVisitor)
    }
}
