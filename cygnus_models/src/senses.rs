#[derive(Clone, Debug, Default)]
pub struct Builder {
    blindsight: Option<usize>,
    darkvision: Option<usize>,
    tremorsense: Option<usize>,
    truesight: Option<usize>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn blindsight(mut self, distance: usize) -> Self {
        let _ = self.blindsight.insert(distance);

        self
    }

    pub fn darkvision(mut self, distance: usize) -> Self {
        let _ = self.darkvision.insert(distance);

        self
    }

    pub fn tremorsense(mut self, distance: usize) -> Self {
        let _ = self.tremorsense.insert(distance);

        self
    }

    pub fn truesight(mut self, distance: usize) -> Self {
        let _ = self.truesight.insert(distance);

        self
    }

    pub fn build(self) -> Senses {
        Senses {
            blindsight: self.blindsight,
            darkvision: self.darkvision,
            tremorsense: self.tremorsense,
            truesight: self.truesight,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Senses {
    blindsight: Option<usize>,
    darkvision: Option<usize>,
    tremorsense: Option<usize>,
    truesight: Option<usize>,
}

impl Senses {
    pub fn get_blindsight(&self) -> Option<usize> {
        self.blindsight
    }

    pub fn get_darkvision(&self) -> Option<usize> {
        self.darkvision
    }

    pub fn get_tremorsense(&self) -> Option<usize> {
        self.tremorsense
    }

    pub fn get_truesight(&self) -> Option<usize> {
        self.truesight
    }

    pub fn get_passive_perception(&self, perception_modifier: isize) -> usize {
        (10 + perception_modifier) as usize
    }

    pub fn get_passive_investigation(&self, investigation_modifier: isize) -> usize {
        (10 + investigation_modifier) as usize
    }

    pub fn get_passive_insight(&self, insight_modifier: isize) -> usize {
        (10 + insight_modifier) as usize
    }
}
