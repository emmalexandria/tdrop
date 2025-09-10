pub mod grapheme;
pub mod line;
pub mod span;
pub mod text;

// use std::fmt::Display;
//
// use crossterm::style::{ContentStyle, StyledContent};
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct SectionStyle {
//     start: usize,
//     end: usize,
//     style: ContentStyle,
// }
//
// impl SectionStyle {
//     fn new(start: usize, end: usize, style: ContentStyle) -> Self {
//         Self { start, end, style }
//     }
//
//     fn update(&mut self, new_chars: usize, index: usize) {
//         if index <= self.start {
//             self.start += new_chars;
//             self.end += new_chars
//         }
//     }
//
//     fn contains(&self, idx: usize) -> bool {
//         return idx >= self.start && idx <= self.end;
//     }
//
//     fn rebase(&mut self, idx: usize) {
//         let dist = self.start - idx;
//         let len = self.end - self.start;
//         self.start = dist;
//         self.end = self.start + len;
//     }
//
//     fn split(&self, idx: usize) -> (SectionStyle, SectionStyle) {
//         let mut first = *self;
//         let diff = first.end - idx;
//         first.end = idx - 1;
//         let second = SectionStyle::new(0, diff, first.style);
//         (first, second)
//     }
// }
//
// impl PartialOrd for SectionStyle {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.start.cmp(&other.start))
//     }
// }
//
// impl Ord for SectionStyle {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.start.cmp(&other.start)
//     }
// }
//
// pub trait TextInput {
//     fn content(&self) -> String;
//     fn style(&self) -> Option<ContentStyle>;
// }
//
// impl<D: Display> TextInput for StyledContent<D> {
//     fn content(&self) -> String {
//         self.content().to_string()
//     }
//
//     fn style(&self) -> Option<ContentStyle> {
//         Some(*self.style())
//     }
// }
//
// impl TextInput for String {
//     fn content(&self) -> String {
//         self.to_string()
//     }
//
//     fn style(&self) -> Option<ContentStyle> {
//         None
//     }
// }
//
// impl TextInput for &String {
//     fn content(&self) -> String {
//         self.to_string()
//     }
//
//     fn style(&self) -> Option<ContentStyle> {
//         None
//     }
// }
//
// impl<'a> TextInput for &'a str {
//     fn content(&self) -> String {
//         self.to_string()
//     }
//
//     fn style(&self) -> Option<ContentStyle> {
//         None
//     }
// }
//
// impl<S: ToString> TextInput for (S, ContentStyle) {
//     fn content(&self) -> String {
//         self.0.to_string()
//     }
//
//     fn style(&self) -> Option<ContentStyle> {
//         Some(self.1)
//     }
// }
//
// /** Text is a struct for containing styled text, allowing for easy editing of rich text */
// #[derive(Debug, Clone)]
// pub struct Text {
//     content: String,
//     style: ContentStyle,
//     sec_styles: Vec<SectionStyle>,
// }
//
// impl Default for Text {
//     fn default() -> Self {
//         Self {
//             content: String::new(),
//             style: ContentStyle::default(),
//             sec_styles: vec![],
//         }
//     }
// }
//
// impl<I: TextInput> From<I> for Text {
//     fn from(value: I) -> Self {
//         Self {
//             content: value.content(),
//             style: value.style().unwrap_or_default(),
//             sec_styles: vec![],
//         }
//     }
// }
//
// impl Display for Text {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.display())
//     }
// }
//
// impl Text {
//     pub fn new<S: ToString>(content: S) -> Self {
//         Self {
//             content: content.to_string(),
//             style: ContentStyle::default(),
//             sec_styles: vec![],
//         }
//     }
//
//     pub fn len(&self) -> usize {
//         return self.content.len();
//     }
//
//     pub fn style(mut self, style: &ContentStyle) -> Self {
//         self.style = *style;
//         self
//     }
//
//     pub fn section<I: TextInput>(mut self, text: I) -> Self {
//         self.push(text);
//         self
//     }
//
//     pub fn push<I: TextInput>(&mut self, text: I) {
//         self.insert(self.content.len(), text);
//     }
//
//     pub fn get_content(&self) -> String {
//         self.content.clone()
//     }
//
//     pub fn insert<I: TextInput>(&mut self, idx: usize, text: I) {
//         let style = match text.style() {
//             Some(s) => s,
//             None => self.style,
//         };
//
//         self.insert_str(idx, text.content(), style);
//     }
//
//     pub fn split(&mut self, idx: usize, retain_style: bool) -> Self {
//         let (first, second) = self.content.split_at(idx);
//         let containing_section = self.sec_styles.iter().find(|s| s.contains(idx)).copied();
//
//         let mut new_text = match retain_style {
//             true => Text::new(second).style(&self.style),
//             false => Text::new(second),
//         };
//
//         self.content = first.to_string();
//
//         let mut applicable: Vec<SectionStyle> = self
//             .sec_styles
//             .iter()
//             .filter(|s| s.start > idx)
//             .copied()
//             .collect();
//
//         self.sec_styles = self
//             .sec_styles
//             .iter()
//             .filter(|s| s.start <= idx)
//             .copied()
//             .collect();
//
//         if let Some(cs) = containing_section {
//             let (first, second) = cs.split(idx);
//             let cs_index = self.sec_styles.iter().position(|s| *s == cs);
//             if let Some(i) = cs_index {
//                 self.sec_styles.remove(i);
//             }
//             self.sec_styles.push(first);
//             new_text.sec_styles.push(second);
//         }
//
//         applicable.iter_mut().for_each(|s| {
//             s.rebase(idx);
//             new_text.sec_styles.push(*s)
//         });
//
//         return new_text;
//     }
//
//     pub fn append(&mut self, other: &Text) {
//         let content = self.content.push_str(&other.content);
//         let idx = self.content.len();
//
//         let other_secs = other.sec_styles.clone();
//
//         // Ugly if statement, but its better than nesting
//         if  let Some(last) = self.sec_styles.last() && let Some(second_last) = other_secs.last() && second_last.start == 0 && last.end == idx && second_last.style == last.style  {
//
//         }
//     }
//
//     fn insert_str<S: ToString>(&mut self, idx: usize, content: S, style: ContentStyle) {
//         let s = content.to_string();
//         if s.len() == 0 {
//             return;
//         }
//
//         let end = idx + s.len() - 1;
//         self.content.insert_str(idx, &s);
//
//         self.update_section_styles(s.len(), idx);
//         if style != self.style {
//             self.sec_styles.push(SectionStyle::new(idx, end, style));
//         }
//
//         self.order_section_styles();
//     }
//
//     fn display(&self) -> String {
//         let styled = self.apply_styles();
//         styled
//     }
//
//     fn apply_styles(&self) -> String {
//         let mut idx = 0;
//         let mut sty_idx = 0;
//         let mut strs: Vec<String> = vec![];
//
//         if self.sec_styles.len() == 0 {
//             return self.style.apply(&self.content).to_string();
//         }
//
//         while idx < self.content.len() {
//             if sty_idx >= self.sec_styles.len() {
//                 let sub = &self.content[idx..];
//                 strs.push(self.style.apply(sub).to_string());
//                 break;
//             }
//
//             let sty = &self.sec_styles[sty_idx];
//             if idx == sty.start {
//                 let sub = &self.content[sty.start..=sty.end];
//                 idx += sty.end - sty.start + 1;
//                 strs.push(sty.style.apply(sub).to_string());
//
//                 sty_idx += 1;
//             } else {
//                 let sub = &self.content[idx..sty.start];
//                 idx += sub.len();
//                 strs.push(self.style.apply(sub).to_string());
//             }
//         }
//
//         return strs.join("");
//     }
//
//     fn order_section_styles(&mut self) {
//         self.sec_styles.sort();
//     }
//
//     fn update_section_styles(&mut self, new_chars: usize, index: usize) {
//         self.sec_styles
//             .iter_mut()
//             .for_each(|s| s.update(new_chars, index));
//     }
// }
//
// #[cfg(test)]
// mod text_tests {
//     use crossterm::style::Stylize;
//
//     use crate::{
//         text::{SectionStyle, Text},
//         style::Style
//     };
//
//     #[test]
//     fn test_split_section() {
//         let style = ContentStyle::new().red();
//         let sec = SectionStyle::new(3, 10, ContentStyle::new().red());
//         let (first, second) = sec.split(5);
//
//         assert!(first == SectionStyle::new(3, 4, style));
//         assert!(second == SectionStyle::new(0, 5, style));
//     }
//
//     #[test]
//     fn test_rebase_section() {
//         let mut section = SectionStyle::new(10, 18, ContentStyle::new());
//
//         section.rebase(5);
//         assert_eq!(section.start, 5);
//         assert_eq!(section.end, 13);
//     }
//
//     #[test]
//     fn test_split_text_basic() {
//         let mut text = Text::new("hello")
//             .section("stuff".red())
//             .section("more stuff");
//         let second = text.split(9, true);
//
//         assert_eq!(text.get_content(), "hellostuf");
//         assert_eq!(second.get_content(), "fmore stuff");
//     }
// }
