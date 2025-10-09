use core::{ffi::c_void, mem::MaybeUninit, pin::Pin};

use ra_fsp_sys::generated::{
    e_timer_compare_match, //
    timer_api_t,
    timer_callback_args_t,
    timer_compare_match_t,
    timer_info_t,
    timer_instance_t,
    timer_status_t,
};

use crate::{Block, Result, fsp_try_unsafe};

pub enum CompareMatchChannel {
    A,
    B,
    /// for concrete timer implementations providing more then A/B
    Other(timer_compare_match_t),
}

pub trait TimerApi {
    /// Start the counter
    fn start(self: Pin<&mut Self>) -> Result<()>;
    /// Stop the counter
    fn stop(self: Pin<&mut Self>) -> Result<()>;
    /// Reset the counter to the initial value
    fn reset(self: Pin<&mut Self>) -> Result<()>;
    /// Enable input capture
    fn enable(self: Pin<&mut Self>) -> Result<()>;
    /// Disable input capture
    fn disable(self: Pin<&mut Self>) -> Result<()>;
    /// Set the time until the timer expires.  See implementation for details of period update timing.
    ///
    /// `period` - Time until timer should expire.
    fn period_set(self: Pin<&mut Self>, period: u32) -> Result<()>;

    /// Sets the number of counts for the pin level to be high.  If the timer is counting, the updated duty cycle is
    /// reflected after the next timer expiration.
    ///
    /// `duty_cycle_counts` - Time until duty cycle should expire.
    /// `pin` - Which output pin to update.  See implementation for details.
    fn duty_cycle_set(self: Pin<&mut Self>, duty_cycle_counts: u32, pin: u32) -> Result<()>;

    /// Set a compare match value in raw counts.
    /// `compare_match_value` - Timer value to trigger a compare match event.
    /// `match_channel` - Which channel to update.
    fn compare_match_set(
        self: Pin<&mut Self>,
        compare_match_value: u32,
        match_channel: CompareMatchChannel,
    ) -> Result<()>;

    /// Retrieve timer information
    fn info_get(self: Pin<&mut Self>) -> Result<timer_info_t>;

    /// Get the current counter value and timer state and store it in status.
    fn status_get(self: Pin<&mut Self>) -> Result<timer_status_t>;

    /// Specify callback function and optional context pointer and working memory pointer.
    ///
    /// `p_callback` - Callback function to register
    /// `p_context` - Pointer to send to callback function
    /// `p_working_memory` - Pointer to volatile memory where callback structure can be allocated.
    ///                      Callback arguments allocated here are only valid during the callback.
    unsafe fn callback_set(
        self: Pin<&mut Self>,
        callback: Option<unsafe extern "C" fn(*mut timer_callback_args_t)>,
        p_context: *mut c_void,
        args_alloc: *mut timer_callback_args_t,
    ) -> Result<()>;

    // Allows driver to be reconfigured and may reduce power consumption.
    // fn close(self);
}

impl<T> TimerApi for T
where
    T: Block<CInstance = timer_instance_t>,
    T: Block<CApi = timer_api_t>,
{
    fn start(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.start.unwrap())(ctrl))
    }

    fn stop(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.stop.unwrap())(ctrl))
    }

    fn reset(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.reset.unwrap())(ctrl))
    }

    fn enable(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.enable.unwrap())(ctrl))
    }

    fn disable(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.disable.unwrap())(ctrl))
    }

    fn period_set(self: Pin<&mut Self>, period: u32) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.periodSet.unwrap())(ctrl, period))
    }

    fn duty_cycle_set(self: Pin<&mut Self>, duty_cycle_counts: u32, pin: u32) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.dutyCycleSet.unwrap())(ctrl, duty_cycle_counts, pin))
    }

    fn compare_match_set(
        self: Pin<&mut Self>,
        compare_match_value: u32,
        match_channel: CompareMatchChannel,
    ) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.compareMatchSet.unwrap())(
            ctrl,
            compare_match_value,
            match_channel.into()
        ))
    }

    fn info_get(self: Pin<&mut Self>) -> Result<timer_info_t> {
        let ctrl = self.instance().p_ctrl;
        let mut res = MaybeUninit::<timer_info_t>::uninit();
        fsp_try_unsafe!((T::API.infoGet.unwrap())(ctrl, res.as_mut_ptr()))?;
        unsafe { Ok(res.assume_init()) }
    }

    fn status_get(self: Pin<&mut Self>) -> Result<timer_status_t> {
        let ctrl = self.instance().p_ctrl;
        let mut res = MaybeUninit::<timer_status_t>::uninit();
        fsp_try_unsafe!((T::API.statusGet.unwrap())(ctrl, res.as_mut_ptr()))?;
        unsafe { Ok(res.assume_init()) }
    }

    unsafe fn callback_set(
        self: Pin<&mut Self>,
        callback: Option<unsafe extern "C" fn(arg: *mut timer_callback_args_t)>,
        p_context: *mut c_void,
        args_alloc: *mut timer_callback_args_t,
    ) -> Result<()> {
        let ctrl = self.instance().p_ctrl;
        fsp_try_unsafe!((T::API.callbackSet.unwrap())(
            ctrl, callback, p_context, args_alloc
        ))
    }

    /*
    fn close(self: Pin<&mut Self>) {
        fsp_try_unsafe! ((T::API.close.unwrap())(self.instance()))
            .expect("Error closing timer");
    }
    */
}

impl From<CompareMatchChannel> for timer_compare_match_t {
    fn from(value: CompareMatchChannel) -> Self {
        match value {
            CompareMatchChannel::A => e_timer_compare_match::TIMER_COMPARE_MATCH_A,
            CompareMatchChannel::B => e_timer_compare_match::TIMER_COMPARE_MATCH_B,
            CompareMatchChannel::Other(v) => v,
        }
    }
}
