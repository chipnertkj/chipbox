//! # Attribution
//! Adapted from [n3r4zzurr0/svg-spinners](https://github.com/n3r4zzurr0/svg-spinners).
//! This module uses a modified version of the [90-ring-with-bg.svg](https://github.com/n3r4zzurr0/svg-spinners/blob/main/svg-css/90-ring-with-bg.svg) file.
//! `90-ring-with-bg.svg` is licensed under the **MIT License** (see below).
//!
//! # Original source license
//! Below is the license applicable to the original file, copied from [here](https://github.com/n3r4zzurr0/svg-spinners/blob/main/LICENSE).
//!
//! **BEGIN LICENSE TEXT**
//!
//! The MIT License (MIT)
//!
//! Copyright (c) Utkarsh Verma
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of
//! this software and associated documentation files (the "Software"), to deal in
//! the Software without restriction, including without limitation the rights to
//! use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
//! the Software, and to permit persons to whom the Software is furnished to do so,
//! subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
//! FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
//! COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
//! IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
//! CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//!
//! **END LICENSE TEXT**

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct SpinnerProps {
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub class: AttrValue,
}

#[function_component]
pub(crate) fn Spinner(props: &SpinnerProps) -> Html {
    let SpinnerProps { style, class } = props;
    html! {
        <svg style={format!("{style}")} class={format!("spinner {class}")} viewBox={"0 0 24 24"} xmlns={"http://www.w3.org/2000/svg"}>
            <path class="spinner-bg" d="M12,1A11,11,0,1,0,23,12,11,11,0,0,0,12,1Zm0,19a8,8,0,1,1,8-8A8,8,0,0,1,12,20Z"/>
            <path class="spinner-arc" d="M10.14,1.16a11,11,0,0,0-9,8.92A1.59,1.59,0,0,0,2.46,12,1.52,1.52,0,0,0,4.11,10.7a8,8,0,0,1,6.66-6.61A1.42,1.42,0,0,0,12,2.69h0A1.57,1.57,0,0,0,10.14,1.16Z"/>
        </svg>
    }
}
