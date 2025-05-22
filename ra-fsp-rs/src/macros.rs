#[macro_export]
macro_rules! const_c_dyn {
    (
        $p:path,
        $cfg:expr
    ) => {
        const {
            use $p as srcpath;
            static CTRL: srcpath::for_c_dyn_macro_Instance =
                srcpath::for_c_dyn_macro_Instance::new();
            static CONF: srcpath::for_c_dyn_macro_Config = $cfg.c_conf();

            // SAFETY: CTRL is private to that scope, thus no other mutable
            //         references will be used and will not cause data races
            let static_mut = unsafe { &mut *CTRL.ptr().cast() };

            srcpath::c_dyn(::core::pin::Pin::static_mut(static_mut), &CONF)
        }
    };
}

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
