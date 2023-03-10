// @generated
impl serde::Serialize for WrrLocality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint_picking_policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.wrr_locality.v3.WrrLocality", len)?;
        if let Some(v) = self.endpoint_picking_policy.as_ref() {
            struct_ser.serialize_field("endpointPickingPolicy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WrrLocality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint_picking_policy",
            "endpointPickingPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EndpointPickingPolicy,
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
                            "endpointPickingPolicy" | "endpoint_picking_policy" => Ok(GeneratedField::EndpointPickingPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WrrLocality;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.wrr_locality.v3.WrrLocality")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WrrLocality, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint_picking_policy__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EndpointPickingPolicy => {
                            if endpoint_picking_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointPickingPolicy"));
                            }
                            endpoint_picking_policy__ = map.next_value()?;
                        }
                    }
                }
                Ok(WrrLocality {
                    endpoint_picking_policy: endpoint_picking_policy__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.wrr_locality.v3.WrrLocality", FIELDS, GeneratedVisitor)
    }
}
