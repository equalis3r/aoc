use itertools::Itertools;
use multimap::MultiMap;
use std::fmt;

type BagSpec<'a> = (&'a str, &'a str);
type Rules<'a> = MultiMap<BagSpec<'a>, (usize, BagSpec<'a>)>;

fn main() {
    let rules = parse_rules(include_str!("input.txt"));
    let rev_rules = reverse_graph(&rules);

    let needle = &("shiny", "gold");
    let answer_1 = walk_subgraph(&rev_rules, &needle).unique().count();
    println!("Part 1: {} colors can contain {:?} bags", answer_1, needle);

    let root = ("shiny", "gold");
    let answer_2: usize = walk_subgraph_2(&rules, &root).sum();
    println!("Part 2: {} bags needed to fill a {:?} bag", answer_2, root);
}

fn _subgraph_contain(graph: &Rules<'_>, root: &(&str, &str), needle: &(&str, &str)) -> bool {
    graph
        .get_vec(root)
        .into_iter()
        .flatten()
        .any(|(_, neighbor)| neighbor == needle || _subgraph_contain(graph, neighbor, needle))
}

fn reverse_graph<'a>(graph: &Rules<'a>) -> Rules<'a> {
    graph
        .iter_all()
        .map(|(&node, neighbors)| {
            neighbors
                .iter()
                .map(move |&(quant, neighbor)| (neighbor, (quant, node)))
        })
        .flatten()
        .collect()
}

fn walk_subgraph<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = (&'elems str, &'elems str)> + 'iter> {
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .map(move |&(_, neighbor)| {
                std::iter::once(neighbor).chain(walk_subgraph(graph, &neighbor))
            })
            .flatten(),
    )
}

fn walk_subgraph_2<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = usize> + 'iter> {
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .map(move |&(quant, n)| {
                std::iter::once(quant).chain(walk_subgraph_2(graph, &n).map(move |x| x * quant))
            })
            .flatten(),
    )
}

fn parse_rules(input: &str) -> Rules<'_> {
    let mut rules: Rules = Default::default();

    peg::parser! {
        pub(crate) grammar parser() for str {
            pub(crate) rule root(r: &mut Rules<'input>)
                = (line(r) "." whitespace()*)* ![_]

            rule line(r: &mut Rules<'input>)
                = spec:bag_spec() " contain " rules:rules() {
                    if let Some(rules) = rules {
                        for rule in rules {
                            r.insert(spec, rule)
                        }
                    }
                }

            rule bag_spec() -> BagSpec<'input>
                = adjective:name() " " color:name() " bag" "s"? { (adjective, color) }

            rule rules() -> Option<Vec<(usize, BagSpec<'input>)>>
                = rules:one_bag()+ { Some(rules) }
                / "no other bags" { None }

            /// A single rule
            rule one_bag() -> (usize, BagSpec<'input>)
                = quantity:number() " " spec:bag_spec() ", "? { (quantity, spec) }

            rule number() -> usize
                = e:$(['0'..='9']+) { e.parse().unwrap() }

            rule name() -> &'input str
                = $((!whitespace()[_])*)

            rule whitespace()
                = [' ' | '\t' | '\r' | '\n']
        }
    }

    parser::root(input, &mut rules).unwrap();
    rules
}

struct FormattedRules<'a>(Rules<'a>);

impl fmt::Display for FormattedRules<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, vv) in &self.0 {
            write!(f, "{} {} bags contain ", k.0, k.1)?;
            if vv.is_empty() {
                write!(f, "no other bags")?;
            } else {
                for (i, v) in vv.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(
                        f,
                        "{} {} {} {}",
                        v.0,
                        v.1 .0,
                        v.1 .1,
                        if v.0 == 1 { "bag" } else { "bags" }
                    )?;
                }
            }
            writeln!(f, ".")?;
        }
        Ok(())
    }
}
