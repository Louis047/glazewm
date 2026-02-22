use wm_common::Direction;

pub enum WorkspaceTarget {
  Name(String),
  Recent,
  NextActive,
  PreviousActive,
  NextActiveInMonitor,
  PreviousActiveInMonitor,
  Next,
  Previous,
  NextPopulated,
  PreviousPopulated,
  #[allow(dead_code)]
  Direction(Direction),
}
