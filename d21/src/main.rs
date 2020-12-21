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

fn find_diff_map(allergy_map: &AllergyMap) -> ContainMap {
    let mut outer: ContainMap = HashMap::new();

    for (k, val) in allergy_map {
        let mut diff_map: HashMap<usize, HashSet<Ingredient>> = HashMap::new();
        for v in val {
            let set = diff_map.entry(v.0).or_insert(HashSet::new());
            set.insert(v.1.clone());
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
            let i_set = outer.entry(d.clone()).or_insert(HashSet::new());
            i_set.insert(k.clone());
        }
    }
    outer
}

fn find_pairs(
    ingredients: &HashMap<Allergen, HashSet<Ingredient>>,
) -> HashMap<Allergen, Ingredient> {
    let mut pairs = HashMap::new();
    let mut handled = HashSet::new();

    let mut work_map = ingredients.clone();
    loop {
        let mut copy = HashMap::new();
        for (k, val) in &work_map {
            if val.len() == 1 {
                for v in val {
                    pairs.insert(v.clone(), k.clone());
                    handled.insert(v.clone());
                }
            } else {
                let mut new = HashSet::new();
                for v in val {
                    if !handled.contains(v) {
                        new.insert(v.clone());
                    }
                }
                copy.insert(k.clone(), new);
            }
        }
        if copy.is_empty() {
            break;
        }
        work_map = copy;
    }

    pairs
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let foods: Vec<Food> = input.lines().map(|x| parse_to_food(x)).collect();

    let a_cont_in = assemble_allergy_map(&foods);
    let can_contain = assemble_contain_map(&foods);
    let cannot_contain = find_diff_map(&a_cont_in);

    let mut allergen_free = HashSet::new();

    let mut dangers = HashMap::new();

    for (i, a_set) in can_contain {
        if let Some(cant) = cannot_contain.get(&i) {
            let can_really_contain: HashSet<&Allergen> = a_set.difference(&cant).collect();
            println!("{:<8} ----------", i);
            for crc in &can_really_contain {
                println!("{}", crc);
            }
            println!("-------------------");
            if can_really_contain.is_empty() {
                allergen_free.insert(i);
            } else {
                let mut crc: HashSet<Allergen> = HashSet::new();
                for c in can_really_contain {
                    crc.insert(c.clone());
                }
                dangers.insert(i.clone(), crc);
            }
        }
    }

    for (k, val) in &dangers {
        println!("{}", k);
        for v in val {
            println!("\t{}", v);
        }
    }

    let pairs = find_pairs(&dangers);

    let mut allergens = Vec::new();
    for (a, _) in &pairs {
        allergens.push(a.clone());
    }
    allergens.sort();

    println!("Pairs:");
    let mut danger = Vec::new();
    for a in allergens {
        let i = pairs.get(&a).unwrap();
        println!("{}: {}", a, i);
        danger.push(i);
    }

    for i in danger {
        write!(io::stdout(), "{},", i)?;
    }
    writeln!(io::stdout(), "")?;

    Ok(())
}
