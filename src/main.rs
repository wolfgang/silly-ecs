use silly_ecs::{impl_entity, system};

#[derive(Debug)]
struct NumComponent { pub num: u32 }

#[derive(Debug)]
struct StringComponent { str: String }

#[derive(Debug)]
struct FloatComponent { pub val: f64 }

#[derive(Debug)]
struct DummyComponent {}

impl_entity!(NumComponent, StringComponent, FloatComponent, DummyComponent);


type Entities = Vec<Entity>;

#[system(mut NumComponent)]
fn inc_num(entity: &mut Entity) {
    let mut num_comp = entity.get_mut_num_component();
    num_comp.num += 5;
}

#[system(NumComponent, StringComponent)]
fn print_data(entity: &Entity) {
    let num_comp = entity.get_num_component();
    let string_comp = entity.get_string_component();
    println!("print_data: {} {}", num_comp.num, string_comp.str);
}

#[system[NumComponent, mut StringComponent]]
fn print_num_and_modify_str(entity: &mut Entity) {
    entity.get_mut_string_component().str += "XXXX";
    let num_comp = entity.get_num_component();
    let string_comp = entity.get_string_component();

    println!("print_num_and_modify_str: {} {}", num_comp.num, string_comp.str);
}


fn main() {
    let mut entities = vec![
        Entity {
            num_component: Some(NumComponent { num: 32 }),
            string_component: Some(StringComponent { str: String::from("string1") }),
            ..Default::default()
        },
        Entity {
            num_component: Some(NumComponent { num: 64 }),
            string_component: None,
            ..Default::default()
        },
    ];

    sys_inc_num(&mut entities);
    sys_inc_num(&mut entities);
    sys_print_data(&entities);
    sys_inc_num(&mut entities);
    sys_print_data(&entities);
    sys_print_num_and_modify_str(&mut entities);
    sys_print_data(&entities);
}

