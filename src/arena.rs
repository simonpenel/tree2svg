use taxonomy::Taxonomy;
// use log::{info, warn};

/// Structure Noeud.
///
#[derive(Debug)]
pub struct Noeud<T>
where
    T: PartialEq
{
    pub idx: usize,
    val: T,
    pub name: String,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub x: f32,
    pub xmod: f32,
    pub y: f32,
    pub e: Event,
}
/// Structure ArenaTree.
///
/// Taken from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq
{
    pub arena: Vec<Noeud<T>>,
}

impl<T> Noeud<T>
where
    T: PartialEq
{
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            name :String::from("Undefined"),
            parent: None,
            children: vec![],
            x: 0.0,
            xmod: 0.0,
            y: 0.0,
            e: Event::Undef,
        }
    }
    pub fn get_val(&mut self) -> &T {
        &self.val
    }
    pub fn get_index(&mut self) -> &usize {
        &self.idx
    }
    pub fn get_x(&mut self) -> &f32 {
        &self.x
    }
    pub fn get_y(&mut self) -> &f32 {
        &self.y
    }
    pub fn get_coords(&mut self) -> ( &f32, &f32) {
        (&self.x,&self.y)
    }

    pub fn set_x(&mut self, x: &f32)
    {
        self.x = *x;
    }
    pub fn set_x_noref(&mut self, x: f32)
    {
        self.x = x;
    }
    pub fn set_xmod_noref(&mut self, xmod: f32)
    {
        self.xmod = xmod;
    }
    pub fn set_y(&mut self, y: &f32)
    {
        self.y = *y;
    }
    pub fn set_y_noref(&mut self, y: f32)
    {
        self.y = y;
    }
    pub fn get_event(&mut self) -> &Event {
        &self.e
    }
    pub fn set_event(&mut self, e: Event)
    {
        self.e = e;
    }
}

/// Arena structure taken from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
impl<T> ArenaTree<T>
where
    T: PartialEq
{
    /// Add a node and send its new index. If the
    /// node already exists, send its index.
    pub fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Noeud::new(idx, val));
        idx
    }
    /// Add a node and send its new index. If the
    /// node already exists, send a panic alert.
    pub fn new_node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                    panic!("Le noeud existe dèja");
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Noeud::new(idx, val));
        // Ok(idx)
        idx
    }
    //A AMELIORER : RENVOYER UN RESULTS
    /// Send the index of the root
    pub fn get_root(&mut self) -> usize {
        //first see if it exists
        for node in &self.arena {
    //        match node.parent {
    //            None => return node.idx,
    //            Some (t) => 0,
            if node.parent == None {
                return node.idx
             }

            }
        0
    }
    /// Send the depth of the tree
    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
}

/// enum of the possible events in a gene tree
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Event {
    Speciation,
    Duplication,
    Loss,
    Transfer,
    Undef,
}
/// There  is no Default pour enum, we define one.
impl Default for Event {
    fn default() -> Self { Event::Undef }
}

/// Fill an ArenaTree structure with the contents of a GeneralTaxonomy structure
pub fn taxo2tree(t: &taxonomy::GeneralTaxonomy, n: usize, tree: &mut ArenaTree<String>) {
    let children = &t.children(n).expect("Pas de fils");
    let name = t.from_internal_id(n).expect("Pas de nom");
    let parent = t.parent(n).expect("Pas de parent");
    let parent_name = match parent {
        None => "root",
        Some ((id, _dist)) => t.from_internal_id(id).expect("Pas de nom")
    };
    let parent_index = match parent {
        None => 0,
        Some ((id, _dist)) => id
    };
    // println!("N = {} Name={:?}  Parent Name={:?} Parent Index={}",n,name,parent_name,parent_index);
    let initial_name = name.clone();
    let initial_parent_name = parent_name.clone();
    let name = "N".to_owned()+&n.to_string()+"_"+name;
    let parent_name = "N".to_owned()+&parent_index.to_string()+"_"+parent_name;
    let name = tree.new_node(name.to_string());

    let parent = tree.node(parent_name.to_string());
    tree.arena[parent].name = initial_parent_name.to_string();
    tree.arena[parent].children.push(name);
    tree.arena[name].parent = Some(parent);

    tree.arena[name].name = initial_name.to_string();
    for child in children {
        taxo2tree(& t,*child,  tree);
    }
}


// pub fn set_tree_coords( tree: &mut ArenaTree<String>) {
// let longueur = tree.arena.len();
// let mut count = 0usize;
//  loop {
//      // tree.arena[count].set_x_noref(10.0* (count as f32));
//       tree.arena[count].set_y_noref(15.0);
//      tree.arena[count].set_y_noref(15.0* (count as f32)+30.0);
//      tree.arena[count].set_x_noref(30.0* (count as f32));
//     count += 1;
//
//     if count == longueur {
//         break;
//     }
// }
// }

pub fn shift_initial_x( tree: &mut ArenaTree<String>, index: usize) {
    let x_father = tree.arena[index].x;
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        let x_left = tree.arena[son_left].x;
        tree.arena[son_left].set_x_noref(x_left + x_father);
        let x_right = tree.arena[son_right].x;
        tree.arena[son_right].set_x_noref(x_right + x_father);
        shift_initial_x( tree, son_left);
        shift_initial_x( tree, son_right);
    }
}
pub fn shift_mod_x_ori( tree: &mut ArenaTree<String>, index: usize,mut  xmod: &mut f32) {
    println!(">>>>>Shifting {:?} xmod={}",tree.arena[index],xmod);
    let x_father = tree.arena[index].x;
    let xmod_father = tree.arena[index].xmod;
    let mut xmod = *xmod + xmod_father;
    tree.arena[index].set_x_noref(x_father+xmod);
    tree.arena[index].set_xmod_noref(xmod);
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        shift_mod_x( tree, son_left, &mut xmod);
        shift_mod_x( tree, son_right, &mut xmod);
        // let x_father = tree.arena[index].x;
        // let xmod_father = tree.arena[index].xmod;
        // let mut xmod = *xmod + xmod_father;
        // tree.arena[index].set_x_noref(x_father+xmod);

    }
}


pub fn bottom_tree( tree: &mut ArenaTree<String>) {
    let root = tree.get_root();
    let mut  max_depth = get_maxdepth(tree,root,&mut 0);
    println!("MAX DEPTH = {}",max_depth);
    set_leaves_to_bottom(tree,root,&mut max_depth);
}
pub fn get_maxdepth( tree: &mut ArenaTree<String>, index:usize, max :&mut usize) -> usize {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        if  tree.depth(son_left) > *max {
            *max =  tree.depth(son_left);
        }
        if  tree.depth(son_right) > *max {
            *max =  tree.depth(son_right);
        }
         get_maxdepth(tree,son_left,max);
         get_maxdepth(tree,son_right,max);
    }
*max
}

pub fn set_leaves_to_bottom( tree: &mut ArenaTree<String>, index: usize, max:&mut  usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        set_leaves_to_bottom(tree,son_left,max);
        set_leaves_to_bottom(tree,son_right,max);
    }
    else {
        tree.arena[index].set_y_noref(60.0* (*max as f32 + 1.0));

    }
}




pub fn shift_mod_x( tree: &mut ArenaTree<String>, index: usize,mut  xmod: &mut f32) {
    println!(">>>>>Shifting {:?} xmod={}",tree.arena[index],xmod);
    let x_father = tree.arena[index].x;
    let mut xmod_father = tree.arena[index].xmod;
    let mut xmod = *xmod + xmod_father;
    tree.arena[index].set_x_noref(x_father+xmod);
    tree.arena[index].set_xmod_noref(xmod);
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        shift_mod_x( tree, son_left, &mut xmod);
        shift_mod_x( tree, son_right, &mut xmod);
        // let x_father = tree.arena[index].x;
        // let xmod_father = tree.arena[index].xmod;
        // let mut xmod = *xmod + xmod_father;
        // tree.arena[index].set_x_noref(x_father+xmod);

    }
}

// pub fn  pseudo_knuth_layout(tree: &mut ArenaTree<String>,index: usize){
//     let longueur = tree.arena.len();
//     let mut count = 0usize;
//     loop {
//         let  h = tree.depth(count);
//         println!("Hauteur du noud {} = {}", count,h);
//         tree.arena[count].set_y_noref(30.0* (h as f32));
//         tree.arena[count].set_x_noref(30.0* (count as f32));
//         count += 1;
//
//         if count == longueur {
//             break;
//         }
//     }
// }

/// Set x and y of nodes :  left son x is 0;  right son x is 1; y is depth
pub fn  knuth_layout(tree: &mut ArenaTree<String>,index: usize,depth: &mut usize){
    tree.arena[index].set_y_noref(60.0* (*depth as f32));
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        tree.arena[son_left].set_x_noref(0.0);
        tree.arena[son_right].set_x_noref(60.0);
        knuth_layout(tree,son_left,&mut(*depth+1));
        knuth_layout(tree,son_right,&mut(*depth+1));
    }
}
/// Traverse the tree using post-order traversal
pub fn  postorder(tree: &mut ArenaTree<String>){
    let root = tree.get_root();
    explore_postorder(tree,root);
}
/// Traverse the tree using post-order traversal starting from a given node  defined by its index
pub fn  explore_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        explore_postorder(tree,left);
        explore_postorder(tree,right);
        println!("POST-ORDER TRAVERSAL : INTERNAL NODE  {:?} / DEPTH = {}",tree.arena[index],tree.depth(index));
    }
    else{
        println!("POST-ORDER TRAVERSAL : LEAF           {:?} / DEPTH = {}",tree.arena[index],tree.depth(index));
    }
}

pub fn  check_contour_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        // push_right(tree,left,right);
        check_contour_postorder(tree,left);
        check_contour_postorder(tree,right);
        push_right(tree,left,right);
    }
    else{
    }
}

/// Set x of nodes using post-order traversal: all the nodes of a given depth are given a
///different x value in order to avoid conflicts.
pub fn  set_x_postorder(tree: &mut ArenaTree<String>,index:usize, x_coords: &mut Vec<usize>) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        set_x_postorder(tree,left,x_coords);
        set_x_postorder(tree,right,x_coords);
        x_coords[tree.depth(index)] += 1;
        let posx = x_coords[tree.depth(index)];
        tree.arena[index].set_x_noref((posx as f32) * 60.0);
    }
    else{
        x_coords[tree.depth(index)] += 1;
        let posx = x_coords[tree.depth(index)];
        tree.arena[index].set_x_noref((posx as f32) * 60.0);
    }
}

pub fn  get_contour_left(tree: &mut ArenaTree<String>,index:usize,depth:usize,contour_left: &mut Vec<f32>,parent_xmod: f32)  {
    println!(">>>>> GCL PROCESS NODE {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    if contour_left.len() <= local_depth {
        // contour_left.push(tree.arena[index].x);
        if tree.arena[index].xmod > 0.0 {
            println!("**** WARNING ***** {:?} xmod = {}",tree.arena[index],tree.arena[index].xmod);
        }
        if tree.arena[index].xmod < 0.0 {
            panic!("erreur xmod");
        }
        contour_left.push(tree.arena[index].x+tree.arena[index].xmod+parent_xmod);
        println!(">>>>> GCL INCREMENT CONTOUR IS NOW {:?}",contour_left);
    }
    // if tree.arena[index].x <= contour_left[local_depth] {
    //  contour_left[local_depth] = tree.arena[index].x;
    // }
    if tree.arena[index].xmod > 0.0 {
        println!(">>>>> GCL  WARNING xmod = {}",tree.arena[index].xmod);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("erreur xmod");
    }

    println!(">>>>> GCL COMPARE  {} + {} + {} INFEQ CTL AT DEPTH {} : {} ",tree.arena[index].x, tree.arena[index].xmod,parent_xmod,local_depth, contour_left[local_depth] );
    if tree.arena[index].x + tree.arena[index].xmod + parent_xmod <= contour_left[local_depth] {
     contour_left[local_depth] = tree.arena[index].x + tree.arena[index].xmod + parent_xmod;
     println!(">>>>> GCL UPDATE CONTOUR IS NOW {:?}",contour_left);
    }

    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        get_contour_left(tree,left,depth,contour_left,tree.arena[index].xmod + parent_xmod );
        // get_contour_left(tree,right,depth,contour_left,tree.arena[index].xmod);
    }
}

pub fn  get_contour_right(tree: &mut ArenaTree<String>,index:usize,depth:usize,contour_right: &mut Vec<f32>,parent_xmod: f32)  {
    println!(">>>>> GCR PROCESS NODE {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    if contour_right.len() <= local_depth {
        // contour_right.push(tree.arena[index].x);

        if tree.arena[index].xmod > 0.0 {
            println!("**** WARNING ***** {:?} xmod = {}",tree.arena[index],tree.arena[index].xmod);
        }
        if tree.arena[index].xmod < 0.0 {
            panic!("erreur xmod");
        }

        contour_right.push(tree.arena[index].x+tree.arena[index].xmod+parent_xmod);
        println!(">>>>> GCR INCREMENT CONTOUR IS NOW {:?}",contour_right);
    }
    // if tree.arena[index].x >= contour_right[local_depth] {
    //  contour_right[local_depth] = tree.arena[index].x;
    // }
    if tree.arena[index].xmod > 0.0 {
        println!(">>>>> GCR  WARNING xmod = {}",tree.arena[index].xmod);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("erreur xmod");
    }

    println!(">>>>> GCR COMPARE  {} + {} + {} SUPEQ CTL AT DEPTH {} : {} ",tree.arena[index].x, tree.arena[index].xmod,parent_xmod,local_depth, contour_right[local_depth] );

    if tree.arena[index].x +  tree.arena[index].xmod + parent_xmod  >= contour_right[local_depth] {
     contour_right[local_depth] = tree.arena[index].x +  tree.arena[index].xmod + parent_xmod ;
     println!(">>>>> GCR UPDATE CONTOUR IS NOW {:?}",contour_right);
    }

    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        // get_contour_right(tree,left,depth,contour_right,tree.arena[index].xmod);
        get_contour_right(tree,right,depth,contour_right,tree.arena[index].xmod + parent_xmod );
    }
}

pub fn  push_right(tree: &mut ArenaTree<String>,left_tree:usize,right_tree:usize) -> f32 {
    println!(">>>>>Compare right contour of {} and left contour of {}",left_tree, right_tree);
    let mut right_co_of_left_tr  = vec![tree.arena[left_tree].x]; //contour droit de l'arbre de gauche
    let mut right_co_of_left_tr  = vec![tree.arena[left_tree].x+tree.arena[left_tree].xmod]; //contour droit de l'arbre de gauche
    let mut depth_left_tr  = tree.depth(left_tree);
    get_contour_right(tree,left_tree,depth_left_tr,&mut right_co_of_left_tr,0.0);
    println!(">>>>>Right contour of {} = {:?}",left_tree,right_co_of_left_tr);
    let mut left_co_of_right_tr  = vec![tree.arena[right_tree].x]; //contour droit de l'arbre de gauche
    let mut left_co_of_right_tr  = vec![tree.arena[right_tree].x+tree.arena[right_tree].xmod]; //contour droit de l'arbre de gauche
    let mut depth_right_tr  = tree.depth(right_tree);
    get_contour_left(tree,right_tree,depth_right_tr,&mut left_co_of_right_tr,0.0);
    println!(">>>>>Left contour of {} = {:?}",right_tree,left_co_of_right_tr);
    // let mut iter = left_co_of_right_tr.iter().zip(right_co_of_left_tr);
    //         println!("ITER = {:?}",iter);

    // Si on   a pas le meme longeur de ocntour je comeple le plus petit
    // en remplissant ce qui manque avec la derniere valeur, pour evoter
    // qu'un sous arbre vosins se place sous une feuille
    let right_len = right_co_of_left_tr.len();
    let left_len = left_co_of_right_tr.len();
    if left_len > right_len {
        let last_val =  right_co_of_left_tr[right_len-1];
        let last_vals =  vec![last_val;left_len-right_len];
        right_co_of_left_tr.extend(last_vals.iter().copied());
        println!(">>>>> Comeplete right contour with last value {}", last_val);
    }
    if left_len < right_len {
        let last_val =  left_co_of_right_tr[left_len-1];
        let last_vals =  vec![last_val;right_len - left_len];
        left_co_of_right_tr.extend(last_vals.iter().copied());
        println!(">>>>> Comeplete left contour with last value {}", last_val);
    }

    println!(">>>>>Comparing  ROL {:?} with LOR {:?} ",right_co_of_left_tr,left_co_of_right_tr);

    let mut iter = left_co_of_right_tr.iter().zip(right_co_of_left_tr).map(|(x, y )| (x-y));

    let shift = iter.min_by(|x, y| (*x as i64) .cmp(&(*y as i64 )));
        // println!("ITER = {:?}",iter.min_by(|x, y| (*x as i64) .cmp(&(*y as i64 ))));
    println!("ITER = {:?}",shift);
    match shift {
    Some(val) => {
        println!("SHIFTED = {}",val);
        if val <= 0.0 {// bidouilel
            println!(">>>> ================CONFLIT==========");
            println!(">>>> Modify node {:?}",tree.arena[right_tree]);
            let x_mod =  tree.arena[right_tree].xmod;
            println!("Initial x_mod = {}",x_mod);
            // let x_mod =  x_mod - 1.0 *val + 60.0 ;
                    let x_mod =  x_mod -1.0 *val + 60.0 ;//bidouille
                    // let x_mod =  x_mod - 2.0 *val + 30.0 ;//bidouille
                    // let x_mod =  x_mod - 1.0 *val + 60.0 ;//bidouille

            println!("New x_mod = {}",x_mod);
            tree.arena[right_tree].set_xmod_noref(x_mod);
            println!(">>>> Updated node {:?}",tree.arena[right_tree]);
            println!(">>>> ================CONFLIT==========");
        }
        },
    None => {}
    }


    // for (x, y) in iter  {
    //     println!(" ITER {:?}  {:?} ",x,y);
    //
    // }
    // for (z) in iter  {
    //     println!(" ITER {:?} ",z);
    //
    // }

    // println!("ITER = {:?}",iter);

    0.0
}


pub fn  set_middle_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        set_middle_postorder(tree,left);
        set_middle_postorder(tree,right);
        println!("SETTING MIDDLE  FOR NODE {:?}",index);
        let x_left = tree.arena[left].x;
        let x_right = tree.arena[right].x;
        let x = tree.arena[index].x;
        let x_middle = ( x_right + x_left ) / 2.0 ;
        println!("POST-ORDER MODIF FATHER  X avant  = {} X apres  = {}",x,x_middle);
        // println!("POST-ORDER MODIF FATHER {:?} X = {}",tree.arena[index],tree.arena[index].x);
        // println!("POST-ORDER MODIF FATHER  X LEFT = {} X RIGHT ={}",x_left, x_right);
        tree.arena[index].set_x_noref(x_middle);
        let x_mod =  tree.arena[right].xmod;
        let x_mod =  x_mod + x_middle - x;
        tree.arena[index].set_xmod_noref(x_mod);
        // tree.arena[index].set_x_noref(x_middle);
        // println!("POST-ORDER INT {:?}",tree.arena[index]);

    }
    else{
        // println!("POST-ORDER LEAF {:?}",tree.arena[index]);
    }
}


pub fn  find_leftest(tree: &mut ArenaTree<String>,index:usize) -> usize {
    println!("FIND LEFTEST: {:?}",tree.arena[index]);
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        find_leftest(tree,left)
    }
    else {
        index
    }
}
