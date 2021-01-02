use silly_ecs::{secs_impl_entity, secs_system};
use std::fmt::Debug;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NumComponent { num: u32 }

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatComponent { val: f64 }

secs_impl_entity!(NumComponent, FloatComponent);

type Entities = Vec<Entity>;

#[derive(Default)]
struct SystemSpy {
    called_num_components: Vec<NumComponent>,
    called_float_components: Vec<FloatComponent>,
}

trait Renderer {
    fn render<T>(&mut self, renderable: T) where T: Debug;
}

#[derive(Default)]
struct MyRenderer {
    log: Vec<String>
}

impl Renderer for MyRenderer {
    fn render<T>(&mut self, renderable: T) where T: Debug {
        self.log.push(format!("{:?}", renderable))
    }
}

#[secs_system(NumComponent, FloatComponent)]
fn for_all(entity: &Entity, spy: &mut SystemSpy) {
    spy.called_num_components.push(entity.num_component().clone());
    spy.called_float_components.push(entity.float_component().clone());
}

#[secs_system(NumComponent)]
fn for_all_num_components(entity: &Entity, spy: &mut SystemSpy) {
    spy.called_num_components.push(entity.num_component().clone());
    if entity.has_float_component() {
        spy.called_float_components.push(entity.float_component().clone());
    }
}

#[secs_system(mut NumComponent, FloatComponent)]
fn modify_num_components(entity: &mut Entity, spy: &mut SystemSpy) {
    entity.mut_num_component().num *= 2;
    spy.called_float_components.push(entity.float_component().clone());
}

#[secs_system(NumComponent, FloatComponent)]
fn render_data<T>(entity: &Entity, renderer: &mut T) where T: Renderer {
    renderer.render(entity.num_component().num);
    renderer.render(entity.float_component().val);
}


#[test]
fn for_all_calls_entities_with_both_components() {
    let entities = vec![
        Entity::default(),
        Entity {
            num_component: Some(NumComponent { num: 1111 }),
            float_component: Some(FloatComponent { val: 2222.3 }),
        },
        Entity {
            num_component: Some(NumComponent { num: 3333 }),
            float_component: Some(FloatComponent { val: 4444.4 }),
        },
    ];

    let mut spy = SystemSpy::default();
    sys_for_all(&entities, &mut spy);

    assert_eq!(spy.called_num_components.len(), 2);
    assert_eq!(spy.called_float_components.len(), 2);

    assert_eq!(*entities[1].num_component(), spy.called_num_components[0]);
    assert_eq!(*entities[2].num_component(), spy.called_num_components[1]);

    assert_eq!(*entities[1].float_component(), spy.called_float_components[0]);
    assert_eq!(*entities[2].float_component(), spy.called_float_components[1]);
}

#[test]
fn for_num_components_calls_entities_with_num_components_only() {
    let entities = vec![
        Entity { num_component: Some(NumComponent { num: 1234 }), ..Default::default() },
        Entity::default()];

    let mut spy = SystemSpy::default();
    sys_for_all_num_components(&entities, &mut spy);

    assert_eq!(spy.called_num_components.len(), 1);
    assert_eq!(spy.called_float_components.len(), 0);
    assert_eq!(*entities[0].num_component(), spy.called_num_components[0]);
}

#[test]
fn modify_num_components_multiplies_numbers() {
    let mut entities = vec![
        Entity {
            num_component: Some(NumComponent { num: 1111 }),
            float_component: Some(FloatComponent { val:1.0 }),
        },
        Entity {
            num_component: Some(NumComponent { num: 2222 }),
            float_component: Some(FloatComponent { val: 2.0 }),
        },
    ];

    let mut spy = SystemSpy::default();
    sys_modify_num_components(&mut entities, &mut spy);

    assert_eq!(entities[0].num_component().num, 2222);
    assert_eq!(entities[1].num_component().num, 4444);
    assert_eq!(*entities[0].float_component(), spy.called_float_components[0]);
    assert_eq!(*entities[1].float_component(), spy.called_float_components[1]);
}

#[test]
fn can_use_generics_in_systems() {
    let entities = vec![
        Entity {
            num_component: Some(NumComponent { num: 1234 }),
            float_component: Some(FloatComponent { val: 567.8 }),
        },
    ];

    let mut renderer = MyRenderer::default();
    sys_render_data(&entities, &mut renderer);
    assert_eq!(renderer.log, vec!["1234", "567.8"]);

}