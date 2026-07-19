use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div};

impl Gallery {
    pub fn v7_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let _c = cx.theme().colors;
        Card::new()
                    .title("V7 P1 Inputs & Selection")
                    .description("PasswordInput, MaskedInput, PinInput, TimePicker, DateRangePicker, FilePicker, MultiSelect, RangeSlider, Rating, FormMessage, Autocomplete")
                    // PasswordInput
                    .child(Separator::new())
                    .child(div().child("PasswordInput (V7):"))
                    .child(PasswordInput::new("pwd-demo").placeholder("Enter password"))
                    // MaskedInput
                    .child(Separator::new())
                    .child(div().child("MaskedInput (V7):"))
                    .child(MaskedInput::new("mask-phone").mask("(XXX) XXX-XXXX").value("5551234567"))
                    // PinInput
                    .child(Separator::new())
                    .child(div().child("PinInput (V7):"))
                    .child(PinInput::new("pin-demo").digits(6).value("123"))
                    // TimePicker
                    .child(Separator::new())
                    .child(div().child("TimePicker (V7):"))
                    .child(TimePicker::new("time-demo").value("14:30"))
                    // DateRangePicker
                    .child(Separator::new())
                    .child(div().child("DateRangePicker (V7):"))
                    .child(DateRangePicker::new("range-demo").from("2024-01-01").to("2024-12-31"))
                    // FilePicker
                    .child(Separator::new())
                    .child(div().child("FilePicker (V7):"))
                    .child(FilePicker::new("file-demo").path("C:/project/src/main.rs"))
                    // MultiSelect
                    .child(Separator::new())
                    .child(div().child("MultiSelect (V7):"))
                    .child(MultiSelect::new("ms-demo").items(&["Rust", "Go", "Python", "TypeScript"]).selected_items(&["Rust", "Go"]))
                    // RangeSlider
                    .child(Separator::new())
                    .child(div().child("RangeSlider (V7):"))
                    .child(RangeSlider::new("range-slider").min(0.).max(100.).low(20.).high(80.))
                    // Rating
                    .child(Separator::new())
                    .child(div().child("Rating (V7):"))
                    .child(Rating::new("score").max(5).value(3))
                    // FormMessage
                    .child(Separator::new())
                    .child(div().child("FormMessage (V7):"))
                    .child(FormMessage::new("err-msg").tone(Tone::Danger).message("This field is required"))
                    .child(FormMessage::new("warn-msg").tone(Tone::Warning).message("Please review before submitting"))
                    .child(FormMessage::new("info-msg").tone(Tone::Primary).message("All changes saved"))
                    // Autocomplete
                    .child(Separator::new())
                    .child(div().child("Autocomplete (V7):"))
                    .child(Autocomplete::new("ac-demo").value("New").suggestions(&["New York", "London", "Tokyo", "Paris"]))
    }
}
