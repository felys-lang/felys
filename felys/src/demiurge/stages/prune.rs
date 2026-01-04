use crate::cyrene::Label;
use crate::demiurge::meta::Meta;
use crate::demiurge::Function;
use std::collections::HashSet;

impl Function {
    pub fn prune(&mut self, meta: &Meta) -> bool {
        let mut eliminated = HashSet::new();
        self.fragments.retain(|id, _| {
            let label = Label::Id(*id);
            let keep = meta.visited.contains(&label);
            if !keep {
                eliminated.insert(label);
            }
            keep
        });

        if eliminated.is_empty() {
            return false;
        }

        for (_, fragment) in self.dangerous() {
            fragment
                .predecessors
                .retain(|label| !eliminated.contains(label));
            fragment
                .phis
                .iter_mut()
                .for_each(|(_, inputs)| inputs.retain(|(label, _)| !eliminated.contains(label)));
            fragment.phis.retain(|(_, inputs)| !inputs.is_empty());
        }
        true
    }
}
