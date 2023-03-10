// @generated
impl serde::Serialize for Router {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.close_downstream_on_upstream_error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.router.v3.Router", len)?;
        if let Some(v) = self.close_downstream_on_upstream_error.as_ref() {
            struct_ser.serialize_field("closeDownstreamOnUpstreamError", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Router {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "close_downstream_on_upstream_error",
            "closeDownstreamOnUpstreamError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CloseDownstreamOnUpstreamError,
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
                            "closeDownstreamOnUpstreamError" | "close_downstream_on_upstream_error" => Ok(GeneratedField::CloseDownstreamOnUpstreamError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Router;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.router.v3.Router")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Router, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut close_downstream_on_upstream_error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CloseDownstreamOnUpstreamError => {
                            if close_downstream_on_upstream_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closeDownstreamOnUpstreamError"));
                            }
                            close_downstream_on_upstream_error__ = map.next_value()?;
                        }
                    }
                }
                Ok(Router {
                    close_downstream_on_upstream_error: close_downstream_on_upstream_error__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.router.v3.Router", FIELDS, GeneratedVisitor)
    }
}
