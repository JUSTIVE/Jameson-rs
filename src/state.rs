pub mod state{
  pub enum ReduceState{
    Init,
    NonInit('a)
  }
}