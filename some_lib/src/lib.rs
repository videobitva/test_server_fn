pub mod associated_type {
    use serde::{Serialize, Deserialize};

    pub trait NsTrait {
        type Model;
    }
    
    pub struct Namespace;
    
    impl NsTrait for Namespace {
        type Model = Model;
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Model {
        pub field_a: i32,
        pub field_b: f64
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ActiveModel {
        pub field_a: Option<i32>,
        pub field_b: Option<f64>
    }
    
    impl From<<Namespace as NsTrait>::Model> for ActiveModel {
        fn from(m: <Namespace as NsTrait>::Model) -> Self {
            let Model { field_a, field_b } = m;
            Self {
                field_a: Some(field_a),
                field_b: Some(field_b)
            }
        }
    }
}


pub mod just_type {
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Model {
        pub field_a: i32,
        pub field_b: f64
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ActiveModel {
        pub field_a: Option<i32>,
        pub field_b: Option<f64>
    }
    
    impl From<Model> for ActiveModel {
        fn from(m: Model) -> Self {
            let Model { field_a, field_b } = m;
            Self {
                field_a: Some(field_a),
                field_b: Some(field_b)
            }
        }
    }
}