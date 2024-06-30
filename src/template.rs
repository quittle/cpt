use std::marker::PhantomData;

use serde::Serialize;

pub trait TemplateRenderer<TypeId> {
    fn render_text(&self, string: &str) -> String {
        string.to_string()
    }

    fn render(&self, type_id: &TypeId, string: &str) -> String;
}

#[derive(Serialize)]
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
    // set up the array and push each list item into it by recursively invoking this macro
    ($type_id:ident: [$($rest:tt)*]) => {{
        let mut temp_vec = Vec::<$crate::TemplateEntry<$type_id>>::new();
        markup!((temp_vec:$type_id): [$($rest)*]);
        temp_vec
    }};

    // Expand leading `@` into `$type_id::`
    (($temp_vec:ident:$type_id:ident): [@ $($rest:tt)*]) => {
        markup!(($temp_vec:$type_id): [$type_id::$($rest)*])
    };

    // Handle all other entries
    (($temp_vec:ident:$type_id:ident): [$expr:expr$(, $($rest:tt)*)?]) => {
        #[allow(clippy::vec_init_then_push)]
        {
            // Append the entries
            $temp_vec.push(($expr).into());
        }
        // Recursively expand
        $(markup!(($temp_vec:$type_id): [$($rest)*]))?
    };

    // Supports trailing commas
    (($temp_vec:ident:$type_id:ident): []) => {}
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

        pub fn red(str: &str) -> TemplateEntry<TestType> {
            TemplateEntry::Typed(TestType::Red, str.to_string())
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
        ( $( $tokens:tt )* ) => {
            {
                markup!(TestType: [$($tokens)*])
            }
        }
    }

    #[test]
    fn test_renderer_is_logical() {
        let example = test_markup!(
            "abc",
            crate::TemplateEntry::Typed(TestType::Red, "other text".into()),
            TestType::blue("final text"),
            @red("other text")
        );
        let template = Template::new(TestRenderer {});
        assert_eq!(
            template.render(&example),
            "abc<red>other text</red><blue>final text</blue><red>other text</red>"
        );
    }
}
