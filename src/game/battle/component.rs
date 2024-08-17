use std::collections::VecDeque;

use crate::model::sub::DropEquipment;

use super::*;

#[derive(Component, Reflect)]
pub struct Combat {
    pub monster_entity: Entity,
    pub records: Vec<CombatRecord>,
    pub monster_hp_remain: f32,
    pub player_hp_remain: f32,
}

impl Combat {
    pub fn push_update(&mut self, record: CombatRecord) {
        if record.is_player_turn {
            self.monster_hp_remain -= record.damage_out.get();
        } else {
            self.player_hp_remain -= record.damage_out.get();
        }

        self.records.push(record);
    }
}

#[derive(Component)]

pub struct ExitFlag;

// When all flags are ready, it will be update the CombatRecord
#[derive(Component, Reflect, Clone)]
pub struct TurnFlag {
    pub record: Option<CombatRecord>,
    pub is_terminal_ready: bool,
    pub is_animation_ready: bool,
}

impl Default for TurnFlag {
    fn default() -> Self {
        Self {
            record: None,
            is_terminal_ready: true,
            is_animation_ready: true,
        }
    }
}

impl TurnFlag {
    pub fn reset(&mut self) {
        self.record = None;
        self.is_terminal_ready = false;
        self.is_animation_ready = false;
    }

    pub fn is_all_ready(&self) -> bool {
        // info!(
        //     "terminal ready: {}, animation: {}, record: {:?}",
        //     self.is_terminal_ready,
        //     self.is_animation_ready,
        //     self.record.is_some()
        // );
        self.is_terminal_ready && self.is_animation_ready && self.record.is_some()
    }
}

#[derive(Component, PartialEq, Eq, Reflect)]
pub struct TalkFlag {
    pub is_ended: bool,
    pub is_terminal_ready: bool,
    // There two situation would cause InProgress end,
    // If user click anywhere the terminal will finished the current query and start printing the next one
    // another situation: all the animation and terminal has been finished playing naturally

    // battle starts off by printing out a prompt for user input
    // then the state will be set to InProgress,
    // To do : animation handling
    // terminal will be finished playing the current queue and send out signal to wait for the next turn.
    // eventually, I will use animation as the signal but for now I will use terminal printing speed as the guide
}

impl Default for TalkFlag {
    fn default() -> Self {
        Self {
            is_ended: false,
            is_terminal_ready: true,
        }
    }
}

impl TalkFlag {
    pub fn ready(&mut self) {
        self.is_terminal_ready = true;
    }

    pub fn reset(&mut self) {
        self.is_terminal_ready = false;
    }

    pub fn is_all_finished(&self) -> bool {
        self.is_ended && self.is_terminal_ready
    }
}

#[derive(Component, Default, Debug, Reflect)]
pub struct TrashTalk {
    pub talk: VecDeque<String>,
}

impl TrashTalk {
    pub fn pop(&mut self) -> Option<String> {
        self.talk.pop_front()
    }

    pub fn demo() -> Self {
        Self {
            talk: VecDeque::from([
                "You Encountered a Gravy Bear!".to_string(),
                "Battle Started, Touch to Fight!".to_string(),
            ]),
        }
    }

    pub fn push(&mut self, s: String) {
        self.talk.push_back(s);
    }
}

#[derive(Component, Default, Debug, Reflect)]
pub struct DropPopupFlag {
    pub drop: DropEquipment,
    pub counter: i8,
}

impl DropPopupFlag {
    pub fn new(counter: i8, drop: &DropEquipment) -> Self {
        Self {
            counter,
            drop: drop.clone(),
        }
    }

    pub fn count_down(&mut self) {
        if self.counter >= 0 {
            self.counter -= 1;
        } else {
            self.counter = -1;
        }
    }

    pub fn hit(&self) -> bool {
        self.counter == 0
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct CombatRecord {
    pub is_player_turn: bool,
    pub player: Entity,
    pub monster: Entity,
    pub player_name: Name,
    pub monster_name: Name,
    pub damage_out: DamageOut,
}

impl CombatRecord {
    pub fn to_string(&self) -> String {
        use rand::seq::SliceRandom;
        let (attacker, defender) = if self.is_player_turn {
            (&self.player_name, &self.monster_name)
        } else {
            (&self.monster_name, &self.player_name)
        };

        match self.damage_out {
            DamageOut::NormalHit(_) => {
                let attack_phrase: [String; 5] = [
                    format!(
                        "{0} hits {1} with a sword",
                        attacker.as_str(),
                        defender.as_str()
                    ),
                    format!(
                        "{0} slashed {1} in the chest",
                        attacker.as_str(),
                        defender.as_str()
                    ),
                    format!("{0} smiles at {1}", attacker.as_str(), defender.as_str()),
                    format!(
                        "{0} strikes at {1} from below",
                        attacker.as_str(),
                        defender.as_str()
                    ),
                    format!(
                        "{1} got stabbed by {0} from behind",
                        attacker.as_str(),
                        defender.as_str()
                    ),
                ];

                let phrase = attack_phrase
                    .choose(&mut rand::thread_rng())
                    .expect("random picking has bug");

                format!(
                    "{phrase}\ndealt {0:.1} damage to {1}",
                    self.damage_out.get(),
                    defender.as_str(),
                )
            }
            DamageOut::CriticalHit(_) => {
                let attack_phrase: [String; 3] = [
                    format!(
                        "Critical. Thwacking! {1} face is crushed by {0}",
                        attacker.as_str(),
                        defender.as_str()
                    ),
                    format!(
                        "Critical. {0} lashed out a flesh of light,  {1} got blew a way!",
                        attacker.as_str(),
                        defender.as_str()
                    ),
                    format!(
                        "Critical. {0}'s left hand clouted {1}. Sending {1} crashing to the ground.",
                        attacker.as_str(),
                        defender.as_str()
                    ),
                ];
                let phrase = attack_phrase
                    .choose(&mut rand::thread_rng())
                    .expect("random picking has bug");

                format!(
                    "{phrase}\ndealt {0:.1} CRITICAL damage to {1}\nWhat a move",
                    self.damage_out.get(),
                    defender.as_str(),
                )
            }
        }
    }
}
