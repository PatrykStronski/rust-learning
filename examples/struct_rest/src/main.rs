use std::fmt::{Debug, Formatter, Result};

trait Naming {
    fn get_name(&self) -> String;
    fn get_category(&self) -> String;
    fn reassign_category(&mut self, cat: String);
}

struct Product {
    name: String,
    cat: String,
    price: f32
}

struct ProductSet {
    name: String,
    products: Vec<Product>
}

impl Naming for Product {
    fn get_name(&self) -> String {
        return self.name.to_string();
    }

    fn get_category(&self) -> String {
        return self.cat.to_string();
    }

    fn reassign_category(&mut self, cat: String) {
        self.cat = cat;
    }
}

impl Debug for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Product")
        .field("name", &self.name)
        .field("category", &self.cat)
        .field("price", &self.price)
        .finish()
    }
}

impl Naming for ProductSet {
    fn get_name(&self) -> String {
        return self.name.to_string();
    }

    fn get_category(&self) -> String {
        let mut all_categories = String::new();
        for prod in &self.products {
            all_categories.push_str(&prod.cat.to_string());
            all_categories.push_str(",");
        }
        return all_categories;
    }

    fn reassign_category(&mut self, cat: String) {
        for prod in &mut self.products {
            prod.cat = cat.to_string();
        }
    }
}

impl Product {
    pub fn get_price(&self) -> f32 {
        return self.price;
    }

    pub fn get_price_with_discount(&self, disc: f32) -> f32 {
        return self.price - self.price * disc;
    }
}

impl ProductSet {
    pub fn print_products(&self) {
        for prod in &self.products {
            println!("{:?}", prod);
        }
    }
}

fn main() {
    let radio = Product {
        name: String::from("radio"),
        cat: String::from("electronics"),
        price: 45.50
    };
    let computer = Product {
        name: String::from("computer"),
        cat: String::from("electronics"),
        price: 1200.0
    };
    let recorder = Product {
        name: String::from("voice recorder"),
        cat: String::from("electronics"),
        price: 100.0
    };
    let mut ps = ProductSet {
        name: String::from("electronics in the shop"),
        products: vec![radio, computer, recorder]
    };
    ps.print_products();
    ps.reassign_category(String::from("home facilities"));
    ps.print_products();
    println!("{}", radio.get_price_with_discount(0.10));
    ps.print_products();
}
