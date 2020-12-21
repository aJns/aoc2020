use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};

type Allergen = String;
type Ingredient = String;

static mut FOOD_ID: usize = 0;

struct Food {
    id: usize,
    ingr: Vec<Ingredient>,
    allr: Vec<Allergen>,
}

fn parse_to_food(line: &str) -> Food {
    let mut ingr: Vec<Ingredient> = Vec::new();
    let mut allr: Vec<Allergen> = Vec::new();

    let mut split = line.split(" ");

    while let Some(word) = split.next() {
        if word.contains("contains") {
            break;
        }
        ingr.push(word.to_string());
    }

    while let Some(word) = split.next() {
        let trim = word.trim_end_matches(|c| c == ')' || c == ',');
        allr.push(trim.to_string());
    }

    unsafe {
        let id = FOOD_ID;
        FOOD_ID += 1;
        Food { id, ingr, allr }
    }
}

type AllergyMap = HashMap<Allergen, HashSet<(usize, Ingredient)>>;

fn assemble_allergy_map(foods: &Vec<Food>) -> AllergyMap {
    let mut map = HashMap::new();

    for f in foods {
        for a in &f.allr {
            let a_set = map.entry(a.clone()).or_insert(HashSet::new());
            for i in &f.ingr {
                a_set.insert((f.id, i.clone()));
            }
        }
    }

    map
}

type ContainMap = HashMap<Ingredient, HashSet<Allergen>>;

fn assemble_contain_map(foods: &Vec<Food>) -> ContainMap {
    let mut map = HashMap::new();

    for f in foods {
        for i in &f.ingr {
            let i_set = map.entry(i.clone()).or_insert(HashSet::new());
            for a in &f.allr {
                i_set.insert(a.clone());
            }
        }
    }

    map
}

fn count_in_food(foods: &Vec<Food>, ingr: &Ingredient) -> u32 {
    let mut count = 0;
    for f in foods {
        for i in &f.ingr {
            if i == ingr {
                count += 1;
            }
        }
    }
    count
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let foods: Vec<Food> = input.lines().map(|x| parse_to_food(x)).collect();

    let a_cont_in = assemble_allergy_map(&foods);
    let can_contain = assemble_contain_map(&foods);
    let mut cannot_contain: ContainMap = HashMap::new();

    for (k, val) in a_cont_in {
        let mut diff_map: HashMap<usize, HashSet<Ingredient>> = HashMap::new();
        for v in val {
            let set = diff_map.entry(v.0).or_insert(HashSet::new());
            set.insert(v.1);
        }

        let mut diff_vec = Vec::new();
        for (_, v) in diff_map {
            diff_vec.push(v);
        }
        let mut diff = HashSet::new();

        for start in 0..diff_vec.len() {
            let base = &diff_vec[start];
            for i in (start + 1)..diff_vec.len() {
                for d in base.symmetric_difference(&diff_vec[i]) {
                    diff.insert(d.clone());
                }
            }
        }

        for d in &diff {
            let i_set = cannot_contain.entry(d.clone()).or_insert(HashSet::new());
            i_set.insert(k.clone());
        }
    }

    let mut allergen_free = HashSet::new();

    for (i, a_set) in can_contain {
        if let Some(cant) = cannot_contain.get(&i) {
            let can_really_contain: HashSet<&Allergen> = a_set.difference(&cant).collect();
            if can_really_contain.is_empty() {
                allergen_free.insert(i);
            }
        }
    }

    let count = allergen_free
        .iter()
        .fold(0, |acc, x| acc + count_in_food(&foods, &x));

    writeln!(io::stdout(), "appearances: {}", count)?;

    Ok(())
}
