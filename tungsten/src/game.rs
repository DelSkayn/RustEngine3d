pub trait Game: Sized{
    fn new() -> Self;

    fn update(&mut self){}
    fn render(&mut self){}
}
