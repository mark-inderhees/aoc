/// A molecule is made up of multiple atoms. This molecule struct and the Atom
/// struct support to and from string conversions. The molecule struct also
/// supports building the molecule and makeing "random like" replacements to
/// atoms inside the molecule.
#[derive(Debug, Clone, Hash, std::cmp::Eq, PartialEq)]
pub struct Molecule {
    atoms: Vec<Atom>,
}

// Used for doing emun -> string
use strum_macros::Display;

impl Molecule {
    /// Create an empty Molecule.
    pub fn new() -> Molecule {
        Molecule { atoms: vec![] }
    }

    /// Build a molecule from a string of atoms.
    pub fn new_from_string(input: &str) -> Molecule {
        let mut molecule = Molecule::new();

        // Build a slice of all capital letters, this is used to build atoms,
        // as each new atom starts with a capital letter.
        let mut capitals = ['?'; 26];
        for (i, char) in capitals.iter_mut().enumerate() {
            *char = (i as u8 + 'A' as u8) as char;
        }

        // Parse atoms out of the string and add them to the molecule
        let mut atom = String::new();
        for char in input.chars() {
            if capitals.contains(&char) {
                // This is the start of a new atom

                // Store the finished atom
                if atom.chars().count() > 0 {
                    molecule.push_atom(Atom::new(&atom));
                }

                // Start a new atom
                atom = char.to_string();
            } else {
                // Keep building the string for the current atom
                atom.push(char);
            }
        }
        // Store the last atom of the molecule
        molecule.push_atom(Atom::new(&atom));

        molecule
    }

    /// The number of atoms in the molecule.
    pub fn len(&self) -> usize {
        self.atoms.len()
    }

    /// The list of atoms currently in the molecule.
    pub fn atoms(&self) -> Vec<Atom> {
        self.atoms.clone()
    }

    /// Add a new atom to the molecule.
    pub fn push_atom(&mut self, atom: Atom) {
        self.atoms.push(atom);
    }

    /// Check if this molecule starts with the target molecule.
    #[allow(dead_code)]
    pub fn starts_with(&self, target: &Molecule) -> bool {
        // If the target is longer than this molecule, it cannot match
        if target.len() > self.len() {
            return false;
        }

        // Loop throught each atom at the start and ensure matches
        for (i, atom) in target.atoms().iter().enumerate() {
            if *atom != self.atoms[i] {
                return false;
            }
        }

        // The target molecule is at the start of this molecule
        true
    }

    /// Convert this molecule to a string.
    pub fn to_string(&self) -> String {
        self.atoms
            .iter()
            .fold(String::new(), |a, x| a + &x.to_string())
    }

    /// For every from atom found in this module, change to the to pattern.
    /// Only change one atom at a time. If multiple from are present, then
    /// multiple Molecules will be in the output.
    pub fn replace(&self, from: &Atom, to: &Molecule) -> Vec<Molecule> {
        let mut molecules = vec![];

        // Check each atom in this molecule
        for (i, atom) in self.atoms.iter().enumerate() {
            if atom == from {
                // This atom matches, do the replacement
                let mut atoms = self.atoms.clone();

                // Remove the old atom
                atoms.remove(i);

                // Add in each of the replacement atoms
                for (j, replace) in to.atoms().iter().enumerate() {
                    atoms.insert(i + j, replace.clone());
                }

                molecules.push(Molecule { atoms });
            }
        }

        molecules
    }
}

/// An enum of all the known Atoms. Supports to and from string conversion.
#[derive(Debug, Clone, PartialEq, Hash, std::cmp::Eq, Display)]
pub enum Atom {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Uut,
    Fl,
    Uup,
    Lv,
    Uus,
    Uuo,
}

impl Atom {
    pub fn new(input: &str) -> Atom {
        match input {
            "H" => Atom::H,
            "He" => Atom::He,
            "Li" => Atom::Li,
            "Be" => Atom::Be,
            "B" => Atom::B,
            "C" => Atom::C,
            "N" => Atom::N,
            "O" => Atom::O,
            "F" => Atom::F,
            "Ne" => Atom::Ne,
            "Na" => Atom::Na,
            "Mg" => Atom::Mg,
            "Al" => Atom::Al,
            "Si" => Atom::Si,
            "P" => Atom::P,
            "S" => Atom::S,
            "Cl" => Atom::Cl,
            "Ar" => Atom::Ar,
            "K" => Atom::K,
            "Ca" => Atom::Ca,
            "Sc" => Atom::Sc,
            "Ti" => Atom::Ti,
            "V" => Atom::V,
            "Cr" => Atom::Cr,
            "Mn" => Atom::Mn,
            "Fe" => Atom::Fe,
            "Co" => Atom::Co,
            "Ni" => Atom::Ni,
            "Cu" => Atom::Cu,
            "Zn" => Atom::Zn,
            "Ga" => Atom::Ga,
            "Ge" => Atom::Ge,
            "As" => Atom::As,
            "Se" => Atom::Se,
            "Br" => Atom::Br,
            "Kr" => Atom::Kr,
            "Rb" => Atom::Rb,
            "Sr" => Atom::Sr,
            "Y" => Atom::Y,
            "Zr" => Atom::Zr,
            "Nb" => Atom::Nb,
            "Mo" => Atom::Mo,
            "Tc" => Atom::Tc,
            "Ru" => Atom::Ru,
            "Rh" => Atom::Rh,
            "Pd" => Atom::Pd,
            "Ag" => Atom::Ag,
            "Cd" => Atom::Cd,
            "In" => Atom::In,
            "Sn" => Atom::Sn,
            "Sb" => Atom::Sb,
            "Te" => Atom::Te,
            "I" => Atom::I,
            "Xe" => Atom::Xe,
            "Cs" => Atom::Cs,
            "Ba" => Atom::Ba,
            "La" => Atom::La,
            "Ce" => Atom::Ce,
            "Pr" => Atom::Pr,
            "Nd" => Atom::Nd,
            "Pm" => Atom::Pm,
            "Sm" => Atom::Sm,
            "Eu" => Atom::Eu,
            "Gd" => Atom::Gd,
            "Tb" => Atom::Tb,
            "Dy" => Atom::Dy,
            "Ho" => Atom::Ho,
            "Er" => Atom::Er,
            "Tm" => Atom::Tm,
            "Yb" => Atom::Yb,
            "Lu" => Atom::Lu,
            "Hf" => Atom::Hf,
            "Ta" => Atom::Ta,
            "W" => Atom::W,
            "Re" => Atom::Re,
            "Os" => Atom::Os,
            "Ir" => Atom::Ir,
            "Pt" => Atom::Pt,
            "Au" => Atom::Au,
            "Hg" => Atom::Hg,
            "Tl" => Atom::Tl,
            "Pb" => Atom::Pb,
            "Bi" => Atom::Bi,
            "Po" => Atom::Po,
            "At" => Atom::At,
            "Rn" => Atom::Rn,
            "Fr" => Atom::Fr,
            "Ra" => Atom::Ra,
            "Ac" => Atom::Ac,
            "Th" => Atom::Th,
            "Pa" => Atom::Pa,
            "U" => Atom::U,
            "Np" => Atom::Np,
            "Pu" => Atom::Pu,
            "Am" => Atom::Am,
            "Cm" => Atom::Cm,
            "Bk" => Atom::Bk,
            "Cf" => Atom::Cf,
            "Es" => Atom::Es,
            "Fm" => Atom::Fm,
            "Md" => Atom::Md,
            "No" => Atom::No,
            "Lr" => Atom::Lr,
            "Rf" => Atom::Rf,
            "Db" => Atom::Db,
            "Sg" => Atom::Sg,
            "Bh" => Atom::Bh,
            "Hs" => Atom::Hs,
            "Mt" => Atom::Mt,
            "Ds" => Atom::Ds,
            "Rg" => Atom::Rg,
            "Cn" => Atom::Cn,
            "Uut" => Atom::Uut,
            "Fl" => Atom::Fl,
            "Uup" => Atom::Uup,
            "Lv" => Atom::Lv,
            "Uus" => Atom::Uus,
            "Uuo" => Atom::Uuo,
            _ => panic!("Unexpected Atom {input}"),
        }
    }
}
