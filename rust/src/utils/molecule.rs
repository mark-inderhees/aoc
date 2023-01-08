use strum_macros::Display;

#[derive(Debug, Clone, Hash, std::cmp::Eq, PartialEq)]
pub struct Molecule {
    atoms: Vec<Atom>,
}

impl Molecule {
    pub fn new() -> Molecule {
        Molecule { atoms: vec![] }
    }

    pub fn new_from_string(input: &str) -> Molecule {
        let mut output = Molecule::new();
        let mut abc = ['?'; 26];
        for (i, char) in abc.iter_mut().enumerate() {
            *char = (i as u8 + 'A' as u8) as char;
        }

        let mut atom = String::new();
        for char in input.chars() {
            if abc.contains(&char) {
                // This is the start of a new string
                if atom.chars().count() > 0 {
                    output.push_atom(Atom::new(&atom));
                }
                atom = char.to_string();
            } else {
                atom.push(char);
            }
        }
        output.push_atom(Atom::new(&atom));

        output
    }

    pub fn len(&self) -> usize {
        self.atoms.len()
    }

    pub fn atoms(&self) -> Vec<Atom> {
        self.atoms.clone()
    }

    pub fn push_atom(&mut self, atom: Atom) {
        self.atoms.push(atom);
    }

    pub fn starts_with(&self, target: &Molecule) -> bool {
        if target.len() > self.len() {
            return false;
        }

        for (i, atom) in target.atoms().iter().enumerate() {
            if *atom != self.atoms[i] {
                return false;
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        self.atoms
            .iter()
            .fold(String::new(), |a, x| a + &x.to_string())
    }

    /// For every from atom found in this module, change to the to partern.
    /// Only change one atom at a time. If multiple from are present, then
    /// multiple Molecules will be in the output.
    pub fn replace(&self, from: &Atom, to: &Molecule) -> Vec<Molecule> {
        let mut output = vec![];

        for (i, atom) in self.atoms.iter().enumerate() {
            if atom == from {
                let mut atoms = self.atoms.clone();
                atoms.remove(i);
                for (j, replace) in to.atoms().iter().enumerate() {
                    atoms.insert(i + j, replace.clone());
                }
                output.push(Molecule { atoms });
            }
        }

        output
    }
}

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
