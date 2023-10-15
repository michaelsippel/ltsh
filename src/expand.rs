use crate::ast::*;

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\

impl Word {
    pub fn (&mut self, env: &Environment) {
        for x in self.0.iter_mut() {
            x.expand_tilde();
            match x {
                Word::Tilde => Word::Literal( env.get_home() ),
                other => other,
            }
        }
    }

    pub fn expand(&self) -> Vec<String> {
        let mut fields = Vec::new();

        for seg in self.segments.iter() {
            //
        }

        fields
    }

    pub fn split_field(&mut self) {
        
    }
}

impl WordSegment {
    pub fn split_field(&self) -> Word {
        
        match self {
            
        }
    }

    pub fn expand(&self) -> Word {
        match 
    }
}

//<<<<>>>><<>><><<>><<<*>>><<>><><<>><<<<>>>>\\
