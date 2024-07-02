use std::str::FromStr;

use smallvec::SmallVec;

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum EventName {
    Click,
    MiddleClick,
    RightClick,

    MouseDown,
    MouseMove,
    MouseEnter,
    MouseLeave,
    MouseOver,
    MouseOut,

    Wheel,

    PointerMove,
    PointerDown,
    PointerEnter,
    PointerLeave,
    PointerUp,

    KeyDown,
    KeyUp,

    TouchCancel,
    TouchStart,
    TouchMove,
    TouchEnd,

    GlobalClick,
    GlobalPointerUp,
    GlobalMouseDown,
    GlobalMouseMove,
    GlobalFileHover,
    GlobalFileHoverCancelled,

    FileDrop,
}

impl FromStr for EventName {
    type Err = ();

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        match txt {
            "click" => Ok(EventName::Click),
            "rightclick" => Ok(EventName::RightClick),
            "middleclick" => Ok(EventName::MiddleClick),
            "mousedown" => Ok(EventName::MouseDown),
            "mousemove" => Ok(EventName::MouseMove),
            "mouseenter" => Ok(EventName::MouseEnter),
            "mouseleave" => Ok(EventName::MouseLeave),
            "mouseover" => Ok(EventName::MouseOver),
            "mouseout" => Ok(EventName::MouseOut),
            "wheel" => Ok(EventName::Wheel),
            "pointermove" => Ok(EventName::PointerMove),
            "pointerdown" => Ok(EventName::PointerDown),
            "pointerenter" => Ok(EventName::PointerEnter),
            "pointerleave" => Ok(EventName::PointerLeave),
            "pointerup" => Ok(EventName::PointerUp),
            "keydown" => Ok(EventName::KeyDown),
            "keyup" => Ok(EventName::KeyUp),
            "touchcancel" => Ok(EventName::TouchCancel),
            "touchstart" => Ok(EventName::TouchStart),
            "touchmove" => Ok(EventName::TouchMove),
            "touchend" => Ok(EventName::TouchEnd),
            "globalclick" => Ok(EventName::GlobalClick),
            "globalpointerup" => Ok(EventName::GlobalPointerUp),
            "globalmousedown" => Ok(EventName::GlobalMouseDown),
            "globalmousemove" => Ok(EventName::GlobalMouseMove),
            "filedrop" => Ok(EventName::FileDrop),
            "globalfilehover" => Ok(EventName::GlobalFileHover),
            "globalfilehovercancelled" => Ok(EventName::GlobalFileHoverCancelled),
            _ => Err(()),
        }
    }
}

impl From<EventName> for &str {
    fn from(event: EventName) -> Self {
        match event {
            EventName::Click => "click",
            EventName::MiddleClick => "middleclick",
            EventName::RightClick => "rightclick",
            EventName::MouseDown => "mousedown",
            EventName::MouseMove => "mousemove",
            EventName::MouseEnter => "mouseenter",
            EventName::MouseLeave => "mouseleave",
            EventName::MouseOver => "mouseover",
            EventName::MouseOut => "mouseout",
            EventName::Wheel => "wheel",
            EventName::PointerMove => "pointermove",
            EventName::PointerDown => "pointerdown",
            EventName::PointerEnter => "pointerenter",
            EventName::PointerLeave => "pointerleave",
            EventName::PointerUp => "pointerup",
            EventName::KeyDown => "keydown",
            EventName::KeyUp => "keyup",
            EventName::TouchCancel => "touchcancel",
            EventName::TouchStart => "touchstart",
            EventName::TouchMove => "touchmove",
            EventName::TouchEnd => "touchend",
            EventName::GlobalClick => "globalclick",
            EventName::GlobalPointerUp => "globalpointerup",
            EventName::GlobalMouseDown => "globalmousedown",
            EventName::GlobalMouseMove => "globalmousemove",
            EventName::FileDrop => "filedrop",
            EventName::GlobalFileHover => "globalfilehover",
            EventName::GlobalFileHoverCancelled => "globalfilehovercancelled",
        }
    }
}

impl Eq for EventName {}

impl PartialOrd for EventName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            // Always prioritize leave events before anything else
            Self::MouseLeave | Self::MouseOut | Self::PointerLeave => {
                if self == other {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Less
                }
            }
            _ => std::cmp::Ordering::Greater,
        }
    }
}

impl EventName {
    /// Get the equivalent to a global event
    pub fn get_global_event(&self) -> Option<Self> {
        match self {
            Self::Click => Some(Self::GlobalClick),
            Self::PointerUp => Some(Self::GlobalPointerUp),
            Self::MouseDown => Some(Self::GlobalMouseDown),
            Self::MouseMove => Some(Self::GlobalMouseMove),
            Self::GlobalFileHover => Some(Self::GlobalFileHover),
            Self::GlobalFileHoverCancelled => Some(Self::GlobalFileHoverCancelled),
            _ => None,
        }
    }

    /// Some events might cause other events, like for example:
    /// A `mousemove` might also trigger a `mouseenter`
    /// A `mousedown` or a `touchdown` might also trigger a `pointerdown`
    pub fn get_collateral_events(&self) -> SmallVec<[Self; 4]> {
        let mut events = SmallVec::new();

        events.push(*self);

        match self {
            Self::MouseMove | Self::TouchMove => events.extend([
                Self::MouseEnter,
                Self::MouseOver,
                Self::PointerEnter,
                Self::PointerMove,
            ]),
            Self::MouseDown | Self::TouchStart => events.push(Self::PointerDown),
            Self::Click | Self::MiddleClick | Self::RightClick | Self::TouchEnd => {
                events.push(Self::PointerUp)
            }
            Self::MouseLeave => events.push(Self::PointerLeave),
            Self::GlobalFileHover | Self::GlobalFileHoverCancelled => events.clear(),
            _ => {}
        }

        events
    }

    /// Check if the event means that the pointer (e.g. cursor) just entered a Node area
    pub fn is_enter(&self) -> bool {
        matches!(&self, Self::MouseEnter | Self::PointerEnter)
    }

    /// Check if the event means that the pointer (e.g. cursor) just entered a Node visible area
    pub fn is_over(&self) -> bool {
        matches!(&self, Self::MouseOver)
    }

    /// Check if it's one of the Pointer variants
    pub fn is_pointer(&self) -> bool {
        matches!(
            &self,
            Self::PointerEnter
                | Self::PointerLeave
                | Self::PointerMove
                | Self::PointerDown
                | Self::PointerUp
                | Self::GlobalPointerUp
        )
    }

    /// Check if the event means the cursor was moved
    pub fn was_cursor_moved(&self) -> bool {
        matches!(
            &self,
            Self::MouseMove
                | Self::MouseEnter
                | Self::MouseOver
                | Self::PointerEnter
                | Self::PointerMove
        )
    }

    // Bubble all events except:
    // - Keyboard events
    // - Mouse movements events
    pub fn does_bubble(&self) -> bool {
        !matches!(
            self,
            Self::KeyDown
                | Self::KeyUp
                | Self::MouseEnter
                | Self::PointerEnter
                | Self::MouseLeave
                | Self::MouseOut
                | Self::PointerLeave
        )
    }

    // Only let events that do not move the mouse, go through solid nodes
    pub fn does_go_through_solid(&self) -> bool {
        matches!(self, Self::KeyDown | Self::KeyUp)
    }

    // Check if this event can change the hover state of a Node.
    pub fn can_change_hover_state(&self) -> bool {
        matches!(
            self,
            Self::MouseMove
                | Self::MouseEnter
                | Self::MouseOver
                | Self::PointerMove
                | Self::PointerEnter
        )
    }
}
