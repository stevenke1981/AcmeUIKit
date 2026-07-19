use gpui::{AnyElement, App};

/// Standard async operation states following UI_DESIGN_PRINCIPLES.md §6.2.
///
/// # Example
/// ```ignore
/// let mut state: LoadingState<String> = LoadingState::Idle;
///
/// // Start loading
/// state = LoadingState::Loading;
/// assert!(state.is_loading());
///
/// // Populate with a result
/// state = LoadingState::Success("Hello".into());
/// assert_eq!(state.success_value(), Some(&"Hello".to_string()));
///
/// // Or capture an error
/// state = LoadingState::Error("network failure".into());
/// assert_eq!(state.error_message(), Some("network failure"));
/// ```
#[derive(Debug, Clone, Default)]
pub enum LoadingState<T> {
    /// No operation has been initiated.
    #[default]
    Idle,
    /// An operation is in progress.
    Loading,
    /// The operation completed successfully.
    Success(T),
    /// The operation failed.
    Error(String),
}

impl<T> LoadingState<T> {
    /// Returns `true` when the state is [`Idle`](LoadingState::Idle).
    #[inline]
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns `true` when the state is [`Loading`](LoadingState::Loading).
    #[inline]
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    /// Returns `true` when the state is [`Success`](LoadingState::Success).
    #[inline]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }

    /// Returns `true` when the state is [`Error`](LoadingState::Error).
    #[inline]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// Returns a reference to the success value, or `None` if not in the success state.
    #[inline]
    pub fn success_value(&self) -> Option<&T> {
        match self {
            Self::Success(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the error message string, or `None` if not in the error state.
    #[inline]
    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error(msg) => Some(msg.as_str()),
            _ => None,
        }
    }
}

/// Convenience function that renders UI for each [`LoadingState`] variant.
///
/// Exactly one callback is executed based on the current state. Each callback
/// receives a `&mut App` for access to the theme and other context, and must
/// return an [`AnyElement`].
///
/// # Example
/// ```ignore
/// let element = render_loading_state(
///     &state,
///     cx,
///     |cx| div().child("Click to start").into_any_element(),
///     |cx| div().child(Skeleton::line()).into_any_element(),
///     |data, cx| div().child(format!("Result: {}", data)).into_any_element(),
///     |msg, cx| div().child(format!("Error: {}", msg)).into_any_element(),
/// );
/// ```
pub fn render_loading_state<T, IdleFn, LoadingFn, SuccessFn, ErrorFn>(
    state: &LoadingState<T>,
    cx: &mut App,
    idle_fn: IdleFn,
    loading_fn: LoadingFn,
    success_fn: SuccessFn,
    error_fn: ErrorFn,
) -> AnyElement
where
    IdleFn: FnOnce(&mut App) -> AnyElement,
    LoadingFn: FnOnce(&mut App) -> AnyElement,
    SuccessFn: FnOnce(&T, &mut App) -> AnyElement,
    ErrorFn: FnOnce(&str, &mut App) -> AnyElement,
{
    match state {
        LoadingState::Idle => idle_fn(cx),
        LoadingState::Loading => loading_fn(cx),
        LoadingState::Success(value) => success_fn(value, cx),
        LoadingState::Error(msg) => error_fn(msg, cx),
    }
}
