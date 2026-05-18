fn main() {

    let mut flutter_ai_pro_lan = ProductionPlan::new(Areas::AI, Langs::Dart, 4, 450_000);

    flutter_ai_pro_lan.update_emp_number(7);

    flutter_ai_pro_lan.update_yearly_budget(632_000);
}

struct ProductionPlan {

    area: Areas,
    main_lang: Langs,
    number_of_emp: u8,
    yearly_budget_usd: u32
}

impl ProductionPlan {

    fn new(area: Areas, main_lang: Langs, number_of_emp: u8, yearly_budget_usd: u32) -> Self {

        ProductionPlan { area, main_lang, number_of_emp, yearly_budget_usd }
    }

    fn update_yearly_budget(&mut self, new_budget: u32) {

        self.yearly_budget_usd = new_budget;
    }

    fn update_emp_number(&mut self, new_number_of_emp: u8) {

        self.number_of_emp = new_number_of_emp;
    }
}

enum Langs {

    Rust,
    CS,
    Dart
}

enum Areas {
    WebThree,
    AI,
    Game
}
