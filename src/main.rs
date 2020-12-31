fn main() {
    println!("Hello, world!");

    let mut entities = vec![
        Entity {
            num_component: Some(NumComponent { num: 32 }),
            string_component: Some(StringComponent { str: String::from("string1") }),
        },
        Entity {
            num_component: Some(NumComponent { num: 64 }),
            string_component: None,
        },
    ];

    inc_num_system(&mut entities);
    inc_num_system(&mut entities);
    print_data(&entities);
    inc_num_system(&mut entities);
    print_data(&entities);
}


struct NumComponent {
    pub num: u32
}

struct StringComponent {
    str: String
}

struct Entity {
    pub num_component: Option<NumComponent>,
    pub string_component: Option<StringComponent>,
}

type Entities = Vec<Entity>;


fn inc_num_system(entities: &mut Entities) {
    for entity in entities.iter_mut().filter(|entity| { entity.num_component.is_some() }) {
        let mut num_comp = entity.num_component.as_mut().unwrap();
        num_comp.num += 5;
    }
}


fn print_data(entities: &Entities) {
    for entity in entities.iter().filter(|entity| { entity.num_component.is_some() && entity.string_component.is_some() }) {
        let num_comp = entity.num_component.as_ref().unwrap();
        let string_comp = entity.string_component.as_ref().unwrap();
        println!("Data: {} {}", num_comp.num, string_comp.str);
    }
}
