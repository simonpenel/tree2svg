use taxonomy::Taxonomy;
use log::{info};
// const BLOCK: f32 = 30.0;
pub const BLOCK: f32 = 100.0;
// const PIPEBLOCK: f32 = BLOCK / 4.0;
const PIPEBLOCK: f32 = BLOCK / 4.0;
const MINWIDTH: f32 = BLOCK / 4.0;
// Structures
// ==========

/// Structure Noeud.
///
#[derive(Debug)]
pub struct Noeud<T>
where
    T: PartialEq
{
    pub idx: usize,             // index dans la structure
    val: T,                     // valeur unique dans la structure
    pub name: String,           // nom du noeud ou de la feuille
    pub parent: Option<usize>,  // index du parent
    pub children: Vec<usize>,   // indexes des enfants
    pub x: f32,                 // coordonnee x (avant rotation 90 svg)
    pub xmod: f32,              // decalage x a ajouter a x
    pub y: f32,                 // coordonnee y (avant rotation 90 svg)
    pub ymod: f32,              // decalage y a ajouter a y (pour les arbres reconcilies)
    pub l: f32,                 // longueur de branche lue dans le fichier
    pub e: Event,               // evenement (dans le cas d'arbre de gene) Duplication, Loss, etc.
    pub location: String,       // SpeciesLocaton associe evenement (dans le cas d'arbre de gene)
    pub width: f32,             // largeur du tuyeau (dans le cas d'arbre d'espece)
    pub height: f32,            // hauteur du tuyeau (dans le cas d'arbre d'espece)
    pub nbg: usize,             // nombre de noeud  d'arbre de genes associcés à ce noeud  (dans le cas d'arbre d'espece)
    pub nodes: Vec<usize>,      // gene nodes associes (dans le cas d'arbre d'espece)
}

impl<T> Noeud<T>
where
    T: PartialEq
{
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            name: String::from("Undefined"),
            parent: None,
            children: vec![],
            x: 0.0,
            xmod: 0.0,
            y: 0.0,
            ymod: 0.0,
            l: 0.0,
            e: Event::Undef,
            location: String::from("Undefined"),
            width: PIPEBLOCK ,
            height: PIPEBLOCK ,
            nbg: 0,
            nodes: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn get_val(&mut self) -> &T {
        &self.val
    }
    #[allow(dead_code)]
    pub fn get_index(&mut self) -> &usize {
        &self.idx
    }
    #[allow(dead_code)]
    pub fn get_x(&mut self) -> &f32 {
        &self.x
    }
    #[allow(dead_code)]
    pub fn get_y(&mut self) -> &f32 {
        &self.y
    }
    #[allow(dead_code)]
    pub fn get_coords(&mut self) -> ( &f32, &f32) {
        (&self.x,&self.y)
    }
    #[allow(dead_code)]
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
    pub fn set_ymod_noref(&mut self, ymod: f32)
    {
        self.ymod = ymod;
    }

    #[allow(dead_code)]
    pub fn set_y(&mut self, y: &f32)
    {
        self.y = *y;
    }

    pub fn set_y_noref(&mut self, y: f32)
    {
        self.y = y;
    }
    #[allow(dead_code)]
    pub fn get_event(&mut self) -> &Event {
        &self.e
    }
    #[allow(dead_code)]
    pub fn set_event(&mut self, e: Event)
    {
        self.e = e;
    }
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
    #[allow(unreachable_code)]
    pub fn get_root(&mut self) -> usize {
        for node in &self.arena {
    //        match node.parent {
    //            None => return node.idx,
    //            Some (t) => 0,
            if node.parent == None {
                return node.idx
             }

            }
        panic!("Unable to get root of the tree");
        0
    }

    /// is leaf
    pub fn is_leaf(&self, idx: usize) -> bool {
        match self.arena[idx].children.len() {
        0 => true,
        _ => false,
        }
    }

    /// Send the depth of the tree
    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
    pub fn get_largest_x(&mut self) -> f32 {
        let mut max = 0.0;
        for node in &self.arena {
            if node.x + node.width > max {
                max = node.x + node.width;
             }
            }
        max
    }
    pub fn get_largest_y(&mut self) -> f32 {
        let mut max = 0.0;
        for node in &self.arena {
            if node.y + node.height > max {
                max = node.y + node.height ;
             }
            }
        max
    }

    pub fn get_smallest_x(&mut self) -> f32 {
        let mut min = 1000000.0;
        for node in &self.arena {
            if node.x - node.width < min {
                min = node.x - node.width;
             }
            }
        min
    }

    pub fn get_smallest_y(&mut self) -> f32 {
        let mut min = 1000000.0;
        for node in &self.arena {
            if node.y - node.height < min {
                min = node.y - node.height;
             }
            }
        min
    }
    // pub fn get_largest_y(&mut self) -> f32 {
    //     let mut max = 0.0;
    //     for node in &self.arena {
    //         if node.y + node.height > max {
    //             max = node.y;
    //          }
    //         }
    //     max
    // }
    #[allow(dead_code)]
    pub fn rotate(&mut self)  {
        let root = self.get_root();
        let x_0 = self.arena[root].x;
        let y_0 = self.arena[root].y;
        for  node in &mut self.arena {
            let x = node.x;
            let y = node.y;
            node.x = y + -y_0 + x_0;
            node.y = -x + x_0 + y_0;
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
    BranchingOut,
    TransferBack,
    BifurcationOut,
    Leaf,
    Undef,
}
/// There  is no Default pour enum, we define one.
impl Default for Event {
    fn default() -> Self { Event::Undef }
}

// Fonctions
// =========

/// Fill an ArenaTree structure with the contents of a GeneralTaxonomy structure
pub fn taxo2tree(t: &taxonomy::GeneralTaxonomy, n: usize, tree: &mut ArenaTree<String>) {
    let children = &t.children(n).expect("Pas de fils");
    let name = t.from_internal_id(n).expect("Pas de nom");
    let parent = t.parent(n).expect("Pas de parent");
    let parent_name = match parent {
        None => "root",
        Some ((id, _dist)) => t.from_internal_id(id).expect("Pas de nom")
    };
    let parent_dist = match parent {
        None => -1.0,
        Some ((_id, dist)) => {
            dist
        },
    };
    let parent_index = match parent {
        None => 0,
        Some ((id, _dist)) => id
    };
    let initial_name = name.clone();
    let initial_parent_name = parent_name.clone();
    let name = "N".to_owned()+&n.to_string()+"_"+name;
    let parent_name = "N".to_owned()+&parent_index.to_string()+"_"+parent_name;
    let name = tree.new_node(name.to_string());
    let parent = tree.node(parent_name.to_string());
    tree.arena[parent].name = initial_parent_name.to_string();
    tree.arena[parent].children.push(name);
    tree.arena[name].parent = Some(parent);
    tree.arena[name].l = parent_dist;
    tree.arena[name].name = initial_name.to_string();
    for child in children {
        taxo2tree(& t,*child,  tree);
    }
}
/// Fill an ArenaTree structure with the contents of a roxmltre::Node structure
pub fn xml2tree(node: roxmltree::Node, parent: usize, mut numero : &mut usize, mut  tree: &mut ArenaTree<String>) {
        // je cherche les fils
        let children = node.children();
         for child in children {
            if child.has_tag_name("clade"){
                    // increment le numero
                    *numero += 1;
                    // Nouveau nom
                    let name = "N".to_owned()+&numero.to_string();
                    //  index de ce nouveau nom
                    let name = tree.new_node(name.to_string());
                    //Ajoute ce noeud au parent
                    tree.arena[parent].children.push(name);
                    // Attribue un parent a ce noeud
                    tree.arena[name].parent = Some(parent);
                    // Explore le reste de l'arbre a partir de ce noeud
                    xml2tree(child, name, &mut numero, &mut tree);

            }
            // Attribue le nom defini dans le tag id
            if child.has_tag_name("id"){
                let nom = child.first_child().unwrap().text();
                match nom {
                    Some(text) => tree.arena[parent].name = text.to_string(),
                    None    => tree.arena[parent].name = "Unkwnown".to_string(),
                };
            }
            // Attribue le nom defini dans le tag name
            if child.has_tag_name("name"){
                let nom = child.first_child().unwrap().text();
                match nom {
                    Some(text) => tree.arena[parent].name = text.to_string(),
                    None    => tree.arena[parent].name = "Unkwnown".to_string(),
                };
            }
            // Attribue l evenement
            if child.has_tag_name("eventsRec"){
                info!("xml2tree:Event detected");
                let mut event_num = 0; // Le nb d'evenements dans balise eventsRec
                for evenement in child.children() {
                        if evenement.has_tag_name("loss"){
                            event_num += 1;
                            info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                            tree.arena[parent].set_event(Event::Loss);
                            assert!(evenement.has_attribute("speciesLocation"));
                            assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                            let location = evenement.attributes()[0].value();
                            tree.arena[parent].location = location.to_string();
                        }
                        if evenement.has_tag_name("leaf"){
                            event_num += 1;
                            info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                            // TODO
                            // C'est une feuille seulement si c'est le premier evenement:
                            // <eventsRec>
                            //   <leaf speciesLocation="5"></leaf>
                            // </eventsRec>
                            //  mais pas dans les autres cas
                            // <eventsRec>
                            //   <transferBack destinationSpecies="4"></transferBack>
                            //   <leaf speciesLocation="4"></leaf>
                            // </eventsRec>

                            if event_num == 1 {
                                tree.arena[parent].set_event(Event::Leaf);

                                info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                                let nb_att = evenement.attributes().len();
                                info!("Number of attributes  = {}",nb_att);
                                assert!(evenement.has_attribute("speciesLocation"));
                                if nb_att == 1 {
                                    assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                                    let location = evenement.attributes()[0].value();
                                    tree.arena[parent].location = location.to_string();
                                }
                                else {
                                    // TODO tres sale
                                    assert_eq!(evenement.attributes()[1].name(),"speciesLocation");
                                    let location = evenement.attributes()[1].value();
                                    tree.arena[parent].location = location.to_string();
                                }
                            }

                        }
                        if evenement.has_tag_name("speciation"){
                            event_num += 1;
                            info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                            // TODO
                            // C'est une speciation seulement si c'est le premier evenement:
                            if event_num == 1 {
                                tree.arena[parent].set_event(Event::Speciation);
                                info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                                assert!(evenement.has_attribute("speciesLocation"));
                                assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                                let location = evenement.attributes()[0].value();
                                info!("xml2tree: set location = {}",location);
                                tree.arena[parent].location = location.to_string();
                            }
                        }
                        if evenement.has_tag_name("duplication"){
                            event_num += 1;
                            // TODO
                            // C'est une duplication seulement si c'est le premier evenement:
                            if event_num == 1 {
                                info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                                tree.arena[parent].set_event(Event::Duplication);
                                info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                                assert!(evenement.has_attribute("speciesLocation"));
                                assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                                let location = evenement.attributes()[0].value();
                                info!("xml2tree: set location = {}",location);
                                tree.arena[parent].location = location.to_string();
                            }
                        }
                        if evenement.has_tag_name("branchingOut"){
                            event_num += 1;
                            // TODO
                            // C'est une duplication seulement si c'est le premier evenement:
                            if event_num == 1 {
                                info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                                tree.arena[parent].set_event(Event::BranchingOut);
                                info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                                assert!(evenement.has_attribute("speciesLocation"));
                                assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                                let location = evenement.attributes()[0].value();
                                info!("xml2tree: set location = {}",location);
                                tree.arena[parent].location = location.to_string();
                            }
                        }
                        // TODO
                        // a verifier
                        if evenement.has_tag_name("transferBack"){
                            // Ici on plusieurs evenements
                            // Par exemple
                            // <eventsRec>
                            // <transferBack destinationSpecies="5"></transferBack>
                            // <branchingOut speciesLocation="5"></branchingOut>
                            // </eventsRec>
                            // ou
                            // <eventsRec>
                            // <transferBack destinationSpecies="10"></transferBack>
                            // <speciation speciesLocation="10"></speciation>
                            // </eventsRec>
                            // Le destinationSpecies est donc l'emplacement ou doit etre
                            // le noeud représentant l'arivee du transfert
                            // le point de depart du transfer etant le pere de ce noeud
                            event_num += 1;
                            info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                            tree.arena[parent].set_event(Event::TransferBack);
                            info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                            assert!(evenement.has_attribute("destinationSpecies"));
                            assert_eq!(evenement.attributes()[0].name(),"destinationSpecies");
                            let location = evenement.attributes()[0].value();
                            info!("xml2tree: set destinationSpecies = {}",location);
                            tree.arena[parent].location = location.to_string();
                        }
                        // TODO
                        if evenement.has_tag_name("bifurcationOut"){
                            event_num += 1;
                            info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                            tree.arena[parent].set_event(Event::BifurcationOut);
                            info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                            let grandparent =  tree.arena[parent].parent;
                            match grandparent {
                                Some(p)     => {
                                    let location =  &tree.arena[p].location;
                                    info!("xml2tree: set location according to its father = {}",location);
                                    tree.arena[parent].location = location.to_string();},
                                None        => panic!("BifurcationOut node as no parent : {:?}",tree.arena[parent]),
                            };
                            //  A verifier
                            // Meme espece que son pere
                            // assert!(evenement.has_attribute("destinationSpecies"));
                            // assert_eq!(evenement.attributes()[0].name(),"destinationSpecies");
                            // let location = evenement.attributes()[0].value();
                            // tree.arena[parent].location = location.to_string();
                        }






                // match nom {
                //     Some(text) => tree.arena[parent].name = text.to_string(),
                //     None    => tree.arena[parent].name = "Unkwnown".to_string(),
                // };
            }
                info!("xml2tree:Event closed");
                // println!("Event = {:?}",evenement);
            }



    }
}
/// Set the coordinates of the gene tree according to species tree coordinates
pub fn map_gene_tree(mut sp_tree: &mut ArenaTree<String>, mut gene_tree: &mut ArenaTree<String>) {
    for  index in &mut gene_tree.arena {
        let mut mapped = false;
        // println!("MAP node {:?} event {:?} location {:?}",index.idx, index.e,index.location);
        for spindex in  &mut sp_tree.arena {
            if  index.location == spindex.name {
                mapped = true;
                let x = spindex.x;
                index.x = x;
                let y = spindex.y;
                index.y = y;
                info!("map_tree: Gene node {:?} mapped to  species node {:?}",index,spindex);
            }
        }
        if !mapped {
            panic!("Unable to map Node {:?}",index);
        }
    }
}
/// Determine the number of gene nodes associated to a species node
pub fn map_species_tree(mut sp_tree: &mut ArenaTree<String>, mut gene_tree: &mut ArenaTree<String>) {
    for  index in &mut gene_tree.arena {
        let mut mapped = false;
        // println!("MAP node {:?} event {:?} location {:?}",index.idx, index.e,index.location);
        for spindex in  &mut sp_tree.arena {
            if  index.location == spindex.name {
                mapped = true;
                let mut nbg = spindex.nbg;
                nbg = nbg + 1 ;
                spindex.nbg = nbg;
                spindex.nodes.push(index.idx);
                info!("map_tree: Gene node {:?} mapped to  species node {:?}",index,spindex);
            }
        }
        if !mapped {
            panic!("Unable to map Node {:?}",index);
        }
    }
}

pub fn bilan_mapping(mut sp_tree: &mut ArenaTree<String>, mut gene_tree: &mut ArenaTree<String>, index: usize) {
    println!("BILAN MAPPING : Species Node {}",sp_tree.arena[index].name);
        let ratio = 2.0 ; // permet de rglere l'ecrtement entre les noeid de genes dans l'arbre d'espece
        let  mut shift = 0.0;
        for node in &sp_tree.arena[index].nodes {
            println!(">>> {:?} {:?}",gene_tree.arena[*node].name,gene_tree.arena[*node].e);
            match  gene_tree.arena[*node].e {
                Event::Duplication => {
                    let y = gene_tree.arena[*node].y;
                    let y = y - PIPEBLOCK / ratio;
                    gene_tree.arena[*node].set_y_noref(y);
                    // TO DO ou pas:
                    // let x = gene_tree.arena[*node].x;
                    // let x = x + PIPEBLOCK*shift / ratio;
                    // gene_tree.arena[*node].set_x_noref(x);
                    shift = shift + 1.0;

                    let mut children =  &mut  gene_tree.arena[*node].children;
                    if children.len() > 0 {
                        let son_left = children[0];
                        let son_right = children[1];

                        let  xmod = gene_tree.arena[son_left].xmod;
                        let  xmod = xmod  - PIPEBLOCK / ratio ;
                        gene_tree.arena[son_left].set_xmod_noref(xmod);
                    // Si le noeud  droit n'est pas une feuille on le decale en y
                    // TODO a améliorer : feuille a gauche et noeud interne  a droite?
                    let is_leaf = match gene_tree.arena[son_right].e {
                         Event::Leaf => true,
                         _           => false,
                    };

                        let ymod =gene_tree.arena[son_left].ymod;
                        let ymod = ymod  + PIPEBLOCK / ratio * 1.0 ;
                        gene_tree.arena[son_left].set_ymod_noref(ymod);
                        let ymod = gene_tree.arena[son_right].ymod;
                        let ymod = ymod  + PIPEBLOCK / ratio * 1.0 ;
                        gene_tree.arena[son_right].set_ymod_noref(ymod);
                    }


                },
                Event::Speciation => {
                    let x = gene_tree.arena[*node].x;
                    let x = x + PIPEBLOCK*shift / ratio;
                    gene_tree.arena[*node].set_x_noref(x);

                    let y = gene_tree.arena[*node].y;
                    let y = y + PIPEBLOCK*shift / ratio;
                    gene_tree.arena[*node].set_y_noref(y);

                    shift = shift + 1.0;


                },
                Event::Leaf => {
                    let x = gene_tree.arena[*node].x;
                    let x = x + PIPEBLOCK*shift / ratio;
                    gene_tree.arena[*node].set_x_noref(x);

                    // let y = gene_tree.arena[*node].y;
                    // let y = y + PIPEBLOCK*shift;
                    // gene_tree.arena[*node].set_y_noref(y);

                    shift = shift + 1.0;


                },
                Event::Loss => {
                    let x = gene_tree.arena[*node].x;
                    let x = x + PIPEBLOCK*shift / ratio;
                    gene_tree.arena[*node].set_x_noref(x);

                    let y = gene_tree.arena[*node].y;
                    let y = y + PIPEBLOCK*shift / ratio;
                    gene_tree.arena[*node].set_y_noref(y);

                    shift = shift + 1.0;


                },
                _=> {},
            }
        }
    let mut children =  &mut  sp_tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
         bilan_mapping( sp_tree, gene_tree,son_left);
         bilan_mapping( sp_tree, gene_tree,son_right);
         // bilan_mapping(&mut sp_tree, &mut gene_tree,children[1]);
    }
}

pub fn set_species_width(mut sp_tree: &mut ArenaTree<String>) {
    for spindex in  &mut sp_tree.arena {
        let  nbg = spindex.nbg;
        if nbg > 0 {
            spindex.width =  nbg as f32 * PIPEBLOCK;
            spindex.height = nbg as f32 * PIPEBLOCK;
        }
        else {
            spindex.width =   PIPEBLOCK;
            spindex.height =  PIPEBLOCK;
        }
    }
}

// Renvoie le NodeId du premier tag "clade"
pub fn find_first_clade(  doc: &mut roxmltree::Document) -> Result < roxmltree::NodeId, usize> {
    let descendants = doc.root().descendants();
    // Search for the first occurnce of clade tag
    for  node in descendants {
        if node.has_tag_name("clade"){
            // return Ok(node.id().get())
            return Ok(node.id())
        }
    }
    Err(0)
}
// Renvoie le NodeId du premier tag "spTree"
pub fn find_sptree( doc: &mut roxmltree::Document) -> Result < roxmltree::NodeId, usize> {
    let descendants = doc.root().descendants();
    // Search for the first occurnce of clade spTree
    for  node in descendants {
        if node.has_tag_name("spTree"){
            // return Ok(node.id().get())
            return Ok(node.id())
        }
    }
    Err(0)
}

// Renvoie le NodeId du premier tag "regGeneTree"
pub fn find_rgtree( doc: &mut roxmltree::Document) -> Result < roxmltree::NodeId, usize> {
    let descendants = doc.root().descendants();
    // Search for the first occurnce of clade spTree
    for  node in descendants {
        if node.has_tag_name("recGeneTree"){
            // return Ok(node.id().get())
            return Ok(node.id())
        }
    }
    Err(0)
}


//
// pub fn find_first_tag( mut doc: &mut roxmltree::Document, tag: String) -> Result < roxmltree::NodeId, usize> {
// let mut descendants = doc.root().descendants();
// // Search for the first occurnce of clade tag
// for  node in descendants {
//     if node.has_tag_name(tag){
//         // return Ok(node.id().get())
//         return Ok(node.id())
//     }
// }
// Err(0)
// }



/// Set x and y of nodes :  left son x is 0;  right son x is 1; y is depth
pub fn  knuth_layout(tree: &mut ArenaTree<String>,index: usize,depth: &mut usize){
    tree.arena[index].set_y_noref(BLOCK* (*depth as f32));
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        tree.arena[son_left].set_x_noref(0.0);
        tree.arena[son_right].set_x_noref(BLOCK);
        knuth_layout(tree,son_left,&mut(*depth+1));
        knuth_layout(tree,son_right,&mut(*depth+1));
    }
}

/// Transforms the tree into cladogram
pub fn cladogramme( tree: &mut ArenaTree<String>) {
    let root = tree.get_root();
    let mut  max_depth = get_maxdepth(tree,root,&mut 0);
    set_leaves_to_bottom(tree,root,&mut max_depth);
}

/// Transforms the tree into real branch  length representation
pub fn real_length( tree: &mut ArenaTree<String>, index: usize, dist: &mut f32) {
    let  dist_father = tree.arena[index].l;
    let mut dist = *dist + dist_father;
    tree.arena[index].set_y_noref(dist * BLOCK);
    let children  = &mut  tree.arena[index].children;
    if children.len() > 1 {
        let son_left = children[0];
        let son_right = children[1];
        real_length( tree, son_left, &mut dist);
        real_length( tree, son_right, &mut dist);
    }

}

/// Get the depth of the tree
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

/// Set the y values of the leaves of the node index to  max value
pub fn set_leaves_to_bottom( tree: &mut ArenaTree<String>, index: usize, max:&mut  usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        set_leaves_to_bottom(tree,son_left,max);
        set_leaves_to_bottom(tree,son_right,max);
    }
    else {
        match tree.arena[index].e {
            Event::Loss => tree.arena[index].set_y_noref(BLOCK* (*max as f32 )),
            _ => tree.arena[index].set_y_noref(BLOCK* (*max as f32 + 1.0)),
        };
        // tree.arena[index].set_y_noref(BLOCK* (*max as f32 + 1.0));
    }
}

/// Shift the  x values  of a node and its children according to the cumulated xmod values
// pub fn shift_mod_x( tree: &mut ArenaTree<String>, index: usize, xmod: &mut f32) {
//     info!("shift_mod_x: shifting {:?} xmod={}",tree.arena[index],xmod);
//     let x_father = tree.arena[index].x;
//     let  xmod_father = tree.arena[index].xmod;
//     let mut xmod = *xmod + xmod_father;
//     tree.arena[index].set_x_noref(x_father+xmod);
//     tree.arena[index].set_xmod_noref(xmod);
//     let children  = &mut  tree.arena[index].children;
//     if children.len() > 2 {
//         panic!("L'arbre doit être binaire")
//     }
//     if children.len() > 1 {
//         let son_left = children[0];
//         let son_right = children[1];
//         shift_mod_x( tree, son_left, &mut xmod);
//         shift_mod_x( tree, son_right, &mut xmod);
//     }
//
// }

/// Shift the  x values  of a node and its children according to the cumulated xmod values
pub fn shift_mod_xy( tree: &mut ArenaTree<String>, index: usize, xmod: &mut f32, ymod: &mut f32) {
    info!("shift_mod_xy: shifting {:?} xmod={} ymod={}",tree.arena[index],xmod,ymod);
    let x_father = tree.arena[index].x;
    let  xmod_father = tree.arena[index].xmod;
    let mut xmod = *xmod + xmod_father;
    tree.arena[index].set_x_noref(x_father+xmod);
    // tree.arena[index].set_xmod_noref(xmod);inutile
    let y_father = tree.arena[index].y;
    let  ymod_father = tree.arena[index].ymod;
    let mut ymod = *ymod + ymod_father;
    tree.arena[index].set_y_noref(y_father+ymod);
    // tree.arena[index].set_ymod_noref(ymod);inutile
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 1 {
        let son_left = children[0];
        let son_right = children[1];
        shift_mod_xy( tree, son_left, &mut xmod, &mut ymod);
        shift_mod_xy( tree, son_right, &mut xmod, &mut ymod);
    }

}



pub fn shift_duplicated_and_loss(tree: &mut ArenaTree<String>, index: usize) {
    //  Evenement associe au noeud courant:
    let is_dupli = match tree.arena[index].e {
         Event::Duplication => true,
         _                  => false,
    };
    let is_loss = match tree.arena[index].e {
         Event::Loss => true,
         _           => false,
    };
    let is_bifu = match tree.arena[index].e {
         Event::BifurcationOut => true,
         _           => false,
    };
    let is_brout = match tree.arena[index].e {
         Event::BranchingOut => true,
         _           => false,
    };
    let is_trback = match tree.arena[index].e {
         Event::TransferBack => true,
         _           => false,
    };

    if is_bifu {
        // Les noeud bifurcationOut sont affiches juste 1 cran en dessous de leur parent
        //  Et decale en x
        let p = tree.arena[index].parent;
        match p {
            Some(p) => {
                let y = tree.arena[p].y;
                let y = y + BLOCK  ;
                tree.arena[index].set_y_noref(y);
                // TODO
                // x et pas modx car on decal pas les decsendants
                let x = tree.arena[p].x;
                let x = x + PIPEBLOCK/2.0  ;
                tree.arena[index].set_x_noref(x);
                },
            None => {
                panic!("This BifurcationOut node has no parent {:?}",tree.arena[index]);
                },
        }
    }
    if is_loss {
        // Les noeud Loss sont affiches juste 1 cran en dessous de leur parent
        let p = tree.arena[index].parent;
        match p {
            Some(p) => {
                let y = tree.arena[p].y;
                let y = y + BLOCK  ;
                tree.arena[index].set_y_noref(y);
                },
            None => {
                panic!("This Loss node has no parent {:?}",tree.arena[index]);
                },
        }
    }
    if is_brout {
        let p = tree.arena[index].parent;
        match p {
            Some(p) => {
                let y = tree.arena[p].y;
                let y = y + BLOCK  ;
                tree.arena[index].set_y_noref(y);
                },
            None => {
                panic!("This BranchingOut node has no parent {:?}",tree.arena[index]);
                },
        }
    }

    let children  =  &mut tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        if is_dupli || is_bifu  {
            let p = tree.arena[index].parent;
            // Un noeud Duplication est initialement positioné en y  comme
            // le noeud de l'espèce associée. On le positione un peu moins
            // d'un cran en dessous du parent  pour permettre la visualisation
            match p {
                Some(p) => {
                    // en fait ca fou la merde quand plusieur dupli
                    // let y = tree.arena[p].y;
                    // let y = y + BLOCK  - PIPEBLOCK;
                    // tree.arena[index].set_y_noref(y);

                    // je vais plutot baisser les fils
                    // let y = tree.arena[index].y;
                    // let y = y  - PIPEBLOCK;
                    // tree.arena[index].set_y_noref(y);


                    } ,
                None => {
                     // Sinon je relece ub peu
                                         // je vais plutot baisser les fils
                    // let y = tree.arena[index].y;
                    // let y = y  - PIPEBLOCK;
                    // tree.arena[index].set_y_noref(y);
                },
            }
            // Je decale en y versle bas pour me detacher du noeud de dupli
            // let  y = tree.arena[son_right].y;
            // let  y = y  + PIPEBLOCK / 2.0 ;
            // tree.arena[son_right].set_y_noref(y);
            // let  y = tree.arena[son_left].y;
            // let  y = y  + PIPEBLOCK / 1.0 ;
            // tree.arena[son_left].set_y_noref(y);


            // let  ymod = tree.arena[son_right].ymod;
            // let  ymod = ymod  + PIPEBLOCK / 2.0 ;
            // tree.arena[son_right].set_ymod_noref(ymod);
            // let  ymod = tree.arena[son_left].ymod;
            // let  ymod = ymod  + PIPEBLOCK / 2.0 ;
            // tree.arena[son_left].set_ymod_noref(ymod);


            // On décale en x à gauche et à droite les noeuds issus de la duplication
            // Attention je ne mdofie plus le fils de gauche
            // let  xmod = tree.arena[son_left].xmod;
            // let  xmod = xmod  - PIPEBLOCK / 2.0 ;
            // tree.arena[son_left].set_xmod_noref(xmod);
            let  xmod = tree.arena[son_right].xmod;
            let  xmod = xmod  + PIPEBLOCK / 2.0 ;
            tree.arena[son_right].set_xmod_noref(xmod);
            // Si le noeud  droit n'est pas une feuille on le decale en y
            // TODO a améliorer : feuille a gauche et noeud interne  a droite?
            let is_leaf = match tree.arena[son_right].e {
                 Event::Leaf => true,
                 _           => false,
            };
            // if  !is_leaf {



                // let y = tree.arena[son_left].y;
                // let y = y  + PIPEBLOCK / 1.0 ;
                // tree.arena[son_left].set_y_noref(y);
                // let y = tree.arena[son_right].y;
                // let y = y  + PIPEBLOCK / 2.0 ;
                // tree.arena[son_right].set_y_noref(y);

                // Si on veut trsnamettre le decalage vertical!
                let ymod = tree.arena[son_left].ymod;
                let ymod = ymod  + PIPEBLOCK / 1.0 ;
                tree.arena[son_left].set_ymod_noref(ymod);
                let ymod = tree.arena[son_right].ymod;
                let ymod = ymod  + PIPEBLOCK / 2.0 ;
                tree.arena[son_right].set_ymod_noref(ymod);


            // }
        }
        shift_duplicated_and_loss( tree, son_left);
        shift_duplicated_and_loss( tree, son_right);
    }

    //     match event {
    //             Event::Duplication => { println!("Shifting suplicated")},
    //             _ => {},
    //     };
    // }

}
#[allow(dead_code)]
/// Traverse the tree using post-order traversal
pub fn  postorder(tree: &mut ArenaTree<String>){
    let root = tree.get_root();
    explore_postorder(tree,root);
}

#[allow(dead_code)]
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

/// Solve the conflicts between the left subtree and the right subtree
pub fn  check_vertical_contour_postorder(tree: &mut ArenaTree<String>,index:usize, ymod: f32) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        println!("check_vertical_contour_postorder: Father = {} (ymod = {} ) , Left = {}, Right = {}",tree.arena[index].name,tree.arena[index].ymod,tree.arena[left].name,tree.arena[right].name,);
        // push_down(tree,index, left,right,tree.arena[index].ymod);
                push_down(tree,index, left,right,0.0);
        check_vertical_contour_postorder(tree,left,tree.arena[left].ymod + 0.0 *  ymod);
        check_vertical_contour_postorder(tree,right,tree.arena[right].ymod + 0.0 * ymod);
    }
}

pub fn push_down (tree: &mut ArenaTree<String>, parent: usize, left: usize, right: usize, ymod: f32) {
    let node_parent_down_pos = node_ypos(tree,parent,ymod,1);
    let node_left_up_pos = node_ypos(tree,left,ymod,-1);
    let node_right_up_pos = node_ypos(tree,right,ymod,-1);
    if (node_left_up_pos <=  node_parent_down_pos) || (node_right_up_pos <=  node_parent_down_pos) {
        let shift_left = node_parent_down_pos - node_left_up_pos ;
        let shift_right = node_parent_down_pos - node_right_up_pos ;
        let mut shift_down = match shift_left > shift_right {
            true => shift_left,
            false => shift_right,
        };
        if shift_down < PIPEBLOCK {
            shift_down = PIPEBLOCK;

        }
        println!("CONFLIT AT SPEC NODE {}: parent y = {} ymod = {} down = {} left up = {} right up = {} => shift = {}",tree.arena[parent].name,tree.arena[parent].y,tree.arena[parent].ymod,node_parent_down_pos,node_left_up_pos,node_right_up_pos,shift_down);
        println!("SHIFTING Y {} + 1xPIPEBLOCK = {}",shift_down,shift_down + 1.0 * PIPEBLOCK);
        println!("Initial left : y = {}, ymod = {}",tree.arena[left].y,tree.arena[left].ymod);
        let y = tree.arena[left].y;
        // let y = y + shift_down + 1.0 * PIPEBLOCK;
            let y = y + shift_down ;
        tree.arena[left].set_y_noref(y);

        let ymod = tree.arena[left].ymod;
        // let ymod = ymod + shift_down + 1.0 * PIPEBLOCK;
                let ymod = ymod + shift_down;
        tree.arena[left].set_ymod_noref(ymod);
        println!("Final left : y = {}, ymod = {}",tree.arena[left].y,tree.arena[left].ymod);

        println!("Initial right : y = {}, ymod = {}",tree.arena[right].y,tree.arena[right].ymod);
        let y = tree.arena[right].y;
        // let y = y +shift_down + 1.0 * PIPEBLOCK;
                let y = y +shift_down ;
        tree.arena[right].set_y_noref(y);

        let ymod = tree.arena[right].ymod;
        // let ymod = ymod +  shift_down + 1.0 * PIPEBLOCK;
                let ymod = ymod +  shift_down ;
        tree.arena[right].set_ymod_noref(ymod);
        println!("Final right : y = {}, ymod = {}",tree.arena[right].y,tree.arena[right].ymod);

    }

}

/// Solve the conflicts between the left subtree and the right subtree
pub fn  check_contour_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        check_contour_postorder(tree,left);
        check_contour_postorder(tree,right);
        push_right(tree,left,right);
    }
    else{
    }
}
pub fn node_xpos(tree: &mut ArenaTree<String>, index: usize, xmod: f32, operator : i32) -> f32 {
    tree.arena[index].x + tree.arena[index].xmod + operator as f32 * tree.arena[index].nbg as f32 /2.0  *PIPEBLOCK + xmod
    // TODO quand la valeur ng est 0, ca peut etre un noeud  d'arbre de gene mais aussi un noeud
    // d'arbre d'espece sans noeud abre de gene associé et du coup la position extreme n'est pas exacte... Verfier su
    // il y a des conflits ( un noud d'arbre d'espexe sans gene a la meme epaissuer qu'un noeud d'arbre d'espece avec 1 gene)
}
pub fn node_ypos(tree: &mut ArenaTree<String>, index: usize, ymod: f32, operator : i32) -> f32 {
    println!("node_ypos: y ({}) + ymod ({}) +/ nbg ({})/2 x PIPEBLOCK",tree.arena[index].y, tree.arena[index].ymod, tree.arena[index].nbg );
    tree.arena[index].y + tree.arena[index].ymod + operator as f32 * tree.arena[index].nbg as f32 /2.0  *PIPEBLOCK
    // tree.arena[index].y + tree.arena[index].ymod + operator as f32 * tree.arena[index].nbg as f32 /2.0  *PIPEBLOCK + ymod
    // TODO quand la valeur ng est 0, ca peut etre un noeud  d'arbre de gene mais aussi un noeud
    // d'arbre d'espece sans noeud abre de gene associé et du coup la position extreme n'est pas exacte... Verfier su
    // il y a des conflits ( un noud d'arbre d'espexe sans gene a la meme epaissuer qu'un noeud d'arbre d'espece avec 1 gene)
}
/// Get the left 'contout' of a sub tree
pub fn  get_contour_left(tree: &mut ArenaTree<String>,index:usize,depth:usize,contour_left: &mut Vec<f32>,parent_xmod: f32)  {
    info!("get_contour_left: process node {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    let node_left_pos = node_xpos(tree,index,parent_xmod,-1);
    if contour_left.len() <= local_depth {
        if tree.arena[index].xmod < 0.0 {
            panic!("error: negative xmod");
        }
        contour_left.push(node_left_pos);
        info!("get_contour_left: increment contour is now {:?}",contour_left);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("erreur: negative  xmod");
    }
    if node_left_pos <= contour_left[local_depth] {
        contour_left[local_depth] = node_left_pos;
        info!("get_contour_left: contour is now {:?}",contour_left);
    }
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        get_contour_left(tree,left,depth,contour_left,tree.arena[index].xmod + parent_xmod );
    }
}

/// Get the right 'contout' of a sub tree
pub fn  get_contour_right(tree: &mut ArenaTree<String>,index:usize,depth:usize,contour_right: &mut Vec<f32>,parent_xmod: f32)  {
    info!("get_contour_right: process node {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    let node_right_pos = node_xpos(tree,index,parent_xmod,1);
    if contour_right.len() <= local_depth {
        if tree.arena[index].xmod < 0.0 {
            panic!("erreur: negative xmod");
        }
        contour_right.push(node_right_pos);
            info!("get_contour_right: increment contour is now {:?}",contour_right);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("erreur: negative xmod");
    }
    if node_right_pos >= contour_right[local_depth] {
        contour_right[local_depth] = node_right_pos ;
            info!("get_contour_right: contour is now {:?}",contour_right);
    }
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let right = children[1];
        get_contour_right(tree,right,depth,contour_right,tree.arena[index].xmod + parent_xmod );
    }
}

/// Check for conficts between subtrees and shift conflicting right-hand subtrees to the right
/// in order to solve detected  conflicts.
pub fn  push_right(tree: &mut ArenaTree<String>,left_tree:usize,right_tree:usize) -> f32 {
    info!("push_right: compare right contour of {} and left contour of {}",left_tree, right_tree);
    let mut right_co_of_left_tr  = vec![tree.arena[left_tree].x+tree.arena[left_tree].xmod + tree.arena[left_tree].nbg as f32 *PIPEBLOCK]; //contour droit de l'arbre de gauche
    let depth_left_tr  = tree.depth(left_tree);
    get_contour_right(tree,left_tree,depth_left_tr,&mut right_co_of_left_tr,0.0);
    info!("push_right: right contour of {} = {:?}",left_tree,right_co_of_left_tr);
    let mut left_co_of_right_tr  = vec![tree.arena[right_tree].x+tree.arena[right_tree].xmod - tree.arena[right_tree].nbg as f32 *PIPEBLOCK]; //contour droit de l'arbre de gauche
    let depth_right_tr  = tree.depth(right_tree);
    get_contour_left(tree,right_tree,depth_right_tr,&mut left_co_of_right_tr,0.0);
    info!("push_right: left contour of {} = {:?}",right_tree,left_co_of_right_tr);
    // Si on   a pas le meme longeur de contour on complete le plus petit
    // en remplissant ce qui manque avec la derniere valeur, pour eviter
    // qu'un sous arbre vosin se place sous une feuille
    let right_len = right_co_of_left_tr.len();
    let left_len = left_co_of_right_tr.len();
    if left_len > right_len {
        let last_val =  right_co_of_left_tr[right_len-1];
        let last_vals =  vec![last_val;left_len-right_len];
        right_co_of_left_tr.extend(last_vals.iter().copied());
        info!("push_right: complete right contour with last value {}", last_val);
    }
    if left_len < right_len {
        let last_val =  left_co_of_right_tr[left_len-1];
        let last_vals =  vec![last_val;right_len - left_len];
        left_co_of_right_tr.extend(last_vals.iter().copied());
        info!("push_right: complete left contour with last value {}", last_val);
    }
    info!("push_right: comparing  right cont. of left tree: {:?}",right_co_of_left_tr);
    info!("push_right: with left cont. of right tree:       {:?} ",left_co_of_right_tr);

    let iter = left_co_of_right_tr.iter().zip(right_co_of_left_tr).map(|(x, y )| (x-y));
    let shift = iter.min_by(|x, y| (*x as i64) .cmp(&(*y as i64 )));
    info!("push_right: distance max  = {:?}",shift);
    match shift {
        Some(val) => {
            info!("push_right: distance max  = {:?}",shift);
            if val <= 0.0 {// bidouilel
                info!("push_right: ================CONFLIT==========");
                info!("push_right: Modify node {:?}",tree.arena[right_tree]);
                let x_mod =  tree.arena[right_tree].xmod;
                info!("push_right: initial x_mod = {}",x_mod);
                let x_mod =  x_mod -1.0 *val + BLOCK ;//bidouille
                info!("push_right: new x_mod = {}",x_mod);
                tree.arena[right_tree].set_xmod_noref(x_mod);
                info!("push_right: updated node {:?}",tree.arena[right_tree]);
                info!("push_right: ================CONFLIT==========");
            }
        },
        None => {}
    }
    0.0
}

/// Set the x of the father between its chlidren
pub fn  set_middle_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        set_middle_postorder(tree,left);
        set_middle_postorder(tree,right);
        info!("set_middle_postorder: node {:?}",index);
        let x_left = tree.arena[left].x;
        let x_right = tree.arena[right].x;
        let x = tree.arena[index].x;
        let x_middle = ( x_right + x_left ) / 2.0 ;
        info!("set_middle_postorder: x father set from {} to {}",x,x_middle);
        tree.arena[index].set_x_noref(x_middle);
        let x_mod =  tree.arena[right].xmod;
        let x_mod =  x_mod + x_middle - x;
        tree.arena[index].set_xmod_noref(x_mod);

    }
}
