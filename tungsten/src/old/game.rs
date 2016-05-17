///
/// Trait all games must implement in order to run in tungsten.
pub trait Game{
    ///
    ///Function which is called each time logic needs to be updated.
    ///
    fn update(&mut self){}

    ///
    ///Function which is called each time something needs to be rendered.
    ///
    fn render(&mut self){}
}
