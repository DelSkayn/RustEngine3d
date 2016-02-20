pub trait Game{
    fn new() -> Self;

    fn update(&mut self){}
    fn render(&mut self){}
}
