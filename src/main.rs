use silly_ecs::{secs_impl_entity, secs_system};
use std::fmt::Debug;

#[derive(Debug, Default)]
struct NumComponent { pub num: u32 }

#[derive(Debug, Default)]
struct StringComponent { str: String }

#[derive(Debug, Default)]
struct FloatComponent { pub val: f64 }

#[derive(Debug)]
struct DummyComponent {}

secs_impl_entity!(NumComponent, StringComponent, FloatComponent, DummyComponent);


type Entities = Vec<Entity>;

#[secs_system(mut NumComponent)]
fn inc_num(entity: &mut Entity) {
    entity.mut_num_component().num += 5;
}

#[secs_system(NumComponent, StringComponent)]
fn print_data(entity: &Entity) {
    println!("print_data: {} {}",
             entity.num_component().num,
             entity.string_component().str);
}

#[secs_system(NumComponent, mut StringComponent)]
fn print_num_and_modify_str(entity: &mut Entity) {
    entity.mut_string_component().str += "XXXX";
    println!("print_num_and_modify_str: {} {}", entity.num_component().num, entity.string_component().str);
}

#[secs_system(mut NumComponent, mut FloatComponent)]
fn inc_numbers(entity: &mut Entity) {
    entity.mut_num_component().num += 10;
    entity.mut_float_component().val += 20.0;
}

#[secs_system(NumComponent, FloatComponent)]
fn print_numbers(entity: &Entity) {
    println!("print_numbers: {} {}", entity.num_component().num, entity.float_component().val);
}

trait Renderer {
    fn render<T>(&self, renderable: T) where T: Debug;
}

struct MyRenderer {
}

impl Renderer for MyRenderer {
    fn render<T>(&self, renderable: T) where T: Debug {
        println!("[RENDER] {:?}", renderable);
    }
}

#[secs_system(NumComponent, FloatComponent, StringComponent)]
fn render_data<T>(entity: &Entity, renderer: &T) where T: Renderer {
    renderer.render(entity.num_component().num);
    renderer.render(entity.float_component().val);
    renderer.render(entity.string_component().str.as_str());
}

fn main() {
    let mut entities = vec![
        Entity {
            num_component: Some(NumComponent::default()),
            float_component: Some(FloatComponent { val: 100.0 }),
            string_component: Some(StringComponent { str: String::from("string1") }),
            ..Default::default()
        },
        Entity {
            num_component: Some(NumComponent { num: 64 }),
            float_component: Some(FloatComponent::default()),
            string_component: None,
            ..Default::default()
        },
    ];

    let renderer = MyRenderer {};

    sys_inc_num(&mut entities);
    sys_inc_num(&mut entities);
    sys_print_data(&entities);
    sys_inc_num(&mut entities);
    sys_print_data(&entities);
    sys_print_num_and_modify_str(&mut entities);
    sys_print_data(&entities);
    sys_inc_numbers(&mut entities);
    sys_print_numbers(&entities);
    sys_render_data(&entities, &renderer)


}

