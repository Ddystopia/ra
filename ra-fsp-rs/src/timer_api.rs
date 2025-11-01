use core::{mem::MaybeUninit, pin::Pin};

use ra_fsp_sys::generated::{
    // -
    timer_api_t,
    timer_cfg_t,
    timer_compare_match_t,
    timer_info_t,
    timer_instance_t,
    timer_mode_t,
    timer_source_div_t,
    timer_status_t,
};

use crate::{Block, Irq, Result, fsp_try_unsafe, state_markers};

pub struct TimerConf<Extended> {
    pub mode: timer_mode_t,
    pub period_counts: u32,
    pub source_div: timer_source_div_t,
    pub duty_cycle_counts: u32,
    pub channel: u8,
    pub cycle_end: Option<Irq>,
    pub extend: Extended,
}

impl<Extended> TimerConf<Extended> {
    pub const fn c_conf(&mut self) -> timer_cfg_t {
        timer_cfg_t {
            mode: self.mode,
            period_counts: self.period_counts,
            source_div: self.source_div,
            duty_cycle_counts: self.duty_cycle_counts,
            channel: self.channel,
            cycle_end_irq: Irq::extract_irq(self.cycle_end),
            cycle_end_ipl: Irq::extract_ipl(self.cycle_end),
            p_callback: None,
            p_context: core::ptr::null_mut(),
            p_extend: core::ptr::null(),
        }
    }
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
        match_channel: timer_compare_match_t,
    ) -> Result<()>;

    /// Retrieve timer information
    fn info_get(self: Pin<&mut Self>) -> Result<timer_info_t>;

    /// Get the current counter value and timer state and store it in status.
    fn status_get(self: Pin<&mut Self>) -> Result<timer_status_t>;

    // Allows driver to be reconfigured and may reduce power consumption.
    // fn close(self);
}

impl<T> TimerApi for T
where
    T: Block<Instance = timer_instance_t>,
    T: Block<Api = timer_api_t>,
    T: Block<State = state_markers::Opened>,
{
    fn start(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.start.unwrap())(ctrl))
    }

    fn stop(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.stop.unwrap())(ctrl))
    }

    fn reset(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.reset.unwrap())(ctrl))
    }

    fn enable(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.enable.unwrap())(ctrl))
    }

    fn disable(self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.disable.unwrap())(ctrl))
    }

    fn period_set(self: Pin<&mut Self>, period: u32) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.periodSet.unwrap())(ctrl, period))
    }

    fn duty_cycle_set(self: Pin<&mut Self>, duty_cycle_counts: u32, pin: u32) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.dutyCycleSet.unwrap())(ctrl, duty_cycle_counts, pin))
    }

    fn compare_match_set(
        self: Pin<&mut Self>,
        compare_match_value: u32,
        match_channel: timer_compare_match_t,
    ) -> Result<()> {
        let ctrl = self.ctrl();
        fsp_try_unsafe!((T::API.compareMatchSet.unwrap())(
            ctrl,
            compare_match_value,
            match_channel,
        ))
    }

    fn info_get(self: Pin<&mut Self>) -> Result<timer_info_t> {
        let ctrl = self.ctrl();
        let mut res = MaybeUninit::<timer_info_t>::uninit();
        fsp_try_unsafe!((T::API.infoGet.unwrap())(ctrl, res.as_mut_ptr()))?;
        unsafe { Ok(res.assume_init()) }
    }

    fn status_get(self: Pin<&mut Self>) -> Result<timer_status_t> {
        let ctrl = self.ctrl();
        let mut res = MaybeUninit::<timer_status_t>::uninit();
        fsp_try_unsafe!((T::API.statusGet.unwrap())(ctrl, res.as_mut_ptr()))?;
        unsafe { Ok(res.assume_init()) }
    }

    /*
    fn close(self: Pin<&mut Self>) {
        fsp_try_unsafe! ((T::API.close.unwrap())(self.instance()))
            .expect("Error closing timer");
    }
    */
}
