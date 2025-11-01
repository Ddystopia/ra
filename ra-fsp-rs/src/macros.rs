#[macro_export]
macro_rules! event_link_select {
    // Case 1: Full form with attributes, visibility, name, and type
    (
        $(#[$meta:meta])*
        $vis:vis static $name:ident : [ $ty:ty ; _ ] = {
            $($val:expr => $idx:pat),* $(,)?
        }$(;)?
    ) => {
        $(#[$meta])*
        #[allow(non_upper_case_globals)]
        #[unsafe(no_mangle)]
        $vis static $name: [$ty; $crate::ra_fsp_sys::generated::BSP_ICU_VECTOR_MAX_ENTRIES as usize] = {
            let mut select = [$crate::e_elc_event::ELC_EVENT_NONE; $crate::ra_fsp_sys::generated::BSP_ICU_VECTOR_MAX_ENTRIES as usize];

            let mut i = 0;
            while i < select.len() {
                let int = $crate::pac::Interrupt::try_from_u16(i as u16).unwrap();
                match int {
                    $($idx => select[i] = $val,)*
                    _ => (),
                }
                i += 1;
            }

            select
        };
    };

    // Case 2: Shorthand form — just mapping
    (
        $($val:expr => $idx:pat),* $(,)?
    ) => {
        $crate::event_link_select! {
            pub static g_interrupt_event_link_select: [$crate::ra_fsp_sys::generated::e_elc_event; _] = {
                $($val => $idx),*
            };
        }
    };
}

#[macro_export]
macro_rules! fsp_try_unsafe {
    ($expr:expr) => {{
        #[allow(unused_unsafe)]
        match unsafe { $expr } {
            ::ra_fsp_sys::generated::e_fsp_err::FSP_SUCCESS => Ok(()),
            err_code => Err(err_code),
        }
    }};
}
