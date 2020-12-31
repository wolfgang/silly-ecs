macro_rules! gen_entity {
    ($($comp: ident),*) => {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    struct Entity2 {
        $($comp: Option<$comp>), *
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    impl Entity2 {
        $(fn $comp(&self) -> bool { self.$comp.is_some()}) *
    }

    }

}

gen_entity!(NumComponent,StringComponent);

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

impl Entity {
    fn has_num_component(&self) -> bool {
        self.num_component.is_some()
    }

    fn has_string_component(&self) -> bool {
        self.string_component.is_some()
    }

    fn get_num_component(&self) -> &NumComponent {
        self.num_component.as_ref().unwrap()
    }

    fn get_num_component_mut(&mut self) -> &mut NumComponent {
        self.num_component.as_mut().unwrap()
    }

    fn get_string_component(&self) -> &StringComponent {
        self.string_component.as_ref().unwrap()
    }
}

type Entities = Vec<Entity>;

fn inc_num_system(entities: &mut Entities) {
    for entity in entities.iter_mut().filter(|entity| { entity.has_num_component() }) {
        let mut num_comp = entity.get_num_component_mut();
        num_comp.num += 5;
    }
}


fn print_data(entities: &Entities) {
    for entity in entities.iter().filter(|entity| { entity.has_num_component() && entity.has_string_component() }) {
        let num_comp = entity.get_num_component();
        let string_comp = entity.get_string_component();
        println!("Data: {} {}", num_comp.num, string_comp.str);
    }
}


fn main() {
    println!("Hello, world!");

    let entity2 = Entity2 {NumComponent: Some(NumComponent{num: 17}), StringComponent: None };
    println!("entity2: {} {} {}", entity2.NumComponent.as_ref().unwrap().num, entity2.NumComponent(), entity2.StringComponent());

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

