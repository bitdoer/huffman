use std::collections::HashMap;

/// A custom-made B-tree for doing Huffman coding
pub struct HuffTree {
    /// A pointer to the head(/root) of the tree
    head: Link,
}

/// A type alias for a pointer to a tree node
type Link = Option<Box<Node>>;

/// A node struct containing frequencies, and pointers to children
struct Node {
    /// Leaf nodes will contain a char; others will not
    ch: Option<char>,
    /// All nodes will contain a character frequency; this gets summed up to help with priority queue implementation
    freq: i32,
    /// A pointer to the left child
    left: Link,
    /// A pointer to the right child
    right: Link,
}

impl Node {
    /// Creates a new (leaf) node for the Huffman tree
    ///
    /// ## Arguments
    /// 
    /// * `ch`: the char in the leaf node
    /// * `freq`: that char's frequency
    fn new(ch: char, freq: i32) -> Self {
        Node {
            ch: Some(ch),
            freq: freq,
            left: None,
            right: None,
        }
    }
}

impl HuffTree {
    /// Creates a new empty Huffman tree
    pub fn new() -> Self {
        HuffTree {
            head: None,
        }
    }
    /// Takes an input string and return a hash map of its characters and frequencies
    ///
    /// ## Arguments
    /// 
    /// * `input`: a shared ref to the string to be processed
    pub fn find_input_freqs(input: &String) -> HashMap<char, i32> {
        // make an iterator over the string,
        let mut it = input.chars();
        // prepare an empty hashmap of the kind we need,
        let mut char_map: HashMap<char, i32> = HashMap::new();
        // and then do the iterating
        while let Some(ch) = it.next() {
            // to update the freqs, we see if the char's there and add 1 to its entry---if it's not there, we just
            // pretend it's 0 and then add 1 to it
            let cnt = *char_map.entry(ch).or_insert(0) + 1;
            char_map.insert(ch, cnt);
        }
        char_map
    }

    /// Constructs the huffman tree, given a map of character frequencies
    /// 
    /// ## Arguments
    /// 
    /// * `char_map`: the hash map in question (from `find_input_freqs()`)
    pub fn populate_tree(&mut self, char_map: &HashMap<char, i32>) {
        // set up an empty vector of nodes,
        let mut char_freqs: Vec<Node> = Vec::new();
        // and use a loop to push all the leaves (i.e. the elements of the hash map) into it
        for (key, val) in char_map.iter() {
            let node = Node::new(*key, *val);
            char_freqs.push(node);
        }
        // now we sort from largest to smallest frequency, to turn the thing into a pseudo-priority queue
        char_freqs.sort_by_key(|m| { -m.freq });
        // and while there are at least two things in the queue, repeat the following:
        while char_freqs.len() > 1 {
            // we pop off the smallest two nodes, keeping their frequencies set aside because
            // the memory model hates me,
            let right_freq = char_freqs.last().clone().unwrap().freq;
            let right = char_freqs.pop().map(|node| { Box::new(node) });
            let left_freq = char_freqs.last().clone().unwrap().freq;
            let left = char_freqs.pop().map(|node| { Box::new(node) });
            // then push their parent node onto the vector,
            char_freqs.push(Node {
                ch: None,
                freq: left_freq + right_freq,
                left: left,
                right: right,
            });
            // then re-sort from largest to smallest to again imitate a priority queue
            char_freqs.sort_by_key(|m| { -m.freq });
        }
        // once we're done iterating, whatever is left in the vector of nodes must be the head of our tree
        self.head = char_freqs.pop().map(|node| { Box::new(node) });
    }

    /// Makes the Huffman coding map once the tree is constructed, using tail recursion for tree traversal
    pub fn generate_huffman_map(&mut self) -> HashMap<char, String> {
        let mut huffman_map: HashMap<char, String> = HashMap::new();
        // we begin the tail recursion, passing huffman_map mutably so it gets updated through the recursion
        huffman_map_step(&self.head, String::new(), &mut huffman_map);
        huffman_map
    }

    /// Takes the uncompressed input string and just converts it straight into its huffman coded version
    /// 
    /// ## Arguments
    /// 
    /// `input`: a shared ref to the string to be encoded
    /// `huffman_map`: the Huffman coding map (gotten from `generate_huffman_map()`)
    pub fn encode(input: &String, huffman_map: &HashMap<char, String>) -> String {
        let mut encoded_str = String::new();
        for ch in input.chars() {
            encoded_str += huffman_map.clone().entry(ch).or_insert(String::new());
        }
        encoded_str
    }

    /// Traverses the tree to decode the huffman-coded string, using tail recursion to do so
    ///
    /// ## Arguments
    ///
    /// `encoded_str`: the Huffman-encoded string to be decoded
    pub fn decode(&self, encoded_str: &String) -> String {
        let mut decoded_str = String::new();
        let mut encoded_str_cpy = encoded_str.clone();
        while !encoded_str_cpy.is_empty() {
            decode_step(&self.head, &mut encoded_str_cpy, &mut decoded_str);
        }
        decoded_str
    }
    
    /// Shitty interface wrapper function that, true to name, does it all
    ///
    /// ## Arguments
    /// 
    /// `input`: a shared ref to the string to be manipulated
    pub fn do_it_all(input: &String) -> String {
        let uncompressed_size = input.len() * 8;
        let mut hufftree = HuffTree::new();
        let char_map = HuffTree::find_input_freqs(&(input.clone()));
        println!("Character map:");
        for (key, val) in char_map.clone() {
            println!("{0}: {1}", key, val);
        }
        hufftree.populate_tree(&char_map);
        let huffman_map = hufftree.generate_huffman_map();
        println!("Huffman codes:");
        for (key, val) in huffman_map.clone() {
            println!("{0}: {1}", key, val);
        }
        let encoded_str = HuffTree::encode(input, &huffman_map.clone());
        println!("Encoded string: ");
        println!("{}", encoded_str.clone());
        let compressed_size = encoded_str.len();
        let decoded_str = hufftree.decode(&encoded_str.clone());
        println!("Decoded string: ");
        println!("{}", decoded_str.clone());
        println!("Uncompressed size: {} bits", uncompressed_size);
        println!("Compressed size: {} bits", compressed_size);
        decoded_str
    }
}

/// Tail recursive meat-and-potatoes of the huffman map generation
fn huffman_map_step(curr: &Link, code: String, huffman_map: &mut HashMap<char, String>) {
    // make sure we're not on an empty node, first---that should terminate the recursion
    if curr.is_some() {
        // if we're at a leaf,
        if curr.clone().as_ref().unwrap().left.is_none() && curr.clone().as_ref().unwrap().right.is_none() {
            // then the char in the leaf node gets mapped to the running bitstring
            huffman_map.insert(curr.clone().as_ref().unwrap().ch.clone().unwrap(), code);
        } else {
            // otherwise, step down the tree, and add a 0 to the running bitstring if we go left and a 1 if right
            huffman_map_step(&(curr.clone().as_ref().unwrap().left), code.clone() + "0", huffman_map);
            huffman_map_step(&(curr.clone().as_ref().unwrap().right), code.clone() + "1", huffman_map);
        }
    }
}

/// Tail recursive meat-and-potatoes of the decoding walking; logic is very similar to huffman map gen
fn decode_step(curr: &Link, encoded_str: &mut String, decoded_str: &mut String) {
    // again, empty node should end recursion
    if curr.is_some() {
        // if we're at a leaf,
        if curr.clone().as_ref().unwrap().left.is_none() && curr.clone().as_ref().unwrap().right.is_none() {
            // attach the just-reached character
            decoded_str.push(curr.clone().as_ref().unwrap().ch.clone().unwrap());
        } else {
            // otherwise, traverse left or right depending on the just-removed leftmost bit in the carried encoded bitstring
            if encoded_str.remove(0) == '0' {
                decode_step(&(curr.clone().as_ref().unwrap().left), encoded_str, decoded_str);
            } else {
                decode_step(&(curr.clone().as_ref().unwrap().right), encoded_str, decoded_str);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use super::HuffTree;

    fn whole_thing_works(input: String) -> bool {
        HuffTree::do_it_all(&input.clone()).as_str() == input.clone().as_str()
    }

    fn no_dupes(input: String) -> bool {
        let mut hufftree = HuffTree::new();
        let char_map = HuffTree::find_input_freqs(&(input.clone()));
        hufftree.populate_tree(&char_map);
        let huffman_map = hufftree.generate_huffman_map();
        let mut flag = true;
        for pair in huffman_map.values().combinations(2) {
            if pair[0].as_str() == pair[1].as_str() {
                flag = false;
                break;
            }
        }
        flag
    }

    fn prefix_validity(input: String) -> bool {
        let mut hufftree = HuffTree::new();
        let char_map = HuffTree::find_input_freqs(&(input.clone()));
        hufftree.populate_tree(&char_map);
        let huffman_map = hufftree.generate_huffman_map();
        let mut flag = true;
        for pair in huffman_map.values().permutations(2) {
            if pair[0].starts_with(pair[1].as_str()) {
                flag = false;
                break;
            }
        }
        flag
    }

    #[test]
    fn total_and_freqmap_test() {
        assert!(whole_thing_works("aaabbbbbccddd".to_string()));
        assert!(whole_thing_works("eeffgghhi".to_string()));
        assert!(whole_thing_works("dagoth ur was a hotep".to_string()));
        assert!(whole_thing_works("whether 'tis nobler in the end to suffer th' slings and arrows of outrageous fortune".to_string()));
    }

    #[test]
    fn uniqueness_test() {
        assert!(no_dupes("aaabbbbbccddd".to_string()));
        assert!(no_dupes("eeffgghhi".to_string()));
        assert!(no_dupes("dagoth ur was a hotep".to_string()));
        assert!(no_dupes("whether 'tis nobler in the end to suffer th' slings and arrows of outrageous fortune".to_string()));
    }

    #[test]
    fn valid_prefix_test() {
        assert!(prefix_validity("aaabbbbbccddd".to_string()));
        assert!(prefix_validity("eeffgghhi".to_string()));
        assert!(prefix_validity("dagoth ur was a hotep".to_string()));
        assert!(prefix_validity("whether 'tis nobler in the end to suffer th' slings and arrows of outrageous fortune".to_string()));
    }
}