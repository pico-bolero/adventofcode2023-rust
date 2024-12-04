use std::ops::Range;

/*
Other thoughts... could look for all the keys in the maps and see if any fall into the range. All non-matches pass through, so you could pick the minimum for each range.
*/
pub struct Something {
    components: Vec<String>,
}
       

// pub struct SomethingIntoIterator {
//     ranges: Vec<Range<u64>>,
//     itr: Option<Box<dyn Iterator<Item = u64>>>,
// }
// 
// impl SomethingIntoIterator {
//     fn new(s: Something) -> SomethingIntoIterator {
//        let components = s.components.clone();
//        let parsed: Vec<u64> = components
//            .iter()
//            .next()
//            .unwrap()
//            .trim()
//            .split(" ")
//            .map(|x| x.trim())
//            .filter(|x| !x.is_empty())
//            .map(|x| x.parse::<u64>().expect("Should parse into u64"))
//            .collect();
// 
//         // make pairs out of the parsed and change them into Ranges
//         let mut parsed_itr = parsed.iter();
//         let mut ranges: Vec<Range<u64>> = vec![];
//         loop {
//             let start = parsed_itr.next();
//             let length = parsed_itr.next();
//             match (start, length) {
//                 (Some(s), Some(l)) => {
//                     println!("Creating ranges for {} {}", s, l);
//                     let r = Range {
//                         start: *s,
//                         end: *s + l,
//                     };
//                     ranges.push(r);
//                 }
//                 _ => {
//                     println!("Finished extracting ranges");
//                     break;
//                 }
//             }
//         }
//         SomethingIntoIterator {
//             ranges,
//             itr: None,
//         }
//     }
// }
// 
// impl IntoIterator for Something {
//     type Item = u64;
//     type IntoIter = SomethingIntoIterator;
//     fn into_iter(self) -> Self::IntoIter {
//         SomethingIntoIterator::new(self) 
//     }
// }
// 
// impl Iterator for SomethingIntoIterator {
//     type Item = u64;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.itr.is_none() {
//             self.itr = Some(Box::new(self.ranges.iter().map(|rng| rng.clone()).flatten()))
//         }
//         match self.itr {
//             Some(mut x) => { x.next() }
//             None => { None }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::*;
    
    fn iter_data() -> Result<Box<dyn Iterator<Item = u32>>, Infallible> {
        let a: Range<u32> = 0..10;
        let b: Range<u32> = 20..25;
        let c: Range<u32> = 30..35;
        
        let rngs = vec![a, b, c];
        let combined = rngs.iter().flat_map(|it| it.clone());
        combined.for_each(|x| println!("{}", x));
        
        //let combined = move || rngs.iter().flat_map(|it| it.clone());
        //let iter_chain = a.chain(b.chain(c));
        let b = move ||  Box::new(rngs.iter().flat_map(|it| it.clone()));
        Ok(b())
    }

    #[test]
    fn test_test() {
        let s = Something{components: vec!["10", "2", "20", "3"].iter().map(|x| x.to_string()).collect()}

        assert!(true);
    }
}
