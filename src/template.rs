use std::marker::PhantomData;

pub trait TemplateRenderer<TypeId> {
    fn render_text(&self, string: &str) -> String {
        string.to_string()
    }

    fn render(&self, type_id: &TypeId, string: &str) -> String;
}

pub enum TemplateEntry<TypeId> {
    Text(String),
    Typed(TypeId, String),
}

pub struct Template<TypeId, Renderer>
where
    Renderer: TemplateRenderer<TypeId>,
{
    type_id: PhantomData<TypeId>,
    renderer: Renderer,
}

impl<TypeId, Renderer> Template<TypeId, Renderer>
where
    Renderer: TemplateRenderer<TypeId>,
{
    pub fn new(renderer: Renderer) -> Self {
        Self {
            type_id: Default::default(),
            renderer,
        }
    }

    pub fn render(&self, entries: &Vec<TemplateEntry<TypeId>>) -> String {
        let mut ret = String::new();
        for entry in entries {
            ret += &(match entry {
                TemplateEntry::Text(text) => self.renderer.render_text(text),
                TemplateEntry::Typed(type_id, text) => self.renderer.render(type_id, text),
            });
        }
        ret
    }
}

impl<TypeId> From<&str> for TemplateEntry<TypeId> {
    fn from(val: &str) -> Self {
        TemplateEntry::Text(val.into())
    }
}

impl<TypeId> From<String> for TemplateEntry<TypeId> {
    fn from(val: String) -> Self {
        TemplateEntry::Text(val)
    }
}

#[macro_export]
macro_rules! markup {
    ( $type_id:tt, $( $x:expr ),* ) => {
        {
            vec![
                $(
                    Into::<$crate::TemplateEntry::<$type_id>>::into($x)
                ),*
            ]
        }
    };
}

#[cfg(test)]
mod tests {
    use super::{Template, TemplateEntry, TemplateRenderer};

    enum TestType {
        Red,
        Blue,
    }

    impl TestType {
        pub fn blue(str: &str) -> TemplateEntry<TestType> {
            TemplateEntry::Typed(TestType::Blue, str.to_string())
        }
    }

    struct TestRenderer {}

    impl TemplateRenderer<TestType> for TestRenderer {
        fn render(&self, type_id: &TestType, string: &str) -> String {
            match type_id {
                TestType::Blue => format!("<blue>{}</blue>", string),
                TestType::Red => format!("<red>{}</red>", string),
            }
        }
    }

    macro_rules! test_markup {
        ( $( $x:expr ),* ) => {
            {
                markup!(TestType, $($x),*)
            }
        }
    }

    #[test]
    fn test_renderer_is_logical() {
        let example = test_markup!(
            "abc",
            crate::TemplateEntry::Typed(TestType::Red, "other text".into()),
            TestType::blue("final text")
        );
        let template = Template::new(TestRenderer {});
        assert_eq!(
            template.render(&example),
            "abc<red>other text</red><blue>final text</blue>"
        );
    }
}
