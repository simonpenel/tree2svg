use std::fs::File;
use std::env;
use std::process;
use getopt::Opt;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
mod arena;
use crate::arena::ArenaTree;
use crate::arena::taxo2tree;
use crate::arena::knuth_layout;
use crate::arena::set_middle_postorder;
use crate::arena::shift_mod_x;
use crate::arena::check_contour_postorder;
use crate::arena::cladogramme;
mod drawing;
use log::{info};


// Message d'erreur
// ----------------
fn display_help(programe_name:String) {
    println!("Usage:");
    println!("{} -f input file [-o output file][-v]",programe_name);
    println!("{} -h ",programe_name);
    process::exit(1);
}

fn main()  {
    // Gestion des arguments et des options
    // ------------------------------------
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
         display_help(args[0].to_string());
    }
    let mut opts = getopt::Parser::new(&args, "f:o:hv");
    let mut a_flag = false;
    let mut infile = String::new();
    let mut outfile = String::from("tree2svg.svg");
    loop {
        match opts.next().transpose().expect("Unlnown option") {
            None => break,
            Some(opt) => match opt {
                Opt('v', None) => {
                    env::set_var("RUST_LOG", "info");
                    env_logger::init();
                    info!("Verbosity set to Info");
                    },
                Opt('f', Some(string)) => infile = string.clone(),
                Opt('o', Some(string)) => outfile = string.clone(),
                Opt('h', None) => display_help(args[0].to_string()),
                _ => unreachable!(),
            }
        }
    }

    // Lecture du fichier au format newick
    // ------------------------------------
    let filename = &infile.clone();
    info!("Filename is {}",filename);
    let  f = File::open(filename);
    let  mut file = match f {
            Ok(file) => {
                info!("File exists");
                file},
            Err(error) => {
                info!("Unable to read {}",filename);
                eprintln!("Error: {}", error);
                process::exit(1);
            }
    };
    // Stocke l'arbre dans une structure Taxonomy
    let taxo = newick::load_newick(&mut file);
    let taxo = match taxo {
        Ok(taxo) => {
            info!("File is ok");
            taxo},
        Err(error) => {
                panic!("Probleme lors de la lecture du fichier : {:?}", error);
            }
    };
    // Stocke l'arbre dans une structure ArenaTree
    let racine: &str = taxo.root();
    let racine_tid = taxo.to_internal_id(racine).expect("Pas de racine");
    let children = taxo.children(racine_tid).expect("Pas de fils");
    let mut tree: ArenaTree<String> = ArenaTree::default();
    for child in children {
        taxo2tree(& taxo, child,  &mut tree);
    }

    // Calcul des coordonees x y
    // =========================

    // 1ere etape : profondeur => Y, left => X= 0, right X=1
    // ======================================================
    let  root = tree.get_root();
    knuth_layout(&mut tree,root, &mut 1);
    drawing::draw_tree(&mut tree,"knuth.svg".to_string());

    // Cladogramme
    // ===========
    cladogramme(&mut tree);

    // Veifie les contours
    // ===================
     check_contour_postorder(&mut tree, root);

    // Decale toutes les valeurs de x en finction de xmod
    // ===================================================
    shift_mod_x(&mut tree, root, &mut 0.0);
    drawing::draw_tree(&mut tree,"shifted.svg".to_string());

    // Place le parent entre les enfants
    // =================================
    set_middle_postorder(&mut tree, root);

    // tree.rotate();
    drawing::draw_tree(&mut tree,outfile);

}
