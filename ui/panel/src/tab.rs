pub trait Tab<T>
where
    T: PartialEq + yew::ToHtml + std::fmt::Display + 'static,
    Self: PartialEq + yew::ToHtml + std::fmt::Display + 'static,
{
    const TABS: &'static [T];
}
