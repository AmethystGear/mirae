pub mod playerupdate;
pub mod worldupdate;
use worldupdate::WorldUpdate;

pub enum Update {
    WorldUpdate(WorldUpdate),
}
