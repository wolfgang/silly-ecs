fn main() {
    println!("Hello, world!");

    let mut entities = vec![
        Entity {
            num_component: Some(NumComponent {num: 32}),
            string_component: Some(StringComponent { str: String::from("string1")})
        },
        Entity {
            num_component: Some(NumComponent {num: 64}),
            string_component: None
        },

    ];

    inc_num_system(&mut entities);
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
    pub string_component: Option<StringComponent>
}

type Entities = Vec<Entity>;


fn inc_num_system(entities: &mut Entities) {
    for entity in entities.iter_mut() {
        if let Some(comp) = &mut entity.num_component {
            comp.num += 1;
        }
    }
}

fn print_data(entities: &Entities) {
    for entity in entities.iter() {
        if let Some(num_comp) = &entity.num_component {
            if let Some(string_comp) = &entity.string_component {
                println!("Data: {} {}", num_comp.num, string_comp.str);
            }
        }
    }
}