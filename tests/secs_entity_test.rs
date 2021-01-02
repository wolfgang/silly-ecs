use silly_ecs::{secs_impl_entity};

#[derive(Debug, Default)]
struct NumComponent { num: u32 }
#[derive(Debug, Default)]
struct StringComponent {str: String}

secs_impl_entity!(NumComponent, StringComponent);

#[test]
pub fn default_entity_has_no_components() {
    // assert_eq!(1, 1);
    let entity = Entity::default();
    assert!(!entity.has_num_component());
    assert!(!entity.has_string_component());
}

#[test]
pub fn can_construct_with_components() {
    let entity = Entity {
        num_component: Some(NumComponent::default()),
        string_component: Some(StringComponent::default())
    };
    assert!(entity.has_num_component());
    assert!(entity.has_string_component());
}

#[test]
pub fn can_get_components() {
    let entity = Entity {
        num_component: Some(NumComponent{num: 1234 }),
        string_component: Some(StringComponent { str: "abcd".into() })
    };
    assert_eq!(entity.num_component().num, 1234);
    assert_eq!(entity.string_component().str, "abcd");
}

#[test]
pub fn can_get_components_and_modify() {
    let mut entity = Entity {
        num_component: Some(NumComponent{num: 1000 }),
        string_component: Some(StringComponent { str: "abcd".into() })
    };

    entity.mut_num_component().num += 1000;
    entity.mut_string_component().str += "efgh";

    assert_eq!(entity.num_component().num, 2000);
    assert_eq!(entity.string_component().str, "abcdefgh");
}