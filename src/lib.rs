use std::cell::RefCell;
use leptos_reactive::{on_cleanup, Scope, store_value, StoredValue};
use web_sys::{AbortController, AbortSignal};

#[derive(Copy, Clone)]
pub struct Abort {
    value: StoredValue<AbortValue>
}

struct AbortValue {
    controller: RefCell<Option<AbortController>>,
}

pub fn create_abort(cx: Scope) -> Abort {
    let abort_value = store_value(cx, AbortValue {
        controller: RefCell::new(None)
    });

    let abort = Abort { value: abort_value  };

    on_cleanup(cx, move || abort.clone().abort());

    abort
}

impl Abort {
    /// Creates a new AbortSignal
    /// will abort the previous signal returned from this call
    pub fn signal(&self) -> AbortSignal {
        self.value.with_value(|a| {
            // Abort the previous signal
            self.abort();

            // Create a new controller
            let new_controller = AbortController::new().expect("AbortController to be available");
            let signal = new_controller.signal();
            a.controller.replace(Some(new_controller));
            signal
        })
    }

    /// Aborts the last given signal
    pub fn abort(&self) {
        self.value.with_value(|a| {
            if let Some(ref abort_controller) = *a.controller.borrow_mut() {
                abort_controller.abort();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use leptos_reactive::{create_runtime, create_scope};
    use super::*;

    use wasm_bindgen_test::*;


    #[wasm_bindgen_test]
    fn test_abort() {
        create_scope(create_runtime(), |cx| {
            let abort = create_abort(cx);

            let signal1 = abort.signal();
            assert_eq!(signal1.aborted(), false);

            // signal1 should be aborted because a new one was created
            let signal2 = abort.signal();
            assert_eq!(signal1.aborted(), true);
            assert_eq!(signal2.aborted(), false);

            // signal2 should be aborted because of an explicit .abort() call
            abort.abort();
            assert_eq!(signal2.aborted(), true);

            // signal3 should be aborted because the scope gets disposed
            let signal3 = abort.signal();
            assert_eq!(signal3.aborted(), false);
            cx.dispose();
            assert_eq!(signal3.aborted(), true);
        }).dispose();
    }
}
