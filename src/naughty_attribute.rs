use crate::meta_attributes::show_if::ShowIf;

pub trait INaughtyAttribute {
    fn parse(value: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait INaughtyAttributeMeta {
    const KEY: &'static str;
}

pub fn create_attribute(key: &str, args: &str) -> Option<Box<dyn INaughtyAttribute>> {
    match key {
        ShowIf::KEY => ShowIf::parse(args).map(|attr| Box::new(attr) as Box<dyn INaughtyAttribute>),
        _ => None,
    }
}

pub fn parse_attributes(hint_string: &str) -> Vec<Box<dyn INaughtyAttribute>> {
    hint_string
        .split(';')
        .filter_map(|entry| {
            let (key, args) = entry.split_once(':')?;
            create_attribute(key.trim(), args)
        })
        .collect()
}
