// @generated
impl serde::Serialize for CaresDnsResolverConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resolvers.is_empty() {
            len += 1;
        }
        if self.use_resolvers_as_fallback {
            len += 1;
        }
        if self.filter_unroutable_families {
            len += 1;
        }
        if self.dns_resolver_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig", len)?;
        if !self.resolvers.is_empty() {
            struct_ser.serialize_field("resolvers", &self.resolvers)?;
        }
        if self.use_resolvers_as_fallback {
            struct_ser.serialize_field("useResolversAsFallback", &self.use_resolvers_as_fallback)?;
        }
        if self.filter_unroutable_families {
            struct_ser.serialize_field("filterUnroutableFamilies", &self.filter_unroutable_families)?;
        }
        if let Some(v) = self.dns_resolver_options.as_ref() {
            struct_ser.serialize_field("dnsResolverOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CaresDnsResolverConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resolvers",
            "use_resolvers_as_fallback",
            "useResolversAsFallback",
            "filter_unroutable_families",
            "filterUnroutableFamilies",
            "dns_resolver_options",
            "dnsResolverOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resolvers,
            UseResolversAsFallback,
            FilterUnroutableFamilies,
            DnsResolverOptions,
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
                            "resolvers" => Ok(GeneratedField::Resolvers),
                            "useResolversAsFallback" | "use_resolvers_as_fallback" => Ok(GeneratedField::UseResolversAsFallback),
                            "filterUnroutableFamilies" | "filter_unroutable_families" => Ok(GeneratedField::FilterUnroutableFamilies),
                            "dnsResolverOptions" | "dns_resolver_options" => Ok(GeneratedField::DnsResolverOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CaresDnsResolverConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CaresDnsResolverConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resolvers__ = None;
                let mut use_resolvers_as_fallback__ = None;
                let mut filter_unroutable_families__ = None;
                let mut dns_resolver_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Resolvers => {
                            if resolvers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolvers"));
                            }
                            resolvers__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseResolversAsFallback => {
                            if use_resolvers_as_fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useResolversAsFallback"));
                            }
                            use_resolvers_as_fallback__ = Some(map.next_value()?);
                        }
                        GeneratedField::FilterUnroutableFamilies => {
                            if filter_unroutable_families__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterUnroutableFamilies"));
                            }
                            filter_unroutable_families__ = Some(map.next_value()?);
                        }
                        GeneratedField::DnsResolverOptions => {
                            if dns_resolver_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsResolverOptions"));
                            }
                            dns_resolver_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(CaresDnsResolverConfig {
                    resolvers: resolvers__.unwrap_or_default(),
                    use_resolvers_as_fallback: use_resolvers_as_fallback__.unwrap_or_default(),
                    filter_unroutable_families: filter_unroutable_families__.unwrap_or_default(),
                    dns_resolver_options: dns_resolver_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig", FIELDS, GeneratedVisitor)
    }
}
