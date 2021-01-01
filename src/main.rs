use silly_ecs::{for_components, impl_entity, system};

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

fn inc_num_system(entities: &mut Entities) {
    for entity in entities.iter_mut().filter(|entity| { entity.has_num_component() }) {
        let mut num_comp = entity.get_mut_num_component();
        num_comp.num += 5;
    }
}

fn print_data_all(entities: &Entities) {
    for entity in entities.iter().filter(|entity| { entity.has_num_component() && entity.has_string_component() }) {
        print_data(entity);
    }
}

#[system(NumComponent, StringComponent)]
fn print_data(entity: &Entity) {
    let num_comp = entity.get_num_component();
    let string_comp = entity.get_string_component();
    println!("Data: {} {}", num_comp.num, string_comp.str);
}


fn for_components_test(entities: &Entities) {
    for_components!(NumComponent, {
        println!("{:?}", entity)
    });
}

fn main() {
    let mut entity = Entity {
        num_component: Some(NumComponent { num: 17 }),
        string_component: Some(StringComponent { str: String::from("HELLO") }),
        float_component: Some(FloatComponent { val: 1234.5 }),
        dummy_component: None,
    };

    println!("entity has components: {} {} {} {}",
             entity.has_num_component(),
             entity.has_string_component(),
             entity.has_float_component(),
             entity.has_dummy_component());


    entity.get_mut_float_component().val = 678.9;

    println!("entity values: {} {} {}",
             entity.get_num_component().num,
             entity.get_string_component().str,
             entity.get_float_component().val);

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
    for_components_test(&entities);

    inc_num_system(&mut entities);
    inc_num_system(&mut entities);
    print_data_all(&entities);
    inc_num_system(&mut entities);
    print_data_all(&entities);
}

